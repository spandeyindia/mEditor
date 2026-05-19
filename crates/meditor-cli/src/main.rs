use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, ExitCode, Stdio};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("scan-file") => {
            let Some(path) = args.next() else {
                eprintln!("usage: meditor scan-file <path>");
                return ExitCode::from(2);
            };
            scan_file(&path)
        }
        Some("cvss-schema") => {
            println!("{}", meditor_cvss_repository::schema_sql());
            ExitCode::SUCCESS
        }
        Some("sqlite-setup") => {
            print_sqlite_setup();
            ExitCode::SUCCESS
        }
        Some("language-support") => {
            print_language_support();
            ExitCode::SUCCESS
        }
        Some("compiler-debugger") => {
            print_compiler_debugger();
            ExitCode::SUCCESS
        }
        Some("doc-generator") => {
            print_doc_generator();
            ExitCode::SUCCESS
        }
        Some("uml-modeling") => {
            print_uml_modeling();
            ExitCode::SUCCESS
        }
        Some("local-bugs") => {
            print_local_bugs();
            ExitCode::SUCCESS
        }
        Some("project-planner") => {
            print_project_planner();
            ExitCode::SUCCESS
        }
        Some("about-help") => {
            print_about_help();
            ExitCode::SUCCESS
        }
        Some("init-sqlite") => {
            let working_folder = args.next().unwrap_or_else(|| ".".to_string());
            init_sqlite(&working_folder)
        }
        Some("help") | Some("--help") | Some("-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help();
            ExitCode::from(2)
        }
        None => {
            print_status();
            ExitCode::SUCCESS
        }
    }
}

