#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebuggingMode {
    AutoImplementUntilClean,
    UserControlled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionKind {
    CompiledProgram,
    InterpreterTerminal,
    EmbeddedWebServer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuggestionRisk {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompilerDiagnostic {
    pub severity: DiagnosticSeverity,
    pub file_path: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub code: Option<String>,
    pub message: String,
    pub raw_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixSuggestion {
    pub id: String,
    pub title: String,
    pub rationale: String,
    pub probable_impact: String,
    pub can_auto_apply: bool,
    pub risk: SuggestionRisk,
    pub target_file: Option<String>,
}

impl FixSuggestion {
    pub fn safe_auto_fix(
        id: impl Into<String>,
        title: impl Into<String>,
        rationale: impl Into<String>,
        probable_impact: impl Into<String>,
        target_file: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            rationale: rationale.into(),
            probable_impact: probable_impact.into(),
            can_auto_apply: true,
            risk: SuggestionRisk::Low,
            target_file: Some(target_file.into()),
        }
    }

    pub fn needs_user_control(&self) -> bool {
        !self.can_auto_apply || self.risk == SuggestionRisk::High
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StagingAreaPlan {
    pub root_relative_path: &'static str,
    pub copy_source_tree: bool,
    pub isolate_build_outputs: bool,
    pub preserve_original_sources_until_acceptance: bool,
    pub compiled_output_dir: &'static str,
    pub interpreter_work_dir: &'static str,
    pub transcript_dir: &'static str,
}

impl StagingAreaPlan {
    pub fn default_for_debug_cycle() -> Self {
        Self {
            root_relative_path: ".meditor/debug-staging",
            copy_source_tree: true,
            isolate_build_outputs: true,
            preserve_original_sources_until_acceptance: true,
            compiled_output_dir: "compiled",
            interpreter_work_dir: "terminal-runs",
            transcript_dir: "transcripts",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalTerminalRunPlan {
    pub terminal_command: &'static str,
    pub capture_stdout: bool,
    pub capture_stderr: bool,
    pub capture_screen_transcript: bool,
    pub attach_to_debug_cycle: bool,
}

impl LocalTerminalRunPlan {
    pub fn interpreter_default() -> Self {
        Self {
            terminal_command: "debugger.terminal.runInStagingArea",
            capture_stdout: true,
            capture_stderr: true,
            capture_screen_transcript: true,
            attach_to_debug_cycle: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedWebServerPlan {
    pub server_id: &'static str,
    pub bind_host: &'static str,
    pub preferred_port: u16,
    pub fallback_port_range: (u16, u16),
    pub document_root_relative_path: &'static str,
    pub supports_static_files: bool,
    pub supports_proxy_to_framework_dev_server: bool,
    pub capture_http_requests: bool,
    pub capture_browser_console: bool,
    pub capture_server_logs: bool,
    pub debug_url_action: &'static str,
}

impl EmbeddedWebServerPlan {
    pub fn default_for_web_debugging() -> Self {
        Self {
            server_id: "debugger.embeddedWebServer",
            bind_host: "127.0.0.1",
            preferred_port: 51730,
            fallback_port_range: (51731, 51830),
            document_root_relative_path: ".meditor/debug-staging/web-root",
            supports_static_files: true,
            supports_proxy_to_framework_dev_server: true,
            capture_http_requests: true,
            capture_browser_console: true,
            capture_server_logs: true,
            debug_url_action: "debugger.web.openInEmbeddedBrowser",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramExecutionCapture {
    pub execution_kind: ExecutionKind,
    pub command_line: String,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub screen_transcript: Option<String>,
    pub web_url: Option<String>,
}

impl ProgramExecutionCapture {
    pub fn has_debug_output(&self) -> bool {
        !self.stdout.is_empty()
            || !self.stderr.is_empty()
            || self
                .screen_transcript
                .as_deref()
                .is_some_and(|value| !value.is_empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugCyclePolicy {
    pub mode: DebuggingMode,
    pub max_iterations: u32,
    pub stop_when_zero_errors_and_warnings: bool,
    pub rerun_after_auto_fix: bool,
    pub user_is_master: bool,
    pub require_user_approval_before_every_change: bool,
    pub allow_user_to_enable_auto_debugging: bool,
    pub require_user_confirmation_for_medium_or_high_risk: bool,
    pub require_rollback_for_every_change: bool,
    pub retrain_ai_after_each_cycle: bool,
}

impl DebugCyclePolicy {
    pub fn user_master_default() -> Self {
        Self {
            mode: DebuggingMode::UserControlled,
            max_iterations: 1,
            stop_when_zero_errors_and_warnings: false,
            rerun_after_auto_fix: false,
            user_is_master: true,
            require_user_approval_before_every_change: true,
            allow_user_to_enable_auto_debugging: true,
            require_user_confirmation_for_medium_or_high_risk: true,
            require_rollback_for_every_change: true,
            retrain_ai_after_each_cycle: true,
        }
    }

    pub fn autonomous_until_clean() -> Self {
        Self {
            mode: DebuggingMode::AutoImplementUntilClean,
            max_iterations: 10,
            stop_when_zero_errors_and_warnings: true,
            rerun_after_auto_fix: true,
            user_is_master: true,
            require_user_approval_before_every_change: false,
            allow_user_to_enable_auto_debugging: true,
            require_user_confirmation_for_medium_or_high_risk: true,
            require_rollback_for_every_change: true,
            retrain_ai_after_each_cycle: true,
        }
    }

    pub fn user_controlled() -> Self {
        Self::user_master_default()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RollbackRecord {
    pub id: String,
    pub suggestion_id: String,
    pub target_file: String,
    pub before_snapshot_path: String,
    pub after_snapshot_path: String,
    pub rollback_action: &'static str,
}

impl RollbackRecord {
    pub fn can_restore_user_state(&self) -> bool {
        !self.before_snapshot_path.is_empty() && !self.rollback_action.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugCycleResult {
    pub iteration: u32,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub physical_program: String,
    pub diagnostics: Vec<CompilerDiagnostic>,
    pub suggestions: Vec<FixSuggestion>,
    pub applied_fix_ids: Vec<String>,
    pub skipped_fix_ids: Vec<String>,
    pub rollback_records: Vec<RollbackRecord>,
    pub local_bug_entry: Option<meditor_local_bug_repository::LocalBugEntry>,
    pub execution_capture: Option<ProgramExecutionCapture>,
    pub learning_event: meditor_ai::AiLearningEvent,
}

impl DebugCycleResult {
    pub fn is_clean(&self) -> bool {
        self.diagnostics
            .iter()
            .all(|diagnostic| diagnostic.severity == DiagnosticSeverity::Info)
    }

    pub fn should_continue(&self, policy: &DebugCyclePolicy) -> bool {
        !self.is_clean()
            && policy.mode == DebuggingMode::AutoImplementUntilClean
            && self.iteration < policy.max_iterations
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebuggingAuditDocument {
    pub id: String,
    pub title: String,
    pub workspace_root: String,
    pub report_path: String,
    pub generated_at: String,
    pub cycle_results: Vec<DebugCycleResult>,
    pub handover_to_user: bool,
}

impl DebuggingAuditDocument {
    pub fn new(
        id: impl Into<String>,
        workspace_root: impl Into<String>,
        report_path: impl Into<String>,
        generated_at: impl Into<String>,
        cycle_results: Vec<DebugCycleResult>,
    ) -> Self {
        Self {
            id: id.into(),
            title: "mEditor Compiler Debugging Report".to_string(),
            workspace_root: workspace_root.into(),
            report_path: report_path.into(),
            generated_at: generated_at.into(),
            cycle_results,
            handover_to_user: true,
        }
    }

    pub fn contains_timelines_and_physical_programs(&self) -> bool {
        self.cycle_results.iter().all(|cycle| {
            !cycle.physical_program.is_empty()
                && cycle.started_at.is_some()
                && cycle.finished_at.is_some()
        })
    }
}

pub fn debugging_audit_document_sections() -> &'static [&'static str] {
    &[
        "Debugging session summary",
        "Physical programs and files touched",
        "Timeline by debugging cycle",
        "Compiler errors and warnings captured",
        "Suggestions with rationale and probable impact",
        "Fixes applied automatically",
        "Fixes applied manually or skipped by user",
        "Rollback records for every applied change",
        "Compiled program, interpreter terminal, or web-server execution outputs",
        "AI/ML learning events recorded after each cycle",
        "Final zero-error and zero-warning status",
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompilerDebuggerModule {
    pub id: &'static str,
    pub label: &'static str,
    pub launch_build_action: &'static str,
    pub capture_compiler_output_action: &'static str,
    pub suggest_fixes_action: &'static str,
    pub apply_auto_fixes_action: &'static str,
    pub rerun_cycle_action: &'static str,
    pub user_apply_fix_action: &'static str,
    pub staging_area: StagingAreaPlan,
    pub terminal_run: LocalTerminalRunPlan,
    pub embedded_web_server: EmbeddedWebServerPlan,
    pub default_policy: DebugCyclePolicy,
}

impl CompilerDebuggerModule {
    pub fn default_module() -> Self {
        Self {
            id: "compilerDebugger",
            label: "Compiler Error Debugger",
            launch_build_action: "debugger.compiler.launchBuild",
            capture_compiler_output_action: "debugger.compiler.captureOutput",
            suggest_fixes_action: "debugger.compiler.suggestFixes",
            apply_auto_fixes_action: "debugger.compiler.applyAutoFixes",
            rerun_cycle_action: "debugger.compiler.rerunUntilClean",
            user_apply_fix_action: "debugger.compiler.userApplyFix",
            staging_area: StagingAreaPlan::default_for_debug_cycle(),
            terminal_run: LocalTerminalRunPlan::interpreter_default(),
            embedded_web_server: EmbeddedWebServerPlan::default_for_web_debugging(),
            default_policy: DebugCyclePolicy::user_master_default(),
        }
    }
}

pub fn compiler_debugger_cycle_steps() -> &'static [&'static str] {
    &[
        "Create an isolated staging area and copy the source snapshot into it.",
        "Launch the configured build, compiler, interpreter, test, or web command inside the staging area.",
        "Capture compiler diagnostics, stdout, stderr, local terminal transcript, web-server logs, and browser console output when available.",
        "Generate fix suggestions with rationale, probable impact, target files, and auto-apply safety classification.",
        "Default to user-controlled mode: the user is the master and must approve every source change.",
        "When the user explicitly enables automatic debugging, apply only permitted fixes, keep rollback records, rerun the cycle, and stop only when errors and warnings are zero or the iteration limit is reached.",
        "In user-controlled mode, present suggestions and let the user apply, edit, skip, rollback, or rerun each step.",
        "After every cycle, write a learning event for the embedded AI/ML service so the model can adapt from diagnostics, fixes, and outcomes.",
        "Generate and hand over a debugging audit document with timelines, physical programs, diagnostics, rationale, impact, fixes, and execution evidence.",
    ]
}

pub fn parse_seed_diagnostics(raw_output: &str) -> Vec<CompilerDiagnostic> {
    raw_output
        .lines()
        .filter(|line| line.contains("error") || line.contains("warning"))
        .map(|line| {
            let severity = if line.contains("warning") {
                DiagnosticSeverity::Warning
            } else {
                DiagnosticSeverity::Error
            };
            CompilerDiagnostic {
                severity,
                file_path: extract_file_hint(line).unwrap_or_else(|| "unknown".to_string()),
                line: None,
                column: None,
                code: None,
                message: line.trim().to_string(),
                raw_text: line.to_string(),
            }
        })
        .collect()
}

pub fn seed_suggestions_for_diagnostics(diagnostics: &[CompilerDiagnostic]) -> Vec<FixSuggestion> {
    diagnostics
        .iter()
        .enumerate()
        .map(|(index, diagnostic)| {
            FixSuggestion::safe_auto_fix(
                format!("compiler-fix-{}", index + 1),
                format!("Resolve {}", diagnostic.message),
                "The compiler reported this diagnostic during the staged build, so the fix targets the exact failing build feedback rather than guessing from source alone.",
                "Expected impact is a narrower follow-up build with this diagnostic removed or replaced by a more specific compiler message.",
                diagnostic.file_path.clone(),
            )
        })
        .collect()
}

fn extract_file_hint(line: &str) -> Option<String> {
    let (file, _) = line.split_once(':')?;
    (!file.trim().is_empty()).then(|| file.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_defines_staging_terminal_and_web_debugging_surfaces() {
        let module = CompilerDebuggerModule::default_module();
        assert_eq!(
            module.staging_area.root_relative_path,
            ".meditor/debug-staging"
        );
        assert!(
            module
                .staging_area
                .preserve_original_sources_until_acceptance
        );
        assert!(module.terminal_run.capture_screen_transcript);
        assert_eq!(module.embedded_web_server.bind_host, "127.0.0.1");
        assert!(module.embedded_web_server.supports_static_files);
        assert!(
            module
                .embedded_web_server
                .supports_proxy_to_framework_dev_server
        );
        assert!(module.embedded_web_server.capture_browser_console);
    }

    #[test]
    fn user_master_policy_is_the_default_and_requires_rollback() {
        let module = CompilerDebuggerModule::default_module();
        let policy = module.default_policy;
        assert_eq!(policy.mode, DebuggingMode::UserControlled);
        assert!(policy.user_is_master);
        assert!(policy.require_user_approval_before_every_change);
        assert!(policy.allow_user_to_enable_auto_debugging);
        assert!(policy.require_rollback_for_every_change);
        assert!(policy.retrain_ai_after_each_cycle);
    }

    #[test]
    fn autonomous_policy_runs_until_clean_after_user_enables_it() {
        let policy = DebugCyclePolicy::autonomous_until_clean();
        assert_eq!(policy.mode, DebuggingMode::AutoImplementUntilClean);
        assert!(policy.user_is_master);
        assert!(policy.rerun_after_auto_fix);
        assert!(policy.stop_when_zero_errors_and_warnings);
        assert!(policy.require_rollback_for_every_change);
    }

    #[test]
    fn suggestions_include_rationale_and_impact() {
        let diagnostics = parse_seed_diagnostics("src/main.rs: error: missing semicolon");
        let suggestions = seed_suggestions_for_diagnostics(&diagnostics);
        assert_eq!(suggestions.len(), 1);
        assert!(!suggestions[0].rationale.is_empty());
        assert!(!suggestions[0].probable_impact.is_empty());
        assert!(suggestions[0].can_auto_apply);
    }

    #[test]
    fn cycle_continues_only_in_auto_mode_until_clean() {
        let diagnostics = parse_seed_diagnostics("app.py: warning: unused variable");
        let result = DebugCycleResult {
            iteration: 1,
            started_at: Some("2026-05-19T10:00:00Z".to_string()),
            finished_at: Some("2026-05-19T10:00:15Z".to_string()),
            physical_program: "/workspace/app.py".to_string(),
            diagnostics,
            suggestions: Vec::new(),
            applied_fix_ids: Vec::new(),
            skipped_fix_ids: Vec::new(),
            rollback_records: Vec::new(),
            local_bug_entry: Some(
                meditor_local_bug_repository::LocalBugEntry::from_debugging_cycle(
                    "bug-1",
                    "Python warning",
                    "/workspace/app.py",
                    "unused variable",
                    "Review the variable use or remove it",
                    "rollback-1",
                    "2026-05-19T10:00:16Z",
                ),
            ),
            execution_capture: Some(ProgramExecutionCapture {
                execution_kind: ExecutionKind::InterpreterTerminal,
                command_line: "python app.py".to_string(),
                exit_code: Some(0),
                stdout: "hello".to_string(),
                stderr: String::new(),
                screen_transcript: Some("hello".to_string()),
                web_url: None,
            }),
            learning_event: meditor_ai::AiLearningEvent::debug_cycle(
                "cycle-1", "python", 0, 1, true,
            ),
        };
        assert!(result
            .execution_capture
            .as_ref()
            .unwrap()
            .has_debug_output());
        assert!(result.should_continue(&DebugCyclePolicy::autonomous_until_clean()));
        assert!(!result.should_continue(&DebugCyclePolicy::user_controlled()));
    }

    #[test]
    fn cycle_steps_include_ai_learning_after_every_cycle() {
        assert!(compiler_debugger_cycle_steps()
            .iter()
            .any(|step| step.contains("After every cycle")));
    }

    #[test]
    fn audit_document_records_timelines_programs_and_fix_evidence() {
        let diagnostics = parse_seed_diagnostics("src/main.rs: error: missing semicolon");
        let suggestions = seed_suggestions_for_diagnostics(&diagnostics);
        let result = DebugCycleResult {
            iteration: 1,
            started_at: Some("2026-05-19T10:00:00Z".to_string()),
            finished_at: Some("2026-05-19T10:00:30Z".to_string()),
            physical_program: "/workspace/src/main.rs".to_string(),
            diagnostics,
            suggestions,
            applied_fix_ids: vec!["compiler-fix-1".to_string()],
            skipped_fix_ids: Vec::new(),
            rollback_records: vec![RollbackRecord {
                id: "rollback-1".to_string(),
                suggestion_id: "compiler-fix-1".to_string(),
                target_file: "/workspace/src/main.rs".to_string(),
                before_snapshot_path: ".meditor/debug-staging/snapshots/main.rs.before".to_string(),
                after_snapshot_path: ".meditor/debug-staging/snapshots/main.rs.after".to_string(),
                rollback_action: "debugger.compiler.rollbackChange",
            }],
            local_bug_entry: Some(
                meditor_local_bug_repository::LocalBugEntry::from_debugging_cycle(
                    "bug-2",
                    "Compiler failure",
                    "/workspace/src/main.rs",
                    "missing semicolon",
                    "Add semicolon at end of expression",
                    "rollback-1",
                    "2026-05-19T10:00:31Z",
                ),
            ),
            execution_capture: None,
            learning_event: meditor_ai::AiLearningEvent::debug_cycle("cycle-1", "rust", 1, 0, true),
        };
        let document = DebuggingAuditDocument::new(
            "debug-report-1",
            "/workspace",
            ".meditor/debug-reports/debug-report-1.md",
            "2026-05-19T10:00:31Z",
            vec![result],
        );
        assert!(document.handover_to_user);
        assert!(document.contains_timelines_and_physical_programs());
        assert!(document.cycle_results[0].rollback_records[0].can_restore_user_state());
        assert!(document.cycle_results[0].local_bug_entry.is_some());
        assert!(debugging_audit_document_sections()
            .contains(&"Suggestions with rationale and probable impact"));
        assert!(
            debugging_audit_document_sections().contains(&"Physical programs and files touched")
        );
    }
}
