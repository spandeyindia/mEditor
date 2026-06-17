use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::{Read, Write};
use std::path::Component;
use std::path::{Path, PathBuf};
use std::process::Child;
use std::process::ChildStdin;
use std::process::Command;
use std::process::Stdio;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::WindowBuilder;
use wry::WebView;
use wry::WebViewBuilder;
use xml::name::OwnedName;
use xml::reader::EventReader;
use xml::reader::XmlEvent;

#[derive(Clone, Debug)]
enum HostEvent {
    Ipc(String),
}

struct TerminalSession {
    id: String,
    label: String,
    command_line: String,
    pty_backed: bool,
    child: Arc<Mutex<Child>>,
    stdin: Arc<Mutex<ChildStdin>>,
    output: Arc<Mutex<String>>,
    started_at_epoch: u64,
}

static TERMINAL_SESSIONS: OnceLock<Mutex<BTreeMap<String, TerminalSession>>> = OnceLock::new();

fn terminal_sessions() -> &'static Mutex<BTreeMap<String, TerminalSession>> {
    TERMINAL_SESSIONS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn main() -> wry::Result<()> {
    let workspace_root = startup_workspace_root();
    let event_loop = EventLoopBuilder::<HostEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window_title = format!("mEditor {}", meditor_core::CURRENT_BASELINE_VERSION);
    let window = WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(default_window_size())
        .with_min_inner_size(LogicalSize::new(900.0, 640.0))
        .build(&event_loop)
        .expect("mEditor GUI window should be created");

    let html = build_gui_html(&workspace_root);
    let webview = WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |request| {
            let _ = proxy.send_event(HostEvent::Ipc(request.body().clone()));
        })
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        let _keep_window_alive = &window;
        let _keep_webview_alive = &webview;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(HostEvent::Ipc(payload)) => {
                respond_to_ipc(&webview, &workspace_root, &payload);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn default_window_size() -> LogicalSize<f64> {
    let width = configured_window_dimension("mEditor_WINDOW_WIDTH", 1240.0, 900.0, 1280.0);
    let height = configured_window_dimension("mEditor_WINDOW_HEIGHT", 820.0, 640.0, 880.0);
    LogicalSize::new(width, height)
}

fn configured_window_dimension(name: &str, default: f64, min: f64, max: f64) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn startup_workspace_root() -> PathBuf {
    choose_startup_workspace_root(
        std::env::var_os("mEditor_WORKSPACE").map(PathBuf::from),
        std::env::current_dir().ok(),
        std::env::current_exe().ok(),
    )
}

fn choose_startup_workspace_root(
    configured_workspace: Option<PathBuf>,
    current_dir: Option<PathBuf>,
    current_exe: Option<PathBuf>,
) -> PathBuf {
    for candidate in configured_workspace
        .into_iter()
        .chain(current_dir)
        .chain(current_exe.as_deref().and_then(package_root_from_exe))
    {
        if workspace_candidate_is_writable(&candidate) {
            return candidate;
        }
    }

    let fallback = user_home_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("mEditor-workspace");
    let _ = fs::create_dir_all(&fallback);
    fallback
}

fn package_root_from_exe(executable_path: &Path) -> Option<PathBuf> {
    let executable_dir = executable_path.parent()?;
    if executable_dir.file_name().and_then(|name| name.to_str()) == Some("MacOS") {
        let contents_dir = executable_dir.parent()?;
        if contents_dir.file_name().and_then(|name| name.to_str()) == Some("Contents") {
            let app_dir = contents_dir.parent()?;
            if app_dir.extension().and_then(|extension| extension.to_str()) == Some("app") {
                return app_dir.parent().map(Path::to_path_buf);
            }
        }
    }
    Some(executable_dir.to_path_buf())
}

fn workspace_candidate_is_writable(path: &Path) -> bool {
    if root_or_app_bundle_path(path) {
        return false;
    }
    let state_dir = path.join(".meditor");
    let probe = state_dir.join(".startup-write-test");
    fs::create_dir_all(&state_dir)
        .and_then(|_| fs::write(&probe, b"mEditor startup workspace probe"))
        .and_then(|_| fs::remove_file(&probe))
        .is_ok()
}

fn root_or_app_bundle_path(path: &Path) -> bool {
    let normal_components = path
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count();
    if path.is_absolute() && normal_components == 0 {
        return true;
    }
    path.components().any(|component| {
        matches!(component, Component::Normal(value) if value.to_string_lossy().ends_with(".app"))
    })
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn respond_to_ipc(webview: &WebView, workspace_root: &Path, payload: &str) {
    let response = handle_ipc(workspace_root, payload);
    let script = format!("mEditorHostResponse({response});");
    if let Err(error) = webview.evaluate_script(&script) {
        eprintln!("mEditor GUI could not send host response to webview: {error}");
    }
}

fn handle_ipc(workspace_root: &Path, payload: &str) -> serde_json::Value {
    let request = match serde_json::from_str::<serde_json::Value>(payload) {
        Ok(request) => request,
        Err(error) => {
            return json_error("unknown", format!("Invalid GUI request: {error}"));
        }
    };

    let action = request
        .get("action")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown");

    match action {
        "openFile" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("openFileResult", "Missing file path");
            };
            match read_workspace_file(workspace_root, path) {
                Ok(content) => serde_json::json!({
                    "action": "openFileResult",
                    "ok": true,
                    "path": path,
                    "content": content,
                }),
                Err(error) => json_error("openFileResult", error),
            }
        }
        "openFileForVi" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("openFileForViResult", "Missing file path");
            };
            match read_workspace_file(workspace_root, path) {
                Ok(content) => serde_json::json!({
                    "action": "openFileForViResult",
                    "ok": true,
                    "path": path,
                    "content": content,
                }),
                Err(error) => json_error("openFileForViResult", error),
            }
        }
        "openFileForFormatter" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("openFileForFormatterResult", "Missing file path");
            };
            match read_workspace_file(workspace_root, path) {
                Ok(content) => serde_json::json!({
                    "action": "openFileForFormatterResult",
                    "ok": true,
                    "path": path,
                    "content": content,
                }),
                Err(error) => json_error("openFileForFormatterResult", error),
            }
        }
        "saveFile" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("saveFileResult", "Missing file path");
            };
            let Some(content) = request.get("content").and_then(|value| value.as_str()) else {
                return json_error("saveFileResult", "Missing editor content");
            };
            match write_workspace_file(workspace_root, path, content) {
                Ok(bytes) => serde_json::json!({
                    "action": "saveFileResult",
                    "ok": true,
                    "path": path,
                    "bytes": bytes,
                }),
                Err(error) => json_error("saveFileResult", error),
            }
        }
        "scanFile" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("scanFileResult", "Missing file path");
            };
            let content = match request.get("content").and_then(|value| value.as_str()) {
                Some(content) => Ok(content.to_string()),
                None => read_workspace_file(workspace_root, path),
            };
            match content {
                Ok(content) => {
                    let Ok(full_path) = resolve_workspace_path(workspace_root, path) else {
                        return json_error("scanFileResult", "File path is outside the workspace");
                    };
                    let findings = meditor_code_security::CodeSecurityScanner::with_seed_rules()
                        .scan_text(&full_path, &content)
                        .into_iter()
                        .map(|finding| {
                            format!(
                                "{} line {} [{} CVSS {} {}]: {} Remediation: {}",
                                finding.rule_id,
                                finding.line_number,
                                finding.severity,
                                finding.cvss_score,
                                finding.cwe,
                                finding.summary,
                                finding.remediation
                            )
                        })
                        .collect::<Vec<_>>();
                    let findings = if findings.is_empty() {
                        vec!["No findings".to_string()]
                    } else {
                        findings
                    };
                    let valid = findings.len() == 1 && findings[0] == "No findings";
                    serde_json::json!({
                        "action": "scanFileResult",
                        "ok": true,
                        "path": path,
                        "valid": valid,
                        "findings": findings,
                    })
                }
                Err(error) => json_error("scanFileResult", error),
            }
        }
        "validateFile" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("validateFileResult", "Missing file path");
            };
            let kind = request
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("json");
            let content = request
                .get("content")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let realtime = request
                .get("realtime")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            let (valid, messages) = validate_editor_text(kind, content);
            serde_json::json!({
                "action": "validateFileResult",
                "ok": true,
                "path": path,
                "kind": kind,
                "realtime": realtime,
                "valid": valid,
                "messages": messages,
            })
        }
        "reformatFile" => {
            let Some(path) = request.get("path").and_then(|value| value.as_str()) else {
                return json_error("reformatFileResult", "Missing file path");
            };
            let kind = request
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("json");
            let content = request
                .get("content")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            match reformat_editor_text(kind, content) {
                Ok(formatted) => serde_json::json!({
                    "action": "reformatFileResult",
                    "ok": true,
                    "path": path,
                    "kind": kind,
                    "content": formatted,
                    "messages": [
                        format!("{} reformat completed in memory.", kind.to_uppercase()),
                        "Review the formatted text, then press Save to write the file.".to_string()
                    ],
                }),
                Err(error) => json_error("reformatFileResult", error),
            }
        }
        "formatText" => {
            let target = request
                .get("target")
                .and_then(|value| value.as_str())
                .unwrap_or("formatter");
            let kind = request
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("json");
            let content = request
                .get("content")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            if content.trim().is_empty() {
                return serde_json::json!({
                    "action": "formatTextResult",
                    "ok": true,
                    "target": target,
                    "kind": kind,
                    "valid": false,
                    "content": "",
                    "messages": ["Paste, type, or upload JSON/XML text to see a live formatted result."],
                });
            }

            let (valid, mut messages) = validate_editor_text(kind, content);
            if valid {
                match reformat_editor_text(kind, content) {
                    Ok(formatted) => {
                        messages.push(format!("{} formatted successfully.", kind.to_uppercase()));
                        serde_json::json!({
                            "action": "formatTextResult",
                            "ok": true,
                            "target": target,
                            "kind": kind,
                            "valid": true,
                            "content": formatted,
                            "messages": messages,
                        })
                    }
                    Err(error) => serde_json::json!({
                        "action": "formatTextResult",
                        "ok": true,
                        "target": target,
                        "kind": kind,
                        "valid": false,
                        "content": "",
                        "messages": [error],
                    }),
                }
            } else {
                serde_json::json!({
                    "action": "formatTextResult",
                    "ok": true,
                    "target": target,
                    "kind": kind,
                    "valid": false,
                    "content": "",
                    "messages": messages,
                })
            }
        }
        "detectProject" => {
            let project_path = request
                .get("projectPath")
                .and_then(|value| value.as_str())
                .unwrap_or(".");
            match detect_project(workspace_root, project_path) {
                Ok(project) => serde_json::json!({
                    "action": "detectProjectResult",
                    "ok": true,
                    "projectPath": project.relative_path,
                    "projectKind": project.kind,
                    "messages": project.messages,
                    "tasks": project.tasks.iter().map(task_profile_json).collect::<Vec<_>>(),
                }),
                Err(error) => json_error("detectProjectResult", error),
            }
        }
        "runTask" => {
            let project_path = request
                .get("projectPath")
                .and_then(|value| value.as_str())
                .unwrap_or(".");
            let task_action = request
                .get("taskAction")
                .and_then(|value| value.as_str())
                .unwrap_or("build");
            let source_path = request
                .get("sourcePath")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let timeout_seconds = request
                .get("timeoutSeconds")
                .and_then(|value| value.as_u64())
                .unwrap_or(60)
                .clamp(5, 600);
            match run_sdlc_task(
                workspace_root,
                project_path,
                task_action,
                source_path,
                timeout_seconds,
            ) {
                Ok(result) => serde_json::json!({
                    "action": "runTaskResult",
                    "ok": true,
                    "projectPath": result.project_path,
                    "projectKind": result.project_kind,
                    "taskAction": result.task_action,
                    "taskLabel": result.task_label,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "timedOut": result.timed_out,
                    "durationMs": result.duration_ms,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                    "diagnostics": result.diagnostics,
                    "suggestions": result.suggestions,
                    "reportPath": result.report_path,
                    "fileTreeHtml": build_file_tree_html(workspace_root),
                }),
                Err(error) => json_error("runTaskResult", error),
            }
        }
        "refreshExplorer" => serde_json::json!({
            "action": "refreshExplorerResult",
            "ok": true,
            "html": build_file_tree_html(workspace_root),
        }),
        "createProject" => {
            let name = request
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let kind = request
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("major");
            let tech_stack = request
                .get("techStack")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            match create_project_workspace(workspace_root, name, kind, tech_stack) {
                Ok(messages) => {
                    let mut result = generic_action_result("project.create", "Project Created", messages);
                    if let Some(object) = result.as_object_mut() {
                        object.insert(
                            "fileTreeHtml".to_string(),
                            serde_json::Value::String(build_file_tree_html(workspace_root)),
                        );
                    }
                    result
                }
                Err(error) => generic_action_error("project.create", "Project Create", error),
            }
        }
        "importProject" => match write_project_import_report(workspace_root) {
            Ok(messages) => {
                let mut result =
                    generic_action_result("file.importProject.open", "Project Import Report", messages);
                if let Some(object) = result.as_object_mut() {
                    object.insert(
                        "fileTreeHtml".to_string(),
                        serde_json::Value::String(build_file_tree_html(workspace_root)),
                    );
                }
                result
            }
            Err(error) => generic_action_error("file.importProject.open", "Project Import", error),
        },
        "moduleOpened" => {
            let command = request
                .get("command")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown");
            let label = request
                .get("label")
                .and_then(|value| value.as_str())
                .unwrap_or(command);
            match run_module_open_workflow(workspace_root, command, label) {
                Ok(messages) => generic_action_result(command, format!("{label} Backend"), messages),
                Err(error) => generic_action_error(command, format!("{label} Backend"), error),
            }
        }
        "dbaRunSql" => {
            let sql = request
                .get("sql")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            if sql.trim().is_empty() {
                generic_action_error("tools.dbaWorkshop.open", "DBA Worksheet", "SQL text is empty")
            } else {
                generic_action_result(
                    "tools.dbaWorkshop.open",
                    "DBA Worksheet",
                    vec![
                        "SQL text accepted by the Rust worksheet backend.".to_string(),
                        format!("Statement length: {} bytes", sql.len()),
                        format!("Worksheet actions available: {}", meditor_db_workbench::worksheet_actions().len()),
                        "No database connection is selected yet, so execution is held as a dry run.".to_string(),
                    ],
                )
            }
        }
        "dbaExplainPlan" => {
            let sql = request
                .get("sql")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            generic_action_result(
                "tools.dbaWorkshop.open",
                "Explain Plan",
                vec![
                    "Explain Plan request reached the Rust DBA backend.".to_string(),
                    format!("SQL preview: {}", sql.trim().chars().take(120).collect::<String>()),
                    "A live plan will require an authenticated database connection.".to_string(),
                ],
            )
        }
        "detectToolchains" => generic_action_result(
            "setup.languageSupport.open",
            "Toolchain Detection",
            detect_toolchain_messages(),
        ),
        "verifyDependencies" => serde_json::json!({
            "action": "dependencyVerifyResult",
            "ok": true,
            "records": dependency_verification_records(),
        }),
        "installDependency" => {
            let dependency_id = request
                .get("dependencyId")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match install_dependency(dependency_id) {
                Ok(result) => serde_json::json!({
                    "action": "dependencyInstallResult",
                    "ok": true,
                    "dependencyId": dependency_id,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("dependencyInstallResult", error),
            }
        }
        "setupSqlitePreview" => generic_action_result(
            "setup.languageSupport.open",
            "SQLite Setup Preview",
            sqlite_setup_preview_messages(workspace_root),
        ),
        "createSqliteFolders" => match create_sqlite_security_folder(workspace_root) {
            Ok(messages) => generic_action_result("setup.languageSupport.open", "SQLite Setup", messages),
            Err(error) => generic_action_error("setup.languageSupport.open", "SQLite Setup", error),
        },
        "oneClickSetup" => match run_one_click_setup(workspace_root) {
            Ok(messages) => generic_action_result("setup.languageSupport.open", "One-click Setup", messages),
            Err(error) => generic_action_error("setup.languageSupport.open", "One-click Setup", error),
        },
        "projectImportPreview" => generic_action_result(
            "file.importProject.open",
            "Project Import Preview",
            project_import_preview_messages(workspace_root),
        ),
        "vcsCommand" => {
            let provider = request
                .get("provider")
                .and_then(|value| value.as_str())
                .unwrap_or("git");
            let operation = request
                .get("operation")
                .and_then(|value| value.as_str())
                .unwrap_or("status");
            let project_path = request
                .get("projectPath")
                .and_then(|value| value.as_str())
                .unwrap_or(".");
            match run_vcs_command(workspace_root, provider, operation, project_path) {
                Ok(result) => serde_json::json!({
                    "action": "vcsCommandResult",
                    "ok": true,
                    "provider": provider,
                    "operation": operation,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("vcsCommandResult", error),
            }
        }
        "saveDbConnection" => {
            let name = request.get("name").and_then(|value| value.as_str()).unwrap_or("");
            let kind = request.get("kind").and_then(|value| value.as_str()).unwrap_or("sqlite");
            let target = request.get("target").and_then(|value| value.as_str()).unwrap_or("");
            match save_named_record(
                workspace_root,
                ".meditor/connections/db-connections.json",
                serde_json::json!({
                    "id": record_id("db"),
                    "name": name,
                    "kind": kind,
                    "target": target,
                    "created_at_epoch": now_epoch_seconds(),
                    "secret_storage": "passwords are not stored in this pre-pilot profile",
                }),
            ) {
                Ok(messages) => generic_action_result("tools.dbaWorkshop.open", "DB Connection Saved", messages),
                Err(error) => generic_action_error("tools.dbaWorkshop.open", "DB Connection Save", error),
            }
        }
        "runSqliteSql" => {
            let db_path = request.get("dbPath").and_then(|value| value.as_str()).unwrap_or("");
            let sql = request.get("sql").and_then(|value| value.as_str()).unwrap_or("");
            match run_sqlite_sql(workspace_root, db_path, sql) {
                Ok(result) => serde_json::json!({
                    "action": "sqliteSqlResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("sqliteSqlResult", error),
            }
        }
        "addJdbcDriver" => {
            let jar_path = request.get("jarPath").and_then(|value| value.as_str()).unwrap_or("");
            match save_named_record(
                workspace_root,
                ".meditor/connections/jdbc-drivers.json",
                serde_json::json!({
                    "id": record_id("jdbc"),
                    "jar_path": jar_path,
                    "created_at_epoch": now_epoch_seconds(),
                    "status": "registered",
                }),
            ) {
                Ok(messages) => generic_action_result("tools.dbaWorkshop.open", "JDBC Driver Registered", messages),
                Err(error) => generic_action_error("tools.dbaWorkshop.open", "JDBC Driver Register", error),
            }
        }
        "saveSshProfile" => {
            let name = request.get("name").and_then(|value| value.as_str()).unwrap_or("");
            let group = request.get("group").and_then(|value| value.as_str()).unwrap_or("Default");
            let host = request.get("host").and_then(|value| value.as_str()).unwrap_or("");
            let user = request.get("user").and_then(|value| value.as_str()).unwrap_or("");
            match save_named_record(
                workspace_root,
                ".meditor/connections/ssh-profiles.json",
                serde_json::json!({
                    "id": record_id("ssh"),
                    "name": name,
                    "group": group,
                    "host": host,
                    "user": user,
                    "created_at_epoch": now_epoch_seconds(),
                    "secret_storage": "keys/passwords are not stored in this pre-pilot profile",
                }),
            ) {
                Ok(messages) => generic_action_result("tools.sshTerminus.open", "SSH Profile Saved", messages),
                Err(error) => generic_action_error("tools.sshTerminus.open", "SSH Profile Save", error),
            }
        }
        "testSshConnection" => {
            let host = request.get("host").and_then(|value| value.as_str()).unwrap_or("");
            let user = request.get("user").and_then(|value| value.as_str()).unwrap_or("");
            match test_ssh_connection(host, user) {
                Ok(result) => serde_json::json!({
                    "action": "sshTestResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("sshTestResult", error),
            }
        }
        "sftpList" => {
            let target = request.get("target").and_then(|value| value.as_str()).unwrap_or("");
            match run_sftp_list(target) {
                Ok(result) => serde_json::json!({
                    "action": "sftpListResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("sftpListResult", error),
            }
        }
        "saveLocalBug" => {
            let title = request.get("title").and_then(|value| value.as_str()).unwrap_or("");
            let severity = request.get("severity").and_then(|value| value.as_str()).unwrap_or("Medium");
            let details = request.get("details").and_then(|value| value.as_str()).unwrap_or("");
            match save_named_record(
                workspace_root,
                ".meditor/local-bugs/bugs.json",
                serde_json::json!({
                    "id": record_id("bug"),
                    "title": title,
                    "severity": severity,
                    "details": details,
                    "status": "Open",
                    "created_at_epoch": now_epoch_seconds(),
                    "local_only": true,
                }),
            ) {
                Ok(messages) => generic_action_result("report.center.open", "Local Bug Saved", messages),
                Err(error) => generic_action_error("report.center.open", "Local Bug Save", error),
            }
        }
        "listLocalBugs" => match read_named_records(workspace_root, ".meditor/local-bugs/bugs.json") {
            Ok(records) => serde_json::json!({"action":"localBugsResult","ok":true,"records":records}),
            Err(error) => json_error("localBugsResult", error),
        },
        "listReports" => match list_local_reports(workspace_root) {
            Ok(records) => serde_json::json!({"action":"reportListResult","ok":true,"records":records}),
            Err(error) => json_error("reportListResult", error),
        },
        "savePlannerItem" => {
            let title = request.get("title").and_then(|value| value.as_str()).unwrap_or("");
            let model = request.get("model").and_then(|value| value.as_str()).unwrap_or("objective");
            let status = request.get("status").and_then(|value| value.as_str()).unwrap_or("Planned");
            match save_named_record(
                workspace_root,
                ".meditor/planner/items.json",
                serde_json::json!({
                    "id": record_id("plan"),
                    "title": title,
                    "model": model,
                    "status": status,
                    "created_at_epoch": now_epoch_seconds(),
                }),
            ) {
                Ok(messages) => generic_action_result("tools.projectPlanner.open", "Planner Item Saved", messages),
                Err(error) => generic_action_error("tools.projectPlanner.open", "Planner Item Save", error),
            }
        }
        "saveAiKnowledge" => {
            let title = request.get("title").and_then(|value| value.as_str()).unwrap_or("knowledge");
            let content = request.get("content").and_then(|value| value.as_str()).unwrap_or("");
            match save_ai_knowledge(workspace_root, title, content) {
                Ok(messages) => generic_action_result("tools.aimlAssistant.open", "Knowledge Saved", messages),
                Err(error) => generic_action_error("tools.aimlAssistant.open", "Knowledge Save", error),
            }
        }
        "createExtensionSkeleton" => match create_extension_skeleton(workspace_root) {
            Ok(messages) => generic_action_result("tools.extensionSdk.open", "Extension Skeleton", messages),
            Err(error) => generic_action_error("tools.extensionSdk.open", "Extension Skeleton", error),
        },
        "scanWorkspaceIndex" => match scan_workspace_index(workspace_root) {
            Ok(messages) => generic_action_result("tools.workspaceIndexer.open", "Workspace Index", messages),
            Err(error) => generic_action_error("tools.workspaceIndexer.open", "Workspace Index", error),
        },
        "submitFeedback" => {
            let feedback_type = request
                .get("feedbackType")
                .and_then(|value| value.as_str())
                .unwrap_or("BUG");
            let title = request.get("title").and_then(|value| value.as_str()).unwrap_or("");
            let details = request.get("details").and_then(|value| value.as_str()).unwrap_or("");
            match save_feedback_outbox(workspace_root, feedback_type, title, details) {
                Ok(messages) => generic_action_result("feedback.open", "Feedback Saved", messages),
                Err(error) => generic_action_error("feedback.open", "Feedback Save", error),
            }
        }
        "licenseStatus" => match license_acceptance_status(workspace_root) {
            Ok(status) => serde_json::json!({
                "action": "licenseStatusResult",
                "ok": true,
                "accepted": status.accepted,
                "registered": status.registered,
                "displayName": status.display_name,
                "acceptedAtEpoch": status.accepted_at_epoch,
            }),
            Err(error) => json_error("licenseStatusResult", error),
        },
        "acceptEula" => {
            let registered = request
                .get("registered")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            let name = request.get("name").and_then(|value| value.as_str()).unwrap_or("");
            let email = request
                .get("email")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match accept_eula(workspace_root, registered, name, email) {
                Ok(status) => serde_json::json!({
                    "action": "eulaAcceptanceResult",
                    "ok": true,
                    "accepted": status.accepted,
                    "registered": status.registered,
                    "displayName": status.display_name,
                    "acceptedAtEpoch": status.accepted_at_epoch,
                }),
                Err(error) => json_error("eulaAcceptanceResult", error),
            }
        }
        "credentialStoreStatus" => generic_action_result(
            "settings.secretsCredentials.open",
            "Credential Store",
            credential_store_status_messages(),
        ),
        "storeCredential" => {
            let service = request
                .get("service")
                .and_then(|value| value.as_str())
                .unwrap_or("mEditor");
            let account = request
                .get("account")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let secret = request.get("secret").and_then(|value| value.as_str()).unwrap_or("");
            match store_credential(workspace_root, service, account, secret) {
                Ok(messages) => generic_action_result(
                    "settings.secretsCredentials.open",
                    "Credential Stored",
                    messages,
                ),
                Err(error) => generic_action_error(
                    "settings.secretsCredentials.open",
                    "Credential Store",
                    error,
                ),
            }
        }
        "saveUpdateSource" => {
            let source = request
                .get("source")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let channel = request
                .get("channel")
                .and_then(|value| value.as_str())
                .unwrap_or("stable");
            match save_update_source(workspace_root, source, channel) {
                Ok(messages) => {
                    generic_action_result("help.updateChannelManager.open", "Update Source Saved", messages)
                }
                Err(error) => generic_action_error(
                    "help.updateChannelManager.open",
                    "Update Source Save",
                    error,
                ),
            }
        }
        "checkUpdateSource" => match check_update_source(workspace_root) {
            Ok(messages) => {
                generic_action_result("help.updateChannelManager.open", "Update Check", messages)
            }
            Err(error) => {
                generic_action_error("help.updateChannelManager.open", "Update Check", error)
            }
        },
        "detectLspServers" => generic_action_result(
            "source.refactor.open",
            "LSP Server Detection",
            detect_lsp_server_messages(),
        ),
        "runLocalAi" => {
            let prompt = request
                .get("prompt")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            let model = request
                .get("model")
                .and_then(|value| value.as_str())
                .unwrap_or("llama3");
            match run_local_ai_model(prompt, model) {
                Ok(result) => serde_json::json!({
                    "action": "aiRuntimeResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("aiRuntimeResult", error),
            }
        }
        "retrainAiKnowledge" => match retrain_ai_knowledge_index(workspace_root) {
            Ok(messages) => {
                generic_action_result("tools.aimlAssistant.open", "AI Knowledge Reindexed", messages)
            }
            Err(error) => generic_action_error(
                "tools.aimlAssistant.open",
                "AI Knowledge Reindex",
                error,
            ),
        },
        "trainAiLocalModel" => match train_ai_local_model(workspace_root) {
            Ok(messages) => {
                generic_action_result("tools.aimlAssistant.open", "AI Local Model Trained", messages)
            }
            Err(error) => generic_action_error("tools.aimlAssistant.open", "AI Local Training", error),
        },
        "exportAiTrainingDataset" => match export_ai_training_dataset(workspace_root) {
            Ok(messages) => generic_action_result(
                "tools.aimlAssistant.open",
                "AI Fine-tune Dataset Exported",
                messages,
            ),
            Err(error) => generic_action_error(
                "tools.aimlAssistant.open",
                "AI Fine-tune Dataset Export",
                error,
            ),
        },
        "saveAiTrainerConfig" => {
            let executable = request
                .get("executable")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let args = request.get("args").and_then(|value| value.as_str()).unwrap_or("");
            match save_ai_trainer_config(workspace_root, executable, args) {
                Ok(messages) => generic_action_result(
                    "tools.aimlAssistant.open",
                    "AI Trainer Config Saved",
                    messages,
                ),
                Err(error) => generic_action_error(
                    "tools.aimlAssistant.open",
                    "AI Trainer Config",
                    error,
                ),
            }
        },
        "runAiTrainingJob" => match run_ai_training_job(workspace_root) {
            Ok(result) => serde_json::json!({
                "action": "aiRuntimeResult",
                "ok": true,
                "commandLine": result.command_line,
                "exitCode": result.exit_code,
                "success": result.success,
                "stdout": result.stdout,
                "stderr": result.stderr,
            }),
            Err(error) => json_error("aiRuntimeResult", error),
        },
        "detectDebugAdapters" => generic_action_result(
            "source.refactor.open",
            "Debug Adapter Detection",
            detect_debug_adapter_messages(),
        ),
        "applyRenameRefactor" => {
            let path = request.get("path").and_then(|value| value.as_str()).unwrap_or("");
            let from = request.get("from").and_then(|value| value.as_str()).unwrap_or("");
            let to = request.get("to").and_then(|value| value.as_str()).unwrap_or("");
            match apply_rename_refactor(workspace_root, path, from, to) {
                Ok(messages) => {
                    generic_action_result("source.refactor.open", "Rename Refactor Applied", messages)
                }
                Err(error) => {
                    generic_action_error("source.refactor.open", "Rename Refactor", error)
                }
            }
        }
        "crossPlatformPackageAudit" => generic_action_result(
            "help.diagnosticsBundle.open",
            "Cross-platform Package Audit",
            cross_platform_package_audit_messages(workspace_root),
        ),
        "startTerminalSession" => {
            let host = request.get("host").and_then(|value| value.as_str()).unwrap_or("");
            let user = request.get("user").and_then(|value| value.as_str()).unwrap_or("");
            let local = request
                .get("local")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            match start_terminal_session(host, user, local) {
                Ok(snapshot) => serde_json::json!({
                    "action": "terminalSessionResult",
                    "ok": true,
                    "session": snapshot,
                }),
                Err(error) => json_error("terminalSessionResult", error),
            }
        }
        "sendTerminalInput" => {
            let session_id = request
                .get("sessionId")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let input = request.get("input").and_then(|value| value.as_str()).unwrap_or("");
            match send_terminal_input(session_id, input) {
                Ok(snapshot) => serde_json::json!({
                    "action": "terminalSessionResult",
                    "ok": true,
                    "session": snapshot,
                }),
                Err(error) => json_error("terminalSessionResult", error),
            }
        }
        "pollTerminalSession" => {
            let session_id = request
                .get("sessionId")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match terminal_session_snapshot(session_id) {
                Ok(snapshot) => serde_json::json!({
                    "action": "terminalSessionResult",
                    "ok": true,
                    "session": snapshot,
                }),
                Err(error) => json_error("terminalSessionResult", error),
            }
        }
        "stopTerminalSession" => {
            let session_id = request
                .get("sessionId")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match stop_terminal_session(session_id) {
                Ok(snapshot) => serde_json::json!({
                    "action": "terminalSessionResult",
                    "ok": true,
                    "session": snapshot,
                }),
                Err(error) => json_error("terminalSessionResult", error),
            }
        }
        "runSshCommand" => {
            let host = request.get("host").and_then(|value| value.as_str()).unwrap_or("");
            let user = request.get("user").and_then(|value| value.as_str()).unwrap_or("");
            let remote_command = request
                .get("remoteCommand")
                .and_then(|value| value.as_str())
                .unwrap_or("uname -a");
            match run_ssh_command(host, user, remote_command) {
                Ok(result) => serde_json::json!({
                    "action": "sshCommandResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("sshCommandResult", error),
            }
        }
        "enqueueTransfer" => {
            let direction = request
                .get("direction")
                .and_then(|value| value.as_str())
                .unwrap_or("download");
            let local_path = request
                .get("localPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let remote_target = request
                .get("remoteTarget")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match enqueue_transfer(workspace_root, direction, local_path, remote_target) {
                Ok(messages) => {
                    generic_action_result("tools.sftpScpTransfer.open", "Transfer Queued", messages)
                }
                Err(error) => generic_action_error(
                    "tools.sftpScpTransfer.open",
                    "Transfer Queue",
                    error,
                ),
            }
        }
        "runFileTransfer" => {
            let direction = request
                .get("direction")
                .and_then(|value| value.as_str())
                .unwrap_or("download");
            let local_path = request
                .get("localPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let remote_target = request
                .get("remoteTarget")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            match run_file_transfer(workspace_root, direction, local_path, remote_target) {
                Ok(result) => serde_json::json!({
                    "action": "fileTransferResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("fileTransferResult", error),
            }
        }
        "runJdbcSql" => {
            let jar_path = request
                .get("jarPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let jdbc_url = request
                .get("jdbcUrl")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let db_user = request.get("dbUser").and_then(|value| value.as_str()).unwrap_or("");
            let db_password = request
                .get("dbPassword")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let sql = request.get("sql").and_then(|value| value.as_str()).unwrap_or("");
            match run_jdbc_sql(workspace_root, jar_path, jdbc_url, db_user, db_password, sql) {
                Ok(result) => serde_json::json!({
                    "action": "jdbcSqlResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("jdbcSqlResult", error),
            }
        }
        "jdbcDashboard" | "jdbcListObjects" | "jdbcListColumns" => {
            let jar_path = request
                .get("jarPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let jdbc_url = request
                .get("jdbcUrl")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let db_user = request.get("dbUser").and_then(|value| value.as_str()).unwrap_or("");
            let db_password = request
                .get("dbPassword")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let schema = request.get("schema").and_then(|value| value.as_str()).unwrap_or("");
            let object = request.get("object").and_then(|value| value.as_str()).unwrap_or("");
            let mode = match action {
                "jdbcDashboard" => "dashboard",
                "jdbcListColumns" => "columns",
                _ => "objects",
            };
            match run_jdbc_metadata(
                workspace_root,
                jar_path,
                jdbc_url,
                db_user,
                db_password,
                mode,
                schema,
                object,
            ) {
                Ok(result) => serde_json::json!({
                    "action": "jdbcMetadataResult",
                    "ok": true,
                    "mode": mode,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("jdbcMetadataResult", error),
            }
        }
        "jdbcMetadata" => {
            let jar_path = request
                .get("jarPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let jdbc_url = request
                .get("jdbcUrl")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let db_user = request.get("dbUser").and_then(|value| value.as_str()).unwrap_or("");
            let db_password = request
                .get("dbPassword")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let mode = request
                .get("mode")
                .and_then(|value| value.as_str())
                .unwrap_or("objects");
            let schema = request.get("schema").and_then(|value| value.as_str()).unwrap_or("");
            let object = request.get("object").and_then(|value| value.as_str()).unwrap_or("");
            match run_jdbc_metadata(
                workspace_root,
                jar_path,
                jdbc_url,
                db_user,
                db_password,
                mode,
                schema,
                object,
            ) {
                Ok(result) => serde_json::json!({
                    "action": "jdbcMetadataResult",
                    "ok": true,
                    "mode": mode,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("jdbcMetadataResult", error),
            }
        }
        "jdbcDbaProbe" => {
            let jar_path = request
                .get("jarPath")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let jdbc_url = request
                .get("jdbcUrl")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let db_user = request.get("dbUser").and_then(|value| value.as_str()).unwrap_or("");
            let db_password = request
                .get("dbPassword")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let probe = request
                .get("probe")
                .and_then(|value| value.as_str())
                .unwrap_or("generic_version");
            match run_jdbc_dba_probe(workspace_root, jar_path, jdbc_url, db_user, db_password, probe)
            {
                Ok(result) => serde_json::json!({
                    "action": "jdbcSqlResult",
                    "ok": true,
                    "commandLine": result.command_line,
                    "exitCode": result.exit_code,
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr,
                }),
                Err(error) => json_error("jdbcSqlResult", error),
            }
        }
        "cvssUpdatePreview" => generic_action_result(
            "tools.codeSecurityAnalyzer.open",
            "CVSS Repository Update",
            vec![
                format!(
                    "Update sources: {}",
                    meditor_cvss_repository::update_sources().join(", ")
                ),
                "SQLite repository update is planned but not run automatically in this verification build.".to_string(),
                "User approval will be required before network sync or local repository mutation.".to_string(),
            ],
        ),
        "askAssistant" => {
            let prompt = request
                .get("prompt")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            generic_action_result(
                "tools.aimlAssistant.open",
                "AI/ML Assistant",
                ai_assistant_messages(prompt),
            )
        }
        "feedbackPreview" => {
            let feedback_type = request
                .get("feedbackType")
                .and_then(|value| value.as_str())
                .unwrap_or("BUG");
            let title = request
                .get("title")
                .and_then(|value| value.as_str())
                .unwrap_or_default();
            generic_action_result(
                "feedback.open",
                "Feedback Payload Preview",
                vec![
                    format!("Type: {feedback_type}"),
                    format!("Title: {title}"),
                    "Payload will be sent to the configured APEX REST endpoint only when the submit transport is enabled.".to_string(),
                    "Owner contact shown to user: s.pandey.india@gmail.com".to_string(),
                ],
            )
        }
        "diagnosticsPreview" => generic_action_result(
            "help.diagnosticsBundle.open",
            "Diagnostics Preview",
            diagnostics_preview_messages(workspace_root),
        ),
        "productionReadiness" => match production_readiness_messages(workspace_root) {
            Ok(messages) => generic_action_result(
                "help.diagnosticsBundle.open",
                "Production Readiness Gate",
                messages,
            ),
            Err(error) => generic_action_error(
                "help.diagnosticsBundle.open",
                "Production Readiness Gate",
                error,
            ),
        },
        "updateCheckPreview" => match check_update_source(workspace_root) {
            Ok(messages) => generic_action_result("help.about.open", "Update Check", messages),
            Err(error) => generic_action_error("help.about.open", "Update Check", error),
        },
        _ => json_error("unknown", format!("Unknown GUI action: {action}")),
    }
}

fn read_workspace_file(workspace_root: &Path, relative_path: &str) -> Result<String, String> {
    let path = resolve_workspace_path(workspace_root, relative_path)?;
    fs::read_to_string(&path).map_err(|error| format!("Unable to read {relative_path}: {error}"))
}

fn write_workspace_file(
    workspace_root: &Path,
    relative_path: &str,
    content: &str,
) -> Result<usize, String> {
    let path = resolve_workspace_path(workspace_root, relative_path)?;
    fs::write(&path, content)
        .map_err(|error| format!("Unable to save {relative_path}: {error}"))?;
    Ok(content.len())
}

fn resolve_workspace_path(workspace_root: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let path = Path::new(relative_path);
    if path.is_absolute() {
        return Err("Absolute paths are not accepted from the GUI".to_string());
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        return Err("Parent-directory traversal is not accepted from the GUI".to_string());
    }
    Ok(workspace_root.join(path))
}

fn ensure_parent(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to create {}: {error}", display_path(parent)))?;
    }
    Ok(())
}

fn record_id(prefix: &str) -> String {
    format!("{prefix}-{}", now_epoch_seconds())
}

fn read_named_records(
    workspace_root: &Path,
    relative_path: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let path = resolve_workspace_path(workspace_root, relative_path)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
    serde_json::from_str::<Vec<serde_json::Value>>(&content)
        .map_err(|error| format!("Unable to parse {}: {error}", display_path(&path)))
}

fn write_named_records(
    workspace_root: &Path,
    relative_path: &str,
    records: &[serde_json::Value],
) -> Result<(), String> {
    let path = resolve_workspace_path(workspace_root, relative_path)?;
    ensure_parent(&path)?;
    let content = serde_json::to_string_pretty(records)
        .map_err(|error| format!("Unable to serialize records: {error}"))?;
    fs::write(&path, content)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))
}

fn save_named_record(
    workspace_root: &Path,
    relative_path: &str,
    record: serde_json::Value,
) -> Result<Vec<String>, String> {
    let mut records = read_named_records(workspace_root, relative_path)?;
    records.push(record);
    write_named_records(workspace_root, relative_path, &records)?;
    Ok(vec![
        format!("Record saved to {relative_path}"),
        format!("Total records: {}", records.len()),
    ])
}

fn run_one_click_setup(workspace_root: &Path) -> Result<Vec<String>, String> {
    let layout = meditor_cvss_repository::RepositoryLayout::under_working_folder(workspace_root);
    fs::create_dir_all(&layout.security_dir).map_err(|error| {
        format!(
            "Unable to create security folder {}: {error}",
            display_path(&layout.security_dir)
        )
    })?;
    let schema_path = layout.security_dir.join("init_meditor_sqlite.sql");
    fs::write(&schema_path, meditor_cvss_repository::schema_sql())
        .map_err(|error| format!("Unable to write SQLite schema copy: {error}"))?;

    let mut messages = vec![
        format!(
            "Security folder ready: {}",
            display_path(&layout.security_dir)
        ),
        format!("Schema copy written: {}", display_path(&schema_path)),
    ];

    if command_exists("sqlite3") {
        let mut child = Command::new("sqlite3")
            .arg(&layout.vulnerability_db)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("Unable to start sqlite3: {error}"))?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(meditor_cvss_repository::schema_sql().as_bytes())
                .map_err(|error| format!("Unable to send schema to sqlite3: {error}"))?;
        }
        let output = child
            .wait_with_output()
            .map_err(|error| format!("Unable to complete sqlite3 setup: {error}"))?;
        if output.status.success() {
            messages.push(format!(
                "SQLite database initialized: {}",
                display_path(&layout.vulnerability_db)
            ));
        } else {
            messages.push(format!(
                "sqlite3 returned {:?}: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    } else {
        messages.push("sqlite3 executable was not found; schema is staged for the setup script to apply later.".to_string());
    }
    Ok(messages)
}

fn run_vcs_command(
    workspace_root: &Path,
    provider: &str,
    operation: &str,
    project_path: &str,
) -> Result<CommandCapture, String> {
    let root = resolve_workspace_path(workspace_root, project_path)?;
    if !root.is_dir() {
        return Err(format!("{project_path} is not a folder"));
    }
    let (executable, raw_args): (&str, &[&str]) = match (provider, operation) {
        ("git", "status") => ("git", &["status", "--short"]),
        ("git", "log") => ("git", &["log", "--oneline", "-20"]),
        ("git", "branch") => ("git", &["branch", "--all"]),
        ("git", "diff") => ("git", &["diff", "--stat"]),
        ("git", "remote") => ("git", &["remote", "-v"]),
        ("svn", "status") => ("svn", &["status"]),
        ("svn", "log") => ("svn", &["log", "-l", "20"]),
        ("svn", "info") => ("svn", &["info"]),
        _ => return Err(format!("Unsupported VCS command: {provider} {operation}")),
    };
    if !command_exists(executable) {
        return Err(format!("{executable} executable was not found"));
    }
    let args = raw_args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    let profile = TaskProfile {
        action: "vcs",
        label: "Version Control",
        executable: executable.to_string(),
        args,
        description: "Guarded version-control command.",
    };
    run_command_capture(&root, &profile, 30)
}

fn run_sqlite_sql(
    workspace_root: &Path,
    db_path: &str,
    sql: &str,
) -> Result<CommandCapture, String> {
    if sql.trim().is_empty() {
        return Err("SQL text is empty".to_string());
    }
    if !command_exists("sqlite3") {
        return Err("sqlite3 executable was not found".to_string());
    }
    let db = resolve_workspace_path(workspace_root, db_path)?;
    ensure_parent(&db)?;
    let profile = TaskProfile {
        action: "sqlite",
        label: "SQLite SQL",
        executable: "sqlite3".to_string(),
        args: vec![display_path(&db), sql.to_string()],
        description: "SQLite worksheet execution.",
    };
    run_command_capture(workspace_root, &profile, 30)
}

fn test_ssh_connection(host: &str, user: &str) -> Result<CommandCapture, String> {
    if host.trim().is_empty() {
        return Err("Host is required".to_string());
    }
    if !command_exists("ssh") {
        return Err("ssh executable was not found".to_string());
    }
    let target = if user.trim().is_empty() {
        host.trim().to_string()
    } else {
        format!("{}@{}", user.trim(), host.trim())
    };
    let profile = TaskProfile {
        action: "ssh-test",
        label: "SSH Test",
        executable: "ssh".to_string(),
        args: vec![
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ConnectTimeout=8".to_string(),
            target,
            "echo mEditor-ssh-ok".to_string(),
        ],
        description: "Non-interactive SSH connectivity test.",
    };
    run_command_capture(Path::new("."), &profile, 12)
}

fn run_sftp_list(target: &str) -> Result<CommandCapture, String> {
    if target.trim().is_empty() {
        return Err("SFTP target is required, for example user@host:/path".to_string());
    }
    if !command_exists("ssh") {
        return Err("ssh executable was not found".to_string());
    }
    let (host, path) = target
        .trim()
        .split_once(':')
        .ok_or_else(|| "Use target format user@host:/remote/path".to_string())?;
    let profile = TaskProfile {
        action: "sftp-list",
        label: "SFTP List",
        executable: "ssh".to_string(),
        args: vec![
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ConnectTimeout=8".to_string(),
            host.to_string(),
            format!("ls -la {}", shell_word(path)),
        ],
        description: "SFTP non-interactive listing.",
    };
    run_command_capture(Path::new("."), &profile, 12)
}

fn list_local_reports(workspace_root: &Path) -> Result<Vec<serde_json::Value>, String> {
    let report_root = workspace_root.join(".meditor");
    let mut reports = Vec::new();
    collect_report_files(workspace_root, &report_root, &mut reports)?;
    Ok(reports)
}

fn collect_report_files(
    workspace_root: &Path,
    folder: &Path,
    reports: &mut Vec<serde_json::Value>,
) -> Result<(), String> {
    if !folder.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(folder)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(folder)))?
    {
        let entry = entry.map_err(|error| format!("Unable to read report entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_report_files(workspace_root, &path, reports)?;
        } else if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|extension| matches!(extension, "md" | "toml" | "json"))
        {
            reports.push(serde_json::json!({
                "path": path.strip_prefix(workspace_root).ok().and_then(|p| p.to_str()).unwrap_or(""),
                "size": entry.metadata().ok().map(|m| m.len()).unwrap_or(0),
            }));
        }
    }
    Ok(())
}

fn save_ai_knowledge(
    workspace_root: &Path,
    title: &str,
    content: &str,
) -> Result<Vec<String>, String> {
    if content.trim().is_empty() {
        return Err("Knowledge content is empty".to_string());
    }
    let file_name = format!("{}-{}.txt", now_epoch_seconds(), sanitize_file_stem(title));
    let path = resolve_workspace_path(
        workspace_root,
        &format!(".meditor/ai/knowledge/{file_name}"),
    )?;
    ensure_parent(&path)?;
    fs::write(&path, content)
        .map_err(|error| format!("Unable to write knowledge file: {error}"))?;
    Ok(vec![
        format!("Knowledge source saved: {}", display_path(&path)),
        "Retraining is recorded as a local learning event; model mutation remains approval-gated."
            .to_string(),
    ])
}

fn retrain_ai_knowledge_index(workspace_root: &Path) -> Result<Vec<String>, String> {
    let knowledge_root = workspace_root.join(".meditor/ai/knowledge");
    if !knowledge_root.exists() {
        return Err(
            "No local knowledge sources found. Save knowledge before retraining.".to_string(),
        );
    }
    let mut documents = Vec::new();
    collect_text_documents(workspace_root, &knowledge_root, &mut documents)?;
    if documents.is_empty() {
        return Err("No text knowledge sources were found for indexing.".to_string());
    }

    let mut vocabulary: BTreeMap<String, usize> = BTreeMap::new();
    let mut document_records = Vec::new();
    for path in documents {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
        let tokens = tokenize_learning_text(&content);
        for token in &tokens {
            *vocabulary.entry(token.clone()).or_default() += 1;
        }
        document_records.push(serde_json::json!({
            "path": path.strip_prefix(workspace_root).ok().and_then(|p| p.to_str()).unwrap_or(""),
            "bytes": content.len(),
            "tokens": tokens.len(),
        }));
    }

    let mut top_terms = vocabulary
        .iter()
        .map(|(term, count)| (term.clone(), *count))
        .collect::<Vec<_>>();
    top_terms.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    top_terms.truncate(200);
    let index_path = workspace_root.join(".meditor/ai/model/local-knowledge-index.json");
    ensure_parent(&index_path)?;
    let document_count = document_records.len();
    fs::write(
        &index_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "created_at_epoch": now_epoch_seconds(),
            "documents": document_records,
            "document_count": document_count,
            "vocabulary_size": vocabulary.len(),
            "top_terms": top_terms.into_iter().map(|(term, count)| serde_json::json!({"term": term, "count": count})).collect::<Vec<_>>(),
            "training_mode": "local retrieval index",
            "model_weight_mutation": false,
            "approval_required_for_model_fine_tuning": true,
        }))
        .map_err(|error| format!("Unable to serialize AI knowledge index: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&index_path)))?;
    Ok(vec![
        format!("Knowledge sources indexed: {document_count}"),
        format!("Vocabulary terms: {}", vocabulary.len()),
        format!("Local AI index written: {}", display_path(&index_path)),
        "This is a real local learning index for retrieval. Model weight retraining remains external-runtime gated.".to_string(),
    ])
}

fn train_ai_local_model(workspace_root: &Path) -> Result<Vec<String>, String> {
    let knowledge_root = workspace_root.join(".meditor/ai/knowledge");
    if !knowledge_root.exists() {
        return Err(
            "No local knowledge sources found. Save knowledge before local training.".to_string(),
        );
    }

    let mut documents = Vec::new();
    collect_text_documents(workspace_root, &knowledge_root, &mut documents)?;
    if documents.is_empty() {
        return Err("No text knowledge sources were found for local training.".to_string());
    }

    let mut document_frequencies: BTreeMap<String, usize> = BTreeMap::new();
    let mut document_term_counts: Vec<(PathBuf, usize, BTreeMap<String, usize>)> = Vec::new();
    for path in documents {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
        let mut term_counts: BTreeMap<String, usize> = BTreeMap::new();
        for token in tokenize_learning_text(&content) {
            *term_counts.entry(token).or_default() += 1;
        }
        for term in term_counts.keys() {
            *document_frequencies.entry(term.clone()).or_default() += 1;
        }
        document_term_counts.push((path, content.len(), term_counts));
    }

    let document_count = document_term_counts.len();
    let document_count_f64 = document_count as f64;
    let mut trained_documents = Vec::new();
    for (document_index, (path, bytes, term_counts)) in document_term_counts.iter().enumerate() {
        let total_terms = term_counts.values().sum::<usize>().max(1) as f64;
        let mut weighted_terms = term_counts
            .iter()
            .map(|(term, count)| {
                let document_frequency = *document_frequencies.get(term).unwrap_or(&1) as f64;
                let term_frequency = *count as f64 / total_terms;
                let inverse_document_frequency =
                    ((1.0 + document_count_f64) / (1.0 + document_frequency)).ln() + 1.0;
                (
                    term.clone(),
                    *count,
                    term_frequency * inverse_document_frequency,
                )
            })
            .collect::<Vec<_>>();
        weighted_terms.sort_by(|left, right| {
            right
                .2
                .partial_cmp(&left.2)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.0.cmp(&right.0))
        });
        weighted_terms.truncate(40);
        trained_documents.push(serde_json::json!({
            "document_id": document_index + 1,
            "path": path.strip_prefix(workspace_root).ok().and_then(|p| p.to_str()).unwrap_or(""),
            "bytes": bytes,
            "term_count": term_counts.values().sum::<usize>(),
            "unique_terms": term_counts.len(),
            "top_weighted_terms": weighted_terms
                .into_iter()
                .map(|(term, count, score)| serde_json::json!({
                    "term": term,
                    "count": count,
                    "tf_idf_score": score,
                }))
                .collect::<Vec<_>>(),
        }));
    }

    let model_path = workspace_root.join(".meditor/ai/model/local-trained-model.json");
    ensure_parent(&model_path)?;
    fs::write(
        &model_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "created_at_epoch": now_epoch_seconds(),
            "version": meditor_core::CURRENT_BASELINE_VERSION,
            "training_algorithm": "tf-idf lexical retrieval/ranking model",
            "documents": trained_documents,
            "document_count": document_count,
            "vocabulary_size": document_frequencies.len(),
            "model_weight_mutation": false,
            "notes": [
                "This is a real fitted local retrieval model generated from user-provided knowledge sources.",
                "LLM weight fine-tuning requires a configured external trainer/runtime and explicit user approval."
            ],
        }))
        .map_err(|error| format!("Unable to serialize local AI model: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&model_path)))?;

    let report_path = workspace_root.join(".meditor/ai/training/training-report.md");
    ensure_parent(&report_path)?;
    let mut report = String::new();
    let _ = writeln!(report, "# mEditor Local AI Training Report");
    let _ = writeln!(report);
    let _ = writeln!(
        report,
        "- Version: {}",
        meditor_core::CURRENT_BASELINE_VERSION
    );
    let _ = writeln!(report, "- Documents trained: {document_count}");
    let _ = writeln!(report, "- Vocabulary size: {}", document_frequencies.len());
    let _ = writeln!(report, "- Model artifact: {}", display_path(&model_path));
    let _ = writeln!(report, "- Training mode: local TF-IDF retrieval/ranking");
    let _ = writeln!(report, "- LLM weight mutation: false");
    let _ = writeln!(
        report,
        "- External fine-tuning: requires configured trainer and user approval"
    );
    fs::write(&report_path, report)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&report_path)))?;

    Ok(vec![
        format!("Local AI training documents: {document_count}"),
        format!("Vocabulary terms: {}", document_frequencies.len()),
        format!("Trained local model: {}", display_path(&model_path)),
        format!("Training report: {}", display_path(&report_path)),
        "Production note: mEditor now fits a local retrieval/ranking model; LLM weight fine-tuning remains an external-runtime feature.".to_string(),
    ])
}

fn export_ai_training_dataset(workspace_root: &Path) -> Result<Vec<String>, String> {
    let knowledge_root = workspace_root.join(".meditor/ai/knowledge");
    if !knowledge_root.exists() {
        return Err(
            "No local knowledge sources found. Save knowledge before exporting a training dataset."
                .to_string(),
        );
    }

    let mut documents = Vec::new();
    collect_text_documents(workspace_root, &knowledge_root, &mut documents)?;
    if documents.is_empty() {
        return Err("No text knowledge sources were found for dataset export.".to_string());
    }

    let dataset_path = workspace_root.join(".meditor/ai/training/fine-tune-dataset.jsonl");
    ensure_parent(&dataset_path)?;
    let mut dataset = String::new();
    let mut example_count = 0usize;
    let mut source_records = Vec::new();
    for path in documents {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
        let relative = path
            .strip_prefix(workspace_root)
            .ok()
            .and_then(|path| path.to_str())
            .unwrap_or("")
            .to_string();
        source_records.push(serde_json::json!({
            "path": relative,
            "bytes": content.len(),
        }));
        for chunk in text_chunks(&content, 2800).into_iter().take(50) {
            let line = serde_json::json!({
                "messages": [
                    {
                        "role": "system",
                        "content": "You are the local mEditor coding assistant. Learn only from user-approved local project knowledge."
                    },
                    {
                        "role": "user",
                        "content": format!("Use the knowledge source {} to answer future programming, architecture, design, and debugging questions.", relative)
                    },
                    {
                        "role": "assistant",
                        "content": chunk
                    }
                ],
                "source_path": relative,
                "created_at_epoch": now_epoch_seconds(),
            });
            dataset.push_str(
                &serde_json::to_string(&line)
                    .map_err(|error| format!("Unable to serialize dataset row: {error}"))?,
            );
            dataset.push('\n');
            example_count += 1;
        }
    }
    fs::write(&dataset_path, dataset)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&dataset_path)))?;

    let manifest_path = workspace_root.join(".meditor/ai/training/fine-tune-dataset-manifest.json");
    fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "created_at_epoch": now_epoch_seconds(),
            "version": meditor_core::CURRENT_BASELINE_VERSION,
            "dataset": path_relative_to(workspace_root, &dataset_path),
            "format": "jsonl.chat.messages",
            "examples": example_count,
            "sources": source_records,
            "approval_required_before_external_training": true,
            "model_weight_mutation_inside_meditor": false,
        }))
        .map_err(|error| format!("Unable to serialize dataset manifest: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&manifest_path)))?;

    Ok(vec![
        format!("Training examples exported: {example_count}"),
        format!("Dataset: {}", display_path(&dataset_path)),
        format!("Manifest: {}", display_path(&manifest_path)),
        "The dataset is ready for an explicit external trainer. mEditor does not mutate model weights without the configured trainer command.".to_string(),
    ])
}

fn text_chunks(content: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for line in content.lines() {
        if current.len() + line.len() + 1 > max_chars && !current.trim().is_empty() {
            chunks.push(current.trim().to_string());
            current.clear();
        }
        current.push_str(line);
        current.push('\n');
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }
    chunks
}

fn save_ai_trainer_config(
    workspace_root: &Path,
    executable: &str,
    args_text: &str,
) -> Result<Vec<String>, String> {
    let executable = executable.trim();
    if executable.is_empty() {
        return Err("Trainer executable is required.".to_string());
    }
    let args = args_text
        .lines()
        .map(str::trim)
        .filter(|arg| !arg.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let path = workspace_root.join(".meditor/ai/training/external-trainer.json");
    ensure_parent(&path)?;
    fs::write(
        &path,
        serde_json::to_string_pretty(&serde_json::json!({
            "executable": executable,
            "args": args,
            "saved_at_epoch": now_epoch_seconds(),
            "dataset_placeholder": "{dataset}",
            "model_placeholder": "{model}",
            "approval_required_before_run": true,
        }))
        .map_err(|error| format!("Unable to serialize trainer config: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))?;
    Ok(vec![
        format!("Trainer config saved: {}", display_path(&path)),
        format!(
            "Executable availability: {}",
            command_exists(executable) || Path::new(executable).exists()
        ),
        "Run External Training Job will execute only this saved command after user confirmation."
            .to_string(),
    ])
}

fn run_ai_training_job(workspace_root: &Path) -> Result<CommandCapture, String> {
    let dataset_path = workspace_root.join(".meditor/ai/training/fine-tune-dataset.jsonl");
    if !dataset_path.is_file() {
        export_ai_training_dataset(workspace_root)?;
    }
    let config_path = workspace_root.join(".meditor/ai/training/external-trainer.json");
    if !config_path.is_file() {
        return Err("No external trainer is configured. Save trainer config first.".to_string());
    }
    let config = fs::read_to_string(&config_path)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(&config_path)))?;
    let value = serde_json::from_str::<serde_json::Value>(&config)
        .map_err(|error| format!("Unable to parse {}: {error}", display_path(&config_path)))?;
    let executable = value
        .get("executable")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if executable.is_empty() {
        return Err("Trainer config does not contain an executable.".to_string());
    }
    if !command_exists(&executable) && !Path::new(&executable).exists() {
        return Err(format!("Trainer executable was not found: {executable}"));
    }
    let model_path = workspace_root.join(".meditor/ai/model/external-trained-model");
    ensure_parent(&model_path)?;
    let args = value
        .get("args")
        .and_then(|value| value.as_array())
        .map(|args| {
            args.iter()
                .filter_map(|arg| arg.as_str())
                .map(|arg| {
                    arg.replace("{dataset}", &display_path(&dataset_path))
                        .replace("{model}", &display_path(&model_path))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let profile = TaskProfile {
        action: "ai-external-training",
        label: "External AI Training",
        executable,
        args,
        description: "Run the configured external model trainer against the exported dataset.",
    };
    let capture = run_command_capture(workspace_root, &profile, 900)?;
    let report_path = workspace_root.join(".meditor/ai/training/external-training-report.md");
    ensure_parent(&report_path)?;
    fs::write(
        &report_path,
        format!(
            "# mEditor External AI Training Report\n\n- Command: `{}`\n- Exit code: {:?}\n- Success: {}\n- Dataset: {}\n- Model target: {}\n\n## stdout\n```text\n{}\n```\n\n## stderr\n```text\n{}\n```\n",
            capture.command_line,
            capture.exit_code,
            capture.success,
            display_path(&dataset_path),
            display_path(&model_path),
            capture.stdout,
            capture.stderr
        ),
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&report_path)))?;
    Ok(capture)
}

fn collect_text_documents(
    workspace_root: &Path,
    folder: &Path,
    documents: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(folder)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(folder)))?
    {
        let entry = entry.map_err(|error| format!("Unable to read AI knowledge entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_text_documents(workspace_root, &path, documents)?;
        } else if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|extension| matches!(extension, "txt" | "md" | "json" | "csv"))
        {
            if path.starts_with(workspace_root) {
                documents.push(path);
            }
        }
    }
    Ok(())
}

fn tokenize_learning_text(content: &str) -> Vec<String> {
    content
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .map(|token| token.trim().to_ascii_lowercase())
        .filter(|token| token.len() >= 3)
        .take(100_000)
        .collect()
}

fn detect_debug_adapter_messages() -> Vec<String> {
    let adapters = [
        ("C/C++/Rust", "lldb-dap"),
        ("C/C++/Rust", "codelldb"),
        ("C/C++", "gdb"),
        ("Python", "debugpy"),
        ("JavaScript/TypeScript", "node"),
        ("Java", "jdb"),
        ("PHP", "php"),
        ("Go", "dlv"),
    ];
    let mut messages = vec![
        "Debug adapter detection completed.".to_string(),
        "Launch/debug remains user-controlled and uses .meditor/debug-staging plus handover reports.".to_string(),
    ];
    messages.extend(adapters.into_iter().map(|(language, executable)| {
        format!(
            "{language}: {executable} => {}",
            if command_exists(executable) {
                "available"
            } else {
                "not found"
            }
        )
    }));
    messages
}

fn apply_rename_refactor(
    workspace_root: &Path,
    relative_path: &str,
    from: &str,
    to: &str,
) -> Result<Vec<String>, String> {
    if from.is_empty() || to.is_empty() {
        return Err("Both old symbol/text and new symbol/text are required.".to_string());
    }
    if from == to {
        return Err("Old and new values are the same.".to_string());
    }
    let path = resolve_workspace_path(workspace_root, relative_path)?;
    if !path.is_file() {
        return Err(format!("Refactor target is not a file: {relative_path}"));
    }
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
    let replacements = content.matches(from).count();
    if replacements == 0 {
        return Err(format!(
            "No occurrences of '{from}' found in {relative_path}."
        ));
    }
    let rollback_dir = workspace_root.join(format!(
        ".meditor/refactor-rollback/{}",
        now_epoch_seconds()
    ));
    fs::create_dir_all(&rollback_dir)
        .map_err(|error| format!("Unable to create rollback folder: {error}"))?;
    let backup_name = sanitize_file_stem(relative_path);
    let backup_path = rollback_dir.join(format!("{backup_name}.bak"));
    fs::write(&backup_path, &content)
        .map_err(|error| format!("Unable to write rollback backup: {error}"))?;
    let next = content.replace(from, to);
    fs::write(&path, next).map_err(|error| format!("Unable to write refactored file: {error}"))?;
    let report_path = rollback_dir.join("refactor-report.md");
    fs::write(
        &report_path,
        format!(
            "# mEditor Refactor Report\n\n- File: {relative_path}\n- Replacements: {replacements}\n- From: `{}`\n- To: `{}`\n- Rollback backup: {}\n- User approval: required before action\n",
            from,
            to,
            display_path(&backup_path)
        ),
    )
    .map_err(|error| format!("Unable to write refactor report: {error}"))?;
    Ok(vec![
        format!("Refactored file: {relative_path}"),
        format!("Replacements applied: {replacements}"),
        format!("Rollback backup: {}", display_path(&backup_path)),
        format!("Refactor report: {}", display_path(&report_path)),
    ])
}

fn cross_platform_package_audit_messages(workspace_root: &Path) -> Vec<String> {
    let version = meditor_core::CURRENT_BASELINE_VERSION;
    let mut messages = vec![
        format!("Current platform package version: {version}"),
        format!(
            "macOS arm64 package present: {}",
            workspace_root
                .join(format!("dist/mEditor-{version}-darwin-arm64/bin/mEditor"))
                .exists()
        ),
        "Windows installer test: not executable from this macOS workspace; package script and setup.cmd are staged.".to_string(),
        "Linux installer test: not executable from this macOS workspace unless a Linux container/runner is provided; setup.sh and package layout are staged.".to_string(),
        "Required external validation: run package smoke tests on Windows x64, Linux x64, and macOS Intel before production release.".to_string(),
    ];
    for path in [
        "packaging/build-local-package.sh",
        "setup/setup.cmd",
        "setup/setup.sh",
        "setup/setup.command",
    ] {
        messages.push(format!("{path}: {}", workspace_root.join(path).exists()));
    }
    messages
}

fn create_extension_skeleton(workspace_root: &Path) -> Result<Vec<String>, String> {
    let root = workspace_root.join(".meditor/extensions/sample-extension");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create extension folder: {error}"))?;
    fs::write(
        root.join("plugin.json"),
        "{\n  \"id\": \"sample-extension\",\n  \"name\": \"Sample Extension\",\n  \"version\": \"0.1.0\",\n  \"permissions\": []\n}\n",
    )
    .map_err(|error| format!("Unable to write plugin.json: {error}"))?;
    fs::write(
        root.join("README.md"),
        "# Sample Extension\n\nDeclare commands, languages, snippets, and permissions here.\n",
    )
    .map_err(|error| format!("Unable to write extension README: {error}"))?;
    Ok(vec![
        format!("Extension skeleton: {}", display_path(&root)),
        "Plugin loading remains permission-gated.".to_string(),
    ])
}

fn scan_workspace_index(workspace_root: &Path) -> Result<Vec<String>, String> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for entry in workspace_entries(workspace_root)
        .into_iter()
        .filter(|entry| !entry.is_dir)
    {
        let extension = Path::new(&entry.display_path)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("[none]")
            .to_string();
        *counts.entry(extension).or_default() += 1;
    }
    let path = workspace_root.join(".meditor/index/workspace-index.json");
    ensure_parent(&path)?;
    let records = counts
        .iter()
        .map(|(extension, count)| serde_json::json!({"extension": extension, "count": count}))
        .collect::<Vec<_>>();
    fs::write(
        &path,
        serde_json::to_string_pretty(&records)
            .map_err(|error| format!("Unable to serialize workspace index: {error}"))?,
    )
    .map_err(|error| format!("Unable to write workspace index: {error}"))?;
    Ok(vec![
        format!("Indexed file extensions: {}", counts.len()),
        format!("Index written: {}", display_path(&path)),
    ])
}

fn save_feedback_outbox(
    workspace_root: &Path,
    feedback_type: &str,
    title: &str,
    details: &str,
) -> Result<Vec<String>, String> {
    save_named_record(
        workspace_root,
        ".meditor/product-feedback/outbox.json",
        serde_json::json!({
            "id": record_id("feedback"),
            "feedback_type": feedback_type,
            "title": title,
            "details": details,
            "created_at_epoch": now_epoch_seconds(),
            "target_endpoint": "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/",
            "transport": "saved locally for guarded submission",
        }),
    )
}

#[derive(Clone, Debug)]
struct LicenseStatus {
    accepted: bool,
    registered: bool,
    display_name: String,
    accepted_at_epoch: u64,
}

fn license_acceptance_status(workspace_root: &Path) -> Result<LicenseStatus, String> {
    let path = workspace_root.join(".meditor/license/acceptance.json");
    if !path.exists() {
        return Ok(LicenseStatus {
            accepted: false,
            registered: false,
            display_name: "UnRegistered but fully functional copy with no obligation".to_string(),
            accepted_at_epoch: 0,
        });
    }
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
    let value = serde_json::from_str::<serde_json::Value>(&content)
        .map_err(|error| format!("Unable to parse {}: {error}", display_path(&path)))?;
    let accepted = value
        .get("accepted")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let registered = value
        .get("registered")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let display_name = value
        .get("display_name")
        .and_then(|value| value.as_str())
        .unwrap_or("UnRegistered but fully functional copy with no obligation")
        .to_string();
    let accepted_at_epoch = value
        .get("accepted_at_epoch")
        .and_then(|value| value.as_u64())
        .unwrap_or_default();
    Ok(LicenseStatus {
        accepted,
        registered,
        display_name,
        accepted_at_epoch,
    })
}

fn accept_eula(
    workspace_root: &Path,
    registered: bool,
    name: &str,
    email: &str,
) -> Result<LicenseStatus, String> {
    let trimmed_name = name.trim();
    let trimmed_email = email.trim();
    if registered && trimmed_name.is_empty() {
        return Err("Name is required to personalize this mEditor copy.".to_string());
    }
    if registered && trimmed_email.is_empty() {
        return Err("Email is required when registering this personal copy.".to_string());
    }
    let display_name = if registered {
        format!("Personal License to {trimmed_name}")
    } else {
        "UnRegistered but fully functional copy with no obligation".to_string()
    };
    let accepted_at_epoch = now_epoch_seconds();
    let path = workspace_root.join(".meditor/license/acceptance.json");
    ensure_parent(&path)?;
    fs::write(
        &path,
        serde_json::to_string_pretty(&serde_json::json!({
            "accepted": true,
            "registered": registered,
            "name": trimmed_name,
            "email": trimmed_email,
            "display_name": display_name,
            "accepted_at_epoch": accepted_at_epoch,
            "license": "mEditor Freeware EULA",
            "version": meditor_core::CURRENT_BASELINE_VERSION,
        }))
        .map_err(|error| format!("Unable to serialize EULA acceptance: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))?;
    Ok(LicenseStatus {
        accepted: true,
        registered,
        display_name,
        accepted_at_epoch,
    })
}

fn credential_store_status_messages() -> Vec<String> {
    let (backend, available) = credential_backend();
    vec![
        format!("Credential backend: {backend}"),
        format!("Available on this machine: {available}"),
        "Secrets are stored in the operating-system encrypted credential store when the backend is available.".to_string(),
        "mEditor stores only non-secret service/account pointers in the workspace.".to_string(),
    ]
}

fn credential_backend() -> (&'static str, bool) {
    if cfg!(target_os = "macos") {
        ("macOS Keychain via security", command_exists("security"))
    } else if cfg!(target_os = "windows") {
        (
            "Windows Credential Manager via cmdkey",
            command_exists("cmdkey"),
        )
    } else {
        (
            "Secret Service via secret-tool",
            command_exists("secret-tool"),
        )
    }
}

fn store_credential(
    workspace_root: &Path,
    service: &str,
    account: &str,
    secret: &str,
) -> Result<Vec<String>, String> {
    let service = service.trim();
    let account = account.trim();
    if service.is_empty() || account.is_empty() || secret.is_empty() {
        return Err("Service, account, and secret are required.".to_string());
    }
    let (backend, available) = credential_backend();
    if !available {
        return Err(format!("{backend} is not available on this machine."));
    }

    if cfg!(target_os = "macos") {
        let output = Command::new("security")
            .args([
                "add-generic-password",
                "-U",
                "-a",
                account,
                "-s",
                service,
                "-w",
                secret,
            ])
            .output()
            .map_err(|error| format!("Unable to invoke macOS Keychain: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "macOS Keychain rejected the credential: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    } else if cfg!(target_os = "windows") {
        let output = Command::new("cmdkey")
            .args([
                format!("/generic:{service}"),
                format!("/user:{account}"),
                format!("/pass:{secret}"),
            ])
            .output()
            .map_err(|error| format!("Unable to invoke Windows Credential Manager: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "Windows Credential Manager rejected the credential: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    } else {
        let mut child = Command::new("secret-tool")
            .args([
                "store",
                "--label",
                "mEditor credential",
                "service",
                service,
                "account",
                account,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("Unable to invoke secret-tool: {error}"))?;
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(secret.as_bytes())
                .map_err(|error| format!("Unable to pass secret to secret-tool: {error}"))?;
        }
        let output = child
            .wait_with_output()
            .map_err(|error| format!("Unable to finish secret-tool: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "Secret Service rejected the credential: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    let mut messages = save_named_record(
        workspace_root,
        ".meditor/credentials/credential-index.json",
        serde_json::json!({
            "id": record_id("secret"),
            "service": service,
            "account": account,
            "backend": backend,
            "created_at_epoch": now_epoch_seconds(),
            "secret_in_workspace": false,
        }),
    )?;
    messages.insert(0, format!("Stored secret in {backend}."));
    Ok(messages)
}

fn save_update_source(
    workspace_root: &Path,
    source: &str,
    channel: &str,
) -> Result<Vec<String>, String> {
    let source = source.trim();
    if source.is_empty() {
        return Err("Release source URL is required.".to_string());
    }
    let channel = channel.trim();
    let path = workspace_root.join(".meditor/settings/update-channel.json");
    ensure_parent(&path)?;
    fs::write(
        &path,
        serde_json::to_string_pretty(&serde_json::json!({
            "source": source,
            "channel": if channel.is_empty() { "stable" } else { channel },
            "saved_at_epoch": now_epoch_seconds(),
            "strategy": "git tags with user-approved install",
        }))
        .map_err(|error| format!("Unable to serialize update source: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))?;
    Ok(vec![
        format!("Update source saved: {source}"),
        format!(
            "Channel: {}",
            if channel.is_empty() {
                "stable"
            } else {
                channel
            }
        ),
        "Future update install must preserve .meditor workspace data and require user approval."
            .to_string(),
    ])
}

fn configured_update_source(workspace_root: &Path) -> Result<(String, String), String> {
    let path = workspace_root.join(".meditor/settings/update-channel.json");
    if path.exists() {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read {}: {error}", display_path(&path)))?;
        let value = serde_json::from_str::<serde_json::Value>(&content)
            .map_err(|error| format!("Unable to parse {}: {error}", display_path(&path)))?;
        if let Some(source) = value.get("source").and_then(|value| value.as_str()) {
            let channel = value
                .get("channel")
                .and_then(|value| value.as_str())
                .unwrap_or("stable")
                .to_string();
            return Ok((source.to_string(), channel));
        }
    }

    if command_exists("git") {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(workspace_root)
            .output()
            .map_err(|error| format!("Unable to query git remote: {error}"))?;
        if output.status.success() {
            let source = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !source.is_empty() {
                return Ok((source, "stable".to_string()));
            }
        }
    }

    Err("No update source is configured. Save a public Git release source first.".to_string())
}

fn check_update_source(workspace_root: &Path) -> Result<Vec<String>, String> {
    if !command_exists("git") {
        return Err("git executable was not found.".to_string());
    }
    let (source, channel) = configured_update_source(workspace_root)?;
    let profile = TaskProfile {
        action: "update-check",
        label: "Update Check",
        executable: "git".to_string(),
        args: vec![
            "ls-remote".to_string(),
            "--tags".to_string(),
            "--refs".to_string(),
            source.clone(),
        ],
        description: "Check configured release source tags.",
    };
    let capture = run_command_capture(workspace_root, &profile, 30)?;
    if !capture.success {
        return Err(format!(
            "Update source check failed for {source}: {}",
            if capture.stderr.trim().is_empty() {
                capture.stdout.trim()
            } else {
                capture.stderr.trim()
            }
        ));
    }
    let tags = capture
        .stdout
        .lines()
        .filter_map(|line| line.rsplit('/').next())
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();
    let latest = tags
        .last()
        .cloned()
        .unwrap_or_else(|| "[no tags]".to_string());
    Ok(vec![
        format!("Release source: {source}"),
        format!("Channel: {channel}"),
        format!(
            "Current mEditor version: {}",
            meditor_core::CURRENT_BASELINE_VERSION
        ),
        format!("Remote tag count: {}", tags.len()),
        format!("Latest advertised tag: {latest}"),
        "Install is not automatic: user approval and rollback confirmation remain mandatory."
            .to_string(),
    ])
}

fn detect_lsp_server_messages() -> Vec<String> {
    let servers = [
        ("Assembly/C/C++", "clangd"),
        ("Rust", "rust-analyzer"),
        ("Python", "pyright-langserver"),
        ("JavaScript/TypeScript", "typescript-language-server"),
        ("Java", "jdtls"),
        ("PHP", "intelephense"),
        ("R", "R"),
        ("SQL", "sqls"),
        ("Lisp", "cl-lsp"),
    ];
    let mut messages = vec![
        "LSP/refactor adapter checks installed language servers.".to_string(),
        "Refactor actions remain user-approved and rollback-recorded.".to_string(),
    ];
    messages.extend(servers.into_iter().map(|(language, executable)| {
        format!(
            "{language}: {executable} => {}",
            if command_exists(executable) {
                "available"
            } else {
                "not found"
            }
        )
    }));
    messages
}

fn run_local_ai_model(prompt: &str, model: &str) -> Result<CommandCapture, String> {
    if prompt.trim().is_empty() {
        return Err("Prompt is required.".to_string());
    }
    if !command_exists("ollama") {
        return Err(
            "Local AI runtime is not available. Install and configure Ollama or another local runtime adapter."
                .to_string(),
        );
    }
    let profile = TaskProfile {
        action: "local-ai",
        label: "Local AI Runtime",
        executable: "ollama".to_string(),
        args: vec![
            "run".to_string(),
            model.trim().to_string(),
            prompt.to_string(),
        ],
        description: "Run a prompt through the configured local AI model.",
    };
    run_command_capture(Path::new("."), &profile, 120)
}

fn run_ssh_command(host: &str, user: &str, remote_command: &str) -> Result<CommandCapture, String> {
    if remote_command.trim().is_empty() {
        return Err("Remote command is required.".to_string());
    }
    let target = if user.trim().is_empty() {
        host.trim().to_string()
    } else {
        format!("{}@{}", user.trim(), host.trim())
    };
    if target.trim().is_empty() {
        return Err("SSH host is required.".to_string());
    }
    if !command_exists("ssh") {
        return Err("ssh executable was not found.".to_string());
    }
    let profile = TaskProfile {
        action: "ssh-command",
        label: "SSH Command",
        executable: "ssh".to_string(),
        args: vec![
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ConnectTimeout=8".to_string(),
            target,
            remote_command.trim().to_string(),
        ],
        description: "Run one guarded remote command through SSH.",
    };
    run_command_capture(Path::new("."), &profile, 30)
}

fn terminal_command(
    host: &str,
    user: &str,
    local: bool,
) -> Result<(String, String, Vec<String>, bool), String> {
    let (label, base_executable, base_args) = if local {
        ("Local Shell".to_string(), default_local_shell(), Vec::new())
    } else {
        if !command_exists("ssh") {
            return Err("ssh executable was not found.".to_string());
        }
        let target = if user.trim().is_empty() {
            host.trim().to_string()
        } else {
            format!("{}@{}", user.trim(), host.trim())
        };
        if target.is_empty() {
            return Err("SSH host is required.".to_string());
        }
        (
            format!("SSH {target}"),
            "ssh".to_string(),
            vec!["-tt".to_string(), target],
        )
    };

    if !command_exists(&base_executable) && !Path::new(&base_executable).exists() {
        return Err(format!("{base_executable} executable was not found."));
    }

    if !cfg!(target_os = "windows") && command_exists("script") {
        if cfg!(target_os = "macos") || cfg!(target_os = "freebsd") || cfg!(target_os = "openbsd") {
            let mut args = vec!["-q".to_string(), "/dev/null".to_string(), base_executable];
            args.extend(base_args);
            return Ok((format!("{label} (PTY)"), "script".to_string(), args, true));
        }
        let command = command_line(&base_executable, &base_args);
        return Ok((
            format!("{label} (PTY)"),
            "script".to_string(),
            vec![
                "-q".to_string(),
                "-c".to_string(),
                command,
                "/dev/null".to_string(),
            ],
            true,
        ));
    }

    Ok((label, base_executable, base_args, false))
}

fn default_local_shell() -> String {
    if cfg!(target_os = "windows") {
        if command_exists("pwsh") {
            "pwsh".to_string()
        } else {
            "cmd".to_string()
        }
    } else {
        std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string())
    }
}

fn start_terminal_session(
    host: &str,
    user: &str,
    local: bool,
) -> Result<serde_json::Value, String> {
    let id = record_id(if local {
        "local-terminal"
    } else {
        "ssh-terminal"
    });
    let (label, executable, args, pty_backed) = terminal_command(host, user, local)?;
    let command_line = command_line(&executable, &args);
    let mut child = Command::new(&executable)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("Unable to start terminal session '{command_line}': {error}"))?;
    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "Unable to open terminal stdin".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Unable to capture terminal stdout".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "Unable to capture terminal stderr".to_string())?;
    let output = Arc::new(Mutex::new(format!("$ {command_line}\n")));
    spawn_terminal_reader(stdout, Arc::clone(&output));
    spawn_terminal_reader(stderr, Arc::clone(&output));
    let session = TerminalSession {
        id: id.clone(),
        label,
        command_line,
        pty_backed,
        child: Arc::new(Mutex::new(child)),
        stdin: Arc::new(Mutex::new(stdin)),
        output,
        started_at_epoch: now_epoch_seconds(),
    };
    terminal_sessions()
        .lock()
        .map_err(|_| "Terminal session lock is poisoned".to_string())?
        .insert(id.clone(), session);
    terminal_session_snapshot(&id)
}

fn spawn_terminal_reader<R>(mut reader: R, output: Arc<Mutex<String>>)
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(count) => {
                    let text = String::from_utf8_lossy(&buffer[..count]);
                    if let Ok(mut output) = output.lock() {
                        output.push_str(&text);
                        if output.len() > 200_000 {
                            let trimmed = output
                                .chars()
                                .rev()
                                .take(160_000)
                                .collect::<String>()
                                .chars()
                                .rev()
                                .collect::<String>();
                            *output = trimmed;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });
}

fn send_terminal_input(session_id: &str, input: &str) -> Result<serde_json::Value, String> {
    if session_id.trim().is_empty() {
        return Err("Terminal session id is required.".to_string());
    }
    let mut sessions = terminal_sessions()
        .lock()
        .map_err(|_| "Terminal session lock is poisoned".to_string())?;
    let session = sessions
        .get_mut(session_id)
        .ok_or_else(|| format!("Unknown terminal session: {session_id}"))?;
    let mut text = input.to_string();
    if !text.ends_with('\n') {
        text.push('\n');
    }
    session
        .stdin
        .lock()
        .map_err(|_| "Terminal stdin lock is poisoned".to_string())?
        .write_all(text.as_bytes())
        .map_err(|error| format!("Unable to write terminal input: {error}"))?;
    drop(sessions);
    terminal_session_snapshot(session_id)
}

fn stop_terminal_session(session_id: &str) -> Result<serde_json::Value, String> {
    if session_id.trim().is_empty() {
        return Err("Terminal session id is required.".to_string());
    }
    let mut sessions = terminal_sessions()
        .lock()
        .map_err(|_| "Terminal session lock is poisoned".to_string())?;
    let Some(session) = sessions.remove(session_id) else {
        return Err(format!("Unknown terminal session: {session_id}"));
    };
    let _ = session
        .child
        .lock()
        .map_err(|_| "Terminal child lock is poisoned".to_string())?
        .kill();
    Ok(serde_json::json!({
        "id": session.id,
        "label": session.label,
        "commandLine": session.command_line,
        "ptyBacked": session.pty_backed,
        "output": session.output.lock().map(|output| output.clone()).unwrap_or_default(),
        "running": false,
        "exitCode": null,
        "startedAtEpoch": session.started_at_epoch,
    }))
}

fn terminal_session_snapshot(session_id: &str) -> Result<serde_json::Value, String> {
    if session_id.trim().is_empty() {
        return Err("Terminal session id is required.".to_string());
    }
    let mut sessions = terminal_sessions()
        .lock()
        .map_err(|_| "Terminal session lock is poisoned".to_string())?;
    let session = sessions
        .get_mut(session_id)
        .ok_or_else(|| format!("Unknown terminal session: {session_id}"))?;
    let status = session
        .child
        .lock()
        .map_err(|_| "Terminal child lock is poisoned".to_string())?
        .try_wait()
        .map_err(|error| format!("Unable to read terminal status: {error}"))?;
    Ok(serde_json::json!({
        "id": session.id,
        "label": session.label,
        "commandLine": session.command_line,
        "ptyBacked": session.pty_backed,
        "output": session.output.lock().map(|output| output.clone()).unwrap_or_default(),
        "running": status.is_none(),
        "exitCode": status.and_then(|status| status.code()),
        "startedAtEpoch": session.started_at_epoch,
    }))
}

fn enqueue_transfer(
    workspace_root: &Path,
    direction: &str,
    local_path: &str,
    remote_target: &str,
) -> Result<Vec<String>, String> {
    validate_transfer(direction, local_path, remote_target)?;
    save_named_record(
        workspace_root,
        ".meditor/transfers/queue.json",
        serde_json::json!({
            "id": record_id("transfer"),
            "direction": direction,
            "local_path": local_path,
            "remote_target": remote_target,
            "status": "Queued",
            "created_at_epoch": now_epoch_seconds(),
        }),
    )
}

fn run_file_transfer(
    workspace_root: &Path,
    direction: &str,
    local_path: &str,
    remote_target: &str,
) -> Result<CommandCapture, String> {
    validate_transfer(direction, local_path, remote_target)?;
    if !command_exists("scp") {
        return Err("scp executable was not found.".to_string());
    }
    let local = resolve_workspace_path(workspace_root, local_path)?;
    if direction == "upload" && !local.exists() {
        return Err(format!("Local upload source does not exist: {local_path}"));
    }
    if direction == "download" {
        ensure_parent(&local)?;
    }
    let args = if direction == "upload" {
        vec![display_path(&local), remote_target.trim().to_string()]
    } else {
        vec![remote_target.trim().to_string(), display_path(&local)]
    };
    let profile = TaskProfile {
        action: "file-transfer",
        label: "SFTP/SCP Transfer",
        executable: "scp".to_string(),
        args,
        description: "Run one queued SCP upload/download.",
    };
    run_command_capture(workspace_root, &profile, 120)
}

fn validate_transfer(direction: &str, local_path: &str, remote_target: &str) -> Result<(), String> {
    if !matches!(direction, "upload" | "download") {
        return Err("Transfer direction must be upload or download.".to_string());
    }
    if local_path.trim().is_empty() {
        return Err("Local path is required.".to_string());
    }
    if remote_target.trim().is_empty() || !remote_target.contains(':') {
        return Err("Remote target must look like user@host:/path.".to_string());
    }
    Ok(())
}

fn run_jdbc_sql(
    workspace_root: &Path,
    jar_path: &str,
    jdbc_url: &str,
    db_user: &str,
    db_password: &str,
    sql: &str,
) -> Result<CommandCapture, String> {
    if sql.trim().is_empty() {
        return Err("SQL text is empty.".to_string());
    }
    if jdbc_url.trim().is_empty() {
        return Err("JDBC URL is required.".to_string());
    }
    if !command_exists("javac") || !command_exists("java") {
        return Err(
            "Java JDK tools javac and java are required for live JDBC execution.".to_string(),
        );
    }
    let driver_jar = resolve_local_file(workspace_root, jar_path)?;
    let runner_root = workspace_root.join(".meditor/jdbc-runner");
    let classes_dir = runner_root.join("classes");
    fs::create_dir_all(&classes_dir)
        .map_err(|error| format!("Unable to create JDBC runner folder: {error}"))?;
    let source_path = runner_root.join("JdbcRunner.java");
    fs::write(&source_path, JDBC_RUNNER_SOURCE)
        .map_err(|error| format!("Unable to write JDBC runner source: {error}"))?;

    let javac_profile = TaskProfile {
        action: "jdbc-compile",
        label: "JDBC Runner Compile",
        executable: "javac".to_string(),
        args: vec![
            "-d".to_string(),
            display_path(&classes_dir),
            "-cp".to_string(),
            display_path(&driver_jar),
            display_path(&source_path),
        ],
        description: "Compile the local JDBC runner.",
    };
    let compile = run_command_capture(workspace_root, &javac_profile, 60)?;
    if !compile.success {
        return Ok(compile);
    }

    let classpath_separator = if cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };
    let classpath = format!(
        "{}{}{}",
        display_path(&classes_dir),
        classpath_separator,
        display_path(&driver_jar)
    );
    let java_profile = TaskProfile {
        action: "jdbc-run",
        label: "JDBC SQL",
        executable: "java".to_string(),
        args: vec![
            "-cp".to_string(),
            classpath,
            "JdbcRunner".to_string(),
            jdbc_url.trim().to_string(),
            db_user.trim().to_string(),
            sql.trim().to_string(),
        ],
        description: "Run SQL through a JDBC connection.",
    };
    run_command_capture_with_env(
        workspace_root,
        &java_profile,
        120,
        &[("mEditor_JDBC_PASSWORD", db_password.to_string())],
    )
}

fn run_jdbc_metadata(
    workspace_root: &Path,
    jar_path: &str,
    jdbc_url: &str,
    db_user: &str,
    db_password: &str,
    mode: &str,
    schema: &str,
    object: &str,
) -> Result<CommandCapture, String> {
    if jdbc_url.trim().is_empty() {
        return Err("JDBC URL is required.".to_string());
    }
    if !command_exists("javac") || !command_exists("java") {
        return Err(
            "Java JDK tools javac and java are required for JDBC metadata browsing.".to_string(),
        );
    }
    let driver_jar = resolve_local_file(workspace_root, jar_path)?;
    let runner_root = workspace_root.join(".meditor/jdbc-runner");
    let classes_dir = runner_root.join("classes");
    fs::create_dir_all(&classes_dir)
        .map_err(|error| format!("Unable to create JDBC runner folder: {error}"))?;
    let source_path = runner_root.join("JdbcMetaRunner.java");
    fs::write(&source_path, JDBC_META_RUNNER_SOURCE)
        .map_err(|error| format!("Unable to write JDBC metadata runner source: {error}"))?;

    let javac_profile = TaskProfile {
        action: "jdbc-meta-compile",
        label: "JDBC Metadata Runner Compile",
        executable: "javac".to_string(),
        args: vec![
            "-d".to_string(),
            display_path(&classes_dir),
            "-cp".to_string(),
            display_path(&driver_jar),
            display_path(&source_path),
        ],
        description: "Compile the local JDBC metadata runner.",
    };
    let compile = run_command_capture(workspace_root, &javac_profile, 60)?;
    if !compile.success {
        return Ok(compile);
    }
    let classpath_separator = if cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };
    let classpath = format!(
        "{}{}{}",
        display_path(&classes_dir),
        classpath_separator,
        display_path(&driver_jar)
    );
    let java_profile = TaskProfile {
        action: "jdbc-meta-run",
        label: "JDBC Metadata",
        executable: "java".to_string(),
        args: vec![
            "-cp".to_string(),
            classpath,
            "JdbcMetaRunner".to_string(),
            jdbc_url.trim().to_string(),
            db_user.trim().to_string(),
            mode.trim().to_string(),
            schema.trim().to_string(),
            object.trim().to_string(),
        ],
        description: "Browse JDBC metadata for object browser and DBA dashboard.",
    };
    run_command_capture_with_env(
        workspace_root,
        &java_profile,
        120,
        &[("mEditor_JDBC_PASSWORD", db_password.to_string())],
    )
}

fn jdbc_probe_sql(probe: &str) -> Option<&'static str> {
    match probe {
        "generic_version" => Some("select 1 as meditor_connection_probe"),
        "oracle_patch_history" => Some(
            "select action_time, action, namespace, version, comments from dba_registry_history order by action_time desc",
        ),
        "oracle_rman_backup" => Some(
            "select start_time, end_time, input_type, status from v$rman_backup_job_details order by start_time desc fetch first 50 rows only",
        ),
        "oracle_sessions" => {
            Some("select status, count(*) as session_count from v$session group by status")
        }
        "postgres_activity" => Some(
            "select datname, usename, state, count(*) as session_count from pg_stat_activity group by datname, usename, state",
        ),
        "postgres_database_size" => Some(
            "select datname, pg_database_size(datname) as bytes from pg_database order by bytes desc",
        ),
        "mysql_processlist" => {
            Some("select user, host, db, command, state, time from information_schema.processlist limit 100")
        }
        "mysql_schema_size" => Some(
            "select table_schema, count(*) as tables, sum(data_length + index_length) as bytes from information_schema.tables group by table_schema order by bytes desc",
        ),
        "sqlserver_sessions" => {
            Some("select status, count(*) as session_count from sys.dm_exec_sessions group by status")
        }
        "sqlserver_database_size" => Some(
            "select db_name(database_id) as database_name, sum(size) * 8 * 1024 as bytes from sys.master_files group by database_id order by bytes desc",
        ),
        _ => None,
    }
}

fn run_jdbc_dba_probe(
    workspace_root: &Path,
    jar_path: &str,
    jdbc_url: &str,
    db_user: &str,
    db_password: &str,
    probe: &str,
) -> Result<CommandCapture, String> {
    let sql = jdbc_probe_sql(probe).ok_or_else(|| {
        format!(
            "Unsupported DBA probe '{probe}'. Choose a built-in generic, Oracle, PostgreSQL, MySQL, or SQL Server probe."
        )
    })?;
    run_jdbc_sql(
        workspace_root,
        jar_path,
        jdbc_url,
        db_user,
        db_password,
        sql,
    )
}

fn resolve_local_file(workspace_root: &Path, value: &str) -> Result<PathBuf, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("File path is required.".to_string());
    }
    let path = Path::new(trimmed);
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        resolve_workspace_path(workspace_root, trimmed)?
    };
    if !resolved.is_file() {
        return Err(format!("File does not exist: {}", display_path(&resolved)));
    }
    Ok(resolved)
}

const JDBC_RUNNER_SOURCE: &str = r#"import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.Statement;

public final class JdbcRunner {
  public static void main(String[] args) throws Exception {
    String url = args[0];
    String user = args[1];
    String sql = args[2];
    String password = System.getenv("mEditor_JDBC_PASSWORD");
    try (Connection connection = user == null || user.isBlank()
        ? DriverManager.getConnection(url)
        : DriverManager.getConnection(url, user, password == null ? "" : password);
        Statement statement = connection.createStatement()) {
      boolean hasResultSet = statement.execute(sql);
      if (!hasResultSet) {
        System.out.println("Update count\t" + statement.getUpdateCount());
        return;
      }
      try (ResultSet rs = statement.getResultSet()) {
        ResultSetMetaData meta = rs.getMetaData();
        int columns = meta.getColumnCount();
        for (int index = 1; index <= columns; index++) {
          if (index > 1) {
            System.out.print("\t");
          }
          System.out.print(meta.getColumnLabel(index));
        }
        System.out.println();
        int rows = 0;
        while (rs.next() && rows < 200) {
          for (int index = 1; index <= columns; index++) {
            if (index > 1) {
              System.out.print("\t");
            }
            Object value = rs.getObject(index);
            System.out.print(value == null ? "[null]" : String.valueOf(value));
          }
          System.out.println();
          rows++;
        }
        System.out.println("-- rows shown: " + rows);
      }
    }
  }
}
"#;

const JDBC_META_RUNNER_SOURCE: &str = r#"import java.sql.Connection;
import java.sql.DatabaseMetaData;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.util.Arrays;

public final class JdbcMetaRunner {
  public static void main(String[] args) throws Exception {
    String url = args[0];
    String user = args[1];
    String mode = args[2];
    String schema = args.length > 3 && !args[3].isBlank() ? args[3] : null;
    String object = args.length > 4 && !args[4].isBlank() ? args[4] : "%";
    String password = System.getenv("mEditor_JDBC_PASSWORD");
    try (Connection connection = user == null || user.isBlank()
        ? DriverManager.getConnection(url)
        : DriverManager.getConnection(url, user, password == null ? "" : password)) {
      DatabaseMetaData meta = connection.getMetaData();
      if ("dashboard".equals(mode)) {
        System.out.println("metric\tvalue");
        System.out.println("database_product\t" + meta.getDatabaseProductName());
        System.out.println("database_version\t" + meta.getDatabaseProductVersion());
        System.out.println("driver\t" + meta.getDriverName() + " " + meta.getDriverVersion());
        System.out.println("user\t" + meta.getUserName());
        System.out.println("catalog\t" + connection.getCatalog());
        try {
          System.out.println("schema\t" + connection.getSchema());
        } catch (Throwable ignored) {
          System.out.println("schema\t[driver does not expose getSchema]");
        }
        System.out.println("read_only\t" + connection.isReadOnly());
        System.out.println("auto_commit\t" + connection.getAutoCommit());
        return;
      }
      if ("catalogs".equals(mode)) {
        dump(meta.getCatalogs(), new String[] {"TABLE_CAT"}, 500);
        return;
      }
      if ("schemas".equals(mode)) {
        dump(meta.getSchemas(), new String[] {"TABLE_SCHEM", "TABLE_CATALOG"}, 500);
        return;
      }
      if ("columns".equals(mode)) {
        dump(meta.getColumns(null, schema, object, "%"), new String[] {
          "TABLE_CAT", "TABLE_SCHEM", "TABLE_NAME", "COLUMN_NAME", "TYPE_NAME",
          "COLUMN_SIZE", "DECIMAL_DIGITS", "NULLABLE", "COLUMN_DEF", "ORDINAL_POSITION",
          "IS_AUTOINCREMENT", "IS_GENERATEDCOLUMN"
        }, 500);
        return;
      }
      if ("indexes".equals(mode)) {
        dump(meta.getIndexInfo(null, schema, object, false, false), new String[] {
          "TABLE_CAT", "TABLE_SCHEM", "TABLE_NAME", "NON_UNIQUE", "INDEX_QUALIFIER",
          "INDEX_NAME", "TYPE", "ORDINAL_POSITION", "COLUMN_NAME", "ASC_OR_DESC",
          "CARDINALITY", "PAGES", "FILTER_CONDITION"
        }, 500);
        return;
      }
      if ("primaryKeys".equals(mode)) {
        dump(meta.getPrimaryKeys(null, schema, object), new String[] {
          "TABLE_CAT", "TABLE_SCHEM", "TABLE_NAME", "COLUMN_NAME", "KEY_SEQ", "PK_NAME"
        }, 500);
        return;
      }
      if ("foreignKeys".equals(mode)) {
        dump(meta.getImportedKeys(null, schema, object), new String[] {
          "PKTABLE_CAT", "PKTABLE_SCHEM", "PKTABLE_NAME", "PKCOLUMN_NAME",
          "FKTABLE_CAT", "FKTABLE_SCHEM", "FKTABLE_NAME", "FKCOLUMN_NAME",
          "KEY_SEQ", "UPDATE_RULE", "DELETE_RULE", "FK_NAME", "PK_NAME", "DEFERRABILITY"
        }, 500);
        return;
      }
      if ("procedures".equals(mode)) {
        dump(meta.getProcedures(null, schema, object), new String[] {
          "PROCEDURE_CAT", "PROCEDURE_SCHEM", "PROCEDURE_NAME", "REMARKS",
          "PROCEDURE_TYPE", "SPECIFIC_NAME"
        }, 500);
        return;
      }
      if ("typeInfo".equals(mode)) {
        dump(meta.getTypeInfo(), new String[] {
          "TYPE_NAME", "DATA_TYPE", "PRECISION", "LITERAL_PREFIX", "LITERAL_SUFFIX",
          "CREATE_PARAMS", "NULLABLE", "CASE_SENSITIVE", "SEARCHABLE",
          "UNSIGNED_ATTRIBUTE", "AUTO_INCREMENT", "LOCAL_TYPE_NAME"
        }, 500);
        return;
      }
      if ("tablePrivileges".equals(mode)) {
        dump(meta.getTablePrivileges(null, schema, object), new String[] {
          "TABLE_CAT", "TABLE_SCHEM", "TABLE_NAME", "GRANTOR", "GRANTEE",
          "PRIVILEGE", "IS_GRANTABLE"
        }, 500);
        return;
      }
      dump(meta.getTables(null, schema, object, new String[] {"TABLE", "VIEW", "SYSTEM TABLE", "SYNONYM", "ALIAS"}), new String[] {
        "TABLE_CAT", "TABLE_SCHEM", "TABLE_NAME", "TABLE_TYPE", "REMARKS", "TYPE_CAT",
        "TYPE_SCHEM", "TYPE_NAME", "SELF_REFERENCING_COL_NAME", "REF_GENERATION"
      }, 500);
    }
  }

  private static void dump(ResultSet rs, String[] columns, int limit) throws Exception {
    try (ResultSet rows = rs) {
      System.out.println(String.join("\t", Arrays.asList(columns)));
      int count = 0;
      while (rows.next() && count < limit) {
        for (int index = 0; index < columns.length; index++) {
          if (index > 0) {
            System.out.print("\t");
          }
          System.out.print(value(rows, columns[index]));
        }
        System.out.println();
        count++;
      }
      System.out.println("-- rows shown: " + count);
    }
  }

  private static String value(ResultSet rs, String column) throws Exception {
    try {
      Object value = rs.getObject(column);
      return value == null ? "[null]" : String.valueOf(value).replace('\t', ' ');
    } catch (Throwable ignored) {
      return "[n/a]";
    }
  }
}
"#;

fn sanitize_file_stem(value: &str) -> String {
    let mut output = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    while output.contains("--") {
        output = output.replace("--", "-");
    }
    let output = output.trim_matches('-').to_string();
    if output.is_empty() {
        "knowledge".to_string()
    } else {
        output
    }
}

fn validate_editor_text(kind: &str, content: &str) -> (bool, Vec<String>) {
    match kind {
        "json" => match serde_json::from_str::<serde_json::Value>(content) {
            Ok(_) => (true, vec!["JSON is valid".to_string()]),
            Err(error) => (false, vec![format!("JSON error: {error}")]),
        },
        "xml" => {
            let mut messages = Vec::new();
            for event in EventReader::from_str(content) {
                if let Err(error) = event {
                    messages.push(format!("XML error: {error}"));
                    break;
                }
            }
            if messages.is_empty() {
                (true, vec!["XML is well formed".to_string()])
            } else {
                (false, messages)
            }
        }
        other => (false, vec![format!("Unknown validator kind: {other}")]),
    }
}

fn reformat_editor_text(kind: &str, content: &str) -> Result<String, String> {
    match kind {
        "json" => {
            let value = serde_json::from_str::<serde_json::Value>(content).map_err(|error| {
                format!("JSON cannot be reformatted until it is valid: {error}")
            })?;
            serde_json::to_string_pretty(&value)
                .map_err(|error| format!("Unable to format JSON: {error}"))
        }
        "xml" => format_xml_text(content),
        other => Err(format!("Unknown formatter kind: {other}")),
    }
}

fn format_xml_text(content: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut depth = 0usize;
    for event in EventReader::from_str(content) {
        match event
            .map_err(|error| format!("XML cannot be reformatted until it is valid: {error}"))?
        {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                write_indent(&mut output, depth);
                output.push('<');
                output.push_str(&xml_name(&name));
                for attribute in attributes {
                    output.push(' ');
                    output.push_str(&xml_name(&attribute.name));
                    output.push_str("=\"");
                    output.push_str(&xml_attr_escape(&attribute.value));
                    output.push('"');
                }
                output.push_str(">\n");
                depth += 1;
            }
            XmlEvent::EndElement { name } => {
                depth = depth.saturating_sub(1);
                write_indent(&mut output, depth);
                output.push_str("</");
                output.push_str(&xml_name(&name));
                output.push_str(">\n");
            }
            XmlEvent::Characters(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    write_indent(&mut output, depth);
                    output.push_str(&xml_text_escape(trimmed));
                    output.push('\n');
                }
            }
            XmlEvent::CData(data) => {
                write_indent(&mut output, depth);
                output.push_str("<![CDATA[");
                output.push_str(&data);
                output.push_str("]]>\n");
            }
            XmlEvent::Comment(comment) => {
                write_indent(&mut output, depth);
                output.push_str("<!--");
                output.push_str(comment.trim());
                output.push_str("-->\n");
            }
            XmlEvent::ProcessingInstruction { name, data } => {
                write_indent(&mut output, depth);
                output.push_str("<?");
                output.push_str(&name);
                if let Some(data) = data {
                    output.push(' ');
                    output.push_str(data.trim());
                }
                output.push_str("?>\n");
            }
            XmlEvent::StartDocument { .. } | XmlEvent::EndDocument | XmlEvent::Whitespace(_) => {}
        }
    }
    Ok(output)
}

fn write_indent(output: &mut String, depth: usize) {
    for _ in 0..depth {
        output.push_str("  ");
    }
}

fn xml_name(name: &OwnedName) -> String {
    match &name.prefix {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}:{}", name.local_name),
        _ => name.local_name.clone(),
    }
}

fn xml_text_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn xml_attr_escape(value: &str) -> String {
    xml_text_escape(value).replace('"', "&quot;")
}

#[derive(Clone, Debug)]
struct TaskProfile {
    action: &'static str,
    label: &'static str,
    executable: String,
    args: Vec<String>,
    description: &'static str,
}

#[derive(Clone, Debug)]
struct ProjectDetection {
    root: PathBuf,
    relative_path: String,
    kind: &'static str,
    tasks: Vec<TaskProfile>,
    messages: Vec<String>,
}

#[derive(Clone, Debug)]
struct TaskRunResult {
    project_path: String,
    project_kind: &'static str,
    task_action: String,
    task_label: String,
    command_line: String,
    exit_code: Option<i32>,
    success: bool,
    timed_out: bool,
    duration_ms: u128,
    stdout: String,
    stderr: String,
    diagnostics: Vec<String>,
    suggestions: Vec<String>,
    report_path: String,
}

fn task_profile_json(profile: &TaskProfile) -> serde_json::Value {
    serde_json::json!({
        "action": profile.action,
        "label": profile.label,
        "executable": profile.executable,
        "args": profile.args,
        "description": profile.description,
        "commandLine": command_line(&profile.executable, &profile.args),
        "ready": command_exists(&profile.executable) || profile.executable.starts_with("./"),
    })
}

fn detect_project(workspace_root: &Path, project_path: &str) -> Result<ProjectDetection, String> {
    let project_path = if project_path.trim().is_empty() {
        "."
    } else {
        project_path.trim()
    };
    let root = resolve_workspace_path(workspace_root, project_path)?;
    if !root.is_dir() {
        return Err(format!("{project_path} is not a project folder"));
    }

    let relative_path = normalize_project_path(project_path);
    let mut messages = vec![format!("Project folder: {}", display_path(&root))];
    let (kind, tasks) = if root.join("Cargo.toml").exists() {
        (
            "Rust/Cargo",
            vec![
                task(
                    "build",
                    "Build",
                    "cargo",
                    &["build"],
                    "Compile the Cargo project.",
                ),
                task(
                    "clean",
                    "Clean",
                    "cargo",
                    &["clean"],
                    "Remove Cargo build output.",
                ),
                task("test", "Test", "cargo", &["test"], "Run Cargo tests."),
                task(
                    "launch",
                    "Launch",
                    "cargo",
                    &["run"],
                    "Run the Cargo binary target.",
                ),
                task(
                    "debug",
                    "Debug",
                    "cargo",
                    &["check"],
                    "Capture compiler diagnostics without producing a binary.",
                ),
                task(
                    "package",
                    "Package",
                    "cargo",
                    &["build", "--release"],
                    "Build a release binary.",
                ),
            ],
        )
    } else if root.join("package.json").exists() {
        (
            "Node.js/package.json",
            vec![
                task(
                    "build",
                    "Build",
                    "npm",
                    &["run", "build"],
                    "Run the package build script.",
                ),
                task(
                    "clean",
                    "Clean",
                    "npm",
                    &["run", "clean"],
                    "Run the package clean script when present.",
                ),
                task("test", "Test", "npm", &["test"], "Run package tests."),
                task(
                    "launch",
                    "Launch",
                    "npm",
                    &["start"],
                    "Start the package application.",
                ),
                task(
                    "debug",
                    "Debug",
                    "npm",
                    &["run", "lint"],
                    "Run lint as the first debugging pass.",
                ),
            ],
        )
    } else if root.join("pom.xml").exists() {
        (
            "Java/Maven",
            vec![
                task(
                    "build",
                    "Build",
                    "mvn",
                    &["package"],
                    "Build the Maven project.",
                ),
                task(
                    "clean",
                    "Clean",
                    "mvn",
                    &["clean"],
                    "Clean the Maven project.",
                ),
                task("test", "Test", "mvn", &["test"], "Run Maven tests."),
                task(
                    "debug",
                    "Debug",
                    "mvn",
                    &["test"],
                    "Capture Maven compiler/test diagnostics.",
                ),
                task(
                    "package",
                    "Package",
                    "mvn",
                    &["package"],
                    "Create Maven package artifacts.",
                ),
            ],
        )
    } else if root.join("build.gradle").exists()
        || root.join("build.gradle.kts").exists()
        || root.join("gradlew").exists()
    {
        let executable = if root.join("gradlew").exists() {
            if cfg!(target_os = "windows") {
                "gradlew.bat"
            } else {
                "./gradlew"
            }
        } else {
            "gradle"
        };
        (
            "Java/Gradle",
            vec![
                task(
                    "build",
                    "Build",
                    executable,
                    &["build"],
                    "Build the Gradle project.",
                ),
                task(
                    "clean",
                    "Clean",
                    executable,
                    &["clean"],
                    "Clean the Gradle project.",
                ),
                task("test", "Test", executable, &["test"], "Run Gradle tests."),
                task(
                    "launch",
                    "Launch",
                    executable,
                    &["run"],
                    "Run the Gradle application plugin task.",
                ),
                task(
                    "debug",
                    "Debug",
                    executable,
                    &["test"],
                    "Capture Gradle compiler/test diagnostics.",
                ),
            ],
        )
    } else if root.join("Makefile").exists() || root.join("makefile").exists() {
        (
            "Makefile",
            vec![
                task(
                    "build",
                    "Build",
                    "make",
                    &[],
                    "Run the default Make target.",
                ),
                task("clean", "Clean", "make", &["clean"], "Run make clean."),
                task("test", "Test", "make", &["test"], "Run make test."),
                task("launch", "Launch", "make", &["run"], "Run make run."),
                task(
                    "debug",
                    "Debug",
                    "make",
                    &["check"],
                    "Run make check when present.",
                ),
            ],
        )
    } else {
        (
            "Generic/source-file",
            vec![
                task(
                    "debug",
                    "Debug Source",
                    python_executable(),
                    &["-m", "py_compile"],
                    "Use the source file field for Python compile checks.",
                ),
                task(
                    "launch",
                    "Launch Source",
                    python_executable(),
                    &[],
                    "Use the source file field for Python launch.",
                ),
            ],
        )
    };

    messages.push(format!("Detected project kind: {kind}"));
    messages.push(format!("Runnable task profiles: {}", tasks.len()));
    Ok(ProjectDetection {
        root,
        relative_path,
        kind,
        tasks,
        messages,
    })
}

fn task(
    action: &'static str,
    label: &'static str,
    executable: &str,
    args: &[&str],
    description: &'static str,
) -> TaskProfile {
    TaskProfile {
        action,
        label,
        executable: executable.to_string(),
        args: args.iter().map(|arg| arg.to_string()).collect(),
        description,
    }
}

fn run_sdlc_task(
    workspace_root: &Path,
    project_path: &str,
    task_action: &str,
    source_path: &str,
    timeout_seconds: u64,
) -> Result<TaskRunResult, String> {
    let project = detect_project(workspace_root, project_path)?;
    let mut profile = project
        .tasks
        .iter()
        .find(|profile| profile.action == task_action)
        .cloned()
        .or_else(|| generic_source_task_profile(task_action, source_path))
        .ok_or_else(|| format!("No task profile found for action '{task_action}'"))?;

    if profile_needs_source(&profile) {
        let source = source_path.trim();
        if source.is_empty() {
            return Err(
                "This task needs a source file path from the active editor or Source field."
                    .to_string(),
            );
        }
        let source_full_path = resolve_workspace_path(workspace_root, source)?;
        if !source_full_path.is_file() {
            return Err(format!("Source file does not exist: {source}"));
        }
        let source_arg = relative_path_for_command(&project.root, &source_full_path, source);
        profile.args.push(source_arg);
    }

    let staging = project.root.join(".meditor/debug-staging");
    fs::create_dir_all(staging.join("compiled"))
        .map_err(|error| format!("Unable to create debug staging area: {error}"))?;
    fs::create_dir_all(staging.join("terminal-runs"))
        .map_err(|error| format!("Unable to create terminal run staging area: {error}"))?;
    fs::create_dir_all(staging.join("transcripts"))
        .map_err(|error| format!("Unable to create transcript staging area: {error}"))?;

    if !profile.executable.starts_with("./") && !command_exists(&profile.executable) {
        return Err(format!(
            "Executable '{}' was not found. Configure it under Setup > Programming Language Support before running {}.",
            profile.executable, profile.label
        ));
    }

    let capture = run_command_capture(&project.root, &profile, timeout_seconds)?;
    let diagnostics = extract_diagnostics(&capture.stdout, &capture.stderr);
    let suggestions = task_suggestions(&capture, &diagnostics, &profile);
    let report_path = write_task_report(&project, &profile, &capture, &diagnostics, &suggestions)?;
    let relative_report = report_path
        .strip_prefix(workspace_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or_else(|| report_path.to_str().unwrap_or(""))
        .to_string();

    Ok(TaskRunResult {
        project_path: project.relative_path,
        project_kind: project.kind,
        task_action: profile.action.to_string(),
        task_label: profile.label.to_string(),
        command_line: capture.command_line,
        exit_code: capture.exit_code,
        success: capture.success,
        timed_out: capture.timed_out,
        duration_ms: capture.duration_ms,
        stdout: capture.stdout,
        stderr: capture.stderr,
        diagnostics,
        suggestions,
        report_path: relative_report,
    })
}

#[derive(Clone, Debug)]
struct CommandCapture {
    command_line: String,
    exit_code: Option<i32>,
    success: bool,
    timed_out: bool,
    duration_ms: u128,
    stdout: String,
    stderr: String,
}

fn run_command_capture(
    root: &Path,
    profile: &TaskProfile,
    timeout_seconds: u64,
) -> Result<CommandCapture, String> {
    run_command_capture_with_env(root, profile, timeout_seconds, &[])
}

fn run_command_capture_with_env(
    root: &Path,
    profile: &TaskProfile,
    timeout_seconds: u64,
    envs: &[(&str, String)],
) -> Result<CommandCapture, String> {
    let started = Instant::now();
    let command_line = command_line(&profile.executable, &profile.args);
    let mut command = Command::new(&profile.executable);
    command
        .args(&profile.args)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in envs {
        command.env(key, value);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("Unable to start '{command_line}': {error}"))?;

    let mut stdout_pipe = child
        .stdout
        .take()
        .ok_or_else(|| "Unable to capture stdout".to_string())?;
    let mut stderr_pipe = child
        .stderr
        .take()
        .ok_or_else(|| "Unable to capture stderr".to_string())?;

    let stdout_thread = thread::spawn(move || {
        let mut output = String::new();
        let _ = stdout_pipe.read_to_string(&mut output);
        output
    });
    let stderr_thread = thread::spawn(move || {
        let mut output = String::new();
        let _ = stderr_pipe.read_to_string(&mut output);
        output
    });

    let timeout = Duration::from_secs(timeout_seconds);
    let mut timed_out = false;
    let status = loop {
        match child
            .try_wait()
            .map_err(|error| format!("Unable to read process status: {error}"))?
        {
            Some(status) => break status,
            None if started.elapsed() >= timeout => {
                timed_out = true;
                let _ = child.kill();
                break child
                    .wait()
                    .map_err(|error| format!("Unable to stop timed-out process: {error}"))?;
            }
            None => thread::sleep(Duration::from_millis(80)),
        }
    };

    let stdout = stdout_thread.join().unwrap_or_default();
    let stderr = stderr_thread.join().unwrap_or_default();
    Ok(CommandCapture {
        command_line,
        exit_code: status.code(),
        success: status.success() && !timed_out,
        timed_out,
        duration_ms: started.elapsed().as_millis(),
        stdout,
        stderr,
    })
}

fn generic_source_task_profile(task_action: &str, source_path: &str) -> Option<TaskProfile> {
    let lower = source_path.to_lowercase();
    match (task_action, lower.rsplit('.').next()) {
        ("debug" | "build", Some("py")) => Some(task(
            "debug",
            "Debug Source",
            python_executable(),
            &["-m", "py_compile"],
            "Compile-check one Python source file.",
        )),
        ("launch", Some("py")) => Some(task(
            "launch",
            "Launch Source",
            python_executable(),
            &[],
            "Run one Python source file.",
        )),
        ("launch" | "debug", Some("js")) => Some(task(
            "launch",
            "Launch Source",
            "node",
            &[],
            "Run one JavaScript source file with Node.js.",
        )),
        ("launch" | "debug", Some("sh")) => Some(task(
            "launch",
            "Launch Source",
            "sh",
            &[],
            "Run one shell script.",
        )),
        _ => None,
    }
}

fn python_executable() -> &'static str {
    if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    }
}

fn profile_needs_source(profile: &TaskProfile) -> bool {
    matches!(
        profile.description,
        "Use the source file field for Python compile checks."
            | "Use the source file field for Python launch."
            | "Compile-check one Python source file."
            | "Run one Python source file."
            | "Run one JavaScript source file with Node.js."
            | "Run one shell script."
    )
}

fn relative_path_for_command(
    project_root: &Path,
    source_full_path: &Path,
    fallback: &str,
) -> String {
    source_full_path
        .strip_prefix(project_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or(fallback)
        .to_string()
}

fn extract_diagnostics(stdout: &str, stderr: &str) -> Vec<String> {
    let combined = format!("{stderr}\n{stdout}");
    combined
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            lower.contains("error")
                || lower.contains("warning")
                || lower.contains("failed")
                || lower.contains("exception")
                || lower.contains("panic")
                || lower.contains("traceback")
        })
        .take(40)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
}

fn task_suggestions(
    capture: &CommandCapture,
    diagnostics: &[String],
    profile: &TaskProfile,
) -> Vec<String> {
    let mut suggestions = Vec::new();
    if capture.timed_out {
        suggestions.push("The task reached its timeout. Increase timeout only for trusted long-running launch/debug tasks, or stop background servers before retrying.".to_string());
    }
    if capture.success && diagnostics.is_empty() {
        suggestions.push("No compiler errors or warnings were captured. Continue with the next workflow step or run tests/package as needed.".to_string());
    } else {
        suggestions.push(format!(
            "Review the first diagnostic from {} before editing code. mEditor has not changed files automatically.",
            profile.label
        ));
        suggestions.push("Use user-controlled remediation first. Auto-debugging should remain disabled until you explicitly trust a workflow.".to_string());
        if diagnostics
            .iter()
            .any(|line| line.contains("not found") || line.contains("No such file"))
        {
            suggestions.push("Check file paths, project root selection, and missing generated files before changing source logic.".to_string());
        }
        if diagnostics
            .iter()
            .any(|line| line.to_lowercase().contains("unused"))
        {
            suggestions.push("Unused-code warnings can usually be fixed safely, but review impact before deleting code.".to_string());
        }
    }
    suggestions.push("Rollback path: no automated source edits were applied in this cycle; restore only the generated .meditor report/staging files if desired.".to_string());
    suggestions
}

fn write_task_report(
    project: &ProjectDetection,
    profile: &TaskProfile,
    capture: &CommandCapture,
    diagnostics: &[String],
    suggestions: &[String],
) -> Result<PathBuf, String> {
    let report_dir = project.root.join(".meditor/debug-reports");
    fs::create_dir_all(&report_dir)
        .map_err(|error| format!("Unable to create debug report folder: {error}"))?;
    let report_path = report_dir.join(format!("debug-report-{}.md", now_epoch_seconds()));
    let mut report = String::new();
    writeln!(report, "# mEditor Debugging Handover Report").unwrap();
    writeln!(report).unwrap();
    writeln!(report, "- Project path: {}", project.relative_path).unwrap();
    writeln!(report, "- Project kind: {}", project.kind).unwrap();
    writeln!(report, "- Task: {}", profile.label).unwrap();
    writeln!(report, "- Command: `{}`", capture.command_line).unwrap();
    writeln!(report, "- Exit code: {:?}", capture.exit_code).unwrap();
    writeln!(report, "- Success: {}", capture.success).unwrap();
    writeln!(report, "- Timed out: {}", capture.timed_out).unwrap();
    writeln!(report, "- Duration ms: {}", capture.duration_ms).unwrap();
    writeln!(
        report,
        "- User approval mode: user controlled; no automatic source edits applied"
    )
    .unwrap();
    writeln!(
        report,
        "- Rollback path: generated report and staging files under `.meditor` only"
    )
    .unwrap();
    writeln!(report).unwrap();
    writeln!(report, "## Diagnostics").unwrap();
    if diagnostics.is_empty() {
        writeln!(report, "- No compiler/debug diagnostics were captured.").unwrap();
    } else {
        for diagnostic in diagnostics {
            writeln!(report, "- {}", diagnostic).unwrap();
        }
    }
    writeln!(report).unwrap();
    writeln!(report, "## Suggestions").unwrap();
    for suggestion in suggestions {
        writeln!(report, "- {}", suggestion).unwrap();
    }
    writeln!(report).unwrap();
    writeln!(report, "## stdout").unwrap();
    writeln!(report, "```text\n{}\n```", capture.stdout).unwrap();
    writeln!(report).unwrap();
    writeln!(report, "## stderr").unwrap();
    writeln!(report, "```text\n{}\n```", capture.stderr).unwrap();
    fs::write(&report_path, report)
        .map_err(|error| format!("Unable to write debug report: {error}"))?;
    Ok(report_path)
}

fn command_line(executable: &str, args: &[String]) -> String {
    std::iter::once(executable.to_string())
        .chain(args.iter().map(|arg| shell_word(arg)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_project_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.is_empty() || trimmed == "." {
        ".".to_string()
    } else {
        trimmed.trim_start_matches("./").to_string()
    }
}

fn create_project_workspace(
    workspace_root: &Path,
    name: &str,
    kind: &str,
    tech_stack: &str,
) -> Result<Vec<String>, String> {
    let project_id = sanitize_project_name(name)?;
    let project_dir = workspace_root.join(&project_id);
    let metadata = meditor_project_workspace::ProjectMetadataPlan::netbeans_style_for_meditor();
    let project_file = project_dir.join(metadata.project_file);
    if project_file.exists() {
        return Err(format!(
            "Project metadata already exists at {}",
            display_path(&project_file)
        ));
    }

    fs::create_dir_all(project_dir.join(metadata.metadata_folder))
        .map_err(|error| format!("Unable to create project metadata folder: {error}"))?;
    fs::create_dir_all(project_dir.join(".meditor/project/docs"))
        .map_err(|error| format!("Unable to create project docs folder: {error}"))?;
    fs::create_dir_all(project_dir.join("src"))
        .map_err(|error| format!("Unable to create src folder: {error}"))?;

    let created_at = now_epoch_seconds();
    fs::write(
        &project_file,
        format!(
            "name = \"{}\"\nkind = \"{}\"\ntech_stack = \"{}\"\ncreated_by = \"mEditor\"\ncreated_at_epoch = {}\nversion = \"{}\"\n",
            toml_escape(name.trim()),
            toml_escape(kind.trim()),
            toml_escape(tech_stack.trim()),
            created_at,
            meditor_core::CURRENT_BASELINE_VERSION
        ),
    )
    .map_err(|error| format!("Unable to write project.toml: {error}"))?;

    fs::write(
        project_dir.join(metadata.modules_file),
        "[modules]\nroot = \"src\"\nsubprojects = []\n",
    )
    .map_err(|error| format!("Unable to write modules.toml: {error}"))?;

    fs::write(
        project_dir.join(metadata.planning_file),
        "[planning]\nmethodology = \"Objective -> Milestone -> Task\"\ntimezone_required = true\n",
    )
    .map_err(|error| format!("Unable to write planning.toml: {error}"))?;

    fs::write(
        project_dir.join(metadata.private_user_file),
        "[private]\nlast_opened_by = \"\"\n",
    )
    .map_err(|error| format!("Unable to write private-user.toml: {error}"))?;

    fs::write(
        project_dir.join(metadata.about_file),
        format!(
            "# {}\n\nCreated by mEditor.\n\nTech stack: {}\n",
            name.trim(),
            tech_stack.trim()
        ),
    )
    .map_err(|error| format!("Unable to write about.md: {error}"))?;

    fs::write(
        project_dir.join(metadata.help_index_file),
        format!(
            "# {} Help\n\n- Project overview\n- Build and run\n- Deployment\n- Troubleshooting\n",
            name.trim()
        ),
    )
    .map_err(|error| format!("Unable to write help-index.md: {error}"))?;

    fs::write(
        project_dir.join(metadata.help_topics_file),
        "topics = [\"Project overview\", \"Build and run\", \"Deployment\", \"Troubleshooting\"]\n",
    )
    .map_err(|error| format!("Unable to write help-topics.toml: {error}"))?;

    fs::write(
        project_dir.join("src/README.md"),
        format!("# {} Source\n\nAdd source files here.\n", name.trim()),
    )
    .map_err(|error| format!("Unable to write src/README.md: {error}"))?;

    let mut messages = vec![
        format!("Created project folder: {}", display_path(&project_dir)),
        format!("Project metadata: {}", metadata.project_file),
        format!("Embedded About Project: {}", metadata.about_file),
        format!("Embedded Project Help: {}", metadata.help_index_file),
    ];
    messages.extend(scaffold_project_sources(
        &project_dir,
        &project_id,
        tech_stack,
    )?);
    messages.push("File Explorer refreshed.".to_string());
    Ok(messages)
}

fn scaffold_project_sources(
    project_dir: &Path,
    project_id: &str,
    tech_stack: &str,
) -> Result<Vec<String>, String> {
    let lower = tech_stack.to_lowercase();
    if lower.contains("rust") || lower.trim().is_empty() {
        let package_name = rust_package_name(project_id);
        fs::write(
            project_dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
                package_name
            ),
        )
        .map_err(|error| format!("Unable to write Cargo.toml: {error}"))?;
        fs::write(
            project_dir.join("src/main.rs"),
            r#"fn main() {
    println!("Hello from mEditor project");
}
"#,
        )
        .map_err(|error| format!("Unable to write src/main.rs: {error}"))?;
        return Ok(vec![
            "Rust scaffold: Cargo.toml".to_string(),
            "Rust scaffold: src/main.rs".to_string(),
            "Run workflow ready: build, debug/check, test, launch, clean, package.".to_string(),
        ]);
    }

    if lower.contains("python") {
        fs::write(
            project_dir.join("src/main.py"),
            "def main():\n    print(\"Hello from mEditor project\")\n\nif __name__ == \"__main__\":\n    main()\n",
        )
        .map_err(|error| format!("Unable to write src/main.py: {error}"))?;
        return Ok(vec![
            "Python scaffold: src/main.py".to_string(),
            "Run workflow ready with Source field set to src/main.py.".to_string(),
        ]);
    }

    if lower.contains("node")
        || lower.contains("javascript")
        || lower.contains("typescript")
        || lower.contains("react")
    {
        fs::write(
            project_dir.join("package.json"),
            format!(
                "{{\n  \"name\": \"{}\",\n  \"version\": \"0.1.0\",\n  \"private\": true,\n  \"scripts\": {{\n    \"build\": \"node src/main.js\",\n    \"start\": \"node src/main.js\",\n    \"test\": \"node src/main.js\",\n    \"lint\": \"node --check src/main.js\"\n  }}\n}}\n",
                rust_package_name(project_id)
            ),
        )
        .map_err(|error| format!("Unable to write package.json: {error}"))?;
        fs::write(
            project_dir.join("src/main.js"),
            "console.log('Hello from mEditor project');\n",
        )
        .map_err(|error| format!("Unable to write src/main.js: {error}"))?;
        return Ok(vec![
            "Node.js scaffold: package.json".to_string(),
            "Node.js scaffold: src/main.js".to_string(),
            "Run workflow ready: build, test, launch, debug/lint.".to_string(),
        ]);
    }

    if lower.contains("html") || lower.contains("web") {
        fs::create_dir_all(project_dir.join("web"))
            .map_err(|error| format!("Unable to create web folder: {error}"))?;
        fs::write(
            project_dir.join("web/index.html"),
            "<!doctype html>\n<html><head><meta charset=\"utf-8\"><title>mEditor Project</title></head><body><h1>Hello from mEditor project</h1></body></html>\n",
        )
        .map_err(|error| format!("Unable to write web/index.html: {error}"))?;
        return Ok(vec![
            "Web scaffold: web/index.html".to_string(),
            "Use the embedded browser workflow for web preview in a later build.".to_string(),
        ]);
    }

    Ok(vec!["Generic source scaffold created.".to_string()])
}

fn rust_package_name(project_id: &str) -> String {
    let mut output = project_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    while output.contains("__") {
        output = output.replace("__", "_");
    }
    output.trim_matches('_').to_string()
}

fn write_project_import_report(workspace_root: &Path) -> Result<Vec<String>, String> {
    let files = workspace_entries(workspace_root)
        .into_iter()
        .map(|entry| entry.display_path)
        .collect::<Vec<_>>();
    let file_refs = files.iter().map(String::as_str).collect::<Vec<_>>();
    let kind = meditor_project_importers::detect_project_kind(&file_refs);
    let vs_files = file_refs
        .iter()
        .filter_map(|file| {
            meditor_project_importers::classify_visual_studio_file(file)
                .map(|kind| format!("{file} => {kind:?}"))
        })
        .collect::<Vec<_>>();

    let metadata = meditor_project_workspace::ProjectMetadataPlan::netbeans_style_for_meditor();
    let project_folder = workspace_root.join(metadata.metadata_folder);
    fs::create_dir_all(&project_folder)
        .map_err(|error| format!("Unable to create import metadata folder: {error}"))?;
    let report_path = workspace_root.join(metadata.generated_from_import_file);
    let mut report = String::new();
    writeln!(report, "detected_project_kind = \"{kind:?}\"").ok();
    writeln!(report, "files_sampled = {}", file_refs.len()).ok();
    writeln!(report, "created_at_epoch = {}", now_epoch_seconds()).ok();
    writeln!(report, "visual_studio_file_count = {}", vs_files.len()).ok();
    writeln!(report).ok();
    writeln!(report, "[visual_studio_files]").ok();
    for (index, file) in vs_files.iter().enumerate() {
        writeln!(report, "file_{} = \"{}\"", index + 1, toml_escape(file)).ok();
    }
    fs::write(&report_path, report)
        .map_err(|error| format!("Unable to write import report: {error}"))?;

    let mut messages = vec![
        format!("Detected project kind: {kind:?}"),
        format!("Files sampled: {}", file_refs.len()),
        format!("Import report written: {}", display_path(&report_path)),
    ];
    if vs_files.is_empty() {
        messages.push("No Visual Studio project files detected in this sample.".to_string());
    } else {
        messages.push(format!("Visual Studio files detected: {}", vs_files.len()));
        messages.extend(vs_files.into_iter().take(8));
    }
    Ok(messages)
}

fn sanitize_project_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Project name is required".to_string());
    }
    let sanitized = trimmed
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                Some(ch)
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if sanitized.is_empty() {
        Err("Project name must contain letters or numbers".to_string())
    } else {
        Ok(sanitized)
    }
}

fn toml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn json_error(action: &str, error: impl ToString) -> serde_json::Value {
    serde_json::json!({
        "action": action,
        "ok": false,
        "error": error.to_string(),
    })
}

fn generic_action_result(
    target: impl ToString,
    title: impl ToString,
    messages: Vec<String>,
) -> serde_json::Value {
    serde_json::json!({
        "action": "genericActionResult",
        "ok": true,
        "target": target.to_string(),
        "title": title.to_string(),
        "messages": messages,
    })
}

fn generic_action_error(
    target: impl ToString,
    title: impl ToString,
    error: impl ToString,
) -> serde_json::Value {
    serde_json::json!({
        "action": "genericActionResult",
        "ok": false,
        "target": target.to_string(),
        "title": title.to_string(),
        "messages": [error.to_string()],
    })
}

fn run_module_open_workflow(
    workspace_root: &Path,
    command: &str,
    label: &str,
) -> Result<Vec<String>, String> {
    let mut messages = module_backend_messages(command);
    match command {
        "file.localHistoryRecovery.open" => {
            let artifact = write_workspace_manifest(
                workspace_root,
                ".meditor/local-history",
                "snapshot",
                "Local History snapshot",
            )?;
            messages.push(format!("Local history snapshot written: {artifact}"));
        }
        "file.workspaceBackupRestore.open" => {
            let artifact = write_workspace_manifest(
                workspace_root,
                ".meditor/backups",
                "backup-manifest",
                "Workspace backup manifest",
            )?;
            messages.push(format!("Workspace backup manifest written: {artifact}"));
        }
        "project.create" => {
            messages.push(
                "Ready to create a project. Enter a name and press Create Project.".to_string(),
            );
        }
        "file.importProject.open" => {
            let import_count = workspace_entries(workspace_root)
                .into_iter()
                .filter(|entry| {
                    let path = entry.display_path.to_ascii_lowercase();
                    path.ends_with(".sln")
                        || path.ends_with(".csproj")
                        || path.ends_with(".vcxproj")
                        || path.ends_with("pom.xml")
                        || path.ends_with("cargo.toml")
                        || path.ends_with("package.json")
                        || path.ends_with(".project")
                        || path.ends_with("nbproject/project.xml")
                })
                .count();
            messages.push(format!(
                "Importable project metadata detected: {import_count}"
            ));
        }
        "source.refactor.open" => {
            messages.extend(detect_lsp_server_messages());
            messages.extend(detect_debug_adapter_messages());
        }
        "source.reformat" | "tools.xmlValidator.open" | "tools.jsonValidator.open" => {
            messages.push("Formatter/validator is live in this tab; paste text or open a file to run Rust validation.".to_string());
        }
        "team.git.open" => {
            messages.push(
                match run_vcs_command(workspace_root, "git", "status", ".") {
                    Ok(result) => format!(
                        "Git status executed: success={} output={} bytes",
                        result.success,
                        result.stdout.len() + result.stderr.len()
                    ),
                    Err(error) => format!("Git status not available: {error}"),
                },
            );
        }
        "team.svn.open" => {
            messages.push(match run_vcs_command(workspace_root, "svn", "info", ".") {
                Ok(result) => format!(
                    "SVN info executed: success={} output={} bytes",
                    result.success,
                    result.stdout.len() + result.stderr.len()
                ),
                Err(error) => format!("SVN info not available: {error}"),
            });
        }
        "run.tasks.open" => match detect_project(workspace_root, ".") {
            Ok(project) => {
                messages.push(format!("Detected project: {}", project.kind));
                messages.push(format!("Runnable task profiles: {}", project.tasks.len()));
            }
            Err(error) => messages.push(format!("Project detection needs user input: {error}")),
        },
        "setup.languageSupport.open" => messages.extend(detect_toolchain_messages()),
        "setup.verifyInstallDependencies.open" => {
            let records = dependency_verification_records();
            let installed = records
                .iter()
                .filter(|record| {
                    record
                        .get("installed")
                        .and_then(|value| value.as_bool())
                        .unwrap_or(false)
                })
                .count();
            messages.push(format!("Dependency records checked: {}", records.len()));
            messages.push(format!("Installed dependencies: {installed}"));
            messages.push(format!(
                "Missing dependencies: {}",
                records.len().saturating_sub(installed)
            ));
        }
        "tools.dbaWorkshop.open" => {
            let setup = create_sqlite_security_folder(workspace_root)?;
            messages.extend(setup);
            messages.push("SQLite/JDBC worksheet controls are active in this tab.".to_string());
        }
        "tools.sshTerminus.open" => {
            messages.push(format!("OpenSSH available: {}", command_exists("ssh")));
            messages
                .push("Start Local Shell is active without requiring a remote host.".to_string());
        }
        "tools.sftpScpTransfer.open" => {
            messages.push(format!("scp available: {}", command_exists("scp")));
            messages.push(format!("ssh available: {}", command_exists("ssh")));
        }
        "tools.webBrowser.open" => {
            messages.push(
                "Browser address bar and iframe navigation are active in this tab.".to_string(),
            );
        }
        "tools.profiler.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/profiler",
                "profiler-report",
                &format!(
                    "# mEditor Profiler Report\n\n- Version: {}\n- Workspace: {}\n- Files indexed: {}\n- Generated: {}\n",
                    meditor_core::CURRENT_BASELINE_VERSION,
                    display_path(workspace_root),
                    workspace_entries(workspace_root).len(),
                    now_epoch_seconds()
                ),
            )?;
            messages.push(format!("Profiler report written: {artifact}"));
        }
        "tools.templatesSnippets.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/templates",
                "snippets",
                serde_json::json!({
                    "created_at_epoch": now_epoch_seconds(),
                    "snippets": [
                        {"language": "rust", "prefix": "main", "body": "fn main() {\\n    println!(\\\"hello from mEditor\\\");\\n}"},
                        {"language": "sql", "prefix": "select", "body": "select * from table_name;"}
                    ]
                }),
            )?;
            messages.push(format!("Snippet catalog written: {artifact}"));
        }
        "tools.cicdGenerator.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/cicd",
                "github-actions-rust",
                "name: mEditor Generated CI\n\non:\n  push:\n  workflow_dispatch:\n\njobs:\n  test:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - uses: dtolnay/rust-toolchain@stable\n      - run: cargo test\n",
            )?;
            messages.push(format!("CI template written: {artifact}"));
        }
        "tools.apiWorkbench.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/api-workbench",
                "requests",
                serde_json::json!({
                    "created_at_epoch": now_epoch_seconds(),
                    "requests": [
                        {"name": "Health check", "method": "GET", "url": "http://localhost:8080/health"}
                    ]
                }),
            )?;
            messages.push(format!("API request catalog written: {artifact}"));
        }
        "tools.databaseMigration.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/db-migration",
                "migration-plan",
                "-- mEditor guarded migration plan\n-- Add SQL migration steps here.\n-- Run dry-run and backup before execution.\n",
            )?;
            messages.push(format!("Migration plan written: {artifact}"));
        }
        "tools.extensionSdk.open" => messages.extend(create_extension_skeleton(workspace_root)?),
        "tools.workspaceIndexer.open" => messages.extend(scan_workspace_index(workspace_root)?),
        "tools.codeSecurityAnalyzer.open" => {
            messages.push(format!(
                "Seed security rules loaded: {}",
                meditor_code_security::CodeSecurityScanner::with_seed_rules()
                    .scan_text(&workspace_root.join("README.md"), "")
                    .len()
            ));
            messages.push(
                "Open a source file and use Security Scan for file-level findings.".to_string(),
            );
        }
        "tools.cvssRepository.open" => {
            messages.extend(create_sqlite_security_folder(workspace_root)?)
        }
        "tools.aimlAssistant.open" => {
            messages.push("Knowledge save, local reindex, local training, and local model runtime buttons are active.".to_string());
        }
        "tools.aiKnowledgeBase.open" => {
            let knowledge_root = workspace_root.join(".meditor/ai/knowledge");
            fs::create_dir_all(&knowledge_root)
                .map_err(|error| format!("Unable to create AI knowledge base: {error}"))?;
            messages.push(format!(
                "Knowledge base ready: {}",
                display_path(&knowledge_root)
            ));
        }
        "tools.aiTrainingStudio.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/ai/training",
                "training-plan",
                "# mEditor AI Training Plan\n\n1. Add knowledge sources.\n2. Reindex local knowledge.\n3. Train local retrieval model.\n4. Evaluate suggestions before applying code changes.\n",
            )?;
            messages.push(format!("Training plan written: {artifact}"));
        }
        "tools.specToSystem.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/spec-to-system",
                "spec-template",
                "# Specification\n\n## Goals\n\n## Functional Requirements\n\n## Non-functional Requirements\n\n## Architecture Notes\n\n## Acceptance Tests\n",
            )?;
            messages.push(format!("Spec template written: {artifact}"));
        }
        "tools.projectDocumentation.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/project/docs",
                "documentation-index",
                "# Project Documentation Index\n\n- PRD\n- TDD\n- Deployment guide\n- API reference\n- ERD\n- Flowcharts\n",
            )?;
            messages.push(format!("Documentation index written: {artifact}"));
        }
        "tools.projectPlanner.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/planner",
                "planner-dashboard",
                serde_json::json!({
                    "created_at_epoch": now_epoch_seconds(),
                    "models": ["objective-milestone-task", "scrum-epic-story-task"],
                    "status": "ready"
                }),
            )?;
            messages.push(format!("Planner dashboard written: {artifact}"));
        }
        "tools.umlModeling.open" => {
            let artifact = write_text_artifact(
                workspace_root,
                ".meditor/uml",
                "sample-class-diagram",
                "@startuml\nclass Project\nclass Task\nProject \"1\" --> \"many\" Task\n@enduml\n",
            )?;
            messages.push(format!("UML source written: {artifact}"));
        }
        "window.fileExplorer.focus" => {
            messages.push(format!(
                "File Explorer entries loaded: {}",
                workspace_entries(workspace_root).len()
            ));
        }
        "window.perspectives.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/window",
                "perspectives",
                serde_json::json!({
                    "active": "SQL Developer style",
                    "perspectives": ["IDE", "Database", "Terminal", "Planning", "Security"]
                }),
            )?;
            messages.push(format!("Perspective profile written: {artifact}"));
        }
        "window.workspaceDashboard.open" => {
            let artifact = write_workspace_manifest(
                workspace_root,
                ".meditor/dashboard",
                "workspace-dashboard",
                "Workspace dashboard",
            )?;
            messages.push(format!("Workspace dashboard written: {artifact}"));
        }
        "settings.keymapsImports.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/settings",
                "keymaps",
                serde_json::json!({
                    "profiles": ["SQL Developer", "NetBeans", "Eclipse", "VS Code", "Vim", "Emacs", "Custom"],
                    "active": "SQL Developer"
                }),
            )?;
            messages.push(format!("Keymap profile written: {artifact}"));
        }
        "settings.workspaceTrust.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/security",
                "workspace-trust",
                serde_json::json!({
                    "trusted": false,
                    "policy": "Restricted until user trusts this workspace",
                    "saved_at_epoch": now_epoch_seconds()
                }),
            )?;
            messages.push(format!("Workspace Trust policy written: {artifact}"));
        }
        "settings.secretsCredentials.open" => messages.extend(credential_store_status_messages()),
        "settings.pluginPermissions.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/extensions",
                "plugin-permissions",
                serde_json::json!({
                    "default": "deny",
                    "permissions": ["filesystem.read", "terminal.run", "network.request", "database.connect"],
                    "audit_required": true
                }),
            )?;
            messages.push(format!("Plugin permission policy written: {artifact}"));
        }
        "settings.accessibilityKeyboard.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/settings",
                "accessibility-keyboard",
                serde_json::json!({
                    "keyboard_navigation": true,
                    "screen_reader_labels": true,
                    "high_contrast_ready": true
                }),
            )?;
            messages.push(format!("Accessibility settings written: {artifact}"));
        }
        "help.registration.open" | "help.about.open" => {
            let status = license_acceptance_status(workspace_root)?;
            messages.push(format!("License display: {}", status.display_name));
        }
        "help.feedback.open" | "feedback.open" => {
            messages.push("Feedback form is active and stores guarded outbox records.".to_string());
        }
        "help.projectAbout.open" | "help.projectHelp.open" => {
            let docs_dir = workspace_root.join(".meditor/project/docs");
            fs::create_dir_all(&docs_dir)
                .map_err(|error| format!("Unable to create project docs folder: {error}"))?;
            messages.push(format!(
                "Project docs folder ready: {}",
                display_path(&docs_dir)
            ));
        }
        "help.about.checkForUpdates" | "help.updateChannelManager.open" => {
            match check_update_source(workspace_root) {
                Ok(update_messages) => messages.extend(update_messages),
                Err(error) => messages.push(format!("Update source not ready: {error}")),
            }
        }
        "help.diagnosticsBundle.open" => {
            messages.extend(diagnostics_preview_messages(workspace_root))
        }
        "help.privacyCenter.open" => {
            messages.push("Privacy Center loaded with install, feedback, error, update, and diagnostics flows.".to_string());
        }
        "report.center.open" => {
            messages.push(format!(
                "Reports found: {}",
                list_local_reports(workspace_root)?.len()
            ));
        }
        "report.auditTrail.open" => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/audit",
                "audit-trail",
                serde_json::json!([{
                    "event": "auditTrailOpened",
                    "epoch": now_epoch_seconds(),
                    "local_only": true
                }]),
            )?;
            messages.push(format!("Audit trail written: {artifact}"));
        }
        _ => {
            let artifact = write_json_artifact(
                workspace_root,
                ".meditor/module-runs",
                &sanitize_file_stem(command),
                serde_json::json!({
                    "command": command,
                    "label": label,
                    "opened_at_epoch": now_epoch_seconds(),
                    "status": "backend acknowledged and recorded"
                }),
            )?;
            messages.push(format!("Module run record written: {artifact}"));
        }
    }
    messages.push("This menu item executed a Rust-backed workflow.".to_string());
    Ok(messages)
}

fn write_workspace_manifest(
    workspace_root: &Path,
    folder: &str,
    name: &str,
    title: &str,
) -> Result<String, String> {
    let entries = workspace_entries(workspace_root)
        .into_iter()
        .take(250)
        .map(|entry| {
            serde_json::json!({
                "path": entry.display_path,
                "is_dir": entry.is_dir,
                "size_bytes": entry.size_bytes,
                "depth": entry.depth,
            })
        })
        .collect::<Vec<_>>();
    write_json_artifact(
        workspace_root,
        folder,
        name,
        serde_json::json!({
            "title": title,
            "created_at_epoch": now_epoch_seconds(),
            "workspace": display_path(workspace_root),
            "entries": entries,
        }),
    )
}

fn write_json_artifact(
    workspace_root: &Path,
    folder: &str,
    name: &str,
    value: serde_json::Value,
) -> Result<String, String> {
    let path = timestamped_artifact_path(workspace_root, folder, name, "json")?;
    let content = serde_json::to_string_pretty(&value)
        .map_err(|error| format!("Unable to serialize module artifact: {error}"))?;
    fs::write(&path, content)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))?;
    Ok(path
        .strip_prefix(workspace_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or(""))
        .to_string())
}

fn write_text_artifact(
    workspace_root: &Path,
    folder: &str,
    name: &str,
    content: &str,
) -> Result<String, String> {
    let path = timestamped_artifact_path(workspace_root, folder, name, "md")?;
    fs::write(&path, content)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&path)))?;
    Ok(path
        .strip_prefix(workspace_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or(""))
        .to_string())
}

fn timestamped_artifact_path(
    workspace_root: &Path,
    folder: &str,
    name: &str,
    extension: &str,
) -> Result<PathBuf, String> {
    let relative = format!(
        "{}/{}-{}.{}",
        folder.trim_end_matches('/'),
        now_epoch_seconds(),
        sanitize_file_stem(name),
        extension
    );
    let path = resolve_workspace_path(workspace_root, &relative)?;
    ensure_parent(&path)?;
    Ok(path)
}

fn module_backend_messages(command: &str) -> Vec<String> {
    match command {
        "project.create" => vec![
            "New Project backend loaded.".to_string(),
            "Create Project writes .meditor project metadata, embedded About Project, Project Help, and src scaffold.".to_string(),
        ],
        "source.reformat" => vec![
            "Reformat opens the two-panel JSON/XML formatter.".to_string(),
            "The left panel accepts paste/edit/upload input and the right panel renders a live Rust-formatted result.".to_string(),
        ],
        "source.viEditor.open" => vec![
            "Vi Editor backend route loaded.".to_string(),
            "The Vi tab uses modal editing and saves through the Rust workspace file bridge.".to_string(),
        ],
        "run.tasks.open" => vec![
            "Task runner backend loaded.".to_string(),
            "Detect Project lists runnable build/test/launch/debug profiles for the selected project folder.".to_string(),
            "Run Selected Task executes a known task inside the workspace, captures stdout/stderr, extracts diagnostics, and writes .meditor/debug-reports handover documentation.".to_string(),
        ],
        "tools.dbaWorkshop.open" => vec![
            "DBA Workshop backend loaded.".to_string(),
            format!(
                "Worksheet actions: {}",
                meditor_db_workbench::worksheet_actions().len()
            ),
            format!(
                "Dashboard sections: {}",
                meditor_db_workbench::dba_dashboard_sections().len()
            ),
        ],
        "tools.sshTerminus.open" => vec![
            "SSH Terminus backend loaded.".to_string(),
            format!(
                "Terminal actions: {}",
                meditor_ssh_terminus::terminal_actions().len()
            ),
            "Connection grouping and tab routing are available in the module contract.".to_string(),
        ],
        "setup.languageSupport.open" => vec![
            "Language support backend loaded.".to_string(),
            format!(
                "Language/tooling catalog entries: {}",
                meditor_toolchains::supported_language_catalog().len()
            ),
            "Use Detect Installed Tooling to run executable checks through Rust.".to_string(),
        ],
        "setup.verifyInstallDependencies.open" => vec![
            "Dependency verification backend loaded.".to_string(),
            format!(
                "Executable requirements: {}",
                meditor_toolchains::seed_executable_requirements().len()
            ),
            "Verify lists missing dependencies and Install Selected runs the platform package manager only after the user clicks it.".to_string(),
        ],
        "file.importProject.open" => vec![
            "Project importer backend loaded.".to_string(),
            format!(
                "Supported project kinds: {}",
                meditor_project_importers::supported_project_kinds().len()
            ),
            format!(
                "Visual Studio file kinds: {}",
                meditor_project_importers::visual_studio_project_file_kinds().len()
            ),
        ],
        "tools.aimlAssistant.open" => vec![
            "AI/ML Assistant backend contract loaded.".to_string(),
            format!(
                "Assistant capabilities: {}",
                meditor_ai::assistant_capabilities().len()
            ),
            format!(
                "Knowledge ingestion formats: {}",
                meditor_ai::supported_knowledge_formats().len()
            ),
        ],
        "tools.xmlValidator.open" | "tools.jsonValidator.open" => vec![
            "Validator and formatter backend loaded.".to_string(),
            "Paste, edit, or upload JSON/XML on the left to see live formatting on the right.".to_string(),
        ],
        _ => vec![
            "Rust backend acknowledged this menu action.".to_string(),
            format!("Command: {command}"),
            "This module opens in the same mEditor window under the frozen Ver 2.2 design."
                .to_string(),
        ],
    }
}

fn detect_toolchain_messages() -> Vec<String> {
    let catalog = meditor_toolchains::supported_language_catalog();
    let statuses = catalog
        .iter()
        .map(|entry| entry.status_with(command_exists))
        .collect::<Vec<_>>();
    let active = statuses.iter().filter(|status| status.is_active()).count();
    let missing = statuses.len().saturating_sub(active);
    let mut messages = vec![
        format!("Catalog entries checked: {}", statuses.len()),
        format!("Active entries: {active}"),
        format!("Missing-tooling entries: {missing}"),
    ];
    messages.extend(statuses.into_iter().take(20).map(|status| {
        format!(
            "{}: {:?}; missing executables: {}",
            status.display_name,
            status.state,
            if status.missing_required_executables.is_empty() {
                "none".to_string()
            } else {
                status.missing_required_executables.join(", ")
            }
        )
    }));
    messages
}

#[derive(Clone, Debug)]
struct DependencySource {
    source_url: String,
    documentation_url: String,
    install_strategy: String,
}

#[derive(Clone, Debug)]
struct DependencyInstallPlan {
    manager: String,
    executable: String,
    args: Vec<String>,
    package: String,
}

fn dependency_verification_records() -> Vec<serde_json::Value> {
    meditor_toolchains::seed_executable_requirements()
        .into_iter()
        .map(|requirement| {
            let installed = command_exists(requirement.id);
            let source = dependency_source(requirement.id);
            let plan = dependency_install_plan(requirement.id);
            serde_json::json!({
                "id": requirement.id,
                "displayName": requirement.display_name,
                "domain": format!("{:?}", requirement.domain),
                "requiredForCore": requirement.required_for_core,
                "installed": installed,
                "sourceUrl": source.source_url,
                "documentationUrl": source.documentation_url,
                "installStrategy": source.install_strategy,
                "installable": plan.is_some(),
                "packageManager": plan.as_ref().map(|plan| plan.manager.as_str()).unwrap_or("manual"),
                "package": plan.as_ref().map(|plan| plan.package.as_str()).unwrap_or("manual"),
                "installCommand": plan
                    .as_ref()
                    .map(|plan| command_line(&plan.executable, &plan.args))
                    .unwrap_or_else(|| "Open source URL and install manually".to_string()),
            })
        })
        .collect()
}

fn dependency_source(executable: &str) -> DependencySource {
    for language in meditor_toolchains::supported_language_catalog() {
        for tooling in language.tooling {
            if tooling
                .executable_ids
                .iter()
                .any(|candidate| *candidate == executable)
            {
                return DependencySource {
                    source_url: tooling.source_url.to_string(),
                    documentation_url: tooling.documentation_url.to_string(),
                    install_strategy: tooling.install_strategy.to_string(),
                };
            }
        }
    }
    DependencySource {
        source_url: "https://www.rust-lang.org/tools/install".to_string(),
        documentation_url: "https://doc.rust-lang.org/".to_string(),
        install_strategy: "Install through the platform package manager or configure the executable path manually.".to_string(),
    }
}

fn install_dependency(dependency_id: &str) -> Result<CommandCapture, String> {
    let dependency_id = dependency_id.trim();
    if dependency_id.is_empty() {
        return Err("Select a dependency to install.".to_string());
    }
    if command_exists(dependency_id) {
        return Ok(CommandCapture {
            command_line: format!("{dependency_id} already installed"),
            exit_code: Some(0),
            success: true,
            timed_out: false,
            duration_ms: 0,
            stdout: format!("{dependency_id} is already present on PATH."),
            stderr: String::new(),
        });
    }
    let plan = dependency_install_plan(dependency_id).ok_or_else(|| {
        let source = dependency_source(dependency_id);
        format!(
            "No automatic installer is configured for {dependency_id} on this platform. Source: {}. Strategy: {}",
            source.source_url, source.install_strategy
        )
    })?;
    if !command_exists(&plan.executable) {
        return Err(format!(
            "Package manager '{}' is not available. Install {} manually or configure its executable path.",
            plan.executable, dependency_id
        ));
    }
    let profile = TaskProfile {
        action: "install-dependency",
        label: "Verify And Install Dependencies",
        executable: plan.executable,
        args: plan.args,
        description:
            "Download and install one missing dependency through the platform package manager.",
    };
    run_command_capture(Path::new("."), &profile, 900)
}

fn dependency_install_plan(executable: &str) -> Option<DependencyInstallPlan> {
    if cfg!(target_os = "macos") && command_exists("brew") {
        package_for_brew(executable).map(|package| DependencyInstallPlan {
            manager: "Homebrew".to_string(),
            executable: "brew".to_string(),
            args: vec!["install".to_string(), package.to_string()],
            package: package.to_string(),
        })
    } else if cfg!(target_os = "windows") && command_exists("winget") {
        package_for_winget(executable).map(|package| DependencyInstallPlan {
            manager: "winget".to_string(),
            executable: "winget".to_string(),
            args: vec![
                "install".to_string(),
                "--id".to_string(),
                package.to_string(),
                "-e".to_string(),
                "--accept-package-agreements".to_string(),
                "--accept-source-agreements".to_string(),
            ],
            package: package.to_string(),
        })
    } else if !cfg!(target_os = "windows") && command_exists("apt-get") {
        package_for_apt(executable).map(|packages| {
            let package = packages.join(" ");
            DependencyInstallPlan {
                manager: "apt-get".to_string(),
                executable: "sudo".to_string(),
                args: std::iter::once("apt-get".to_string())
                    .chain(std::iter::once("install".to_string()))
                    .chain(std::iter::once("-y".to_string()))
                    .chain(packages.iter().map(|package| (*package).to_string()))
                    .collect(),
                package,
            }
        })
    } else if !cfg!(target_os = "windows") && command_exists("dnf") {
        package_for_dnf(executable).map(|packages| {
            let package = packages.join(" ");
            DependencyInstallPlan {
                manager: "dnf".to_string(),
                executable: "sudo".to_string(),
                args: std::iter::once("dnf".to_string())
                    .chain(std::iter::once("install".to_string()))
                    .chain(std::iter::once("-y".to_string()))
                    .chain(packages.iter().map(|package| (*package).to_string()))
                    .collect(),
                package,
            }
        })
    } else if !cfg!(target_os = "windows") && command_exists("pacman") {
        package_for_pacman(executable).map(|packages| {
            let package = packages.join(" ");
            DependencyInstallPlan {
                manager: "pacman".to_string(),
                executable: "sudo".to_string(),
                args: std::iter::once("pacman".to_string())
                    .chain(std::iter::once("-S".to_string()))
                    .chain(std::iter::once("--noconfirm".to_string()))
                    .chain(packages.iter().map(|package| (*package).to_string()))
                    .collect(),
                package,
            }
        })
    } else {
        None
    }
}

fn package_for_brew(executable: &str) -> Option<&'static str> {
    match executable {
        "cargo" | "rustc" => Some("rust"),
        "rust-analyzer" => Some("rust-analyzer"),
        "java" | "javac" => Some("openjdk"),
        "mvn" => Some("maven"),
        "gradle" => Some("gradle"),
        "native-image" => Some("graalvm/tap/graalvm-ce-java17"),
        "node" | "npm" => Some("node"),
        "pnpm" => Some("pnpm"),
        "yarn" => Some("yarn"),
        "deno" => Some("deno"),
        "bun" => Some("bun"),
        "php" => Some("php"),
        "composer" => Some("composer"),
        "python" | "python3" => Some("python"),
        "perl" => Some("perl"),
        "gcc" | "g++" | "gfortran" => Some("gcc"),
        "clang" | "clang++" => Some("llvm"),
        "cmake" => Some("cmake"),
        "nasm" => Some("nasm"),
        "gnat" | "gprbuild" => Some("gcc"),
        "cobc" => Some("gnu-cobol"),
        "julia" => Some("julia"),
        "R" => Some("r"),
        "swipl" => Some("swi-prolog"),
        "gprolog" => Some("gnu-prolog"),
        "sbcl" => Some("sbcl"),
        "clisp" => Some("clisp"),
        "racket" => Some("racket"),
        "guile" => Some("guile"),
        "sqlite3" => Some("sqlite"),
        "psql" => Some("libpq"),
        "mysql" => Some("mysql-client"),
        "git" => Some("git"),
        "svn" => Some("subversion"),
        "go" => Some("go"),
        "dotnet" => Some("dotnet-sdk"),
        "ruby" => Some("ruby"),
        "lua" => Some("lua"),
        "terraform" => Some("terraform"),
        "kubectl" => Some("kubectl"),
        "helm" => Some("helm"),
        "ansible" => Some("ansible"),
        "pwsh" => Some("powershell/tap/powershell"),
        "ollama" => Some("ollama"),
        _ => None,
    }
}

fn package_for_winget(executable: &str) -> Option<&'static str> {
    match executable {
        "cargo" | "rustc" => Some("Rustlang.Rustup"),
        "rust-analyzer" => Some("Rustlang.Rustup"),
        "java" | "javac" => Some("EclipseAdoptium.Temurin.21.JDK"),
        "mvn" => Some("Apache.Maven"),
        "gradle" => Some("Gradle.Gradle"),
        "node" | "npm" => Some("OpenJS.NodeJS.LTS"),
        "deno" => Some("DenoLand.Deno"),
        "bun" => Some("Oven-sh.Bun"),
        "php" => Some("PHP.PHP"),
        "composer" => Some("Composer.Composer"),
        "python" | "python3" => Some("Python.Python.3.12"),
        "perl" => Some("StrawberryPerl.StrawberryPerl"),
        "cmake" => Some("Kitware.CMake"),
        "julia" => Some("Julialang.Julia"),
        "R" => Some("RProject.R"),
        "swipl" => Some("SWIProlog.SWIProlog"),
        "racket" => Some("Racket.Racket"),
        "sqlite3" => Some("SQLite.SQLite"),
        "git" => Some("Git.Git"),
        "svn" => Some("TortoiseSVN.TortoiseSVN"),
        "go" => Some("GoLang.Go"),
        "dotnet" => Some("Microsoft.DotNet.SDK.8"),
        "ruby" => Some("RubyInstallerTeam.RubyWithDevKit.3.2"),
        "terraform" => Some("Hashicorp.Terraform"),
        "kubectl" => Some("Kubernetes.kubectl"),
        "helm" => Some("Helm.Helm"),
        "pwsh" => Some("Microsoft.PowerShell"),
        "ollama" => Some("Ollama.Ollama"),
        _ => None,
    }
}

fn package_for_apt(executable: &str) -> Option<Vec<&'static str>> {
    match executable {
        "cargo" | "rustc" => Some(vec!["rustc", "cargo"]),
        "java" | "javac" => Some(vec!["openjdk-21-jdk"]),
        "mvn" => Some(vec!["maven"]),
        "gradle" => Some(vec!["gradle"]),
        "node" | "npm" => Some(vec!["nodejs", "npm"]),
        "php" => Some(vec!["php"]),
        "composer" => Some(vec!["composer"]),
        "python" | "python3" => Some(vec!["python3"]),
        "perl" => Some(vec!["perl"]),
        "gcc" | "g++" => Some(vec!["build-essential"]),
        "clang" | "clang++" => Some(vec!["clang"]),
        "cmake" => Some(vec!["cmake"]),
        "make" => Some(vec!["make"]),
        "nasm" => Some(vec!["nasm"]),
        "gnat" => Some(vec!["gnat"]),
        "gprbuild" => Some(vec!["gprbuild"]),
        "gfortran" => Some(vec!["gfortran"]),
        "cobc" => Some(vec!["gnucobol"]),
        "fbc" => Some(vec!["freebasic"]),
        "harbour" => Some(vec!["harbour"]),
        "julia" => Some(vec!["julia"]),
        "R" => Some(vec!["r-base"]),
        "swipl" => Some(vec!["swi-prolog"]),
        "gprolog" => Some(vec!["gprolog"]),
        "sbcl" => Some(vec!["sbcl"]),
        "clisp" => Some(vec!["clisp"]),
        "racket" => Some(vec!["racket"]),
        "guile" => Some(vec!["guile-3.0"]),
        "sqlite3" => Some(vec!["sqlite3"]),
        "psql" => Some(vec!["postgresql-client"]),
        "mysql" => Some(vec!["mysql-client"]),
        "git" => Some(vec!["git"]),
        "svn" => Some(vec!["subversion"]),
        "go" => Some(vec!["golang-go"]),
        "dotnet" => Some(vec!["dotnet-sdk-8.0"]),
        "ruby" => Some(vec!["ruby-full"]),
        "lua" => Some(vec!["lua5.4"]),
        "terraform" => Some(vec!["terraform"]),
        "kubectl" => Some(vec!["kubectl"]),
        "helm" => Some(vec!["helm"]),
        "ansible" => Some(vec!["ansible"]),
        "pwsh" => Some(vec!["powershell"]),
        _ => None,
    }
}

fn package_for_dnf(executable: &str) -> Option<Vec<&'static str>> {
    match executable {
        "cargo" | "rustc" => Some(vec!["rust", "cargo"]),
        "java" | "javac" => Some(vec!["java-21-openjdk-devel"]),
        "mvn" => Some(vec!["maven"]),
        "gradle" => Some(vec!["gradle"]),
        "node" | "npm" => Some(vec!["nodejs", "npm"]),
        "php" => Some(vec!["php"]),
        "composer" => Some(vec!["composer"]),
        "python" | "python3" => Some(vec!["python3"]),
        "perl" => Some(vec!["perl"]),
        "gcc" => Some(vec!["gcc"]),
        "g++" => Some(vec!["gcc-c++"]),
        "clang" | "clang++" => Some(vec!["clang"]),
        "cmake" => Some(vec!["cmake"]),
        "make" => Some(vec!["make"]),
        "nasm" => Some(vec!["nasm"]),
        "gnat" => Some(vec!["gcc-gnat"]),
        "gfortran" => Some(vec!["gcc-gfortran"]),
        "cobc" => Some(vec!["gnucobol"]),
        "R" => Some(vec!["R"]),
        "swipl" => Some(vec!["pl"]),
        "sbcl" => Some(vec!["sbcl"]),
        "clisp" => Some(vec!["clisp"]),
        "guile" => Some(vec!["guile"]),
        "sqlite3" => Some(vec!["sqlite"]),
        "psql" => Some(vec!["postgresql"]),
        "mysql" => Some(vec!["mysql"]),
        "git" => Some(vec!["git"]),
        "svn" => Some(vec!["subversion"]),
        "go" => Some(vec!["golang"]),
        "dotnet" => Some(vec!["dotnet-sdk-8.0"]),
        "ruby" => Some(vec!["ruby"]),
        "lua" => Some(vec!["lua"]),
        "terraform" => Some(vec!["terraform"]),
        "kubectl" => Some(vec!["kubectl"]),
        "helm" => Some(vec!["helm"]),
        "ansible" => Some(vec!["ansible"]),
        "pwsh" => Some(vec!["powershell"]),
        _ => None,
    }
}

fn package_for_pacman(executable: &str) -> Option<Vec<&'static str>> {
    match executable {
        "cargo" | "rustc" => Some(vec!["rust"]),
        "java" | "javac" => Some(vec!["jdk-openjdk"]),
        "mvn" => Some(vec!["maven"]),
        "gradle" => Some(vec!["gradle"]),
        "node" | "npm" => Some(vec!["nodejs", "npm"]),
        "pnpm" => Some(vec!["pnpm"]),
        "yarn" => Some(vec!["yarn"]),
        "deno" => Some(vec!["deno"]),
        "php" => Some(vec!["php"]),
        "composer" => Some(vec!["composer"]),
        "python" | "python3" => Some(vec!["python"]),
        "perl" => Some(vec!["perl"]),
        "gcc" | "g++" | "make" => Some(vec!["base-devel"]),
        "clang" | "clang++" => Some(vec!["clang"]),
        "cmake" => Some(vec!["cmake"]),
        "nasm" => Some(vec!["nasm"]),
        "gfortran" => Some(vec!["gcc-fortran"]),
        "cobc" => Some(vec!["gnucobol"]),
        "julia" => Some(vec!["julia"]),
        "R" => Some(vec!["r"]),
        "swipl" => Some(vec!["swi-prolog"]),
        "sbcl" => Some(vec!["sbcl"]),
        "racket" => Some(vec!["racket"]),
        "guile" => Some(vec!["guile"]),
        "sqlite3" => Some(vec!["sqlite"]),
        "psql" => Some(vec!["postgresql"]),
        "mysql" => Some(vec!["mariadb-clients"]),
        "git" => Some(vec!["git"]),
        "svn" => Some(vec!["subversion"]),
        "go" => Some(vec!["go"]),
        "dotnet" => Some(vec!["dotnet-sdk"]),
        "ruby" => Some(vec!["ruby"]),
        "lua" => Some(vec!["lua"]),
        "terraform" => Some(vec!["terraform"]),
        "kubectl" => Some(vec!["kubectl"]),
        "helm" => Some(vec!["helm"]),
        "ansible" => Some(vec!["ansible"]),
        "pwsh" => Some(vec!["powershell"]),
        _ => None,
    }
}

fn command_exists(command: &str) -> bool {
    let checker = if cfg!(target_os = "windows") {
        ("where", "/Q")
    } else {
        ("command", "-v")
    };
    if cfg!(target_os = "windows") {
        Command::new(checker.0)
            .arg(command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    } else {
        Command::new("sh")
            .arg("-lc")
            .arg(format!("command -v {}", shell_word(command)))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
}

fn shell_word(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn sqlite_setup_preview_messages(workspace_root: &Path) -> Vec<String> {
    let layout = meditor_cvss_repository::RepositoryLayout::under_working_folder(workspace_root);
    let mut messages = vec![
        format!("Security folder: {}", display_path(&layout.security_dir)),
        format!(
            "Vulnerability database: {}",
            display_path(&layout.vulnerability_db)
        ),
        format!(
            "Scan history database: {}",
            display_path(&layout.scan_history_db)
        ),
    ];
    messages.extend(
        meditor_cvss_repository::sqlite_setup_files()
            .iter()
            .map(|file| format!("Setup artifact: {file}")),
    );
    messages
}

fn create_sqlite_security_folder(workspace_root: &Path) -> Result<Vec<String>, String> {
    let layout = meditor_cvss_repository::RepositoryLayout::under_working_folder(workspace_root);
    fs::create_dir_all(&layout.security_dir).map_err(|error| {
        format!(
            "Unable to create {}: {error}",
            display_path(&layout.security_dir)
        )
    })?;
    Ok(vec![
        format!("Created or verified {}", display_path(&layout.security_dir)),
        format!(
            "SQLite schema source: {}",
            meditor_cvss_repository::SQLITE_SCHEMA_FILE
        ),
        "Run the one-click setup script to create actual SQLite tables when sqlite is available."
            .to_string(),
    ])
}

fn project_import_preview_messages(workspace_root: &Path) -> Vec<String> {
    let files = workspace_entries(workspace_root)
        .into_iter()
        .map(|entry| entry.display_path)
        .collect::<Vec<_>>();
    let file_refs = files.iter().map(String::as_str).collect::<Vec<_>>();
    let kind = meditor_project_importers::detect_project_kind(&file_refs);
    let mut messages = vec![
        format!("Detected project kind: {kind:?}"),
        format!("Files sampled: {}", file_refs.len()),
        "Import preview only; original files are not changed.".to_string(),
    ];
    messages.extend(
        meditor_project_importers::visual_studio_import_steps()
            .iter()
            .map(|step| format!("Visual Studio step: {step}")),
    );
    messages
}

fn ai_assistant_messages(prompt: &str) -> Vec<String> {
    let plan = meditor_ai::SelfLearningAiServicePlan::ver_1_contract();
    vec![
        "AI/ML Assistant request reached the Rust backend.".to_string(),
        format!("Prompt length: {} bytes", prompt.len()),
        format!(
            "Embedded service contract enabled: {}",
            plan.embedded_service
        ),
        format!(
            "Human review required for code writes: {}",
            plan.human_review_required_for_code_writes
        ),
        "Runtime model execution is available through configured local adapters; model weight training is delegated to the user-approved external trainer.".to_string(),
    ]
}

fn diagnostics_preview_messages(workspace_root: &Path) -> Vec<String> {
    let platform = meditor_platform::PlatformProfile::current();
    let workbench = meditor_workbench::WorkbenchBaseline::ver_2_2();
    vec![
        format!(
            "mEditor version: {}",
            meditor_core::CURRENT_BASELINE_VERSION
        ),
        format!("Workspace: {}", display_path(workspace_root)),
        format!("Platform: {:?} {:?}", platform.os, platform.arch),
        format!("Workbench modules: {}", workbench.modules.len()),
        format!("Menu items: {}", workbench.menu_items),
        "Diagnostics export must be reviewed and redacted before sharing.".to_string(),
    ]
}

#[derive(Clone, Debug)]
struct ProductionReadinessCheck {
    id: &'static str,
    label: &'static str,
    required: bool,
    passed: bool,
    detail: String,
}

fn production_readiness_messages(workspace_root: &Path) -> Result<Vec<String>, String> {
    let checks = production_readiness_checks(workspace_root);
    let required_total = checks.iter().filter(|check| check.required).count();
    let required_passed = checks
        .iter()
        .filter(|check| check.required && check.passed)
        .count();
    let warnings = checks
        .iter()
        .filter(|check| !check.required && !check.passed)
        .count();
    let status = if required_total == required_passed {
        "PRODUCTION CANDIDATE"
    } else {
        "BLOCKED"
    };

    let report_dir = workspace_root.join(".meditor/production-readiness");
    fs::create_dir_all(&report_dir)
        .map_err(|error| format!("Unable to create production readiness folder: {error}"))?;
    let json_path = report_dir.join("production-readiness-report.json");
    let md_path = report_dir.join("production-readiness-report.md");
    let check_json = checks
        .iter()
        .map(|check| {
            serde_json::json!({
                "id": check.id,
                "label": check.label,
                "required": check.required,
                "passed": check.passed,
                "detail": check.detail,
            })
        })
        .collect::<Vec<_>>();
    fs::write(
        &json_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "product": "mEditor",
            "version": meditor_core::CURRENT_BASELINE_VERSION,
            "created_at_epoch": now_epoch_seconds(),
            "status": status,
            "required_total": required_total,
            "required_passed": required_passed,
            "warnings": warnings,
            "checks": check_json,
        }))
        .map_err(|error| format!("Unable to serialize readiness report: {error}"))?,
    )
    .map_err(|error| format!("Unable to write {}: {error}", display_path(&json_path)))?;

    let mut markdown = String::new();
    writeln!(markdown, "# mEditor Production Readiness Report").unwrap();
    writeln!(markdown).unwrap();
    writeln!(
        markdown,
        "- Version: {}",
        meditor_core::CURRENT_BASELINE_VERSION
    )
    .unwrap();
    writeln!(markdown, "- Status: {status}").unwrap();
    writeln!(
        markdown,
        "- Required checks: {required_passed}/{required_total}"
    )
    .unwrap();
    writeln!(markdown, "- Warnings: {warnings}").unwrap();
    writeln!(markdown).unwrap();
    writeln!(markdown, "| Check | Required | Result | Detail |").unwrap();
    writeln!(markdown, "| --- | --- | --- | --- |").unwrap();
    for check in &checks {
        writeln!(
            markdown,
            "| {} | {} | {} | {} |",
            check.label.replace('|', "/"),
            check.required,
            if check.passed { "PASS" } else { "FAIL" },
            check.detail.replace('|', "/")
        )
        .unwrap();
    }
    fs::write(&md_path, markdown)
        .map_err(|error| format!("Unable to write {}: {error}", display_path(&md_path)))?;

    let mut messages = vec![
        format!("Production readiness status: {status}"),
        format!("Required checks passed: {required_passed}/{required_total}"),
        format!("Warning checks open: {warnings}"),
        format!("JSON report: {}", display_path(&json_path)),
        format!("Markdown report: {}", display_path(&md_path)),
    ];
    messages.extend(checks.into_iter().map(|check| {
        format!(
            "{} [{}]: {}",
            if check.passed { "PASS" } else { "FAIL" },
            if check.required {
                "required"
            } else {
                "warning"
            },
            check.detail
        )
    }));
    Ok(messages)
}

fn production_readiness_checks(workspace_root: &Path) -> Vec<ProductionReadinessCheck> {
    let version_file = workspace_root.join("VERSION");
    let version_consistent = fs::read_to_string(&version_file)
        .map(|value| value.trim() == meditor_core::CURRENT_BASELINE_VERSION)
        .unwrap_or(false);
    let current_package = current_host_package_root(workspace_root);
    let package_binary = if cfg!(target_os = "windows") {
        current_package.join("bin/mEditor.exe")
    } else {
        current_package.join("bin/mEditor")
    };
    let ci_workflow = workspace_root.join(".github/workflows/mEditor-ci.yml");
    let ci_text = fs::read_to_string(&ci_workflow).unwrap_or_default();
    let docs_text =
        fs::read_to_string(workspace_root.join("docs/USER_GUIDE.md")).unwrap_or_default();
    let forbidden_scan_clean = forbidden_product_metadata_is_clean(workspace_root);
    let packaged_verifiers_present = workspace_root
        .join("packaging/verify-cross-platform-package.sh")
        .is_file()
        && workspace_root
            .join("packaging/verify-cross-platform-package.ps1")
            .is_file();

    vec![
        readiness_check(
            "version",
            "Version file matches runtime",
            true,
            version_consistent,
            if version_consistent {
                format!("VERSION matches {}", meditor_core::CURRENT_BASELINE_VERSION)
            } else {
                format!(
                    "VERSION is missing or does not match {}",
                    meditor_core::CURRENT_BASELINE_VERSION
                )
            },
        ),
        readiness_check(
            "workspace-write",
            "Workspace state is writable",
            true,
            workspace_candidate_is_writable(workspace_root),
            format!("Workspace: {}", display_path(workspace_root)),
        ),
        readiness_check(
            "current-package",
            "Current host package exists",
            true,
            package_binary.is_file(),
            format!("Expected package binary: {}", display_path(&package_binary)),
        ),
        readiness_check(
            "docs",
            "End-user documentation is current",
            true,
            docs_text.contains(meditor_core::CURRENT_BASELINE_VERSION)
                && docs_text.contains("Production Readiness Gate"),
            "docs/USER_GUIDE.md contains version and production readiness instructions".to_string(),
        ),
        readiness_check(
            "ci",
            "Cross-platform CI or package verification is staged",
            true,
            (ci_text.contains("macos-14")
                && ci_text.contains("ubuntu-24.04")
                && ci_text.contains("windows-2022")
                && ci_text.contains("build-local-package"))
                || packaged_verifiers_present,
            "Source checkout has GitHub Actions for macOS/Linux/Windows; packaged app has shell and PowerShell verification scripts.".to_string(),
        ),
        readiness_check(
            "metadata",
            "Product metadata is clean",
            true,
            forbidden_scan_clean,
            "No forbidden product identity or email strings found in active source/docs metadata scan".to_string(),
        ),
        readiness_check(
            "credential-store",
            "OS credential backend available",
            false,
            credential_backend().1,
            credential_store_status_messages().join("; "),
        ),
        readiness_check(
            "ssh-stack",
            "SSH/SFTP runtime tools available",
            false,
            command_exists("ssh") && command_exists("scp"),
            format!(
                "ssh={}, scp={}, script={}",
                command_exists("ssh"),
                command_exists("scp"),
                command_exists("script")
            ),
        ),
        readiness_check(
            "jdbc-stack",
            "JDBC runtime tools available",
            false,
            command_exists("java") && command_exists("javac"),
            format!("java={}, javac={}", command_exists("java"), command_exists("javac")),
        ),
        readiness_check(
            "ai-runtime",
            "Optional local AI runtime available",
            false,
            command_exists("ollama"),
            "Ollama is optional; external trainer config can also be used for model weight training.".to_string(),
        ),
        readiness_check(
            "platform-package-validation",
            "External platform validation evidence",
            false,
            workspace_root
                .join(".meditor/production-readiness/platform-validation")
                .is_dir(),
            "Add signed Windows/Linux/macOS runner evidence under .meditor/production-readiness/platform-validation after native device tests.".to_string(),
        ),
    ]
}

fn readiness_check(
    id: &'static str,
    label: &'static str,
    required: bool,
    passed: bool,
    detail: String,
) -> ProductionReadinessCheck {
    ProductionReadinessCheck {
        id,
        label,
        required,
        passed,
        detail,
    }
}

fn current_host_package_root(workspace_root: &Path) -> PathBuf {
    let packaged_launcher = if cfg!(target_os = "windows") {
        workspace_root.join("bin/mEditor.exe")
    } else {
        workspace_root.join("bin/mEditor")
    };
    if packaged_launcher.is_file() && workspace_root.join("VERSION").is_file() {
        return workspace_root.to_path_buf();
    }

    let os = uname_token("-s")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| {
            if cfg!(target_os = "macos") {
                "darwin".to_string()
            } else if cfg!(target_os = "windows") {
                "windows".to_string()
            } else {
                std::env::consts::OS.to_string()
            }
        });
    let arch = uname_token("-m").unwrap_or_else(|| std::env::consts::ARCH.to_string());
    workspace_root.join(format!(
        "dist/mEditor-{}-{}-{}/",
        meditor_core::CURRENT_BASELINE_VERSION,
        os,
        arch
    ))
}

fn uname_token(arg: &str) -> Option<String> {
    let output = Command::new("uname").arg(arg).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn forbidden_product_metadata_is_clean(workspace_root: &Path) -> bool {
    let mut files = Vec::new();
    collect_metadata_scan_files(workspace_root, workspace_root, 0, &mut files);
    files.into_iter().all(|path| {
        fs::read_to_string(&path).map_or(true, |content| {
            let forbidden_upper = ["MED", "ITOR"].concat();
            let forbidden_mixed = ["ME", "ditor"].concat();
            let forbidden_email_domain = ["@oracle", ".com"].concat();
            !content.contains(&forbidden_upper)
                && !content.contains(&forbidden_mixed)
                && !content.contains(&forbidden_email_domain)
        })
    })
}

fn collect_metadata_scan_files(root: &Path, folder: &Path, depth: usize, files: &mut Vec<PathBuf>) {
    if depth > 5 || files.len() >= 800 {
        return;
    }
    let Ok(read_dir) = fs::read_dir(folder) else {
        return;
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if matches!(
            name.as_str(),
            ".git" | "target" | "dist" | ".DS_Store" | "node_modules"
        ) {
            continue;
        }
        if path.is_dir() {
            collect_metadata_scan_files(root, &path, depth + 1, files);
        } else if path.starts_with(root)
            && path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| {
                    matches!(
                        extension,
                        "rs" | "md"
                            | "toml"
                            | "yml"
                            | "yaml"
                            | "sh"
                            | "cmd"
                            | "ps1"
                            | "sql"
                            | "txt"
                    )
                })
        {
            files.push(path);
        }
    }
}

fn build_gui_html(workspace_root: &Path) -> String {
    let workbench = meditor_workbench::WorkbenchBaseline::ver_2_2();
    let platform = meditor_platform::PlatformProfile::current();
    let rest_config = meditor_installation_metrics::BackendEndpointConfig::oracle_apex_default();
    let menu_html = build_menu_html();
    let file_tree_html = build_file_tree_html(workspace_root);
    let modules = build_module_script();
    let module_cards = build_module_cards(&workbench);
    let toolbar_html = build_toolbar_html();
    let connections_html = build_connections_tree_html();
    let reports_html = build_reports_tree_html();

    format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>mEditor</title>
<style>
:root {{
  color-scheme: light;
  --bg: #9fb8d0;
  --panel: #ffffff;
  --line: #d6dbe3;
  --ink: #17202a;
  --muted: #667085;
  --brand: #244c73;
  --accent: #1f8a70;
  --warn: #a15c07;
  --chrome: #a9c0d6;
  --chrome-dark: #89a2ba;
}}
* {{ box-sizing: border-box; }}
body {{
  margin: 0;
  width: 100vw;
  max-width: 100vw;
  height: 100vh;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  color: var(--ink);
  background: var(--bg);
}}
button, select, input, textarea {{
  font: inherit;
}}
.app {{
  width: 100vw;
  max-width: 100vw;
  min-width: 0;
  height: 100vh;
  display: grid;
  grid-template-rows: 34px 36px 1fr;
  background: var(--bg);
}}
.menubar {{
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 10px;
  border-bottom: 1px solid #aebdcc;
  background: #f7f9fb;
  overflow-x: auto;
  overflow-y: visible;
}}
.menu-group {{
  position: relative;
}}
.menu-trigger {{
  border: 0;
  background: transparent;
  cursor: pointer;
  padding: 5px 8px;
  border-radius: 3px;
  color: var(--ink);
}}
.menu-trigger:hover, .menu-group.open .menu-trigger {{ background: #e8eef5; }}
.menu-items {{
  display: none;
  position: absolute;
  z-index: 30;
  top: 30px;
  left: 0;
  min-width: 230px;
  max-height: 520px;
  overflow: auto;
  border: 1px solid #aebdcc;
  background: var(--panel);
  box-shadow: 0 12px 30px rgba(15, 23, 42, .16);
  padding: 6px;
}}
.menu-group.open .menu-items {{
  display: block;
}}
.menu-items button, .context-menu button {{
  width: 100%;
  text-align: left;
  border: 0;
  background: transparent;
  padding: 7px 8px;
  border-radius: 4px;
  cursor: pointer;
}}
.menu-items button:hover, .context-menu button:hover {{ background: #edf4fb; }}
.top-spacer {{ flex: 1; }}
.feedback-button, .primary-button {{
  border: 1px solid var(--brand);
  color: #fff;
  background: var(--brand);
  border-radius: 4px;
  padding: 6px 10px;
  cursor: pointer;
}}
.secondary-button {{
  border: 1px solid var(--line);
  background: #fff;
  border-radius: 4px;
  padding: 6px 10px;
  cursor: pointer;
}}
.toolbar {{
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 8px;
  border-bottom: 1px solid #8ea7bf;
  background: var(--chrome);
  overflow-x: auto;
  overflow-y: hidden;
}}
#windowMode {{
  max-width: 220px;
}}
.tool-button {{
  min-width: 38px;
  width: 38px;
  height: 30px;
  border: 1px solid #8ea7bf;
  border-radius: 3px;
  background: linear-gradient(#f7fbff, #dbe7f2);
  color: #183a59;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}}
.tool-button svg {{
  width: 21px;
  height: 21px;
  stroke-width: 2;
}}
.tool-button:hover {{
  background: linear-gradient(#ffffff, #cfe0ef);
}}
.tool-separator {{
  width: 1px;
  height: 24px;
  margin: 0 3px;
  background: #7e98b0;
}}
.workbench {{
  width: 100%;
  max-width: 100vw;
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-columns: var(--left-width, 320px) 5px minmax(0, 1fr);
  grid-template-rows: minmax(0, 1fr);
  grid-template-areas:
    "left split workspace";
  gap: 0;
  padding: 3px;
  background: var(--bg);
}}
.left-dock {{
  grid-area: left;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}}
.workspace-area {{
  grid-area: workspace;
  min-height: 0;
  min-width: 0;
  display: grid;
  grid-template-rows: minmax(0, 1fr) 5px var(--log-height, 20%);
  overflow: hidden;
}}
.vertical-splitter {{
  grid-area: split;
  cursor: col-resize;
  background: #7e98b0;
  border-left: 1px solid #63829f;
  border-right: 1px solid #bed0df;
}}
.horizontal-splitter, .panel-splitter {{
  cursor: row-resize;
  background: #7e98b0;
  border-top: 1px solid #63829f;
  border-bottom: 1px solid #bed0df;
}}
.horizontal-splitter {{
  min-height: 5px;
}}
.panel-splitter {{
  flex: 0 0 5px;
}}
.dock-pane {{
  min-width: 0;
  min-height: 0;
  flex: 1 1 0;
  background: var(--panel);
  border: 1px solid var(--chrome-dark);
  overflow: auto;
}}
.dock-pane.embedded {{
  overflow: hidden;
  display: grid;
  grid-template-rows: 32px 1fr;
}}
.panel-header {{
  padding: 6px 8px;
  min-height: 32px;
  display: flex;
  align-items: center;
  gap: 7px;
  border-bottom: 1px solid #d8dde4;
  background: #f8f9fb;
}}
.panel-header h2, .panel-header h3 {{
  margin: 0;
  font-size: 15px;
}}
.panel-header .close-mark {{
  margin-left: auto;
  color: #a1a8b0;
}}
.pane-body {{
  min-height: 0;
  overflow: auto;
}}
.hint {{
  color: var(--muted);
  font-size: 12px;
}}
.dock-tools {{
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 5px 7px;
  border-bottom: 1px solid #e0e4ea;
  background: #fbfcfe;
}}
.mini-button {{
  min-width: 22px;
  height: 22px;
  border: 1px solid #aebdcc;
  border-radius: 3px;
  background: #f8fafc;
  color: #244c73;
  cursor: pointer;
  font-size: 12px;
}}
.tree {{
  padding: 4px;
}}
.tree-row {{
  padding: 4px 6px;
  border-radius: 2px;
  cursor: default;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}}
.tree-row:hover {{ background: #eef4fa; }}
.tree-kind {{ color: var(--muted); font-size: 12px; margin-right: 6px; }}
.main {{
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-rows: 34px 1fr;
  border: 1px solid var(--chrome-dark);
  background: #fff;
}}
.tabs {{
  display: flex;
  align-items: end;
  gap: 3px;
  padding: 4px 6px 0;
  overflow: auto;
  border-bottom: 1px solid #aebdcc;
  background: var(--chrome);
}}
.tab-button {{
  border: 1px solid #98acbf;
  border-bottom: 0;
  background: #dce6f0;
  padding: 6px 10px;
  border-radius: 3px 3px 0 0;
  cursor: pointer;
  white-space: nowrap;
}}
.tab-button.active {{
  color: #111827;
  background: #fff;
  border-color: #8ea7bf;
}}
.tab-close {{
  margin-left: 8px;
  color: inherit;
}}
.panels {{
  min-height: 0;
  overflow: hidden;
}}
.tab-panel {{
  display: none;
  height: 100%;
  min-height: 0;
  overflow: auto;
  padding: 0;
  background: var(--panel);
}}
.tab-panel.active {{ display: block; }}
.welcome-page {{
  padding: 14px 18px 18px;
}}
.welcome-header {{
  display: flex;
  align-items: center;
  min-height: 74px;
  border-bottom: 1px solid #d7dde5;
}}
.product-mark {{
  width: 54px;
  height: 54px;
  margin-right: 14px;
  border-radius: 4px;
  background:
    linear-gradient(90deg, #2f7fbf 0 46%, transparent 46% 54%, #65a832 54%),
    linear-gradient(#e9f2fb, #cfe0ef);
  border: 1px solid #9fb8d0;
}}
.welcome-title {{
  font-size: 24px;
  color: #333b45;
}}
.welcome-version {{
  margin-left: auto;
  color: #495463;
  font-size: 14px;
}}
.welcome-grid {{
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
  margin-top: 18px;
}}
.welcome-card {{
  border: 1px solid #d8dde4;
  border-radius: 6px;
  background: #fff;
  min-height: 190px;
  overflow: hidden;
}}
.welcome-card h2 {{
  margin: 0;
  padding: 14px 16px;
  background: #f2f5f9;
  font-size: 18px;
  font-weight: 500;
}}
.welcome-card-body {{
  padding: 16px;
}}
.welcome-tabs {{
  display: flex;
  align-items: end;
  gap: 0;
  border-bottom: 1px solid #d8dde4;
  margin-bottom: 12px;
}}
.welcome-tab {{
  padding: 9px 14px;
  border: 1px solid transparent;
  border-bottom: 0;
  color: #426b9d;
}}
.welcome-tab.active {{
  color: #303946;
  border-color: #d8dde4;
  background: #fff;
}}
.link-row {{
  display: block;
  margin: 8px 0;
  color: #2f6db5;
  cursor: pointer;
}}
.wide-field {{
  width: 100%;
  border: 1px solid #cfd7e1;
  border-radius: 4px;
  padding: 8px;
}}
.metric-grid {{
  display: grid;
  grid-template-columns: repeat(4, minmax(120px, 1fr));
  gap: 10px;
  margin: 10px 0 14px;
}}
.metric, .module-card, .surface {{
  border: 1px solid var(--line);
  background: #fff;
  border-radius: 6px;
  padding: 10px;
}}
.surface {{
  margin: 10px;
}}
.metric label {{
  display: block;
  color: var(--muted);
  font-size: 12px;
  margin-bottom: 4px;
}}
.metric strong {{ font-size: 18px; }}
.module-grid {{
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 8px;
}}
.module-card h4 {{
  margin: 0 0 4px;
  font-size: 14px;
}}
.module-card p {{
  margin: 0;
  color: var(--muted);
  font-size: 12px;
}}
.split {{
  display: grid;
  grid-template-columns: minmax(220px, 280px) minmax(0, 1fr);
  gap: 12px;
}}
.formatter-shell {{
  padding: 10px;
  height: 100%;
  min-height: 0;
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 8px;
}}
.formatter-toolbar {{
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
}}
.formatter-kind {{
  border: 1px solid var(--line);
  border-radius: 4px;
  padding: 6px 8px;
  background: #fff;
}}
.formatter-file {{
  max-width: 260px;
}}
.formatter-grid {{
  min-height: 440px;
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  gap: 10px;
}}
.formatter-panel {{
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-rows: 30px 1fr;
  border: 1px solid var(--line);
  background: #fff;
}}
.formatter-panel h3 {{
  margin: 0;
  padding: 6px 8px;
  border-bottom: 1px solid var(--line);
  background: #f2f5f8;
  font-size: 13px;
}}
.formatter-editor, .formatter-output {{
  width: 100%;
  height: 100%;
  min-height: 390px;
  border: 0;
  padding: 10px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  line-height: 1.45;
  resize: none;
  white-space: pre;
  overflow: auto;
}}
.formatter-output {{
  background: #f8fbff;
}}
.formatter-status {{
  border: 1px solid var(--line);
  background: #f8f9fb;
  padding: 7px 9px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}}
@media (max-width: 900px) {{
  .workbench {{
    grid-template-columns: minmax(220px, 30vw) 5px minmax(0, 1fr);
  }}
  .menubar {{
    gap: 2px;
    padding: 0 4px;
  }}
  .menu-trigger {{
    padding: 5px 6px;
  }}
  #windowMode {{
    max-width: 190px;
  }}
  .split {{
    grid-template-columns: 1fr;
  }}
  .formatter-grid {{
    grid-template-columns: 1fr;
  }}
}}
.editor {{
  width: 100%;
  min-height: 560px;
  border: 1px solid var(--line);
  border-radius: 5px;
  padding: 10px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  line-height: 1.45;
  resize: vertical;
}}
.vi-shell {{
  height: 100%;
  display: grid;
  grid-template-rows: auto 1fr 28px 30px;
  padding: 10px;
  background: #fff;
}}
.vi-toolbar {{
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
  margin-bottom: 8px;
}}
.vi-editor {{
  width: 100%;
  height: 100%;
  min-height: 420px;
  border: 1px solid #6f88a1;
  border-radius: 3px;
  padding: 10px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  line-height: 1.45;
  resize: none;
  white-space: pre;
}}
.vi-editor.normal {{
  caret-color: #244c73;
}}
.vi-editor.insert {{
  caret-color: #1f8a70;
}}
.vi-editor.command {{
  caret-color: #a15c07;
}}
.vi-status {{
  display: flex;
  align-items: center;
  gap: 10px;
  border: 1px solid #d6dbe3;
  border-top: 0;
  background: #eef2f6;
  padding: 4px 8px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}}
.vi-command {{
  border: 1px solid #9fb8d0;
  border-radius: 3px;
  padding: 4px 8px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}}
.vi-command.hidden {{
  visibility: hidden;
}}
.editor-toolbar {{
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
  margin: 8px 0;
}}
.result-list {{
  margin: 8px 0 0;
  padding-left: 18px;
}}
.result-list li {{
  margin-bottom: 8px;
}}
.table {{
  width: 100%;
  border-collapse: collapse;
}}
.table th, .table td {{
  border-bottom: 1px solid var(--line);
  text-align: left;
  padding: 6px;
  vertical-align: top;
}}
.table th {{ background: #f2f5f8; }}
.bottom-log {{
  min-width: 0;
  min-height: 0;
  display: grid;
  grid-template-rows: 30px 1fr 26px;
  background: #fff;
  border: 1px solid var(--chrome-dark);
}}
.log-title {{
  display: flex;
  align-items: center;
  padding: 0 8px;
  border-bottom: 1px solid #d8dde4;
  background: #f8f9fb;
  font-weight: 600;
}}
.log-body {{
  overflow: auto;
  padding: 8px 10px;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}}
.log-tabs {{
  display: flex;
  align-items: end;
  gap: 3px;
  padding-left: 8px;
  background: #eef2f6;
}}
.log-tab {{
  padding: 4px 12px;
  border: 1px solid #d8dde4;
  border-bottom: 0;
  background: #fff;
}}
.command-filter {{
  width: 100%;
  border: 1px solid var(--line);
  border-radius: 5px;
  padding: 7px;
}}
.command-list button {{
  width: 100%;
  text-align: left;
  margin-top: 5px;
}}
.context-menu {{
  display: none;
  position: fixed;
  z-index: 50;
  width: 210px;
  border: 1px solid var(--line);
  background: #fff;
  box-shadow: 0 14px 28px rgba(15, 23, 42, .18);
  padding: 6px;
}}
.modal-backdrop {{
  position: fixed;
  inset: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(31, 47, 66, .42);
}}
.modal-backdrop.hidden {{
  display: none;
}}
.modal {{
  width: min(760px, calc(100vw - 40px));
  max-height: calc(100vh - 40px);
  overflow: auto;
  border: 1px solid #7e98b0;
  border-radius: 6px;
  background: #fff;
  box-shadow: 0 24px 60px rgba(15, 23, 42, .28);
}}
.modal-header {{
  padding: 12px 14px;
  border-bottom: 1px solid var(--line);
  background: #f2f5f8;
}}
.modal-header h2 {{
  margin: 0;
  font-size: 18px;
}}
.modal-body {{
  padding: 14px;
}}
.modal-actions {{
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 12px 14px;
  border-top: 1px solid var(--line);
  background: #fbfcfe;
}}
.license-text {{
  width: 100%;
  min-height: 170px;
  border: 1px solid var(--line);
  border-radius: 4px;
  padding: 8px;
  resize: vertical;
  background: #f8fbff;
}}
.pill {{
  display: inline-block;
  margin: 2px 4px 2px 0;
  padding: 3px 7px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: #f8fafc;
  font-size: 12px;
}}
</style>
</head>
<body>
<div class="app">
  <div class="menubar">
    {menu_html}
    <div class="top-spacer"></div>
    <select id="windowMode" title="Window behavior">
      <option>Same-window tabs</option>
      <option>Allow detached windows</option>
      <option>Ask each time</option>
    </select>
    <button class="feedback-button" onclick="openFeedback()">Feedback</button>
  </div>
  <div class="toolbar" role="toolbar" aria-label="Main toolbar">{toolbar_html}</div>
  <div id="workbench" class="workbench">
    <aside id="leftDock" class="left-dock">
      <section id="connectionsPane" class="dock-pane embedded" style="flex-basis:48%">
        <div class="panel-header"><h2>Connections</h2><span class="close-mark">x</span></div>
        <div class="pane-body">
          <div class="dock-tools">
            <button class="mini-button" title="New connection" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">+</button>
            <button class="mini-button" title="Refresh" onclick="refreshExplorer()">R</button>
            <button class="mini-button" title="Filter">F</button>
            <button class="mini-button" title="SQL worksheet" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">SQL</button>
          </div>
          <div class="tree">{connections_html}</div>
        </div>
      </section>
      <div class="panel-splitter" data-splitter="left-stack" data-before="connectionsPane" data-after="browserPane"></div>
      <section id="browserPane" class="dock-pane embedded" style="flex-basis:32%">
        <div class="panel-header"><h2>Browser</h2><span class="close-mark">x</span></div>
        <div class="pane-body">
          <div class="dock-tools">
            <button class="mini-button" title="Refresh" onclick="refreshExplorer()">R</button>
            <button class="mini-button" title="Import project" onclick="openModule('file.importProject.open','Import Project')">P</button>
            <span class="hint">{workspace}</span>
          </div>
          <div id="fileTree" class="tree">{file_tree_html}</div>
        </div>
      </section>
      <div class="panel-splitter" data-splitter="left-stack" data-before="browserPane" data-after="reportsPane"></div>
      <section id="reportsPane" class="dock-pane embedded" style="flex-basis:20%">
        <div class="panel-header"><h2>Reports</h2><span class="close-mark">x</span></div>
        <div class="pane-body">
          <div class="dock-tools">
            <button class="mini-button" title="New report">+</button>
            <button class="mini-button" title="Refresh">R</button>
            <button class="mini-button" title="Export">E</button>
          </div>
          <div class="tree">{reports_html}</div>
        </div>
      </section>
    </aside>
    <div class="vertical-splitter" data-splitter="left-width"></div>
    <section id="workspaceArea" class="workspace-area">
      <main class="main">
        <div id="tabs" class="tabs"></div>
        <div id="panels" class="panels"></div>
      </main>
      <div class="horizontal-splitter" data-splitter="log-height"></div>
      <section class="bottom-log">
        <div class="log-title">Messages - Log</div>
        <div class="log-body"><span id="status">Ready</span><br>mEditor {version} workbench loaded. Platform: {platform:?}. Modules: {module_count}. APEX: {apex_base}</div>
        <div class="log-tabs"><div class="log-tab">Messages</div><div class="log-tab">Logging Page</div></div>
      </section>
    </section>
  </div>
</div>
<div id="contextMenu" class="context-menu">
  <button onclick="openContextFile()">Open in Editor Tab</button>
  <button onclick="openContextFormatter()">Open in Formatter</button>
  <button onclick="openContextVi()">Open in Vi Editor</button>
  <button onclick="scanContextFile()">Security Scan File</button>
  <button onclick="copyContextPath()">Copy Path</button>
</div>
<div id="eulaGate" class="modal-backdrop">
  <div class="modal">
    <div class="modal-header"><h2>mEditor Freeware EULA</h2></div>
    <div class="modal-body">
      <p class="hint">mEditor is freeware. Registration is optional and has no licensing or financial obligation for you or your organization. Registration only personalizes this copy and helps future update notifications.</p>
      <textarea class="license-text" readonly>mEditor Freeware EULA

You may use this freeware copy without payment or licensing obligation. Registration is optional. If you register, the About dialog displays "Personal License to USERNAME". If you do not register, the About dialog displays "UnRegistered but fully functional copy with no obligation".

Product diagnostics and installation records are used to understand install base and fix mEditor issues. Local project bug repositories created for your own programs remain local unless you export them yourself.

Owner contact: s.pandey.india@gmail.com</textarea>
      <p><input id="eulaName" class="command-filter" placeholder="Name for optional registration"></p>
      <p><input id="eulaEmail" class="command-filter" placeholder="Email for optional registration"></p>
      <p id="eulaStatus" class="hint">Accept the EULA to continue. Registration is optional.</p>
    </div>
    <div class="modal-actions">
      <button class="secondary-button" onclick="acceptEula(false)">Accept And Continue Unregistered</button>
      <button class="primary-button" onclick="acceptEula(true)">Accept And Register</button>
    </div>
  </div>
</div>
<div id="errorDialog" class="modal-backdrop hidden">
  <div class="modal">
    <div class="modal-header"><h2 id="errorDialogTitle">mEditor Error</h2></div>
    <div class="modal-body"><pre id="errorDialogMessage" style="white-space:pre-wrap;margin:0"></pre></div>
    <div class="modal-actions"><button class="primary-button" onclick="closeErrorDialog()">OK</button></div>
  </div>
</div>
<script>
{modules}
const defaultDashboard = {dashboard};
let activeContextPath = null;
let activeEditorPath = null;
let licenseState = {{ accepted: false, registered: false, displayName: 'UnRegistered but fully functional copy with no obligation' }};
const validationTimers = {{}};
const realtimeValidation = {{}};
const viStates = {{}};
const formatterTimers = {{}};
const formatterState = {{}};

function escapeHtml(value) {{
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}}

function setStatus(message) {{
  document.getElementById('status').textContent = message;
}}

function hostRequest(message) {{
  if (window.ipc && window.ipc.postMessage) {{
    window.ipc.postMessage(JSON.stringify(message));
  }} else {{
    setStatus('Native host bridge is not available');
  }}
}}

function showErrorDialog(title, message) {{
  const dialog = document.getElementById('errorDialog');
  const titleNode = document.getElementById('errorDialogTitle');
  const messageNode = document.getElementById('errorDialogMessage');
  if (titleNode) titleNode.textContent = title || 'mEditor Error';
  if (messageNode) messageNode.textContent = message || 'Operation failed';
  if (dialog) dialog.classList.remove('hidden');
}}

function closeErrorDialog() {{
  const dialog = document.getElementById('errorDialog');
  if (dialog) dialog.classList.add('hidden');
}}

function requireField(id, label) {{
  const node = document.getElementById(id);
  const value = node ? String(node.value || '').trim() : '';
  if (!value) {{
    showErrorDialog('Missing Field', label + ' is required.');
    if (node) node.focus();
    return null;
  }}
  return value;
}}

function refreshLicenseDisplay() {{
  document.querySelectorAll('[data-license-display]').forEach(node => {{
    node.textContent = licenseState.displayName || 'UnRegistered but fully functional copy with no obligation';
  }});
}}

function acceptEula(registered) {{
  const name = document.getElementById('eulaName')?.value || '';
  const email = document.getElementById('eulaEmail')?.value || '';
  document.getElementById('eulaStatus').textContent = 'Saving EULA acceptance...';
  hostRequest({{ action: 'acceptEula', registered, name, email }});
}}

function credentialStoreStatus() {{
  hostRequest({{ action: 'credentialStoreStatus' }});
}}

function storeCredential() {{
  hostRequest({{
    action: 'storeCredential',
    service: document.getElementById('credentialService')?.value || 'mEditor',
    account: document.getElementById('credentialAccount')?.value || '',
    secret: document.getElementById('credentialSecret')?.value || ''
  }});
}}

function saveUpdateSource() {{
  hostRequest({{
    action: 'saveUpdateSource',
    source: document.getElementById('updateSource')?.value || '',
    channel: document.getElementById('updateChannel')?.value || 'stable'
  }});
}}

function checkUpdateSource() {{
  hostRequest({{ action: 'checkUpdateSource' }});
}}

function detectLspServers() {{
  hostRequest({{ action: 'detectLspServers' }});
}}

function detectDebugAdapters() {{
  hostRequest({{ action: 'detectDebugAdapters' }});
}}

function applyRenameRefactor() {{
  const path = requireField('refactorPath', 'File path');
  const from = requireField('refactorFrom', 'Old symbol/text');
  const to = requireField('refactorTo', 'New symbol/text');
  if (!path || !from || !to) return;
  if (!confirm('Apply rename/refactor to ' + path + '? mEditor will create a rollback backup first.')) return;
  hostRequest({{ action: 'applyRenameRefactor', path, from, to }});
}}

function runLocalAi() {{
  hostRequest({{
    action: 'runLocalAi',
    model: document.getElementById('assistantModel')?.value || 'llama3',
    prompt: document.getElementById('assistantPrompt')?.value || ''
  }});
}}

function retrainAiKnowledge() {{
  hostRequest({{ action: 'retrainAiKnowledge' }});
}}

function trainAiLocalModel() {{
  hostRequest({{ action: 'trainAiLocalModel' }});
}}

function exportAiTrainingDataset() {{
  hostRequest({{ action: 'exportAiTrainingDataset' }});
}}

function saveAiTrainerConfig() {{
  hostRequest({{
    action: 'saveAiTrainerConfig',
    executable: document.getElementById('aiTrainerExecutable')?.value || '',
    args: document.getElementById('aiTrainerArgs')?.value || ''
  }});
}}

function runAiTrainingJob() {{
  const executable = document.getElementById('aiTrainerExecutable')?.value || '';
  if (!executable.trim()) {{
    showErrorDialog('AI Trainer', 'Save a trainer executable before running an external training job.');
    return;
  }}
  if (!confirm('Run the configured external AI trainer now? mEditor will pass the exported dataset path and capture the transcript.')) return;
  hostRequest({{ action: 'runAiTrainingJob' }});
}}

function runSshCommand() {{
  hostRequest({{
    action: 'runSshCommand',
    host: document.getElementById('sshHost')?.value || '',
    user: document.getElementById('sshUser')?.value || '',
    remoteCommand: document.getElementById('sshCommand')?.value || 'uname -a'
  }});
}}

function startTerminalSession(local) {{
  const host = document.getElementById('sshHost')?.value || '';
  const user = document.getElementById('sshUser')?.value || '';
  if (!local && !host.trim()) {{
    showErrorDialog('SSH Terminal', 'Host is required for an SSH terminal session.');
    return;
  }}
  hostRequest({{ action: 'startTerminalSession', host, user, local: Boolean(local) }});
}}

function sendTerminalInput() {{
  const sessionId = document.getElementById('terminalSessionId')?.value || '';
  const input = document.getElementById('terminalInput')?.value || '';
  if (!sessionId) {{
    showErrorDialog('Terminal Session', 'Start a terminal session first.');
    return;
  }}
  hostRequest({{ action: 'sendTerminalInput', sessionId, input }});
  const inputNode = document.getElementById('terminalInput');
  if (inputNode) inputNode.value = '';
}}

function pollTerminalSession() {{
  const sessionId = document.getElementById('terminalSessionId')?.value || '';
  if (sessionId) hostRequest({{ action: 'pollTerminalSession', sessionId }});
}}

function stopTerminalSession() {{
  const sessionId = document.getElementById('terminalSessionId')?.value || '';
  if (sessionId) hostRequest({{ action: 'stopTerminalSession', sessionId }});
}}

function enqueueTransfer() {{
  hostRequest({{
    action: 'enqueueTransfer',
    direction: document.getElementById('transferDirection')?.value || 'download',
    localPath: document.getElementById('transferLocalPath')?.value || '',
    remoteTarget: document.getElementById('transferRemoteTarget')?.value || ''
  }});
}}

function runFileTransfer() {{
  hostRequest({{
    action: 'runFileTransfer',
    direction: document.getElementById('transferDirection')?.value || 'download',
    localPath: document.getElementById('transferLocalPath')?.value || '',
    remoteTarget: document.getElementById('transferRemoteTarget')?.value || ''
  }});
}}

function runJdbcSql() {{
  if (!requireField('jdbcJarPath', 'JDBC driver JAR')) return;
  if (!requireField('jdbcUrl', 'JDBC URL')) return;
  hostRequest({{
    action: 'runJdbcSql',
    jarPath: document.getElementById('jdbcJarPath')?.value || '',
    jdbcUrl: document.getElementById('jdbcUrl')?.value || '',
    dbUser: document.getElementById('jdbcUser')?.value || '',
    dbPassword: document.getElementById('jdbcPassword')?.value || '',
    sql: document.getElementById('dbaSqlText')?.value || ''
  }});
}}

function runJdbcMetadata(mode) {{
  if (!requireField('jdbcJarPath', 'JDBC driver JAR')) return;
  if (!requireField('jdbcUrl', 'JDBC URL')) return;
  hostRequest({{
    action: 'jdbcMetadata',
    jarPath: document.getElementById('jdbcJarPath')?.value || '',
    jdbcUrl: document.getElementById('jdbcUrl')?.value || '',
    dbUser: document.getElementById('jdbcUser')?.value || '',
    dbPassword: document.getElementById('jdbcPassword')?.value || '',
    mode,
    schema: document.getElementById('jdbcSchema')?.value || '',
    object: document.getElementById('jdbcObject')?.value || '%'
  }});
}}

function runJdbcDbaProbe() {{
  if (!requireField('jdbcJarPath', 'JDBC driver JAR')) return;
  if (!requireField('jdbcUrl', 'JDBC URL')) return;
  hostRequest({{
    action: 'jdbcDbaProbe',
    jarPath: document.getElementById('jdbcJarPath')?.value || '',
    jdbcUrl: document.getElementById('jdbcUrl')?.value || '',
    dbUser: document.getElementById('jdbcUser')?.value || '',
    dbPassword: document.getElementById('jdbcPassword')?.value || '',
    probe: document.getElementById('jdbcDbaProbe')?.value || 'generic_version'
  }});
}}

function crossPlatformPackageAudit() {{
  hostRequest({{ action: 'crossPlatformPackageAudit' }});
}}

function moduleResultSlot(command) {{
  return '<div id="' + safeId('module-result-' + command) + '" class="surface" style="margin-top:10px">' +
    '<h3>Backend</h3><p class="hint">Waiting for Rust backend response.</p></div>';
}}

function runDbaSql() {{
  const sql = document.getElementById('dbaSqlText')?.value || '';
  hostRequest({{ action: 'dbaRunSql', sql }});
}}

function explainDbaSql() {{
  const sql = document.getElementById('dbaSqlText')?.value || '';
  hostRequest({{ action: 'dbaExplainPlan', sql }});
}}

function detectToolchains() {{
  hostRequest({{ action: 'detectToolchains' }});
}}

function verifyDependencies() {{
  hostRequest({{ action: 'verifyDependencies' }});
}}

function installSelectedDependency() {{
  const dependencyId = document.getElementById('dependencyInstallSelect')?.value || '';
  const command = document.getElementById('dependencyInstallCommand')?.textContent || '';
  if (!dependencyId) {{
    showErrorDialog('Dependency Install', 'Select a dependency first.');
    return;
  }}
  if (!confirm('mEditor will run this install command now:\\n\\n' + command + '\\n\\nContinue?')) return;
  hostRequest({{ action: 'installDependency', dependencyId }});
}}

function dependencySelectionChanged() {{
  const select = document.getElementById('dependencyInstallSelect');
  const command = document.getElementById('dependencyInstallCommand');
  const source = document.getElementById('dependencySourceUrl');
  if (!select || !command) return;
  const option = select.options[select.selectedIndex];
  command.textContent = option?.dataset.command || '';
  if (source) source.textContent = option?.dataset.source || '';
}}

function previewSqliteSetup() {{
  hostRequest({{ action: 'setupSqlitePreview' }});
}}

function createSqliteFolders() {{
  hostRequest({{ action: 'createSqliteFolders' }});
}}

function runOneClickSetup() {{
  hostRequest({{ action: 'oneClickSetup' }});
}}

function previewProjectImport() {{
  hostRequest({{ action: 'projectImportPreview' }});
}}

function importProject() {{
  hostRequest({{ action: 'importProject' }});
}}

function createProject() {{
  const name = document.getElementById('newProjectName')?.value || '';
  const kind = document.getElementById('newProjectKind')?.value || 'major';
  const techStack = document.getElementById('newProjectTechStack')?.value || '';
  hostRequest({{ action: 'createProject', name, kind, techStack }});
}}

function detectProject() {{
  const projectPath = document.getElementById('taskProjectPath')?.value || '.';
  hostRequest({{ action: 'detectProject', projectPath }});
}}

function runQuickTask(taskAction) {{
  const selector = document.getElementById('taskAction');
  if (selector) selector.value = taskAction;
  runTask();
}}

function runTask() {{
  const projectPath = document.getElementById('taskProjectPath')?.value || '.';
  const taskAction = document.getElementById('taskAction')?.value || 'build';
  const sourcePath = document.getElementById('taskSourcePath')?.value || activeEditorPath || '';
  const timeoutSeconds = Number(document.getElementById('taskTimeoutSeconds')?.value || 60);
  appendTaskLog('Starting ' + taskAction + ' for ' + projectPath + '...');
  hostRequest({{ action: 'runTask', projectPath, taskAction, sourcePath, timeoutSeconds }});
}}

function runVcs(provider, operation) {{
  const projectPath = document.getElementById(provider + 'ProjectPath')?.value || '.';
  hostRequest({{ action: 'vcsCommand', provider, operation, projectPath }});
}}

function saveDbConnection() {{
  hostRequest({{
    action: 'saveDbConnection',
    name: document.getElementById('dbConnName')?.value || '',
    kind: document.getElementById('dbConnKind')?.value || 'sqlite',
    target: document.getElementById('dbConnTarget')?.value || ''
  }});
}}

function runSqliteWorkbench() {{
  hostRequest({{
    action: 'runSqliteSql',
    dbPath: document.getElementById('sqliteDbPath')?.value || '.meditor/security/vulnerability-intel.sqlite',
    sql: document.getElementById('dbaSqlText')?.value || ''
  }});
}}

function addJdbcDriver() {{
  hostRequest({{ action: 'addJdbcDriver', jarPath: document.getElementById('jdbcJarPath')?.value || '' }});
}}

function saveSshProfile() {{
  hostRequest({{
    action: 'saveSshProfile',
    name: document.getElementById('sshName')?.value || '',
    group: document.getElementById('sshGroup')?.value || 'Default',
    host: document.getElementById('sshHost')?.value || '',
    user: document.getElementById('sshUser')?.value || ''
  }});
}}

function testSshConnection() {{
  hostRequest({{
    action: 'testSshConnection',
    host: document.getElementById('sshHost')?.value || '',
    user: document.getElementById('sshUser')?.value || ''
  }});
}}

function sftpList() {{
  hostRequest({{ action: 'sftpList', target: document.getElementById('sftpTarget')?.value || '' }});
}}

function saveLocalBug() {{
  hostRequest({{
    action: 'saveLocalBug',
    title: document.getElementById('localBugTitle')?.value || '',
    severity: document.getElementById('localBugSeverity')?.value || 'Medium',
    details: document.getElementById('localBugDetails')?.value || ''
  }});
}}

function listLocalBugs() {{
  hostRequest({{ action: 'listLocalBugs' }});
}}

function listReports() {{
  hostRequest({{ action: 'listReports' }});
}}

function savePlannerItem() {{
  hostRequest({{
    action: 'savePlannerItem',
    title: document.getElementById('plannerTitle')?.value || '',
    model: document.getElementById('plannerModel')?.value || 'objective',
    status: document.getElementById('plannerStatus')?.value || 'Planned'
  }});
}}

function saveAiKnowledge() {{
  hostRequest({{
    action: 'saveAiKnowledge',
    title: document.getElementById('knowledgeTitle')?.value || '',
    content: document.getElementById('knowledgeContent')?.value || ''
  }});
}}

function createExtensionSkeleton() {{
  hostRequest({{ action: 'createExtensionSkeleton' }});
}}

function scanWorkspaceIndex() {{
  hostRequest({{ action: 'scanWorkspaceIndex' }});
}}

function submitFeedback() {{
  hostRequest({{
    action: 'submitFeedback',
    feedbackType: document.getElementById('feedbackKind')?.value || 'BUG',
    title: document.getElementById('feedbackTitle')?.value || '',
    details: document.getElementById('feedbackDetails')?.value || ''
  }});
}}

function browserNavigate() {{
  const input = document.getElementById('browserUrl');
  const frame = document.getElementById('browserFrame');
  if (!input || !frame) return;
  const raw = input.value || 'about:blank';
  frame.src = raw.match(/^https?:|^file:|^about:/) ? raw : 'https://' + raw;
  setStatus('Browser navigated to ' + frame.src);
}}

function previewCvssUpdate() {{
  hostRequest({{ action: 'cvssUpdatePreview' }});
}}

function askAssistant() {{
  const prompt = document.getElementById('assistantPrompt')?.value || '';
  hostRequest({{ action: 'askAssistant', prompt }});
}}

function previewFeedback(feedbackType) {{
  const email = document.getElementById('feedbackEmail')?.value || '';
  const title = document.getElementById('feedbackTitle')?.value || '';
  const details = document.getElementById('feedbackDetails')?.value || '';
  hostRequest({{ action: 'feedbackPreview', feedbackType, email, title, details }});
}}

function previewDiagnostics() {{
  hostRequest({{ action: 'diagnosticsPreview' }});
}}

function runProductionReadiness() {{
  hostRequest({{ action: 'productionReadiness' }});
}}

function previewUpdateCheck() {{
  hostRequest({{ action: 'updateCheckPreview' }});
}}

function safeId(value) {{
  return 'tab-' + String(value).replace(/[^a-zA-Z0-9_-]/g, '_');
}}

function selectTab(id) {{
  document.querySelectorAll('.tab-button').forEach(tab => tab.classList.toggle('active', tab.dataset.tab === id));
  document.querySelectorAll('.tab-panel').forEach(panel => panel.classList.toggle('active', panel.dataset.tab === id));
  const activePanel = document.querySelector('.tab-panel[data-tab="' + id + '"]');
  if (activePanel && activePanel.dataset.filePath) activeEditorPath = activePanel.dataset.filePath;
}}

function closeTab(id, event) {{
  event.stopPropagation();
  const button = document.querySelector('.tab-button[data-tab="' + id + '"]');
  const panel = document.querySelector('.tab-panel[data-tab="' + id + '"]');
  const wasActive = button && button.classList.contains('active');
  if (button) button.remove();
  if (panel) panel.remove();
  if (wasActive) {{
    const next = document.querySelector('.tab-button');
    if (next) selectTab(next.dataset.tab);
  }}
}}

function openTab(id, title, body) {{
  const tabs = document.getElementById('tabs');
  const panels = document.getElementById('panels');
  const tabId = safeId(id);
  if (!document.querySelector('.tab-button[data-tab="' + tabId + '"]')) {{
    const button = document.createElement('button');
    button.className = 'tab-button';
    button.dataset.tab = tabId;
    button.innerHTML = escapeHtml(title) + '<span class="tab-close" title="Close">x</span>';
    button.onclick = event => {{
      if (event.target.classList.contains('tab-close')) {{
        closeTab(tabId, event);
      }} else {{
        selectTab(tabId);
      }}
    }};
    tabs.appendChild(button);

    const panel = document.createElement('section');
    panel.className = 'tab-panel';
    panel.dataset.tab = tabId;
    panel.innerHTML = body;
    panels.appendChild(panel);
  }}
  selectTab(tabId);
}}

function openModule(command, label) {{
  if (command === 'source.reformat') {{
    openFormatterFromActive();
    return;
  }}
  if (command === 'source.viEditor.open') {{
    openViFromActive();
    return;
  }}
  closeMenus();
  const body = (moduleSurfaces[command] || genericModule(command, label)) + moduleResultSlot(command);
  openTab(command, label, body);
  refreshLicenseDisplay();
  setStatus('Opened ' + label + ' in same-window tab mode');
  hostRequest({{ action: 'moduleOpened', command, label }});
}}

function genericModule(command, label) {{
  return '<div class="surface"><h2>' + escapeHtml(label) + '</h2><p class="hint">' + escapeHtml(command) +
    '</p><p>This module runs its local Rust backend workflow on open and records the result below.</p></div>';
}}

function openEditorTab(path, content) {{
  const editorId = safeId('editor-' + path);
  const resultId = safeId('result-' + path);
  const detectedKind = validationKindForPath(path);
  const realtimeControl = detectedKind
    ? '<label class="hint"><input type="checkbox" checked data-path="' + escapeHtml(path) + '" onchange="setRealtimeValidation(this.dataset.path,this.checked)"> Realtime Validation (' + detectedKind.toUpperCase() + ')</label>'
    : '<span class="hint">Realtime validation is active for JSON and XML-family files.</span>';
  const body = '<h2>' + escapeHtml(path) + '</h2>' +
    '<div class="editor-toolbar">' +
    '<button class="primary-button" data-path="' + escapeHtml(path) + '" onclick="saveFile(this.dataset.path)">Save</button>' +
    '<button class="secondary-button" data-path="' + escapeHtml(path) + '" data-kind="json" onclick="validateFile(this.dataset.path,this.dataset.kind)">Validate JSON</button>' +
    '<button class="secondary-button" data-path="' + escapeHtml(path) + '" data-kind="xml" onclick="validateFile(this.dataset.path,this.dataset.kind)">Validate XML</button>' +
    '<button class="secondary-button" data-path="' + escapeHtml(path) + '" onclick="openFormatterForPath(this.dataset.path)">Open Formatter</button>' +
    '<button class="secondary-button" data-path="' + escapeHtml(path) + '" onclick="openViForPath(this.dataset.path)">Open Vi</button>' +
    '<button class="secondary-button" data-path="' + escapeHtml(path) + '" onclick="securityScanFile(this.dataset.path)">Security Scan</button>' +
    realtimeControl +
    '</div>' +
    '<textarea id="' + editorId + '" class="editor" data-path="' + escapeHtml(path) + '" oninput="scheduleRealtimeValidation(this.dataset.path)">' + escapeHtml(content) + '</textarea>' +
    '<div id="' + resultId + '" class="surface" style="margin-top:10px;display:none"></div>';
  openTab('file:' + path, path, body);
  const panel = document.querySelector('.tab-panel[data-tab="' + safeId('file:' + path) + '"]');
  if (panel) panel.dataset.filePath = path;
  activeEditorPath = path;
  realtimeValidation[path] = Boolean(detectedKind);
  if (detectedKind) scheduleRealtimeValidation(path, 30);
  setStatus('Opened ' + path + ' in an editor tab');
}}

function openFile(path) {{
  hostRequest({{ action: 'openFile', path }});
}}

function openViForPath(path) {{
  const existingEditor = document.getElementById(safeId('editor-' + path));
  if (existingEditor) {{
    openViEditorTab(path, existingEditor.value);
  }} else {{
    hostRequest({{ action: 'openFileForVi', path }});
  }}
}}

function openViFromActive() {{
  if (activeEditorPath) {{
    openViForPath(activeEditorPath);
  }} else {{
    openViEditorTab('untitled-vi.txt', '');
  }}
}}

function saveFile(path) {{
  const editor = document.getElementById(safeId('editor-' + path));
  if (!editor) return;
  hostRequest({{ action: 'saveFile', path, content: editor.value }});
}}

function validateFile(path, kind, options) {{
  const editor = document.getElementById(safeId('editor-' + path));
  hostRequest({{ action: 'validateFile', path, kind, content: editor ? editor.value : '', realtime: Boolean(options && options.realtime) }});
}}

function validationKindForPath(path) {{
  const lower = String(path).toLowerCase();
  if (lower.endsWith('.json') || lower.endsWith('.schema.json')) return 'json';
  if (lower.endsWith('.xml') || lower.endsWith('.xsd') || lower.endsWith('.xsl') || lower.endsWith('.xslt') || lower.endsWith('.wsdl') || lower.endsWith('pom.xml')) return 'xml';
  return null;
}}

function reformatActiveFile() {{
  openFormatterFromActive();
}}

function formatterKindForPath(path, fallback) {{
  return validationKindForPath(path || '') || fallback || 'json';
}}

function openFormatterForKind(kind) {{
  const normalized = kind === 'xml' ? 'xml' : 'json';
  openFormatterWorkbench(normalized, normalized.toUpperCase() + ' Formatter', '', '');
}}

function openFormatterFromActive() {{
  if (!activeEditorPath) {{
    openFormatterWorkbench('json', 'JSON/XML Formatter', '', '');
    setStatus('Opened JSON/XML Formatter. Paste text or upload a file in the left panel.');
    return;
  }}
  openFormatterForPath(activeEditorPath);
}}

function openFormatterForPath(path) {{
  const existingEditor = document.getElementById(safeId('editor-' + path));
  if (existingEditor) {{
    openFormatterWorkbench(formatterKindForPath(path, 'json'), 'Formatter: ' + path, path, existingEditor.value);
  }} else {{
    hostRequest({{ action: 'openFileForFormatter', path }});
  }}
}}

function openFormatterWorkbench(kind, label, path, content) {{
  const normalized = kind === 'xml' ? 'xml' : 'json';
  const sourceLabel = path || label || (normalized.toUpperCase() + ' Formatter');
  const id = safeId('formatter-' + sourceLabel);
  const kindSelect = '<select id="' + id + '-kind" class="formatter-kind" onchange="formatterKindChanged(\'' + id + '\')">' +
    '<option value="json">JSON</option><option value="xml">XML</option></select>';
  const body = '<div class="formatter-shell">' +
    '<div class="formatter-toolbar">' +
    '<strong>' + escapeHtml(label || 'JSON/XML Formatter') + '</strong>' +
    kindSelect +
    '<input id="' + id + '-file" class="formatter-file" type="file" accept=".json,.xml,.xsd,.xsl,.xslt,.wsdl,.txt,application/json,text/xml,application/xml" onchange="formatterUploadFile(\'' + id + '\', this.files && this.files[0])">' +
    '<button class="secondary-button" onclick="formatterPaste(\'' + id + '\')">Paste</button>' +
    '<button class="secondary-button" onclick="formatterCopyInput(\'' + id + '\')">Copy Input</button>' +
    '<button class="secondary-button" onclick="formatterCopyOutput(\'' + id + '\')">Copy Formatted</button>' +
    '<button class="primary-button" onclick="formatterApplyToEditor(\'' + id + '\')">Apply To Open Editor</button>' +
    '</div>' +
    '<div class="formatter-grid">' +
    '<section class="formatter-panel"><h3>Input</h3><textarea id="' + id + '-input" class="formatter-editor" spellcheck="false" data-formatter-id="' + id + '" oninput="scheduleFormatterPreview(this.dataset.formatterId)">' + escapeHtml(content || '') + '</textarea></section>' +
    '<section class="formatter-panel"><h3>Formatted Result</h3><textarea id="' + id + '-output" class="formatter-output" readonly spellcheck="false"></textarea></section>' +
    '</div>' +
    '<div id="' + id + '-status" class="formatter-status">Paste, type, or upload JSON/XML. Formatting runs live through the Rust backend.</div>' +
    '</div>';
  openTab('formatter:' + sourceLabel, label || 'JSON/XML Formatter', body);
  formatterState[id] = {{ path: path || '', kind: normalized }};
  const input = document.getElementById(id + '-input');
  if (input && content !== undefined && content !== null) input.value = String(content);
  const select = document.getElementById(id + '-kind');
  if (select) select.value = normalized;
  scheduleFormatterPreview(id, 30);
}}

function formatterKindChanged(id) {{
  const state = formatterState[id] || {{}};
  const select = document.getElementById(id + '-kind');
  state.kind = select ? select.value : 'json';
  formatterState[id] = state;
  scheduleFormatterPreview(id, 30);
}}

function scheduleFormatterPreview(id, delay) {{
  clearTimeout(formatterTimers[id]);
  formatterTimers[id] = setTimeout(() => formatFormatterText(id), delay === undefined ? 250 : delay);
}}

function formatFormatterText(id) {{
  const input = document.getElementById(id + '-input');
  const select = document.getElementById(id + '-kind');
  if (!input) return;
  const kind = select ? select.value : (formatterState[id]?.kind || 'json');
  formatterState[id] = Object.assign(formatterState[id] || {{}}, {{ kind }});
  hostRequest({{ action: 'formatText', target: id, kind, content: input.value }});
}}

function formatterUploadFile(id, file) {{
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {{
    const input = document.getElementById(id + '-input');
    if (!input) return;
    const kind = formatterKindForPath(file.name, formatterState[id]?.kind || 'json');
    const select = document.getElementById(id + '-kind');
    if (select) select.value = kind;
    formatterState[id] = Object.assign(formatterState[id] || {{}}, {{ kind, path: '' }});
    input.value = String(reader.result || '');
    formatterSetStatus(id, 'Uploaded ' + file.name + '. Live formatting is running.');
    scheduleFormatterPreview(id, 20);
  }};
  reader.onerror = () => formatterSetStatus(id, 'Unable to read uploaded file.');
  reader.readAsText(file);
}}

async function formatterPaste(id) {{
  const input = document.getElementById(id + '-input');
  if (!input) return;
  if (!navigator.clipboard || !navigator.clipboard.readText) {{
    input.focus();
    formatterSetStatus(id, 'Clipboard API is unavailable. Use normal paste in the input panel.');
    return;
  }}
  try {{
    input.value = await navigator.clipboard.readText();
    input.focus();
    formatterSetStatus(id, 'Clipboard text pasted into input.');
    scheduleFormatterPreview(id, 20);
  }} catch (error) {{
    input.focus();
    formatterSetStatus(id, 'Clipboard paste was blocked. Use normal paste in the input panel.');
  }}
}}

function formatterCopyInput(id) {{
  const input = document.getElementById(id + '-input');
  if (input) copyTextToClipboard(input.value, () => formatterSetStatus(id, 'Input copied to clipboard.'));
}}

function formatterCopyOutput(id) {{
  const output = document.getElementById(id + '-output');
  if (output) copyTextToClipboard(output.value, () => formatterSetStatus(id, 'Formatted result copied to clipboard.'));
}}

function formatterApplyToEditor(id) {{
  const state = formatterState[id] || {{}};
  const output = document.getElementById(id + '-output');
  if (!state.path || !output) {{
    formatterSetStatus(id, 'No matching open editor tab. Use Copy Formatted for uploaded or pasted text.');
    return;
  }}
  const editor = document.getElementById(safeId('editor-' + state.path));
  if (!editor) {{
    formatterSetStatus(id, 'Open the source file in an editor tab before applying the formatted text.');
    return;
  }}
  editor.value = output.value;
  scheduleRealtimeValidation(state.path, 30);
  formatterSetStatus(id, 'Formatted text applied in memory to ' + state.path + '. Press Save in the editor tab to write it.');
}}

function formatterSetStatus(id, message) {{
  const status = document.getElementById(id + '-status');
  if (status) status.textContent = message;
  setStatus(message);
}}

function renderFormatterResult(response) {{
  const id = response.target;
  const output = document.getElementById(id + '-output');
  if (!output) return;
  const messages = response.messages || [];
  output.value = response.valid ? (response.content || '') : messages.join('\\n');
  if (output.parentElement) output.parentElement.style.borderColor = response.valid ? '#1f8a70' : '#a15c07';
  formatterSetStatus(id, messages.join(' ') || (response.valid ? 'Formatted.' : 'Waiting for valid input.'));
}}

function copyTextToClipboard(text, done) {{
  if (navigator.clipboard && navigator.clipboard.writeText) {{
    navigator.clipboard.writeText(text).then(done).catch(() => fallbackCopyText(text, done));
  }} else {{
    fallbackCopyText(text, done);
  }}
}}

function fallbackCopyText(text, done) {{
  const scratch = document.createElement('textarea');
  scratch.value = text;
  scratch.style.position = 'fixed';
  scratch.style.left = '-9999px';
  document.body.appendChild(scratch);
  scratch.focus();
  scratch.select();
  document.execCommand('copy');
  scratch.remove();
  if (done) done();
}}

function setRealtimeValidation(path, enabled) {{
  realtimeValidation[path] = enabled;
  if (enabled) {{
    scheduleRealtimeValidation(path, 30);
    setStatus('Realtime validation enabled for ' + path);
  }} else {{
    clearTimeout(validationTimers[path]);
    setStatus('Realtime validation paused for ' + path);
  }}
}}

function scheduleRealtimeValidation(path, delay) {{
  if (!realtimeValidation[path]) return;
  const kind = validationKindForPath(path);
  if (!kind) return;
  clearTimeout(validationTimers[path]);
  validationTimers[path] = setTimeout(() => validateFile(path, kind, {{ realtime: true }}), delay === undefined ? 350 : delay);
}}

function openViEditorTab(path, content) {{
  const id = safeId('vi-' + path);
  const body = '<div class="vi-shell">' +
    '<div class="vi-toolbar">' +
    '<strong>' + escapeHtml(path) + '</strong>' +
    '<button class="secondary-button" data-vi-id="' + id + '" onclick="viSave(this.dataset.viId)">:w</button>' +
    '<button class="secondary-button" data-vi-id="' + id + '" onclick="viSetMode(this.dataset.viId,\'insert\')">Insert</button>' +
    '<button class="secondary-button" data-vi-id="' + id + '" onclick="viSetMode(this.dataset.viId,\'normal\')">Normal</button>' +
    '<button class="secondary-button" data-vi-id="' + id + '" onclick="viShowCommand(this.dataset.viId,\':\')">:</button>' +
    '<button class="secondary-button" data-vi-id="' + id + '" onclick="viShowCommand(this.dataset.viId,\'/\')">/</button>' +
    '<span class="hint">Esc normal, i/a/o insert, h/j/k/l move, dd delete, yy yank, p paste, u undo, :w save, :q close.</span>' +
    '</div>' +
    '<textarea id="' + id + '" class="vi-editor normal" spellcheck="false" data-path="' + escapeHtml(path) + '" onkeydown="viKeydown(event,this.id)" oninput="viInput(this.id)">' + escapeHtml(content) + '</textarea>' +
    '<div class="vi-status"><span id="' + id + '-mode">NORMAL</span><span id="' + id + '-message">Ready</span><span id="' + id + '-pos"></span></div>' +
    '<input id="' + id + '-command" class="vi-command hidden" onkeydown="viCommandKey(event,\'' + id + '\')" autocomplete="off" spellcheck="false">' +
    '</div>';
  openTab('vi:' + path, 'Vi: ' + path, body);
  const panel = document.querySelector('.tab-panel[data-tab="' + safeId('vi:' + path) + '"]');
  if (panel) panel.dataset.filePath = path;
  activeEditorPath = path;
  viStates[id] = {{
    path,
    mode: 'normal',
    yank: '',
    pending: '',
    search: '',
    number: false,
    undo: [content],
    redo: [],
    lastValue: content,
    desiredColumn: 0
  }};
  const editor = document.getElementById(id);
  editor.focus();
  viUpdateStatus(id, 'Ready');
}}

function viEditor(id) {{
  return document.getElementById(id);
}}

function viState(id) {{
  return viStates[id];
}}

function viSetMode(id, mode) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  state.mode = mode;
  state.pending = '';
  editor.classList.remove('normal', 'insert', 'command');
  editor.classList.add(mode);
  viUpdateStatus(id, mode.toUpperCase());
  editor.focus();
}}

function viUpdateStatus(id, message) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  const mode = document.getElementById(id + '-mode');
  const msg = document.getElementById(id + '-message');
  const pos = document.getElementById(id + '-pos');
  if (mode) mode.textContent = state.mode.toUpperCase() + (state.number ? ' number' : '');
  if (msg) msg.textContent = message || '';
  if (pos) {{
    const line = viLineNumber(editor.value, editor.selectionStart);
    const col = editor.selectionStart - viLineStart(editor.value, editor.selectionStart) + 1;
    pos.textContent = 'line ' + line + ', col ' + col;
  }}
}}

function viInput(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  if (editor.value !== state.lastValue) {{
    state.undo.push(editor.value);
    if (state.undo.length > 200) state.undo.shift();
    state.redo = [];
    state.lastValue = editor.value;
  }}
  viUpdateStatus(id, 'Changed');
}}

function viPushUndo(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  if (state.undo[state.undo.length - 1] !== editor.value) state.undo.push(editor.value);
  state.lastValue = editor.value;
}}

function viSetValue(id, value, cursor, message) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  state.redo = [];
  editor.value = value;
  const next = clamp(cursor, 0, editor.value.length);
  editor.selectionStart = next;
  editor.selectionEnd = next;
  state.undo.push(editor.value);
  state.lastValue = editor.value;
  viUpdateStatus(id, message);
}}

function viKeydown(event, id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  if (event.key === 'Escape') {{
    event.preventDefault();
    viSetMode(id, 'normal');
    return;
  }}
  if (state.mode === 'insert') {{
    return;
  }}
  event.preventDefault();
  if (event.ctrlKey && event.key.toLowerCase() === 'r') {{
    viRedo(id);
    return;
  }}
  const key = event.key;
  if (state.pending === 'g' && key === 'g') {{
    state.pending = '';
    viMoveTo(id, 0);
    return;
  }}
  if (state.pending === 'd' && key === 'd') {{
    state.pending = '';
    viDeleteLine(id);
    return;
  }}
  if (state.pending === 'y' && key === 'y') {{
    state.pending = '';
    viYankLine(id);
    return;
  }}
  state.pending = '';
  if (key === 'i') viSetMode(id, 'insert');
  else if (key === 'a') {{ viMoveTo(id, editor.selectionStart + 1); viSetMode(id, 'insert'); }}
  else if (key === 'A') {{ viMoveTo(id, viLineEnd(editor.value, editor.selectionStart)); viSetMode(id, 'insert'); }}
  else if (key === 'I') {{ viMoveTo(id, viLineStart(editor.value, editor.selectionStart)); viSetMode(id, 'insert'); }}
  else if (key === 'o') viOpenLine(id, false);
  else if (key === 'O') viOpenLine(id, true);
  else if (key === 'h' || key === 'ArrowLeft') viMoveTo(id, editor.selectionStart - 1);
  else if (key === 'l' || key === 'ArrowRight') viMoveTo(id, editor.selectionStart + 1);
  else if (key === 'j' || key === 'ArrowDown') viMoveVertical(id, 1);
  else if (key === 'k' || key === 'ArrowUp') viMoveVertical(id, -1);
  else if (key === '0') viMoveTo(id, viLineStart(editor.value, editor.selectionStart));
  else if (key === '$') viMoveTo(id, viLineEnd(editor.value, editor.selectionStart));
  else if (key === 'w') viMoveWord(id, 1);
  else if (key === 'b') viMoveWord(id, -1);
  else if (key === 'G') viMoveTo(id, editor.value.length);
  else if (key === 'g') state.pending = 'g';
  else if (key === 'd') state.pending = 'd';
  else if (key === 'y') state.pending = 'y';
  else if (key === 'x') viDeleteChar(id);
  else if (key === 'p') viPaste(id, false);
  else if (key === 'P') viPaste(id, true);
  else if (key === 'u') viUndo(id);
  else if (key === ':') viShowCommand(id, ':');
  else if (key === '/') viShowCommand(id, '/');
  else if (key === 'n') viSearchNext(id, 1);
  else if (key === 'N') viSearchNext(id, -1);
  else viUpdateStatus(id, 'Unknown command: ' + key);
}}

function viShowCommand(id, prefix) {{
  const state = viState(id);
  const command = document.getElementById(id + '-command');
  if (!state || !command) return;
  state.mode = 'command';
  command.classList.remove('hidden');
  command.value = prefix;
  command.focus();
  viUpdateStatus(id, 'COMMAND');
}}

function viCommandKey(event, id) {{
  if (event.key === 'Escape') {{
    event.preventDefault();
    document.getElementById(id + '-command').classList.add('hidden');
    viSetMode(id, 'normal');
    return;
  }}
  if (event.key !== 'Enter') return;
  event.preventDefault();
  const command = document.getElementById(id + '-command');
  const value = command.value;
  command.classList.add('hidden');
  viRunCommand(id, value);
}}

function viRunCommand(id, raw) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  if (raw.startsWith('/')) {{
    state.search = raw.slice(1);
    viSearchNext(id, 1);
    viSetMode(id, 'normal');
    return;
  }}
  const command = raw.startsWith(':') ? raw.slice(1).trim() : raw.trim();
  if (command === 'w') viSave(id);
  else if (command === 'q') closeTab(safeId('vi:' + state.path), new Event('click'));
  else if (command === 'wq' || command === 'x') {{ viSave(id); closeTab(safeId('vi:' + state.path), new Event('click')); }}
  else if (command === 'set number') {{ state.number = true; viUpdateStatus(id, 'Line numbers on'); }}
  else if (command === 'set nonumber') {{ state.number = false; viUpdateStatus(id, 'Line numbers off'); }}
  else if (command === '$') viMoveTo(id, editor.value.length);
  else if (/^\\d+$/.test(command)) viMoveToLine(id, Number(command));
  else if (command.startsWith('%s/')) viSubstitute(id, command);
  else viUpdateStatus(id, 'Unsupported command: :' + command);
  viSetMode(id, 'normal');
}}

function viSave(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  hostRequest({{ action: 'saveFile', path: state.path, content: editor.value }});
  viUpdateStatus(id, 'Written');
}}

function viUndo(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor || state.undo.length <= 1) return;
  state.redo.push(state.undo.pop());
  editor.value = state.undo[state.undo.length - 1];
  state.lastValue = editor.value;
  viUpdateStatus(id, 'Undo');
}}

function viRedo(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor || state.redo.length === 0) return;
  const value = state.redo.pop();
  state.undo.push(value);
  editor.value = value;
  state.lastValue = value;
  viUpdateStatus(id, 'Redo');
}}

function viMoveTo(id, pos) {{
  const editor = viEditor(id);
  if (!editor) return;
  const next = clamp(pos, 0, editor.value.length);
  editor.selectionStart = next;
  editor.selectionEnd = next;
  const state = viState(id);
  if (state) state.desiredColumn = next - viLineStart(editor.value, next);
  viUpdateStatus(id, 'Moved');
}}

function viLineStart(text, pos) {{
  const found = text.lastIndexOf('\\n', Math.max(0, pos - 1));
  return found < 0 ? 0 : found + 1;
}}

function viLineEnd(text, pos) {{
  const found = text.indexOf('\\n', pos);
  return found < 0 ? text.length : found;
}}

function viLineNumber(text, pos) {{
  return text.slice(0, pos).split('\\n').length;
}}

function viMoveVertical(id, delta) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  const text = editor.value;
  const start = viLineStart(text, editor.selectionStart);
  const col = state.desiredColumn || editor.selectionStart - start;
  const targetStart = delta > 0 ? viLineEnd(text, editor.selectionStart) + 1 : viLineStart(text, Math.max(0, start - 1));
  if (targetStart < 0 || targetStart > text.length) return;
  viMoveTo(id, Math.min(targetStart + col, viLineEnd(text, targetStart)));
}}

function viMoveWord(id, direction) {{
  const editor = viEditor(id);
  if (!editor) return;
  const text = editor.value;
  let pos = editor.selectionStart;
  if (direction > 0) {{
    pos++;
    while (pos < text.length && /\\w/.test(text[pos])) pos++;
    while (pos < text.length && !/\\w/.test(text[pos])) pos++;
  }} else {{
    pos--;
    while (pos > 0 && !/\\w/.test(text[pos])) pos--;
    while (pos > 0 && /\\w/.test(text[pos - 1])) pos--;
  }}
  viMoveTo(id, pos);
}}

function viDeleteChar(id) {{
  const editor = viEditor(id);
  if (!editor) return;
  const pos = editor.selectionStart;
  viPushUndo(id);
  viSetValue(id, editor.value.slice(0, pos) + editor.value.slice(pos + 1), pos, 'Deleted char');
}}

function viLineRange(text, pos) {{
  const start = viLineStart(text, pos);
  let end = text.indexOf('\\n', pos);
  if (end < 0) end = text.length; else end += 1;
  return {{ start, end }};
}}

function viDeleteLine(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  const range = viLineRange(editor.value, editor.selectionStart);
  state.yank = editor.value.slice(range.start, range.end);
  viPushUndo(id);
  viSetValue(id, editor.value.slice(0, range.start) + editor.value.slice(range.end), range.start, 'Deleted line');
}}

function viYankLine(id) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor) return;
  const range = viLineRange(editor.value, editor.selectionStart);
  state.yank = editor.value.slice(range.start, range.end);
  viUpdateStatus(id, 'Yanked line');
}}

function viPaste(id, before) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor || !state.yank) return;
  const pos = before ? editor.selectionStart : editor.selectionStart + 1;
  viPushUndo(id);
  viSetValue(id, editor.value.slice(0, pos) + state.yank + editor.value.slice(pos), pos + state.yank.length, 'Pasted');
}}

function viOpenLine(id, above) {{
  const editor = viEditor(id);
  if (!editor) return;
  const pos = above ? viLineStart(editor.value, editor.selectionStart) : viLineEnd(editor.value, editor.selectionStart);
  const insert = above ? '\\n' : '\\n';
  viPushUndo(id);
  viSetValue(id, editor.value.slice(0, pos) + insert + editor.value.slice(pos), pos + 1, 'Opened line');
  viSetMode(id, 'insert');
}}

function viMoveToLine(id, lineNumber) {{
  const editor = viEditor(id);
  if (!editor) return;
  const lines = editor.value.split('\\n');
  let pos = 0;
  for (let i = 0; i < Math.max(0, lineNumber - 1) && i < lines.length; i++) pos += lines[i].length + 1;
  viMoveTo(id, pos);
}}

function viSearchNext(id, direction) {{
  const state = viState(id);
  const editor = viEditor(id);
  if (!state || !editor || !state.search) return;
  const text = editor.value;
  const start = direction > 0 ? editor.selectionStart + 1 : editor.selectionStart - 1;
  let found = direction > 0 ? text.indexOf(state.search, start) : text.lastIndexOf(state.search, start);
  if (found < 0 && direction > 0) found = text.indexOf(state.search, 0);
  if (found < 0 && direction < 0) found = text.lastIndexOf(state.search);
  if (found >= 0) {{
    editor.selectionStart = found;
    editor.selectionEnd = found + state.search.length;
    viUpdateStatus(id, 'Found ' + state.search);
  }} else {{
    viUpdateStatus(id, 'Pattern not found: ' + state.search);
  }}
}}

function viSubstitute(id, command) {{
  const editor = viEditor(id);
  if (!editor) return;
  const parts = command.split('/');
  if (parts.length < 4) {{
    viUpdateStatus(id, 'Substitute format: :%s/from/to/g');
    return;
  }}
  const from = parts[1];
  const to = parts[2];
  const global = parts[3].includes('g');
  viPushUndo(id);
  const pattern = new RegExp(from.replace(/[.*+?^${{}}()|[\]\\]/g, '\\$&'), global ? 'g' : '');
  viSetValue(id, editor.value.replace(pattern, to), editor.selectionStart, 'Substituted');
}}

function securityScanFile(path) {{
  const editor = document.getElementById(safeId('editor-' + path));
  hostRequest({{ action: 'scanFile', path, content: editor ? editor.value : null }});
}}

function showFileMenu(event, path) {{
  event.preventDefault();
  activeContextPath = path;
  const menu = document.getElementById('contextMenu');
  menu.style.left = event.clientX + 'px';
  menu.style.top = event.clientY + 'px';
  menu.style.display = 'block';
}}

function hideContextMenu() {{
  document.getElementById('contextMenu').style.display = 'none';
}}

function openContextFile() {{
  if (activeContextPath) openFile(activeContextPath);
  hideContextMenu();
}}

function scanContextFile() {{
  if (activeContextPath) securityScanFile(activeContextPath);
  hideContextMenu();
}}

function copyContextPath() {{
  if (activeContextPath && navigator.clipboard) navigator.clipboard.writeText(activeContextPath);
  hideContextMenu();
}}

function openContextVi() {{
  if (activeContextPath) openViForPath(activeContextPath);
  hideContextMenu();
}}

function openContextFormatter() {{
  if (activeContextPath) openFormatterForPath(activeContextPath);
  hideContextMenu();
}}

function openFeedback() {{
  openModule('feedback.open', 'Feedback And Bugs');
}}

function filterCommands() {{
  const filter = document.getElementById('commandFilter').value.toLowerCase();
  document.querySelectorAll('[data-command-search]').forEach(button => {{
    button.style.display = button.dataset.commandSearch.includes(filter) ? 'block' : 'none';
  }});
}}

function refreshExplorer() {{
  hostRequest({{ action: 'refreshExplorer' }});
}}

function clamp(value, min, max) {{
  return Math.min(max, Math.max(min, value));
}}

function initLayoutSplitters() {{
  const root = document.getElementById('workbench');
  const workspace = document.getElementById('workspaceArea');
  const leftDock = document.getElementById('leftDock');

  document.querySelector('[data-splitter="left-width"]').addEventListener('mousedown', event => {{
    event.preventDefault();
    const startX = event.clientX;
    const startWidth = leftDock.getBoundingClientRect().width;
    const onMove = moveEvent => {{
      const maxLeft = Math.max(220, Math.floor(root.clientWidth * 0.42));
      const next = clamp(startWidth + moveEvent.clientX - startX, 220, maxLeft);
      root.style.setProperty('--left-width', next + 'px');
    }};
    const onUp = () => {{
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    }};
    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }});

  document.querySelector('[data-splitter="log-height"]').addEventListener('mousedown', event => {{
    event.preventDefault();
    const startY = event.clientY;
    const startHeight = workspace.querySelector('.bottom-log').getBoundingClientRect().height;
    const onMove = moveEvent => {{
      const delta = startY - moveEvent.clientY;
      const next = clamp(startHeight + delta, 120, Math.floor(workspace.clientHeight * 0.55));
      workspace.style.setProperty('--log-height', next + 'px');
    }};
    const onUp = () => {{
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    }};
    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }});

  document.querySelectorAll('[data-splitter="left-stack"]').forEach(splitter => {{
    splitter.addEventListener('mousedown', event => {{
      event.preventDefault();
      const before = document.getElementById(splitter.dataset.before);
      const after = document.getElementById(splitter.dataset.after);
      const startY = event.clientY;
      const beforeHeight = before.getBoundingClientRect().height;
      const afterHeight = after.getBoundingClientRect().height;
      const total = beforeHeight + afterHeight;
      const onMove = moveEvent => {{
        const nextBefore = clamp(beforeHeight + moveEvent.clientY - startY, 110, total - 110);
        before.style.flexBasis = nextBefore + 'px';
        after.style.flexBasis = (total - nextBefore) + 'px';
      }};
      const onUp = () => {{
        document.removeEventListener('mousemove', onMove);
        document.removeEventListener('mouseup', onUp);
      }};
      document.addEventListener('mousemove', onMove);
      document.addEventListener('mouseup', onUp);
    }});
  }});
}}

function renderResult(path, title, rows, ok) {{
  const resultId = safeId('result-' + path);
  const result = document.getElementById(resultId);
  const body = '<h3>' + escapeHtml(title) + '</h3>' +
    '<ul class="result-list">' + rows.map(row => '<li>' + escapeHtml(row) + '</li>').join('') + '</ul>';
  if (result) {{
    result.style.display = 'block';
    result.style.borderColor = ok ? '#1f8a70' : '#a15c07';
    result.innerHTML = body;
  }} else {{
    openTab('result:' + path + ':' + title, title + ': ' + path, '<div class="surface">' + body + '</div>');
  }}
}}

function renderModuleResult(target, title, rows, ok) {{
  const resultId = safeId('module-result-' + target);
  const result = document.getElementById(resultId);
  const body = '<h3>' + escapeHtml(title) + '</h3>' +
    '<ul class="result-list">' + rows.map(row => '<li>' + escapeHtml(row) + '</li>').join('') + '</ul>';
  if (result) {{
    result.style.display = 'block';
    result.style.borderColor = ok ? '#1f8a70' : '#a15c07';
    result.innerHTML = body;
  }} else {{
    openTab('backend:' + target + ':' + title, title, '<div class="surface">' + body + '</div>');
  }}
  setStatus(title + ' completed');
}}

function appendTaskLog(message) {{
  const log = document.getElementById('taskLog');
  if (log) {{
    log.value = (log.value ? log.value + '\\n' : '') + message;
    log.scrollTop = log.scrollHeight;
  }}
  const status = document.getElementById('status');
  if (status) status.textContent = message;
}}

function renderProjectDetection(response) {{
  const summary = document.getElementById('taskProjectSummary');
  if (!summary) return;
  if (!response.ok) {{
    summary.innerHTML = '<h3>Project Detection</h3><p>' + escapeHtml(response.error || 'Detection failed') + '</p>';
    return;
  }}
  const tasks = response.tasks || [];
  summary.innerHTML = '<h3>Project Detection</h3>' +
    '<p><strong>' + escapeHtml(response.projectKind) + '</strong> at ' + escapeHtml(response.projectPath) + '</p>' +
    '<ul class="result-list">' + (response.messages || []).map(row => '<li>' + escapeHtml(row) + '</li>').join('') + '</ul>' +
    '<table class="table"><tr><th>Action</th><th>Command</th><th>Ready</th></tr>' +
    tasks.map(task => '<tr><td>' + escapeHtml(task.label) + '</td><td><code>' + escapeHtml(task.commandLine) + '</code></td><td>' + escapeHtml(String(task.ready)) + '</td></tr>').join('') +
    '</table>';
  appendTaskLog('Detected ' + response.projectKind + ' with ' + tasks.length + ' task profiles.');
}}

function renderTaskResult(response) {{
  const summary = document.getElementById('taskResultSummary');
  const log = document.getElementById('taskLog');
  if (!response.ok) {{
    const message = response.error || 'Task failed before execution';
    if (summary) summary.innerHTML = '<h3>Task Result</h3><p>' + escapeHtml(message) + '</p>';
    appendTaskLog(message);
    return;
  }}
  const diagnostics = response.diagnostics || [];
  const suggestions = response.suggestions || [];
  if (summary) {{
    summary.innerHTML = '<h3>Task Result</h3>' +
      '<div class="metric-grid">' +
      '<div class="metric"><label>Status</label><strong>' + (response.success ? 'Clean' : 'Needs Work') + '</strong></div>' +
      '<div class="metric"><label>Exit code</label><strong>' + escapeHtml(String(response.exitCode)) + '</strong></div>' +
      '<div class="metric"><label>Duration</label><strong>' + escapeHtml(String(response.durationMs)) + ' ms</strong></div>' +
      '<div class="metric"><label>Report</label><strong>' + escapeHtml(response.reportPath || '') + '</strong></div>' +
      '</div>' +
      '<p><code>' + escapeHtml(response.commandLine) + '</code></p>' +
      '<h4>Diagnostics</h4><ul class="result-list">' + (diagnostics.length ? diagnostics : ['No compiler errors or warnings captured.']).map(row => '<li>' + escapeHtml(row) + '</li>').join('') + '</ul>' +
      '<h4>Suggestions</h4><ul class="result-list">' + suggestions.map(row => '<li>' + escapeHtml(row) + '</li>').join('') + '</ul>';
  }}
  if (log) {{
    log.value = [
      '$ ' + response.commandLine,
      '',
      '[stdout]',
      response.stdout || '',
      '',
      '[stderr]',
      response.stderr || ''
    ].join('\\n');
    log.scrollTop = log.scrollHeight;
  }}
  appendTaskLog('Report saved: ' + response.reportPath);
  if (response.fileTreeHtml) document.getElementById('fileTree').innerHTML = response.fileTreeHtml;
}}

function renderCommandOutput(target, title, response) {{
  const rows = [
    'Command: ' + (response.commandLine || ''),
    'Exit code: ' + String(response.exitCode),
    'Success: ' + String(response.success),
    'stdout: ' + ((response.stdout || '').slice(0, 2000) || '[empty]'),
    'stderr: ' + ((response.stderr || '').slice(0, 2000) || '[empty]')
  ];
  renderModuleResult(target, title, rows, Boolean(response.success));
}}

function renderTerminalSession(response) {{
  const session = response.session || {{}};
  const idNode = document.getElementById('terminalSessionId');
  const outputNode = document.getElementById('terminalOutput');
  const labelNode = document.getElementById('terminalSessionLabel');
  if (idNode) idNode.value = session.id || '';
  if (labelNode) labelNode.textContent = (session.label || 'Terminal') + ' | running=' + String(session.running) + ' | pty=' + String(Boolean(session.ptyBacked));
  if (outputNode) {{
    outputNode.textContent = session.output || '';
    outputNode.scrollTop = outputNode.scrollHeight;
  }} else {{
    renderModuleResult('tools.sshTerminus.open', 'Terminal Session', [
      'Session: ' + (session.id || ''),
      'Command: ' + (session.commandLine || ''),
      'PTY backed: ' + String(Boolean(session.ptyBacked)),
      'Running: ' + String(session.running),
      (session.output || '').slice(0, 4000)
    ], Boolean(response.ok));
  }}
  setStatus('Terminal session ' + (session.id || '') + ' running=' + String(session.running));
}}

function renderRecordList(target, title, records) {{
  const rows = (records || []).map(record => JSON.stringify(record));
  renderModuleResult(target, title, rows.length ? rows : ['No records found.'], true);
}}

function renderDependencyVerification(response) {{
  const records = response.records || [];
  const target = document.getElementById('dependencyResults');
  const select = document.getElementById('dependencyInstallSelect');
  if (!target || !select) {{
    renderModuleResult('setup.verifyInstallDependencies.open', 'Dependency Verification', records.map(record => JSON.stringify(record)), true);
    return;
  }}
  const missing = records.filter(record => !record.installed);
  select.innerHTML = missing.map(record =>
    '<option value="' + escapeHtml(record.id) + '" data-command="' + escapeHtml(record.installCommand || '') + '" data-source="' + escapeHtml(record.sourceUrl || '') + '">' +
    escapeHtml(record.displayName + ' (' + record.id + ')') +
    '</option>'
  ).join('');
  target.innerHTML = '<h3>Dependency Verification</h3>' +
    '<div class="metric-grid">' +
    '<div class="metric"><label>Total</label><strong>' + records.length + '</strong></div>' +
    '<div class="metric"><label>Installed</label><strong>' + records.filter(record => record.installed).length + '</strong></div>' +
    '<div class="metric"><label>Missing</label><strong>' + missing.length + '</strong></div>' +
    '<div class="metric"><label>Installable</label><strong>' + missing.filter(record => record.installable).length + '</strong></div>' +
    '</div>' +
    '<table class="table"><tr><th>Dependency</th><th>Status</th><th>Manager</th><th>Package</th><th>Command or source</th></tr>' +
    records.map(record =>
      '<tr><td>' + escapeHtml(record.displayName) + '<br><span class="hint">' + escapeHtml(record.id + ' | ' + record.domain) + '</span></td>' +
      '<td>' + (record.installed ? 'Active' : 'Missing') + (record.requiredForCore ? '<br><span class="hint">core</span>' : '') + '</td>' +
      '<td>' + escapeHtml(record.packageManager || 'manual') + '</td>' +
      '<td>' + escapeHtml(record.package || 'manual') + '</td>' +
      '<td><code>' + escapeHtml(record.installCommand || '') + '</code><br><span class="hint">' + escapeHtml(record.sourceUrl || '') + '</span></td></tr>'
    ).join('') +
    '</table>';
  dependencySelectionChanged();
  setStatus('Verified dependencies: ' + missing.length + ' missing');
}}

function mEditorHostResponse(response) {{
  if (!response || !response.action) return;
  if (response.ok === false) {{
    const detail = response.error || (response.messages || []).join('\\n') || 'Operation failed';
    setStatus(detail);
    showErrorDialog(response.title || response.action || 'mEditor Error', detail);
  }}
  if (response.action === 'licenseStatusResult') {{
    if (response.ok && response.accepted) {{
      licenseState = {{
        accepted: true,
        registered: Boolean(response.registered),
        displayName: response.displayName || 'UnRegistered but fully functional copy with no obligation'
      }};
      document.getElementById('eulaGate').classList.add('hidden');
      refreshLicenseDisplay();
    }}
  }}
  if (response.action === 'eulaAcceptanceResult') {{
    if (response.ok && response.accepted) {{
      licenseState = {{
        accepted: true,
        registered: Boolean(response.registered),
        displayName: response.displayName || 'UnRegistered but fully functional copy with no obligation'
      }};
      document.getElementById('eulaGate').classList.add('hidden');
      refreshLicenseDisplay();
      setStatus('EULA accepted: ' + licenseState.displayName);
    }} else {{
      const status = document.getElementById('eulaStatus');
      if (status) status.textContent = response.error || 'EULA acceptance failed';
    }}
  }}
  if (response.action === 'openFileResult') {{
    if (response.ok) openEditorTab(response.path, response.content);
    else setStatus(response.error || 'Open failed');
  }}
  if (response.action === 'openFileForViResult') {{
    if (response.ok) openViEditorTab(response.path, response.content);
    else setStatus(response.error || 'Open Vi failed');
  }}
  if (response.action === 'openFileForFormatterResult') {{
    if (response.ok) openFormatterWorkbench(formatterKindForPath(response.path, 'json'), 'Formatter: ' + response.path, response.path, response.content);
    else setStatus(response.error || 'Open Formatter failed');
  }}
  if (response.action === 'saveFileResult') {{
    setStatus(response.ok ? ('Saved ' + response.path) : (response.error || 'Save failed'));
    if (response.ok) renderResult(response.path, 'Save Result', ['Saved to disk', response.bytes + ' bytes written'], true);
  }}
  if (response.action === 'scanFileResult') {{
    if (response.ok) renderResult(response.path, 'Security Scan Result', response.findings, response.findings.length === 1 && response.findings[0] === 'No findings');
    else setStatus(response.error || 'Scan failed');
  }}
  if (response.action === 'validateFileResult') {{
    if (response.ok) renderResult(response.path, (response.realtime ? 'Realtime ' : '') + response.kind.toUpperCase() + ' Validation Result', response.messages, response.valid);
    else setStatus(response.error || 'Validation failed');
  }}
  if (response.action === 'reformatFileResult') {{
    if (response.ok) {{
      const editor = document.getElementById(safeId('editor-' + response.path));
      if (editor) editor.value = response.content;
      renderResult(response.path, response.kind.toUpperCase() + ' Reformat Result', response.messages || [], true);
      setStatus('Reformatted ' + response.path + ' in memory');
    }} else {{
      renderResult(response.path || 'active file', 'Reformat Result', [response.error || 'Reformat failed'], false);
    }}
  }}
  if (response.action === 'formatTextResult') {{
    renderFormatterResult(response);
  }}
  if (response.action === 'detectProjectResult') {{
    renderProjectDetection(response);
  }}
  if (response.action === 'dependencyVerifyResult') {{
    renderDependencyVerification(response);
  }}
  if (response.action === 'dependencyInstallResult') {{
    renderCommandOutput('setup.verifyInstallDependencies.open', 'Dependency Install Result', response);
    verifyDependencies();
  }}
  if (response.action === 'runTaskResult') {{
    renderTaskResult(response);
  }}
  if (response.action === 'vcsCommandResult') {{
    renderCommandOutput(response.provider === 'svn' ? 'team.svn.open' : 'team.git.open', 'Version Control Result', response);
  }}
  if (response.action === 'sqliteSqlResult') {{
    renderCommandOutput('tools.dbaWorkshop.open', 'SQLite SQL Result', response);
  }}
  if (response.action === 'sshTestResult') {{
    renderCommandOutput('tools.sshTerminus.open', 'SSH Test Result', response);
  }}
  if (response.action === 'sftpListResult') {{
    renderCommandOutput('tools.sftpScpTransfer.open', 'SFTP/SCP Result', response);
  }}
  if (response.action === 'sshCommandResult') {{
    renderCommandOutput('tools.sshTerminus.open', 'SSH Terminal Tab Result', response);
  }}
  if (response.action === 'terminalSessionResult') {{
    renderTerminalSession(response);
  }}
  if (response.action === 'fileTransferResult') {{
    renderCommandOutput('tools.sftpScpTransfer.open', 'SFTP/SCP Transfer Result', response);
  }}
  if (response.action === 'jdbcSqlResult') {{
    renderCommandOutput('tools.dbaWorkshop.open', 'JDBC SQL Result', response);
  }}
  if (response.action === 'jdbcMetadataResult') {{
    renderCommandOutput('tools.dbaWorkshop.open', 'JDBC Object Browser / DBA Dashboard', response);
  }}
  if (response.action === 'aiRuntimeResult') {{
    renderCommandOutput('tools.aimlAssistant.open', 'Local AI Runtime Result', response);
  }}
  if (response.action === 'localBugsResult') {{
    renderRecordList('report.center.open', 'Local Bugs', response.records);
  }}
  if (response.action === 'reportListResult') {{
    renderRecordList('report.center.open', 'Local Reports', response.records);
  }}
  if (response.action === 'refreshExplorerResult') {{
    if (response.ok) {{
      document.getElementById('fileTree').innerHTML = response.html;
      setStatus('File Explorer refreshed');
    }} else {{
      setStatus(response.error || 'Refresh failed');
    }}
  }}
  if (response.action === 'genericActionResult') {{
    if (response.fileTreeHtml) document.getElementById('fileTree').innerHTML = response.fileTreeHtml;
    renderModuleResult(response.target || 'dashboard.open', response.title || 'Backend Result', response.messages || [], response.ok);
  }}
}}

document.addEventListener('click', event => {{
  if (!event.target.closest('#contextMenu')) hideContextMenu();
  if (!event.target.closest('.menu-group')) closeMenus();
}});

function toggleMenu(menuId, event) {{
  event.stopPropagation();
  const target = document.getElementById(menuId);
  const shouldOpen = target && !target.classList.contains('open');
  closeMenus();
  if (shouldOpen) target.classList.add('open');
}}

function closeMenus() {{
  document.querySelectorAll('.menu-group.open').forEach(menu => menu.classList.remove('open'));
}}

initLayoutSplitters();
openTab('dashboard.open', 'Welcome Page', defaultDashboard);
hostRequest({{ action: 'licenseStatus' }});
</script>
</body>
</html>"#,
        version = meditor_core::CURRENT_BASELINE_VERSION,
        workspace = html_escape(&display_path(&workspace_root)),
        platform = platform,
        module_count = workbench.modules.len(),
        apex_base = html_escape(rest_config.base_url.as_deref().unwrap_or("not configured")),
        dashboard = js_string(&dashboard_html(
            &workbench,
            &platform,
            rest_config.base_url.as_deref().unwrap_or("not configured"),
            &module_cards,
        )),
        toolbar_html = toolbar_html,
        connections_html = connections_html,
        reports_html = reports_html,
    )
}

fn build_toolbar_html() -> String {
    [
        r#"<button class="tool-button" title="New Project" aria-label="New Project" onclick="openModule('project.create','New Project')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><path d="M14 2v6h6"></path><path d="M12 18v-6"></path><path d="M9 15h6"></path></svg></button>"#,
        r#"<button class="tool-button" title="Open Project" aria-label="Open Project" onclick="openModule('file.importProject.open','Import Project')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M3 7h5l2 3h11"></path><path d="M3 7v12a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-9H10"></path></svg></button>"#,
        r#"<button class="tool-button" title="Save Active File" aria-label="Save Active File" onclick="setStatus('Use the editor tab Save button for the active file')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path><path d="M17 21v-8H7v8"></path><path d="M7 3v5h8"></path></svg></button>"#,
        r#"<span class="tool-separator"></span>"#,
        r#"<button class="tool-button" title="Undo" aria-label="Undo"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M9 14 4 9l5-5"></path><path d="M4 9h10a6 6 0 0 1 0 12h-2"></path></svg></button>"#,
        r#"<button class="tool-button" title="Redo" aria-label="Redo"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="m15 14 5-5-5-5"></path><path d="M20 9H10a6 6 0 0 0 0 12h2"></path></svg></button>"#,
        r#"<span class="tool-separator"></span>"#,
        r#"<button class="tool-button" title="Run" aria-label="Run" onclick="openModule('run.tasks.open','Tasks')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><polygon points="8 5 19 12 8 19 8 5" fill="currentColor" stroke="none"></polygon></svg></button>"#,
        r#"<button class="tool-button" title="DBA Workshop" aria-label="DBA Workshop" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><ellipse cx="12" cy="5" rx="7" ry="3"></ellipse><path d="M5 5v6c0 1.7 3.1 3 7 3s7-1.3 7-3V5"></path><path d="M5 11v6c0 1.7 3.1 3 7 3s7-1.3 7-3v-6"></path></svg></button>"#,
        r#"<button class="tool-button" title="SSH Terminus" aria-label="SSH Terminus" onclick="openModule('tools.sshTerminus.open','SSH Terminus')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><rect x="3" y="5" width="18" height="14" rx="2"></rect><path d="m7 9 3 3-3 3"></path><path d="M12 15h5"></path></svg></button>"#,
        r#"<button class="tool-button" title="Web Browser" aria-label="Web Browser" onclick="openModule('tools.webBrowser.open','Web Browser')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><circle cx="12" cy="12" r="9"></circle><path d="M3 12h18"></path><path d="M12 3a14 14 0 0 1 0 18"></path><path d="M12 3a14 14 0 0 0 0 18"></path></svg></button>"#,
        r#"<span class="tool-separator"></span>"#,
        r#"<button class="tool-button" title="Find" aria-label="Find" onclick="setStatus('Find command selected')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><circle cx="11" cy="11" r="7"></circle><path d="m16 16 5 5"></path></svg></button>"#,
        r#"<button class="tool-button" title="Security Scan" aria-label="Security Scan" onclick="openModule('tools.codeSecurityAnalyzer.open','Code Security Analyzer')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path><path d="m9 12 2 2 4-5"></path></svg></button>"#,
    ]
    .join("")
}

fn build_connections_tree_html() -> String {
    [
        r#"<div class="tree-row"><span class="tree-kind">[+]</span>Database Connections</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[db]</span>Local SQLite</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[db]</span>JDBC Connections</div>"#,
        r#"<div class="tree-row" style="padding-left:42px"><span class="tree-kind">[drv]</span>Add JDBC Driver</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[db]</span>Cloud Connections</div>"#,
        r#"<div class="tree-row"><span class="tree-kind">[+]</span>SSH Connections</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[grp]</span>Development</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[grp]</span>Production</div>"#,
        r#"<div class="tree-row"><span class="tree-kind">[+]</span>SFTP/SCP Sites</div>"#,
    ]
    .join("")
}

fn build_reports_tree_html() -> String {
    [
        r#"<div class="tree-row"><span class="tree-kind">[+]</span>All Reports</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>About Your Database</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>All Objects</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>Database Administration</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>Data Dictionary</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>Data Modeler Reports</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>PL/SQL</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>Security</div>"#,
        r#"<div class="tree-row" style="padding-left:22px"><span class="tree-kind">[dir]</span>XML</div>"#,
    ]
    .join("")
}

fn build_menu_html() -> String {
    let mut grouped: BTreeMap<&str, Vec<meditor_shell::MenuItem>> = BTreeMap::new();
    for item in meditor_shell::default_menu_items() {
        grouped.entry(item.menu).or_default().push(item);
    }
    let order = [
        "File", "Source", "Team", "Run", "Setup", "Tools", "Window", "Settings", "Help", "Report",
    ];

    order
        .iter()
        .filter_map(|menu| grouped.get(menu).map(|items| (*menu, items)))
        .enumerate()
        .map(|(index, (menu, items))| {
            let menu_id = format!("menu-{index}");
            let buttons = items
                .iter()
                .map(|item| {
                    format!(
                        r#"<button onclick="openModule('{}','{}')">{}</button>"#,
                        html_escape(item.command),
                        html_escape(item.label),
                        html_escape(item.label)
                    )
                })
                .collect::<String>();
            format!(
                r#"<div id="{}" class="menu-group"><button class="menu-trigger" onclick="toggleMenu('{}',event)">{}</button><div class="menu-items">{}</div></div>"#,
                menu_id,
                menu_id,
                html_escape(menu),
                buttons
            )
        })
        .collect::<String>()
}

fn build_file_tree_html(workspace_root: &Path) -> String {
    workspace_entries(workspace_root)
        .into_iter()
        .map(|entry| {
            let size = if entry.is_dir {
                String::new()
            } else {
                format!(" ({})", format_size(entry.size_bytes))
            };
            if entry.is_dir {
                format!(
                    r#"<div class="tree-row" style="padding-left:{}px"><span class="tree-kind">[dir]</span>{}</div>"#,
                    8 + entry.depth * 14,
                    html_escape(&entry.display_path)
                )
            } else {
                format!(
                    r#"<div class="tree-row" style="padding-left:{}px" data-path="{}" ondblclick="openFile(this.dataset.path)" oncontextmenu="showFileMenu(event,this.dataset.path)"><span class="tree-kind">[file]</span>{}{}</div>"#,
                    8 + entry.depth * 14,
                    html_escape(&entry.display_path),
                    html_escape(&entry.display_path),
                    html_escape(&size)
                )
            }
        })
        .collect::<String>()
}

fn build_module_script() -> String {
    let mut surfaces = BTreeMap::new();
    surfaces.insert("project.create", new_project_html());
    surfaces.insert(
        "file.localHistoryRecovery.open",
        action_module_html(
            "Local History And Recovery",
            "file.localHistoryRecovery.open",
            "Creates a timestamped workspace file manifest under .meditor/local-history and exposes recovery evidence.",
        ),
    );
    surfaces.insert(
        "file.workspaceBackupRestore.open",
        action_module_html(
            "Workspace Backup And Restore",
            "file.workspaceBackupRestore.open",
            "Creates a guarded backup manifest under .meditor/backups for review before restore.",
        ),
    );
    surfaces.insert("source.refactor.open", refactor_html());
    surfaces.insert("team.git.open", vcs_html("git"));
    surfaces.insert("team.svn.open", vcs_html("svn"));
    surfaces.insert("run.tasks.open", run_tasks_html());
    surfaces.insert("tools.dbaWorkshop.open", dba_workshop_html());
    surfaces.insert("tools.sshTerminus.open", ssh_terminus_html());
    surfaces.insert("tools.sftpScpTransfer.open", sftp_scp_html());
    surfaces.insert("tools.webBrowser.open", web_browser_html());
    surfaces.insert(
        "tools.profiler.open",
        action_module_html(
            "Profiler",
            "tools.profiler.open",
            "Generates a local profiler readiness report with workspace and runtime facts.",
        ),
    );
    surfaces.insert(
        "tools.templatesSnippets.open",
        action_module_html(
            "Templates And Snippets",
            "tools.templatesSnippets.open",
            "Writes a starter snippet catalog under .meditor/templates.",
        ),
    );
    surfaces.insert(
        "tools.cicdGenerator.open",
        action_module_html(
            "CI/CD Generator",
            "tools.cicdGenerator.open",
            "Generates a starter CI workflow template under .meditor/cicd.",
        ),
    );
    surfaces.insert(
        "tools.apiWorkbench.open",
        action_module_html(
            "API Workbench",
            "tools.apiWorkbench.open",
            "Creates a local API request catalog under .meditor/api-workbench.",
        ),
    );
    surfaces.insert(
        "tools.databaseMigration.open",
        action_module_html(
            "Database Migration",
            "tools.databaseMigration.open",
            "Creates a guarded migration plan requiring dry-run, backup, and rollback review.",
        ),
    );
    surfaces.insert("setup.languageSupport.open", language_support_html());
    surfaces.insert(
        "setup.verifyInstallDependencies.open",
        verify_install_dependencies_html(),
    );
    surfaces.insert("file.importProject.open", project_importers_html());
    surfaces.insert("tools.codeSecurityAnalyzer.open", security_analyzer_html());
    surfaces.insert(
        "tools.cvssRepository.open",
        action_module_html(
            "CVSS Repository",
            "tools.cvssRepository.open",
            "Creates or verifies the local SQLite security repository setup artifacts.",
        ),
    );
    surfaces.insert("tools.aimlAssistant.open", ai_assistant_html());
    surfaces.insert(
        "tools.aiKnowledgeBase.open",
        action_module_html(
            "AI Knowledge Base",
            "tools.aiKnowledgeBase.open",
            "Creates the local knowledge-base folder and reports stored knowledge sources.",
        ),
    );
    surfaces.insert(
        "tools.aiTrainingStudio.open",
        action_module_html(
            "AI Training Studio",
            "tools.aiTrainingStudio.open",
            "Writes a local training plan for reindexing and fitting the retrieval model.",
        ),
    );
    surfaces.insert(
        "tools.specToSystem.open",
        action_module_html(
            "Spec-to-System Workbench",
            "tools.specToSystem.open",
            "Creates a specification template for goals, requirements, architecture, and tests.",
        ),
    );
    surfaces.insert(
        "tools.projectDocumentation.open",
        action_module_html(
            "Project Documentation",
            "tools.projectDocumentation.open",
            "Creates a project documentation index for PRD, TDD, deployment, ERD, and flowcharts.",
        ),
    );
    surfaces.insert("tools.projectPlanner.open", project_planner_html());
    surfaces.insert("tools.umlModeling.open", uml_modeling_html());
    surfaces.insert(
        "window.fileExplorer.focus",
        action_module_html(
            "File Explorer",
            "window.fileExplorer.focus",
            "Refreshes the left File Explorer and reports the loaded workspace entries.",
        ),
    );
    surfaces.insert(
        "window.perspectives.open",
        action_module_html(
            "Perspectives",
            "window.perspectives.open",
            "Writes a perspective profile for IDE, database, terminal, planning, and security views.",
        ),
    );
    surfaces.insert(
        "window.workspaceDashboard.open",
        action_module_html(
            "Workspace Dashboard",
            "window.workspaceDashboard.open",
            "Generates a workspace dashboard manifest from the current files and local state.",
        ),
    );
    surfaces.insert(
        "settings.keymapsImports.open",
        action_module_html(
            "Keymaps And Imports",
            "settings.keymapsImports.open",
            "Writes the built-in keymap profile catalog.",
        ),
    );
    surfaces.insert(
        "settings.workspaceTrust.open",
        action_module_html(
            "Workspace Trust",
            "settings.workspaceTrust.open",
            "Writes the restricted-by-default workspace trust policy.",
        ),
    );
    surfaces.insert("tools.extensionSdk.open", extension_sdk_html());
    surfaces.insert("tools.workspaceIndexer.open", workspace_indexer_html());
    surfaces.insert("settings.secretsCredentials.open", credentials_html());
    surfaces.insert(
        "settings.pluginPermissions.open",
        action_module_html(
            "Plugin Permissions",
            "settings.pluginPermissions.open",
            "Writes a deny-by-default plugin permission policy.",
        ),
    );
    surfaces.insert(
        "settings.accessibilityKeyboard.open",
        action_module_html(
            "Accessibility And Keyboard",
            "settings.accessibilityKeyboard.open",
            "Writes keyboard and accessibility defaults for the workspace.",
        ),
    );
    surfaces.insert("help.privacyCenter.open", privacy_center_html());
    surfaces.insert("help.diagnosticsBundle.open", diagnostics_html());
    surfaces.insert("help.registration.open", registration_html());
    surfaces.insert("help.updateChannelManager.open", update_channel_html());
    surfaces.insert("help.about.checkForUpdates", update_channel_html());
    surfaces.insert("help.projectHelp.open", user_guide_html());
    surfaces.insert("help.projectAbout.open", project_about_html());
    surfaces.insert(
        "tools.xmlValidator.open",
        validators_html("XML Validator / Formatter", "xml"),
    );
    surfaces.insert(
        "tools.jsonValidator.open",
        validators_html("JSON Validator / Formatter", "json"),
    );
    surfaces.insert("feedback.open", feedback_html());
    surfaces.insert("help.feedback.open", feedback_html());
    surfaces.insert("report.center.open", report_center_html());
    surfaces.insert(
        "report.auditTrail.open",
        action_module_html(
            "Audit Trail",
            "report.auditTrail.open",
            "Writes and displays the local audit trail seed event.",
        ),
    );
    surfaces.insert("help.about.open", about_html());

    let body = surfaces
        .into_iter()
        .map(|(command, html)| format!("{}: {}", js_string(command), js_string(&html)))
        .collect::<Vec<_>>()
        .join(",");
    format!("const moduleSurfaces = {{{body}}};")
}

fn action_module_html(title: &str, command: &str, description: &str) -> String {
    format!(
        r#"<h1>{}</h1>
<p class="hint">{}</p>
<div class="surface">
  <h3>Backend Workflow</h3>
  <p>This module runs a concrete local Rust workflow when opened. Use the button below to run it again.</p>
  <p><button class="primary-button" onclick="hostRequest({{ action: 'moduleOpened', command: '{}', label: '{}' }})">Run Backend Workflow</button></p>
</div>"#,
        html_escape(title),
        html_escape(description),
        html_escape(command),
        html_escape(title),
    )
}

fn dashboard_html(
    workbench: &meditor_workbench::WorkbenchBaseline,
    platform: &meditor_platform::PlatformProfile,
    apex_base: &str,
    module_cards: &str,
) -> String {
    format!(
        r#"<div class="welcome-page">
  <div class="welcome-header">
    <div class="product-mark"></div>
    <div>
      <div class="welcome-title">mEditor</div>
      <div class="hint">Multi Editor Workbench</div>
    </div>
    <div class="welcome-version">Version {}</div>
  </div>
  <div class="welcome-grid">
    <section class="welcome-card">
      <h2>Database Connection</h2>
      <div class="welcome-card-body">
        <div class="welcome-tabs">
          <span class="welcome-tab active">Recent</span>
          <span class="welcome-tab">Databases Detected</span>
        </div>
        <a class="link-row" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">Local SQLite</a>
        <a class="link-row" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">JDBC Connections</a>
        <a class="link-row" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">Cloud Connections</a>
        <button class="secondary-button" style="width:100%;margin-top:12px" onclick="openModule('tools.dbaWorkshop.open','DBA Workshop')">Create a Connection Manually</button>
      </div>
    </section>
    <section class="welcome-card">
      <h2>Getting Started</h2>
      <div class="welcome-card-body">
        <div class="welcome-tabs">
          <span class="welcome-tab active">Get a Database</span>
          <span class="welcome-tab">Information</span>
          <span class="welcome-tab">Tutorials</span>
          <span class="welcome-tab">Demos</span>
          <span class="welcome-tab">Training</span>
        </div>
        <a class="link-row" onclick="openModule('setup.languageSupport.open','Programming Language Support')">Programming Language Support</a>
        <a class="link-row" onclick="openModule('file.importProject.open','Import Project')">Import Existing Project</a>
        <a class="link-row" onclick="openModule('tools.projectPlanner.open','Project Planner')">Project Planner</a>
      </div>
    </section>
    <section class="welcome-card">
      <h2>Resources</h2>
      <div class="welcome-card-body">
        <a class="link-row" onclick="openModule('tools.aimlAssistant.open','AI/ML Assistant')">AI/ML Assistant</a>
        <a class="link-row" onclick="openModule('tools.codeSecurityAnalyzer.open','Code Security Analyzer')">Code Security Analyzer</a>
        <a class="link-row" onclick="openModule('tools.umlModeling.open','UML Modeling')">UML Modeling</a>
        <div class="hint">Platform: {:?}. Menu items: {}. APEX: {}</div>
      </div>
    </section>
    <section class="welcome-card">
      <h2>Related Tools</h2>
      <div class="welcome-card-body">
        <a class="link-row" onclick="openModule('tools.webBrowser.open','Web Browser')">mEditor Web Browser</a>
        <a class="link-row" onclick="openModule('tools.sshTerminus.open','SSH Terminus')">SSH Terminus</a>
        <a class="link-row" onclick="openModule('tools.jsonValidator.open','JSON Validator')">JSON Validator</a>
        <a class="link-row" onclick="openModule('tools.xmlValidator.open','XML Validator')">XML Validator</a>
      </div>
    </section>
  </div>
  <h2 style="margin:18px 0 8px">Frozen Module Map</h2>
  <div class="module-grid">{}</div>
</div>"#,
        html_escape(meditor_core::CURRENT_BASELINE_VERSION),
        platform,
        workbench.menu_items,
        html_escape(apex_base),
        module_cards,
    )
}

fn build_module_cards(workbench: &meditor_workbench::WorkbenchBaseline) -> String {
    workbench
        .modules
        .iter()
        .map(|module| {
            format!(
                r#"<div class="module-card"><h4>{}</h4><p>{:?} | same-window={}</p></div>"#,
                html_escape(module.label),
                module.area,
                module.same_window
            )
        })
        .collect::<String>()
}

fn new_project_html() -> String {
    r#"<h1>New Project</h1>
<div class="split">
  <div class="surface">
    <h3>Project</h3>
    <p><input id="newProjectName" class="command-filter" value="SampleProject" placeholder="Project name"></p>
    <p><select id="newProjectKind" class="command-filter">
      <option value="major">Major Project</option>
      <option value="subproject">Sub Project</option>
    </select></p>
    <p><input id="newProjectTechStack" class="command-filter" value="Rust" placeholder="Tech stack"></p>
    <p><button class="primary-button" onclick="createProject()">Create Project</button></p>
  </div>
  <div class="surface">
    <h3>Created Structure</h3>
    <ul>
      <li>.meditor/project/project.toml</li>
      <li>.meditor/project/modules.toml</li>
      <li>.meditor/project/planning.toml</li>
      <li>.meditor/project/docs/about.md</li>
      <li>.meditor/project/docs/help-index.md</li>
      <li>src/README.md</li>
    </ul>
  </div>
</div>"#
        .to_string()
}

fn run_tasks_html() -> String {
    let rows = meditor_sdlc::standard_actions()
        .into_iter()
        .map(|action| {
            format!(
                "<tr><td>{}</td><td>{:?}</td><td>{}</td></tr>",
                html_escape(action.label),
                action.kind,
                html_escape(action.executable_id.unwrap_or("project profile"))
            )
        })
        .collect::<String>();
    format!(
        r#"<h1>Tasks</h1>
<p class="hint">End-to-end project workflow: detect project, run build/test/launch/debug tasks, capture stdout/stderr/compiler diagnostics, and write a local debugging handover report.</p>
<div class="split">
  <div class="surface">
    <h3>Project Task</h3>
    <p><label class="hint">Project folder, relative to workspace</label><input id="taskProjectPath" class="command-filter" value="." placeholder="., SampleProject, app"></p>
    <p><label class="hint">Source file for single-file/interpreter tasks</label><input id="taskSourcePath" class="command-filter" placeholder="uses active editor file when blank"></p>
    <p><label class="hint">Action</label><select id="taskAction" class="command-filter">
      <option value="build">Build</option>
      <option value="debug">Debug / Check</option>
      <option value="test">Test</option>
      <option value="launch">Launch / Run</option>
      <option value="clean">Clean</option>
      <option value="package">Package</option>
    </select></p>
    <p><label class="hint">Timeout seconds</label><input id="taskTimeoutSeconds" class="command-filter" type="number" value="60" min="5" max="600"></p>
    <p>
      <button class="secondary-button" onclick="detectProject()">Detect Project</button>
      <button class="primary-button" onclick="runTask()">Run Selected Task</button>
    </p>
    <p>
      <button class="secondary-button" onclick="runQuickTask('build')">Build</button>
      <button class="secondary-button" onclick="runQuickTask('debug')">Debug</button>
      <button class="secondary-button" onclick="runQuickTask('test')">Test</button>
      <button class="secondary-button" onclick="runQuickTask('launch')">Launch</button>
      <button class="secondary-button" onclick="runQuickTask('clean')">Clean</button>
    </p>
    <div id="taskProjectSummary" class="surface" style="margin:10px 0 0"><h3>Project Detection</h3><p class="hint">Press Detect Project to list runnable profiles.</p></div>
  </div>
  <div class="surface">
    <h3>Task Console</h3>
    <div id="taskResultSummary" class="surface" style="margin:0 0 10px"><h3>Task Result</h3><p class="hint">Run a task to capture compiler output and suggestions.</p></div>
    <textarea id="taskLog" class="editor" style="min-height:300px" readonly spellcheck="false"></textarea>
  </div>
</div>
<div class="surface"><h3>Standard SDLC Actions</h3><table class="table"><tr><th>Action</th><th>Kind</th><th>Executable</th></tr>{rows}</table></div>"#
    )
}

fn refactor_html() -> String {
    r#"<h1>Refactor And Navigation</h1>
<p class="hint">Language-server discovery, guarded refactor workflow, and user-approved change policy.</p>
<div class="split">
  <div class="surface">
    <h3>LSP Runtime</h3>
    <p><button class="primary-button" onclick="detectLspServers()">Detect LSP Servers</button> <button class="secondary-button" onclick="detectDebugAdapters()">Detect Debug Adapters</button></p>
    <p class="hint">mEditor uses installed language servers where present. Refactors must show rationale, impact, and rollback path before source files are changed.</p>
  </div>
  <div class="surface">
    <h3>Guarded Rename Refactor</h3>
    <p><input id="refactorPath" class="command-filter" placeholder="workspace-relative file path"></p>
    <p><input id="refactorFrom" class="command-filter" placeholder="old symbol or text"></p>
    <p><input id="refactorTo" class="command-filter" placeholder="new symbol or text"></p>
    <p><button class="primary-button" onclick="applyRenameRefactor()">Apply With Rollback Backup</button></p>
  </div>
</div>
<div class="surface">
  <h3>Refactor Policy</h3>
    <ul>
      <li>User remains the master for every code change.</li>
      <li>Auto-remediation stays configurable and approval-gated.</li>
      <li>Rollback records are written with debug/refactor reports.</li>
      <li>Navigation and symbol intelligence are driven by detected tooling.</li>
    </ul>
</div>"#
        .to_string()
}

fn vcs_html(provider: &str) -> String {
    let title = if provider == "svn" { "SVN" } else { "Git" };
    let operations = if provider == "svn" {
        r#"<button class="secondary-button" onclick="runVcs('svn','status')">Status</button>
      <button class="secondary-button" onclick="runVcs('svn','log')">Log</button>
      <button class="secondary-button" onclick="runVcs('svn','info')">Info</button>"#
    } else {
        r#"<button class="secondary-button" onclick="runVcs('git','status')">Status</button>
      <button class="secondary-button" onclick="runVcs('git','log')">Log</button>
      <button class="secondary-button" onclick="runVcs('git','branch')">Branch</button>
      <button class="secondary-button" onclick="runVcs('git','diff')">Diff Summary</button>
      <button class="secondary-button" onclick="runVcs('git','remote')">Remote</button>"#
    };
    format!(
        r#"<h1>{title}</h1>
<p class="hint">Guarded version-control operations run inside the selected workspace folder.</p>
<div class="surface">
  <p><label class="hint">Project folder</label><input id="{provider}ProjectPath" class="command-filter" value="."></p>
  <p>{operations}</p>
</div>"#
    )
}

fn dba_workshop_html() -> String {
    let rows = meditor_db_workbench::dba_dashboard_sections()
        .into_iter()
        .map(|section| format!("<tr><td>{section:?}</td><td>ready</td></tr>"))
        .collect::<String>();
    format!(
        r#"<h1>DBA Workshop</h1>
<p class="hint">SQL Developer-style connection navigator, worksheet, object browser, result grid, DBA Panel, patch history, and backup history.</p>
<div class="split">
  <div class="surface"><h3>Connections</h3>
    <p><input id="dbConnName" class="command-filter" placeholder="Connection name" value="Local SQLite"></p>
    <p><select id="dbConnKind" class="command-filter"><option value="sqlite">SQLite</option><option value="jdbc">JDBC</option></select></p>
    <p><input id="dbConnTarget" class="command-filter" placeholder="DB path or JDBC URL" value=".meditor/security/vulnerability-intel.sqlite"></p>
    <p><button class="primary-button" onclick="saveDbConnection()">Save Connection</button></p>
    <h3>JDBC Drivers</h3>
    <p><input id="jdbcJarPath" class="command-filter" placeholder="Relative path to JDBC driver JAR"></p>
    <p><input id="jdbcUrl" class="command-filter" placeholder="jdbc:vendor://host:port/database"></p>
    <p><input id="jdbcUser" class="command-filter" placeholder="Database user"></p>
    <p><input id="jdbcPassword" class="command-filter" type="password" placeholder="Password used for this run only"></p>
    <p><input id="jdbcSchema" class="command-filter" placeholder="Schema pattern, optional"></p>
    <p><input id="jdbcObject" class="command-filter" value="%" placeholder="Object/table pattern"></p>
    <p><button class="secondary-button" onclick="addJdbcDriver()">Register JDBC Driver</button> <button class="secondary-button" onclick="runJdbcMetadata('dashboard')">DBA Dashboard</button> <button class="secondary-button" onclick="runJdbcMetadata('objects')">Object Browser</button> <button class="secondary-button" onclick="runJdbcMetadata('columns')">Columns</button> <button class="secondary-button" onclick="runJdbcMetadata('indexes')">Indexes</button> <button class="secondary-button" onclick="runJdbcMetadata('primaryKeys')">PK</button> <button class="secondary-button" onclick="runJdbcMetadata('foreignKeys')">FK</button> <button class="secondary-button" onclick="runJdbcMetadata('procedures')">Procedures</button> <button class="secondary-button" onclick="runJdbcMetadata('schemas')">Schemas</button> <button class="secondary-button" onclick="runJdbcMetadata('catalogs')">Catalogs</button> <button class="secondary-button" onclick="runJdbcMetadata('typeInfo')">Types</button> <button class="primary-button" onclick="runJdbcSql()">Run JDBC SQL</button></p>
    <p><label class="hint">DBA probe</label><select id="jdbcDbaProbe" class="command-filter">
      <option value="generic_version">Generic connection probe</option>
      <option value="oracle_patch_history">Oracle patch history</option>
      <option value="oracle_rman_backup">Oracle RMAN backup history</option>
      <option value="oracle_sessions">Oracle sessions</option>
      <option value="postgres_activity">PostgreSQL activity</option>
      <option value="postgres_database_size">PostgreSQL database size</option>
      <option value="mysql_processlist">MySQL process list</option>
      <option value="mysql_schema_size">MySQL schema size</option>
      <option value="sqlserver_sessions">SQL Server sessions</option>
      <option value="sqlserver_database_size">SQL Server database size</option>
    </select></p>
    <p><button class="primary-button" onclick="runJdbcDbaProbe()">Run DBA Probe</button></p>
  </div>
  <div class="surface"><h3>SQL Worksheet</h3>
  <p><input id="sqliteDbPath" class="command-filter" value=".meditor/security/vulnerability-intel.sqlite" placeholder="SQLite DB path relative to workspace"></p>
  <textarea id="dbaSqlText" class="editor" style="min-height:180px">select current_timestamp as checked_at;</textarea>
  <p><button class="primary-button" onclick="runSqliteWorkbench()">Run SQLite SQL</button> <button class="secondary-button" onclick="runDbaSql()">Dry-run Generic SQL</button> <button class="secondary-button" onclick="explainDbaSql()">Explain Plan</button></p>
  <table class="table"><tr><th>Dashboard Section</th><th>Status</th></tr>{rows}</table></div>
</div>"#
    )
}

fn ssh_terminus_html() -> String {
    let actions = meditor_ssh_terminus::terminal_actions()
        .into_iter()
        .map(|action| format!("<span class=\"pill\">{action:?}</span>"))
        .collect::<String>();
    format!(
        r#"<h1>SSH Terminus</h1>
<p class="hint">Grouped SSH terminal manager with multiple same-window terminal tabs.</p>
<div class="split">
  <div class="surface"><h3>Profile</h3>
    <p><input id="sshName" class="command-filter" placeholder="Profile name"></p>
    <p><input id="sshGroup" class="command-filter" value="Development" placeholder="Group"></p>
    <p><input id="sshHost" class="command-filter" placeholder="host or ip"></p>
    <p><input id="sshUser" class="command-filter" placeholder="user"></p>
    <p><button class="primary-button" onclick="saveSshProfile()">Save Profile</button> <button class="secondary-button" onclick="testSshConnection()">Test SSH</button></p>
  </div>
  <div class="surface"><h3>Terminal Tab</h3>{actions}
    <textarea id="sshCommand" class="editor" style="min-height:140px">uname -a</textarea>
    <p><button class="primary-button" onclick="runSshCommand()">Run One SSH Command</button> <button class="secondary-button" onclick="startTerminalSession(false)">Start SSH Session</button> <button class="secondary-button" onclick="startTerminalSession(true)">Start Local Shell</button></p>
    <p><input id="terminalSessionId" class="command-filter" readonly placeholder="terminal session id"></p>
    <p><span id="terminalSessionLabel" class="hint">No terminal session running.</span></p>
    <pre id="terminalOutput" class="surface" style="height:240px;overflow:auto;white-space:pre-wrap;background:#0f1720;color:#d8dee9"></pre>
    <p><input id="terminalInput" class="command-filter" placeholder="command/input for active terminal" onkeydown="if(event.key==='Enter') sendTerminalInput()"></p>
    <p><button class="secondary-button" onclick="sendTerminalInput()">Send</button> <button class="secondary-button" onclick="pollTerminalSession()">Poll</button> <button class="secondary-button" onclick="stopTerminalSession()">Stop</button></p>
    <p class="hint">Persistent sessions use the installed shell or OpenSSH process with piped input/output. A native PTY crate is still required for perfect terminal-control emulation.</p>
  </div>
</div>"#
    )
}

fn sftp_scp_html() -> String {
    r#"<h1>SFTP/SCP Transfer</h1>
<p class="hint">FileZilla-style remote transfer profile, upload/download queue, and guarded SCP execution.</p>
<div class="split">
  <div class="surface">
    <h3>Remote Target</h3>
    <p><input id="sftpTarget" class="command-filter" placeholder="user@host:/remote/path"></p>
    <p><button class="primary-button" onclick="sftpList()">List Remote Path</button></p>
  </div>
  <div class="surface">
    <h3>Transfer Queue</h3>
    <p><select id="transferDirection" class="command-filter"><option value="download">Download</option><option value="upload">Upload</option></select></p>
    <p><input id="transferRemoteTarget" class="command-filter" placeholder="user@host:/remote/file"></p>
    <p><input id="transferLocalPath" class="command-filter" placeholder="workspace-relative local file path"></p>
    <p><button class="secondary-button" onclick="enqueueTransfer()">Add To Queue</button> <button class="primary-button" onclick="runFileTransfer()">Run Transfer Now</button></p>
    <p class="hint">Queue metadata is stored locally under .meditor/transfers; SCP uses the installed OpenSSH tooling.</p>
  </div>
</div>"#
        .to_string()
}

fn web_browser_html() -> String {
    let commands = meditor_browser::default_toolbar_commands()
        .into_iter()
        .map(|command| format!("<button class=\"secondary-button\" onclick=\"browserNavigate()\">{command:?}</button>"))
        .collect::<String>();
    format!(
        r#"<h1>Web Browser</h1>
<p class="hint">Same-window browser tab surface using the native webview shell.</p>
<p><input id="browserUrl" class="command-filter" value="https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/"></p>
<p>{commands}</p>
<iframe id="browserFrame" class="surface" style="width:100%;height:520px;border:1px solid var(--line)" src="about:blank"></iframe>"#
    )
}

fn language_support_html() -> String {
    let rows = meditor_toolchains::supported_language_catalog()
        .into_iter()
        .take(80)
        .map(|entry| {
            let tools = entry
                .tooling
                .iter()
                .map(|tool| tool.display_name)
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "<tr><td>{}</td><td>{:?}</td><td>{}</td></tr>",
                html_escape(entry.display_name),
                entry.kind,
                html_escape(&tools)
            )
        })
        .collect::<String>();
    format!(
        r#"<h1>Programming Language Support</h1>
<p class="hint">Active/missing tooling discovery, open-source tooling installation, network testing, and proxy validation.</p>
<p><button class="primary-button" onclick="detectToolchains()">Detect Installed Tooling</button> <button class="secondary-button" onclick="previewSqliteSetup()">Preview SQLite Setup</button> <button class="secondary-button" onclick="createSqliteFolders()">Create SQLite Folder</button> <button class="primary-button" onclick="runOneClickSetup()">Run One-click Setup</button></p>
<table class="table"><tr><th>Language or stack</th><th>Kind</th><th>Tooling</th></tr>{rows}</table>"#
    )
}

fn verify_install_dependencies_html() -> String {
    let options = meditor_toolchains::seed_executable_requirements()
        .into_iter()
        .map(|requirement| {
            format!(
                r#"<option value="{}">{}</option>"#,
                html_escape(requirement.id),
                html_escape(format!("{} ({})", requirement.display_name, requirement.id))
            )
        })
        .collect::<String>();
    format!(
        r#"<h1>Verify And Install Dependencies</h1>
<p class="hint">Verify missing compilers, runtimes, database clients, VCS tools, and language servers. Install Selected downloads and installs through the detected platform package manager after explicit user confirmation.</p>
<div class="split">
  <div class="surface">
    <h3>Verification</h3>
    <p><button class="primary-button" onclick="verifyDependencies()">Verify Dependencies</button></p>
    <p><select id="dependencyInstallSelect" class="command-filter" onchange="dependencySelectionChanged()">{options}</select></p>
    <p><button class="secondary-button" onclick="installSelectedDependency()">Install Selected Missing Dependency</button></p>
    <p class="hint">Install command</p>
    <pre id="dependencyInstallCommand" class="surface" style="white-space:pre-wrap;min-height:58px"></pre>
    <p class="hint">Source URL</p>
    <pre id="dependencySourceUrl" class="surface" style="white-space:pre-wrap;min-height:48px"></pre>
  </div>
  <div class="surface">
    <h3>Install Policy</h3>
    <ul>
      <li>No dependency is downloaded or installed until the user clicks Install Selected and confirms.</li>
      <li>mEditor uses Homebrew, winget, apt-get, dnf, or pacman when available.</li>
      <li>If automatic install is unsupported, the source URL and manual strategy are shown.</li>
      <li>After installation, run Verify Dependencies again to refresh language services.</li>
    </ul>
  </div>
</div>
<div id="dependencyResults" class="surface"><h3>Dependency Verification</h3><p class="hint">Press Verify Dependencies to scan the machine.</p></div>"#
    )
}

fn project_importers_html() -> String {
    let kinds = meditor_project_importers::supported_project_kinds()
        .into_iter()
        .map(|kind| format!("<span class=\"pill\">{kind:?}</span>"))
        .collect::<String>();
    let vs = meditor_project_importers::visual_studio_project_file_kinds()
        .into_iter()
        .map(|kind| format!("<span class=\"pill\">{kind:?}</span>"))
        .collect::<String>();
    format!(
        r#"<h1>Import Project</h1>
<p class="hint">Import NetBeans, Eclipse, JDeveloper, and Visual Studio projects.</p>
<p><button class="primary-button" onclick="importProject()">Write Import Report</button> <button class="secondary-button" onclick="previewProjectImport()">Preview Import For Workspace</button></p>
<div class="surface"><h3>Project Kinds</h3>{kinds}</div>
<div class="surface"><h3>Visual Studio and MSBuild Files</h3>{vs}</div>"#
    )
}

fn security_analyzer_html() -> String {
    let rows = meditor_code_security::seed_rules()
        .into_iter()
        .map(|rule| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                html_escape(rule.id),
                rule.severity,
                rule.cvss_score,
                html_escape(rule.summary)
            )
        })
        .collect::<String>();
    let repo = meditor_cvss_repository::RepositoryLayout::under_working_folder(".");
    format!(
        r#"<h1>Code Security Analyzer</h1>
<p class="hint">Open-file security scan and SQLite CVSS repository.</p>
<p><button class="primary-button" onclick="previewCvssUpdate()">Preview CVSS Repository Update</button></p>
<p>Repository: {}</p>
<table class="table"><tr><th>Rule</th><th>Severity</th><th>CVSS</th><th>Summary</th></tr>{rows}</table>"#,
        html_escape(&display_path(&repo.vulnerability_db))
    )
}

fn ai_assistant_html() -> String {
    let formats = meditor_ai::supported_knowledge_formats()
        .into_iter()
        .map(|format| format!("<span class=\"pill\">{format:?}</span>"))
        .collect::<String>();
    format!(
        r#"<h1>AI/ML Assistant</h1>
<p class="hint">Workspace-aware coding help, approval-gated fixes, knowledge ingestion, and retraining contract.</p>
<p><input id="assistantModel" class="command-filter" value="llama3" placeholder="Local model name, for example llama3"></p>
<textarea id="assistantPrompt" class="editor" style="min-height:160px">Explain the active file and suggest safe next steps.</textarea>
<p><button class="primary-button" onclick="askAssistant()">Ask Contract Assistant</button> <button class="secondary-button" onclick="runLocalAi()">Run Local AI Runtime</button> <button class="secondary-button" onclick="retrainAiKnowledge()">Reindex Local Knowledge</button> <button class="secondary-button" onclick="trainAiLocalModel()">Train Local Retrieval Model</button> <button class="secondary-button" onclick="exportAiTrainingDataset()">Export Fine-tune Dataset</button></p>
<div class="surface">
  <h3>Knowledge Ingestion</h3>
  <p><input id="knowledgeTitle" class="command-filter" placeholder="Knowledge title"></p>
  <textarea id="knowledgeContent" class="editor" style="min-height:160px" placeholder="Paste extracted PDF/EPUB/DOC/TXT/XLS knowledge or notes here"></textarea>
  <p><button class="secondary-button" onclick="saveAiKnowledge()">Save To Local Knowledge Base</button></p>
</div>
<div class="surface">
  <h3>External Trainer</h3>
  <p><input id="aiTrainerExecutable" class="command-filter" placeholder="Trainer executable, for example python3"></p>
  <textarea id="aiTrainerArgs" class="editor" style="min-height:110px" placeholder="One argument per line. Use {{dataset}} for the exported JSONL dataset and {{model}} for the target model artifact path."></textarea>
  <p><button class="secondary-button" onclick="saveAiTrainerConfig()">Save Trainer Config</button> <button class="primary-button" onclick="runAiTrainingJob()">Run External Training Job</button></p>
  <p class="hint">mEditor creates the dataset and captures the training transcript. Any real model weight mutation is performed only by the explicitly configured external trainer.</p>
</div>
<h3>Knowledge Formats</h3>{formats}"#
    )
}

fn project_planner_html() -> String {
    r#"<h1>Project Planner</h1>
<p class="hint">Portfolio and project planning linked to editor projects.</p>
<div class="split">
  <div class="surface">
    <h3>Planning Item</h3>
    <p><select id="plannerModel" class="command-filter"><option value="objective">Objective -> milestone -> task</option><option value="scrum">Epic -> story -> task</option></select></p>
    <p><input id="plannerTitle" class="command-filter" placeholder="Objective, milestone, epic, story, or task"></p>
    <p><select id="plannerStatus" class="command-filter"><option>Planned</option><option>In Progress</option><option>Done</option><option>Blocked</option></select></p>
    <p><button class="primary-button" onclick="savePlannerItem()">Save Planning Item</button></p>
  </div>
  <div class="surface">
    <h3>Dashboard</h3>
    <p class="hint">Saved planner items are stored locally under .meditor/planner and linked to the workspace.</p>
  </div>
</div>
<div class="metric-grid">
  <div class="metric"><label>Planning models</label><strong>2</strong></div>
  <div class="metric"><label>Dashboard</label><strong>Gantt + charts</strong></div>
  <div class="metric"><label>Timezone</label><strong>required</strong></div>
  <div class="metric"><label>Excel template</label><strong>import/export</strong></div>
</div>"#
        .to_string()
}

fn extension_sdk_html() -> String {
    r#"<h1>Extension SDK</h1>
<p class="hint">Create a local extension skeleton with a manifest, permissions placeholder, and README.</p>
<p><button class="primary-button" onclick="createExtensionSkeleton()">Create Sample Extension</button></p>
<div class="surface"><h3>Extension Folder</h3><p>.meditor/extensions/sample-extension/plugin.json</p></div>"#
        .to_string()
}

fn workspace_indexer_html() -> String {
    r#"<h1>Workspace Indexer</h1>
<p class="hint">Build a local file-extension index for navigation, reporting, and future search.</p>
<p><button class="primary-button" onclick="scanWorkspaceIndex()">Scan Workspace</button></p>
<div class="surface"><h3>Index Output</h3><p>.meditor/index/workspace-index.json</p></div>"#
        .to_string()
}

fn credentials_html() -> String {
    r#"<h1>Secrets And Credentials</h1>
<p class="hint">Real encrypted credential storage through the operating-system credential store when available.</p>
<div class="split">
  <div class="surface">
    <h3>Store Secret</h3>
    <p><input id="credentialService" class="command-filter" value="mEditor" placeholder="Service"></p>
    <p><input id="credentialAccount" class="command-filter" placeholder="Account or profile id"></p>
    <p><input id="credentialSecret" class="command-filter" type="password" placeholder="Secret"></p>
    <p><button class="secondary-button" onclick="credentialStoreStatus()">Check Store</button> <button class="primary-button" onclick="storeCredential()">Store In OS Credential Store</button></p>
  </div>
  <div class="surface">
    <h3>Workspace Policy</h3>
    <ul>
      <li>Secrets are never written to workspace JSON.</li>
      <li>Workspace JSON keeps only service/account/backend pointers.</li>
      <li>DBA Workshop, SSH Terminus, and SFTP/SCP can reference stored credential ids.</li>
    </ul>
  </div>
</div>"#
        .to_string()
}

fn report_center_html() -> String {
    r#"<h1>Report Center</h1>
<p class="hint">Local reports and local-only user program bug repository. These records are not sent to the mEditor feedback backend.</p>
<div class="split">
  <div class="surface">
    <h3>Local Bug</h3>
    <p><input id="localBugTitle" class="command-filter" placeholder="Bug title"></p>
    <p><select id="localBugSeverity" class="command-filter"><option>Low</option><option selected>Medium</option><option>High</option><option>Critical</option></select></p>
    <textarea id="localBugDetails" class="editor" style="min-height:160px" placeholder="Steps, actual behavior, expected behavior, files"></textarea>
    <p><button class="primary-button" onclick="saveLocalBug()">Save Local Bug</button> <button class="secondary-button" onclick="listLocalBugs()">List Bugs</button></p>
  </div>
  <div class="surface">
    <h3>Reports</h3>
    <p><button class="primary-button" onclick="listReports()">List Local Reports</button></p>
    <p class="hint">Includes debug handover reports, import reports, planner records, and JSON summaries under .meditor.</p>
  </div>
</div>"#
        .to_string()
}

fn uml_modeling_html() -> String {
    let diagrams = meditor_uml_modeling::supported_diagram_kinds()
        .into_iter()
        .map(|kind| format!("<span class=\"pill\">{kind:?}</span>"))
        .collect::<String>();
    format!(
        r#"<h1>UML Modeling</h1>
<p class="hint">Text live preview and GUI modeling surface.</p>
<textarea class="editor" style="min-height:240px">@startuml
class mEditor
mEditor : same-window tabs
@enduml</textarea>
<h3>Supported Diagram Kinds</h3>{diagrams}"#
    )
}

fn registration_html() -> String {
    r#"<h1>Register mEditor</h1>
<p class="hint">Registration is optional. mEditor remains fully functional without registration and with no obligation.</p>
<div class="surface">
  <p><strong data-license-display>UnRegistered but fully functional copy with no obligation</strong></p>
  <p><input id="eulaNameInline" class="command-filter" placeholder="Name"></p>
  <p><input id="eulaEmailInline" class="command-filter" placeholder="Email"></p>
  <p class="hint">Use the first-run EULA gate to personalize a copy. If this copy is already accepted, the About dialog reflects the current license display.</p>
</div>"#
        .to_string()
}

fn update_channel_html() -> String {
    r#"<h1>Update Channel Manager</h1>
<p class="hint">Check a configured public Git release source for new mEditor tags. Install remains user-approved and must preserve local data.</p>
<div class="split">
  <div class="surface">
    <h3>Release Source</h3>
    <p><input id="updateSource" class="command-filter" value="git@github.com:spandeyindia/mEditor.git" placeholder="Git release source URL"></p>
    <p><select id="updateChannel" class="command-filter"><option value="stable">Stable</option><option value="pilot">Pilot</option><option value="dev">Development</option></select></p>
    <p><button class="secondary-button" onclick="saveUpdateSource()">Save Source</button> <button class="primary-button" onclick="checkUpdateSource()">Check For Updates</button></p>
  </div>
  <div class="surface">
    <h3>Update Safety Rules</h3>
    <ul>
      <li>Do not overwrite .meditor workspace data.</li>
      <li>Show release notes, current version, target version, and rollback path.</li>
      <li>Require user approval before download or install.</li>
      <li>Record update attempts in diagnostics history.</li>
    </ul>
  </div>
</div>"#
        .to_string()
}

fn user_guide_html() -> String {
    r#"<h1>Project Help</h1>
<p class="hint">Embedded help index for mEditor features and generated project documentation.</p>
<div class="surface">
  <h3>Help Content</h3>
  <p>End-user documentation is stored at docs/USER_GUIDE.md and project-specific help is generated under .meditor/project/docs.</p>
  <ul>
    <li>Workbench navigation and same-window tabs</li>
    <li>File explorer, editor, Vi editor, validators, and formatters</li>
    <li>DBA Workshop, SSH Terminus, SFTP/SCP, browser, and SDLC tasks</li>
    <li>AI assistant, security analyzer, reports, planner, UML, and update manager</li>
  </ul>
</div>"#
        .to_string()
}

fn project_about_html() -> String {
    r#"<h1>About Project</h1>
<p class="hint">Every mEditor project gets its own About and Help documents.</p>
<div class="surface">
  <p>Project About is generated at .meditor/project/docs/about.md when a project is created.</p>
  <p>Project Help is generated at .meditor/project/docs/help-index.md and can be expanded with PRD, TDD, deployment, ERD, flowchart, and API documentation.</p>
</div>"#
        .to_string()
}

fn privacy_center_html() -> String {
    let points = meditor_installation_metrics::first_run_notice_points()
        .into_iter()
        .map(|point| format!("<li>{}</li>", html_escape(point)))
        .collect::<String>();
    format!(
        r#"<h1>Privacy Center</h1>
<p class="hint">Plain-language data flow disclosure for mEditor.</p>
<ul>{points}</ul>"#
    )
}

fn diagnostics_html() -> String {
    let fields = meditor_installation_metrics::automatic_error_capture_fields()
        .into_iter()
        .map(|field| format!("<li>{}</li>", html_escape(field)))
        .collect::<String>();
    format!(
        r#"<h1>Diagnostics Bundle</h1>
<p class="hint">Reviewed and redacted diagnostic export before sending.</p>
<p><button class="primary-button" onclick="previewDiagnostics()">Generate Diagnostics Preview</button> <button class="secondary-button" onclick="crossPlatformPackageAudit()">Cross-platform Package Audit</button> <button class="primary-button" onclick="runProductionReadiness()">Run Production Readiness Gate</button></p>
<ul>{fields}</ul>"#
    )
}

fn validators_html(title: &str, default_kind: &str) -> String {
    let formatter_id = format!("tab-inline-{default_kind}-formatter");
    let json_selected = if default_kind == "json" {
        " selected"
    } else {
        ""
    };
    let xml_selected = if default_kind == "xml" {
        " selected"
    } else {
        ""
    };
    let rows = meditor_validators::validator_tools()
        .into_iter()
        .map(|tool| {
            format!(
                "<tr><td>{:?}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                tool.kind,
                html_escape(tool.menu_path),
                tool.can_validate_open_file,
                tool.can_validate_selected_text
            )
        })
        .collect::<String>();
    format!(
        r#"<h1>{}</h1>
<p class="hint">Two-panel formatter. Paste or edit on the left, see the live Rust-formatted result on the right, or upload a JSON/XML file into the same flow.</p>
<p><button class="primary-button" onclick="openFormatterForKind('{}')">Open {} Workbench</button></p>
<div class="formatter-shell" style="height:620px">
  <div class="formatter-toolbar">
    <strong>{}</strong>
    <select id="{}-kind" class="formatter-kind" onchange="formatterKindChanged('{}')">
      <option value="json"{}>JSON</option>
      <option value="xml"{}>XML</option>
    </select>
    <input id="{}-file" class="formatter-file" type="file" accept=".json,.xml,.xsd,.xsl,.xslt,.wsdl,.txt,application/json,text/xml,application/xml" onchange="formatterUploadFile('{}', this.files && this.files[0])">
    <button class="secondary-button" onclick="formatterPaste('{}')">Paste</button>
    <button class="secondary-button" onclick="formatterCopyInput('{}')">Copy Input</button>
    <button class="secondary-button" onclick="formatterCopyOutput('{}')">Copy Formatted</button>
    <button class="primary-button" onclick="formatterApplyToEditor('{}')">Apply To Open Editor</button>
  </div>
  <div class="formatter-grid">
    <section class="formatter-panel"><h3>Input</h3><textarea id="{}-input" class="formatter-editor" spellcheck="false" data-formatter-id="{}" oninput="scheduleFormatterPreview(this.dataset.formatterId)"></textarea></section>
    <section class="formatter-panel"><h3>Formatted Result</h3><textarea id="{}-output" class="formatter-output" readonly spellcheck="false"></textarea></section>
  </div>
  <div id="{}-status" class="formatter-status">Paste, type, or upload JSON/XML. Formatting runs live through the Rust backend.</div>
</div>
<div class="surface"><h3>Validator Backend</h3><table class="table"><tr><th>Kind</th><th>Menu</th><th>Open file</th><th>Selected text</th></tr>{rows}</table></div>"#,
        html_escape(title),
        html_escape(default_kind),
        html_escape(&default_kind.to_uppercase()),
        html_escape(title),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        json_selected,
        xml_selected,
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
        html_escape(&formatter_id),
    )
}

fn feedback_html() -> String {
    r#"<h1>Feedback And Bugs</h1>
<p class="hint">Bug report or enhancement request for mEditor. Owner contact: s.pandey.india@gmail.com</p>
<p><input id="feedbackEmail" class="command-filter" placeholder="Email id for status updates"></p>
<p><select id="feedbackKind" class="command-filter"><option value="BUG">Bug Report</option><option value="ENHANCEMENT">Enhancement Request</option></select></p>
<p><input id="feedbackTitle" class="command-filter" placeholder="Title"></p>
<textarea id="feedbackDetails" class="editor" style="min-height:180px" placeholder="Details, steps, expected behavior, actual behavior"></textarea>
<p><button class="primary-button" onclick="submitFeedback()">Save/Submit Feedback</button> <button class="secondary-button" onclick="previewFeedback('BUG')">Preview Bug Report</button> <button class="secondary-button" onclick="previewFeedback('ENHANCEMENT')">Preview Enhancement</button></p>"#
        .to_string()
}

fn about_html() -> String {
    format!(
        r#"<h1>About mEditor</h1>
<p><strong>{} {}</strong></p>
<p>Multi Editor</p>
<p><strong data-license-display>UnRegistered but fully functional copy with no obligation</strong></p>
<p>mEditor Freeware EULA</p>
<p class="hint">UnRegistered but fully functional copy with no obligation remains the required unregistered display text.</p>
<p><button class="primary-button" onclick="previewUpdateCheck()">Check For Updates</button> <button class="secondary-button" onclick="openModule('help.updateChannelManager.open','Update Channel Manager')">Update Channel Manager</button></p>"#,
        meditor_core::PRODUCT_NAME,
        meditor_core::CURRENT_BASELINE_VERSION
    )
}

#[derive(Clone, Debug)]
struct FileEntry {
    display_path: String,
    is_dir: bool,
    size_bytes: u64,
    depth: usize,
}

fn workspace_entries(workspace_root: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    collect_workspace_entries(workspace_root, workspace_root, 0, &mut entries);
    entries.sort_by(|left, right| {
        left.display_path
            .to_lowercase()
            .cmp(&right.display_path.to_lowercase())
    });
    entries
}

fn collect_workspace_entries(
    root: &Path,
    folder: &Path,
    depth: usize,
    entries: &mut Vec<FileEntry>,
) {
    if depth > 3 || entries.len() >= 350 {
        return;
    }
    let Ok(read_dir) = fs::read_dir(folder) else {
        return;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if matches!(
            name.as_str(),
            ".git" | "target" | ".DS_Store" | "node_modules"
        ) {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        let is_dir = metadata.is_dir();
        entries.push(FileEntry {
            display_path: path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string(),
            is_dir,
            size_bytes: metadata.len(),
            depth,
        });
        if is_dir {
            collect_workspace_entries(root, &path, depth + 1, entries);
        }
    }
}

fn html_escape(value: impl AsRef<str>) -> String {
    value
        .as_ref()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn js_string(value: impl AsRef<str>) -> String {
    let mut escaped = String::from("\"");
    for ch in value.as_ref().chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '<' => escaped.push_str("\\u003c"),
            '>' => escaped.push_str("\\u003e"),
            '&' => escaped.push_str("\\u0026"),
            _ => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

fn path_relative_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or(""))
        .to_string()
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn format_size(size_bytes: u64) -> String {
    if size_bytes >= 1024 * 1024 {
        format!("{:.1} MB", size_bytes as f64 / (1024.0 * 1024.0))
    } else if size_bytes >= 1024 {
        format!("{:.1} KB", size_bytes as f64 / 1024.0)
    } else {
        format!("{size_bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn ipc_opens_saves_and_validates_workspace_files() {
        let root = temp_workspace();
        fs::write(root.join("demo.json"), "{\"name\":\"mEditor\"}").unwrap();

        let open = handle_ipc(&root, r#"{"action":"openFile","path":"demo.json"}"#);
        assert_eq!(open["action"], "openFileResult");
        assert_eq!(open["ok"], true);
        assert_eq!(open["content"], "{\"name\":\"mEditor\"}");

        let open_vi = handle_ipc(&root, r#"{"action":"openFileForVi","path":"demo.json"}"#);
        assert_eq!(open_vi["action"], "openFileForViResult");
        assert_eq!(open_vi["ok"], true);
        assert_eq!(open_vi["content"], "{\"name\":\"mEditor\"}");

        let open_formatter = handle_ipc(
            &root,
            r#"{"action":"openFileForFormatter","path":"demo.json"}"#,
        );
        assert_eq!(open_formatter["action"], "openFileForFormatterResult");
        assert_eq!(open_formatter["ok"], true);
        assert_eq!(open_formatter["content"], "{\"name\":\"mEditor\"}");

        let save = handle_ipc(
            &root,
            r#"{"action":"saveFile","path":"demo.json","content":"{\"saved\":true}"}"#,
        );
        assert_eq!(save["action"], "saveFileResult");
        assert_eq!(save["ok"], true);
        assert_eq!(
            fs::read_to_string(root.join("demo.json")).unwrap(),
            "{\"saved\":true}"
        );

        let valid = handle_ipc(
            &root,
            r#"{"action":"validateFile","path":"demo.json","kind":"json","content":"{\"saved\":true}","realtime":true}"#,
        );
        assert_eq!(valid["action"], "validateFileResult");
        assert_eq!(valid["valid"], true);
        assert_eq!(valid["realtime"], true);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn gui_includes_fast_menus_and_realtime_validation_hooks() {
        let root = temp_workspace();
        let html = build_gui_html(&root);

        assert!(html.contains("menu-trigger"));
        assert!(html.contains("toggleMenu("));
        assert!(html.contains("Realtime Validation"));
        assert!(html.contains("scheduleRealtimeValidation"));
        assert!(html.contains("createProject()"));
        assert!(html.contains("importProject()"));
        assert!(html.contains("source.viEditor.open"));
        assert!(html.contains("openViEditorTab"));
        assert!(html.contains("viKeydown"));
        assert!(html.contains("Open in Vi Editor"));
        assert!(html.contains("formatter-grid"));
        assert!(html.contains("formatterUploadFile"));
        assert!(html.contains("openFormatterFromActive"));
        assert!(html.contains("Open in Formatter"));
        assert!(html.contains("runTask()"));
        assert!(html.contains("detectProject()"));
        assert!(html.contains("Task Console"));
        assert!(html.contains("team.git.open"));
        assert!(html.contains("team.svn.open"));
        assert!(html.contains("runOneClickSetup"));
        assert!(html.contains("Verify And Install Dependencies"));
        assert!(html.contains("verifyDependencies"));
        assert!(html.contains("installSelectedDependency"));
        assert!(html.contains("startTerminalSession"));
        assert!(html.contains("runJdbcMetadata"));
        assert!(html.contains("runJdbcDbaProbe"));
        assert!(html.contains("retrainAiKnowledge"));
        assert!(html.contains("trainAiLocalModel"));
        assert!(html.contains("exportAiTrainingDataset"));
        assert!(html.contains("runAiTrainingJob"));
        assert!(html.contains("applyRenameRefactor"));
        assert!(html.contains("crossPlatformPackageAudit"));
        assert!(html.contains("runProductionReadiness"));
        assert!(html.contains("max-width: 100vw"));
        assert!(html.contains("repeat(auto-fit, minmax(240px, 1fr))"));
        assert!(
            html.contains("const maxLeft = Math.max(220, Math.floor(root.clientWidth * 0.42));")
        );
        assert!(html.contains("SFTP/SCP Transfer"));
        assert!(html.contains("createExtensionSkeleton"));
        assert!(html.contains("Workspace Indexer"));
        assert!(!html.contains("<details class=\"menu\""));
        assert!(!html.contains("Deeper implementation"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn every_menu_command_has_a_same_window_surface() {
        let script = build_module_script();
        for item in meditor_shell::default_menu_items() {
            if matches!(item.command, "source.reformat" | "source.viEditor.open") {
                continue;
            }
            assert!(
                script.contains(item.command),
                "missing module surface for {}",
                item.command
            );
        }
    }

    #[test]
    fn packaged_macos_launch_uses_writable_package_parent_instead_of_root() {
        let root = temp_workspace();
        let package_root = root.join("mEditor-package");
        let executable = package_root.join("mEditor.app/Contents/MacOS/mEditor");
        fs::create_dir_all(executable.parent().unwrap()).unwrap();

        assert_eq!(
            package_root_from_exe(&executable).as_deref(),
            Some(package_root.as_path())
        );

        #[cfg(not(target_os = "windows"))]
        {
            let selected = choose_startup_workspace_root(
                None,
                Some(PathBuf::from("/")),
                Some(executable.clone()),
            );
            assert_eq!(selected, package_root);
            assert!(selected.join(".meditor").is_dir());
        }

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn static_menu_items_execute_backend_workflows_and_write_artifacts() {
        let root = temp_workspace();
        for command in [
            "file.localHistoryRecovery.open",
            "file.workspaceBackupRestore.open",
            "tools.templatesSnippets.open",
            "tools.cicdGenerator.open",
            "tools.apiWorkbench.open",
            "tools.databaseMigration.open",
            "settings.workspaceTrust.open",
            "report.auditTrail.open",
        ] {
            let payload = format!(
                r#"{{"action":"moduleOpened","command":"{}","label":"Test"}}"#,
                command
            );
            let result = handle_ipc(&root, &payload);
            assert_eq!(result["action"], "genericActionResult");
            assert_eq!(result["ok"], true, "command failed: {command}");
            let messages = result["messages"].as_array().unwrap();
            assert!(
                messages.iter().any(|message| message
                    .as_str()
                    .unwrap_or("")
                    .contains("This menu item executed a Rust-backed workflow")),
                "no workflow marker for {command}"
            );
        }
        assert!(root.join(".meditor/local-history").exists());
        assert!(root.join(".meditor/backups").exists());
        assert!(root.join(".meditor/templates").exists());
        assert!(root.join(".meditor/cicd").exists());
        assert!(root.join(".meditor/api-workbench").exists());
        assert!(root.join(".meditor/db-migration").exists());
        assert!(root.join(".meditor/security").exists());
        assert!(root.join(".meditor/audit").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn project_create_import_report_and_reformat_are_backed_by_rust() {
        let root = temp_workspace();
        fs::write(
            root.join("Demo.sln"),
            "Microsoft Visual Studio Solution File",
        )
        .unwrap();

        let created = handle_ipc(
            &root,
            r#"{"action":"createProject","name":"Demo App","kind":"major","techStack":"Rust + SQL"}"#,
        );
        assert_eq!(created["action"], "genericActionResult");
        assert_eq!(created["ok"], true);
        assert!(root.join("Demo-App/.meditor/project/project.toml").exists());
        assert!(root
            .join("Demo-App/.meditor/project/docs/about.md")
            .exists());
        assert!(root.join("Demo-App/Cargo.toml").exists());
        assert!(root.join("Demo-App/src/main.rs").exists());

        let imported = handle_ipc(&root, r#"{"action":"importProject"}"#);
        assert_eq!(imported["action"], "genericActionResult");
        assert_eq!(imported["ok"], true);
        assert!(root.join(".meditor/project/import-report.toml").exists());

        let reformatted = handle_ipc(
            &root,
            r#"{"action":"reformatFile","path":"demo.json","kind":"json","content":"{\"b\":2,\"a\":1}"}"#,
        );
        assert_eq!(reformatted["action"], "reformatFileResult");
        assert_eq!(reformatted["ok"], true);
        assert!(reformatted["content"]
            .as_str()
            .unwrap()
            .contains("\"a\": 1"));

        let formatted = handle_ipc(
            &root,
            r#"{"action":"formatText","target":"formatter-test","kind":"xml","content":"<root><child>value</child></root>"}"#,
        );
        assert_eq!(formatted["action"], "formatTextResult");
        assert_eq!(formatted["ok"], true);
        assert_eq!(formatted["valid"], true);
        assert!(formatted["content"].as_str().unwrap().contains("<child>"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn sdlc_task_runner_detects_runs_and_writes_report() {
        let root = temp_workspace();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"demo_task_runner\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("src/main.rs"),
            "fn main() { println!(\"task runner ok\"); }\n",
        )
        .unwrap();

        let detected = handle_ipc(&root, r#"{"action":"detectProject","projectPath":"."}"#);
        assert_eq!(detected["action"], "detectProjectResult");
        assert_eq!(detected["ok"], true);
        assert_eq!(detected["projectKind"], "Rust/Cargo");

        let run = handle_ipc(
            &root,
            r#"{"action":"runTask","projectPath":".","taskAction":"debug","timeoutSeconds":120}"#,
        );
        assert_eq!(run["action"], "runTaskResult");
        assert_eq!(run["ok"], true);
        assert_eq!(run["success"], true);
        let report_path = run["reportPath"].as_str().unwrap();
        assert!(report_path.starts_with(".meditor/debug-reports/"));
        assert!(root.join(report_path).exists());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn local_pre_pilot_workflows_persist_state() {
        let root = temp_workspace();
        fs::write(root.join("sample.rs"), "fn main() {}\n").unwrap();

        let setup = handle_ipc(&root, r#"{"action":"oneClickSetup"}"#);
        assert_eq!(setup["action"], "genericActionResult");
        assert_eq!(setup["ok"], true);
        assert!(root
            .join(".meditor/security/init_meditor_sqlite.sql")
            .exists());

        let dependencies = handle_ipc(&root, r#"{"action":"verifyDependencies"}"#);
        assert_eq!(dependencies["action"], "dependencyVerifyResult");
        assert_eq!(dependencies["ok"], true);
        assert!(dependencies["records"].as_array().unwrap().len() > 10);

        let bug = handle_ipc(
            &root,
            r#"{"action":"saveLocalBug","title":"Demo bug","severity":"High","details":"Steps"}"#,
        );
        assert_eq!(bug["action"], "genericActionResult");
        assert_eq!(bug["ok"], true);

        let bugs = handle_ipc(&root, r#"{"action":"listLocalBugs"}"#);
        assert_eq!(bugs["action"], "localBugsResult");
        assert_eq!(bugs["ok"], true);
        assert_eq!(bugs["records"].as_array().unwrap().len(), 1);

        let planner = handle_ipc(
            &root,
            r#"{"action":"savePlannerItem","title":"Milestone 1","model":"objective","status":"Planned"}"#,
        );
        assert_eq!(planner["ok"], true);

        let knowledge = handle_ipc(
            &root,
            r#"{"action":"saveAiKnowledge","title":"Rust Notes","content":"Ownership and borrowing"}"#,
        );
        assert_eq!(knowledge["ok"], true);
        let retrain = handle_ipc(&root, r#"{"action":"retrainAiKnowledge"}"#);
        assert_eq!(retrain["ok"], true);
        assert!(root
            .join(".meditor/ai/model/local-knowledge-index.json")
            .exists());
        let trained = handle_ipc(&root, r#"{"action":"trainAiLocalModel"}"#);
        assert_eq!(trained["ok"], true);
        assert!(root
            .join(".meditor/ai/model/local-trained-model.json")
            .exists());
        assert!(root
            .join(".meditor/ai/training/training-report.md")
            .exists());
        let dataset = handle_ipc(&root, r#"{"action":"exportAiTrainingDataset"}"#);
        assert_eq!(dataset["ok"], true);
        assert!(root
            .join(".meditor/ai/training/fine-tune-dataset.jsonl")
            .exists());
        let trainer_config = handle_ipc(
            &root,
            r#"{"action":"saveAiTrainerConfig","executable":"rustc","args":"--version"}"#,
        );
        assert_eq!(trainer_config["ok"], true);
        assert!(root
            .join(".meditor/ai/training/external-trainer.json")
            .exists());

        let refactor = handle_ipc(
            &root,
            r#"{"action":"applyRenameRefactor","path":"sample.rs","from":"main","to":"entrypoint"}"#,
        );
        assert_eq!(refactor["ok"], true);
        assert!(fs::read_to_string(root.join("sample.rs"))
            .unwrap()
            .contains("entrypoint"));

        let extension = handle_ipc(&root, r#"{"action":"createExtensionSkeleton"}"#);
        assert_eq!(extension["ok"], true);
        assert!(root
            .join(".meditor/extensions/sample-extension/plugin.json")
            .exists());

        let index = handle_ipc(&root, r#"{"action":"scanWorkspaceIndex"}"#);
        assert_eq!(index["ok"], true);
        assert!(root.join(".meditor/index/workspace-index.json").exists());

        let reports = handle_ipc(&root, r#"{"action":"listReports"}"#);
        assert_eq!(reports["action"], "reportListResult");
        assert_eq!(reports["ok"], true);

        let readiness = handle_ipc(&root, r#"{"action":"productionReadiness"}"#);
        assert_eq!(readiness["action"], "genericActionResult");
        assert_eq!(readiness["ok"], true);
        assert!(root
            .join(".meditor/production-readiness/production-readiness-report.json")
            .exists());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn production_hardening_helpers_are_wired() {
        assert!(jdbc_probe_sql("generic_version")
            .unwrap()
            .contains("meditor_connection_probe"));
        assert!(jdbc_probe_sql("oracle_rman_backup")
            .unwrap()
            .contains("v$rman_backup_job_details"));
        assert!(jdbc_probe_sql("postgres_activity")
            .unwrap()
            .contains("pg_stat_activity"));
        assert!(jdbc_probe_sql("unknown").is_none());

        let local_terminal = terminal_command("", "", true)
            .expect("local terminal command should resolve to the local shell or wrapper");
        assert!(!local_terminal.0.is_empty());
        assert!(!local_terminal.1.is_empty());
    }

    #[test]
    fn ipc_blocks_path_traversal_and_acknowledges_modules() {
        let root = temp_workspace();
        let blocked = handle_ipc(&root, r#"{"action":"openFile","path":"../outside.txt"}"#);
        assert_eq!(blocked["action"], "openFileResult");
        assert_eq!(blocked["ok"], false);

        let module = handle_ipc(
            &root,
            r#"{"action":"moduleOpened","command":"tools.dbaWorkshop.open","label":"DBA Workshop"}"#,
        );
        assert_eq!(module["action"], "genericActionResult");
        assert_eq!(module["ok"], true);
        assert_eq!(module["target"], "tools.dbaWorkshop.open");

        fs::remove_dir_all(root).unwrap();
    }

    fn temp_workspace() -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let root =
            std::env::temp_dir().join(format!("meditor-gui-test-{}-{id}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }
}