fn print_status() {
    let platform = meditor_platform::PlatformProfile::current();
    let launcher_plan = meditor_packaging::GenericJarPackagingPlan::ver_1_default();
    let scanner = meditor_code_security::CodeSecurityScanner::with_seed_rules();
    let repo_layout = meditor_cvss_repository::RepositoryLayout::under_working_folder(".");
    let workbench = meditor_workbench::WorkbenchBaseline::pre_pilot_ver_2();
    let rest_config = meditor_installation_metrics::BackendEndpointConfig::oracle_apex_default();

    println!(
        "{} {}",
        meditor_core::PRODUCT_NAME,
        meditor_core::CURRENT_BASELINE_VERSION
    );
    println!("version schema: {}", meditor_core::VERSION_SCHEMA);
    println!("current platform: {:?} {:?}", platform.os, platform.arch);
    println!("Ver 2 laptop target: {}", platform.is_ver_1_laptop_target());
    println!(
        "default menu items: {}",
        meditor_shell::default_menu_items().len()
    );
    println!(
        "top-right menu bar items: {}",
        workbench.top_right_menu_bar_items
    );
    println!("workbench modules: {}", workbench.modules.len());
    println!(
        "same-window modules: {}",
        workbench.all_modules_same_window()
    );
    println!("editor buffer routes: {}", workbench.editor_buffer_routes);
    println!("DB worksheet actions: {}", workbench.db_worksheet_actions);
    println!("DB dashboard sections: {}", workbench.db_dashboard_sections);
    println!("SSH terminal actions: {}", workbench.ssh_terminal_actions);
    println!("AI knowledge formats: {}", workbench.ai_knowledge_formats);
    println!("AI capabilities: {}", workbench.ai_capabilities);
    println!("project import modes: {}", workbench.project_import_modes);
    println!("project node kinds: {}", workbench.project_node_kinds);
    println!("planning item kinds: {}", workbench.planning_item_kinds);
    println!(
        "project workspace steps: {}",
        workbench.project_workspace_steps
    );
    println!(
        "project planning steps: {}",
        workbench.project_planning_steps
    );
    println!(
        "embedded project documentation sections: {}",
        workbench.embedded_project_documentation_sections
    );
    println!("project help sources: {}", workbench.project_help_sources);
    println!("test actions: {}", workbench.test_actions);
    println!("coverage formats: {}", workbench.coverage_formats);
    println!(
        "dependency manifest kinds: {}",
        workbench.dependency_manifest_kinds
    );
    println!("dependency actions: {}", workbench.dependency_actions);
    println!(
        "plugin contribution points: {}",
        workbench.plugin_contribution_points
    );
    println!("registration statuses: {}", workbench.registration_statuses);
    println!(
        "license acceptance fields: {}",
        workbench.license_acceptance_fields
    );
    println!(
        "installation event types: {}",
        workbench.installation_event_types
    );
    println!("bug report fields: {}", workbench.bug_report_fields);
    println!("feedback form fields: {}", workbench.feedback_form_fields);
    println!(
        "automatic error capture fields: {}",
        workbench.automatic_error_capture_fields
    );
    println!("feedback statuses: {}", workbench.feedback_statuses);
    println!("user feedback metrics: {}", workbench.user_feedback_metrics);
    println!("APEX metric tables: {}", workbench.apex_metric_tables);
    println!(
        "APEX REST base URL: {}",
        rest_config.base_url.as_deref().unwrap_or("not configured")
    );
    println!(
        "APEX REST endpoint fields: {}",
        meditor_installation_metrics::default_backend_endpoint_values().len()
    );
    println!(
        "OpenAPI catalog configured: {}",
        rest_config.ready_for_openapi_reference()
    );
    println!(
        "first-run notice points: {}",
        workbench.first_run_notice_points
    );
    println!("VCS actions: {}", workbench.vcs_actions);
    println!("generic JAR launchers: {}", launcher_plan.launchers.len());
    println!("setup tasks: {}", workbench.setup_tasks);
    println!("update flow steps: {}", workbench.update_flow_steps);
    println!(
        "update preserved paths: {}",
        workbench.update_preserved_paths
    );
    println!(
        "language support catalog entries: {}",
        workbench.language_support_catalog_entries
    );
    println!(
        "language tooling install steps: {}",
        workbench.language_tooling_install_steps
    );
    println!(
        "language tooling network dialog fields: {}",
        workbench.language_tooling_network_dialog_fields
    );
    println!("SDLC actions: {}", meditor_sdlc::standard_actions().len());
    println!(
        "executable requirements: {}",
        meditor_toolchains::seed_executable_requirements().len()
    );
    println!(
        "compiler debugger cycle steps: {}",
        workbench.compiler_debugger_cycle_steps
    );
    println!(
        "compiler debugger audit sections: {}",
        workbench.compiler_debugger_audit_sections
    );
    println!(
        "deployment doc targets: {}",
        workbench.deployment_doc_targets
    );
    println!("deployment doc steps: {}", workbench.deployment_doc_steps);
    println!(
        "project documentation sections: {}",
        workbench.project_documentation_sections
    );
    println!(
        "flowchart generation steps: {}",
        workbench.flowchart_generation_steps
    );
    println!(
        "ERD supported source kinds: {}",
        workbench.erd_supported_source_kinds
    );
    println!("ERD generation steps: {}", workbench.erd_generation_steps);
    println!("UML diagram kinds: {}", workbench.uml_diagram_kinds);
    println!("UML text dialects: {}", workbench.uml_text_dialects);
    println!("UML workflow steps: {}", workbench.uml_workflow_steps);
    println!(
        "local bug repository fields: {}",
        workbench.local_bug_repository_fields
    );
    println!(
        "bug import/export steps: {}",
        workbench.bug_import_export_steps
    );
    println!(
        "report generation steps: {}",
        workbench.report_generation_steps
    );
    println!("report kinds: {}", workbench.report_kinds);
    println!(
        "self-learning AI service contract enabled: {}",
        workbench.self_learning_ai_service_enabled
    );
    println!("security seed rules: {}", scanner.rules().len());
    println!(
        "vulnerability repository: {}",
        repo_layout.vulnerability_db.display()
    );
    println!(
        "SQLite setup folder: {}",
        meditor_cvss_repository::SQLITE_SETUP_DIR
    );
}

fn print_help() {
    println!("mEditor command line");
    println!("  meditor                 Show current Ver 2 pre-pilot scaffold status");
    println!("  meditor scan-file PATH  Run seed Code Security Analyzer rules on one file");
    println!("  meditor cvss-schema     Print the local SQLite CVSS repository schema");
    println!("  meditor sqlite-setup    Show SQLite setup files and commands");
    println!("  meditor language-support");
    println!("                          Show supported languages and tooling readiness");
    println!("  meditor compiler-debugger");
    println!("                          Show compiler debugging, staging, web server, and AI learning contract");
    println!("  meditor doc-generator");
    println!("                          Show deploy, project documentation, flowchart, and ERD generation contract");
    println!("  meditor uml-modeling   Show UML text/live preview and GUI round-trip contract");
    println!("  meditor local-bugs     Show local-only user bug repository and report contract");
    println!("  meditor project-planner");
    println!("                          Show major project, sub-project, portfolio planning, and planner-to-project contract");
    println!("  meditor about-help      Show embedded About Project and Project Help model");
    println!("  meditor init-sqlite [WORKING_FOLDER]");
    println!("                          Create SQLite data structures using sqlite3 on PATH");
}

fn print_about_help() {
    let docs = meditor_project_workspace::EmbeddedProjectDocumentationModel::default_model();

    println!("mEditor Embedded Project About And Help");
    println!("about menu: {}", docs.about_menu_path);
    println!("help menu: {}", docs.help_menu_path);
    println!("docs folder: {}", docs.docs_folder);
    println!("about file: {}", docs.about_file);
    println!("help index: {}", docs.help_index_file);
    println!("same-window tab: {}", docs.opens_in_same_window_tab);
    println!(
        "generated for every project: {}",
        docs.generated_for_every_project
    );
    println!("editable by user: {}", docs.editable_by_user);
    println!("documentation sections:");
    for section in meditor_project_workspace::embedded_project_documentation_sections() {
        println!("  {section}");
    }
    println!("help sources:");
    for source in meditor_project_workspace::project_help_sources() {
        println!("  {source}");
    }
}

fn print_project_planner() {
    let workspace = meditor_project_workspace::ProjectWorkspaceModel::default_model();
    let planner = meditor_project_workspace::ProjectPlanningModel::default_model();
    let metadata = meditor_project_workspace::ProjectMetadataPlan::netbeans_style_for_meditor();
    println!("mEditor Project Workspace And Project Planner");
    println!("project creation menu: {}", workspace.menu_path);
    println!("planner menu: {}", planner.menu_path);
    println!("metadata folder: {}", metadata.metadata_folder);
    println!("project metadata file: {}", metadata.project_file);
    println!("project about file: {}", metadata.about_file);
    println!("project help index: {}", metadata.help_index_file);
    println!("supports subprojects: {}", metadata.supports_subprojects);
    println!(
        "supports portfolio planning: {}",
        metadata.supports_portfolio_planning
    );
    println!("planning item kinds: {}", planner.supported_items.len());
    println!(
        "planner can create editor project action: {}",
        planner.create_editor_project_action
    );
    println!("workspace steps:");
    for step in meditor_project_workspace::project_workspace_steps() {
        println!("  {step}");
    }
    println!("planning steps:");
    for step in meditor_project_workspace::project_planning_steps() {
        println!("  {step}");
    }
}

fn print_uml_modeling() {
    let surface = meditor_uml_modeling::UmlModelingSurface::default_surface();
    let gui = meditor_uml_modeling::UmlGuiModeler::default_modeler();
    println!("mEditor UML Modeling");
    println!("menu: {}", surface.menu_path);
    println!("default mode: {:?}", surface.default_mode);
    println!("diagram kinds: {}", surface.supported_diagrams.len());
    println!("text dialects: {}", surface.supported_text_dialects.len());
    println!("storage folder: {}", surface.storage_folder);
    println!("GUI palette items: {}", gui.palette_items.len());
    println!("workflow steps:");
    for step in meditor_uml_modeling::uml_modeling_workflow_steps() {
        println!("  {step}");
    }
}

fn print_local_bugs() {
    let repository = meditor_local_bug_repository::LocalBugRepository::default_repository();
    let report_surface = meditor_local_bug_repository::ReportMenuSurface::default_surface();
    println!("mEditor Local Bug Repository");
    println!("storage: {}", repository.storage_path);
    println!(
        "local only, never auto shipped: {}",
        repository.never_auto_ships_user_program_bugs()
    );
    println!(
        "fields: {}",
        meditor_local_bug_repository::local_bug_repository_fields().len()
    );
    println!("import/export steps:");
    for step in meditor_local_bug_repository::bug_repository_import_export_steps() {
        println!("  {step}");
    }
    println!("report menu: {}", report_surface.menu);
    println!("report kinds: {}", report_surface.report_kinds.len());
    println!("report generation steps:");
    for step in meditor_local_bug_repository::report_generation_steps() {
        println!("  {step}");
    }
}

fn print_doc_generator() {
    println!("mEditor Documentation Generator");
    println!(
        "deployment targets: {}",
        meditor_deployment_docs::supported_deployment_targets().len()
    );
    println!(
        "deployment templates: {}",
        meditor_deployment_docs::seed_stack_templates().len()
    );
    println!("deployment generation steps:");
    for step in meditor_deployment_docs::deployment_doc_generation_steps() {
        println!("  {step}");
    }
    println!("project documentation sections:");
    for section in meditor_deployment_docs::project_documentation_sections() {
        println!("  {section}");
    }
    println!("flowchart generation steps:");
    for step in meditor_deployment_docs::flowchart_generation_steps() {
        println!("  {step}");
    }
    println!("ERD source kinds:");
    for source_kind in meditor_deployment_docs::erd_supported_source_kinds() {
        println!("  {:?}", source_kind);
    }
    println!("language-agnostic program documentation languages:");
    for language in meditor_deployment_docs::javadoc_style_language_coverage() {
        println!("  {language}");
    }
}

fn print_language_support() {
    let detected = meditor_toolchains::detect_executables_on_path();
    let statuses = meditor_toolchains::language_support_statuses_from_detected_ids(&detected);
    let active = statuses.iter().filter(|status| status.is_active()).count();
    let missing = statuses.len() - active;
    let dialog = meditor_toolchains::default_tooling_network_dialog();

    println!("mEditor Programming Language Support");
    println!("menu: Setup > Programming Language Support");
    println!("supported entries: {}", statuses.len());
    println!("active entries: {active}");
    println!("missing-tooling entries: {missing}");
    println!("detected executables: {}", detected.len());
    println!("install flow steps:");
    for step in meditor_toolchains::language_tooling_install_flow_steps() {
        println!("  {step}");
    }
    println!("network/proxy dialog: {}", dialog.title);
    println!("network/proxy fields: {}", dialog.fields.len());
    println!("sample supported entries:");
    for status in statuses.iter().take(20) {
        println!(
            "  {}: {:?} missing={:?}",
            status.display_name, status.state, status.missing_required_executables
        );
    }
}

fn print_compiler_debugger() {
    let module = meditor_compiler_debugger::CompilerDebuggerModule::default_module();
    let ai_plan = meditor_ai::SelfLearningAiServicePlan::ver_1_contract();

    println!("mEditor Compiler Error Debugger");
    println!("module: {}", module.label);
    println!("default mode: {:?}", module.default_policy.mode);
    println!("max iterations: {}", module.default_policy.max_iterations);
    println!("staging area: {}", module.staging_area.root_relative_path);
    println!(
        "interpreter terminal screen capture: {}",
        module.terminal_run.capture_screen_transcript
    );
    println!(
        "embedded web server: http://{}:{}",
        module.embedded_web_server.bind_host, module.embedded_web_server.preferred_port
    );
    println!(
        "web debug captures browser console: {}",
        module.embedded_web_server.capture_browser_console
    );
    println!("debug cycle steps:");
    for step in meditor_compiler_debugger::compiler_debugger_cycle_steps() {
        println!("  {step}");
    }
    println!("audit document sections:");
    for section in meditor_compiler_debugger::debugging_audit_document_sections() {
        println!("  {section}");
    }
    println!(
        "self-learning AI service contract enabled: {}",
        ai_plan.embedded_service
    );
    println!(
        "retrain after debug cycle: {}",
        ai_plan.retrain_after_debug_cycle
    );
}

fn scan_file(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("unable to read '{path}': {error}");
            return ExitCode::from(1);
        }
    };
    let scanner = meditor_code_security::CodeSecurityScanner::with_seed_rules();
    let findings = scanner.scan_text(path, &source);
    if findings.is_empty() {
        println!("No findings in {path}");
        return ExitCode::SUCCESS;
    }

    for finding in &findings {
        println!(
            "{}:{} [{} CVSS {:.1} {}] {}",
            finding.file_path,
            finding.line_number,
            finding.severity,
            finding.cvss_score,
            finding.cwe,
            finding.summary
        );
        println!("  snippet: {}", finding.snippet);
        println!("  remediation: {}", finding.remediation);
    }
    ExitCode::from(3)
}

fn print_sqlite_setup() {
    println!("mEditor SQLite setup");
    println!("one-click setup folder: setup");
    println!("registered files:");
    for file in meditor_cvss_repository::sqlite_setup_files() {
        println!("  {file}");
    }
    println!("one-click setup:");
    println!("  macOS Finder: double-click setup/setup.command");
    println!("  Linux: run ./setup/setup.sh");
    println!("  Windows Explorer: double-click setup\\setup.cmd");
    println!("macOS/Linux:");
    println!("  ./setup/sqlite/setup-sqlite.sh --check");
    println!("  ./setup/sqlite/setup-sqlite.sh --install");
    println!("  ./setup/sqlite/setup-sqlite.sh --init-db --working-folder /path/to/workspace");
    println!("Windows:");
    println!("  .\\setup\\sqlite\\setup-sqlite.ps1 -Check");
    println!("  .\\setup\\sqlite\\setup-sqlite.ps1 -Install");
    println!("  .\\setup\\sqlite\\setup-sqlite.ps1 -InitDb -WorkingFolder C:\\path\\to\\workspace");
}

fn init_sqlite(working_folder: &str) -> ExitCode {
    let layout = meditor_cvss_repository::RepositoryLayout::under_working_folder(working_folder);
    if let Err(error) = fs::create_dir_all(&layout.security_dir) {
        eprintln!(
            "unable to create security directory '{}': {error}",
            layout.security_dir.display()
        );
        return ExitCode::from(1);
    }

    let mut child = match Command::new("sqlite3")
        .arg(&layout.vulnerability_db)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            eprintln!("unable to start sqlite3: {error}");
            eprintln!("run 'meditor sqlite-setup' for installation options");
            return ExitCode::from(1);
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(error) = stdin.write_all(meditor_cvss_repository::schema_sql().as_bytes()) {
            eprintln!("unable to send schema to sqlite3: {error}");
            return ExitCode::from(1);
        }
    }

    match child.wait() {
        Ok(status) if status.success() => {
            println!(
                "Initialized mEditor SQLite repository: {}",
                layout.vulnerability_db.display()
            );
            ExitCode::SUCCESS
        }
        Ok(status) => {
            eprintln!("sqlite3 exited with status: {status}");
            ExitCode::from(1)
        }
        Err(error) => {
            eprintln!("unable to wait for sqlite3: {error}");
            ExitCode::from(1)
        }
    }
}
