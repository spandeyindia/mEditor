#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalBugSource {
    UserEntered,
    CompilerDebugger,
    RuntimeDebugger,
    SecurityAnalyzer,
    TestRunner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalBugStatus {
    Open,
    InProgress,
    Fixed,
    Verified,
    Deferred,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalBugSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BugRepositorySharingMode {
    LocalOnlyNeverAutoShip,
    UserExportedArchive,
    UserImportedArchive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBugEntry {
    pub id: String,
    pub title: String,
    pub source: LocalBugSource,
    pub status: LocalBugStatus,
    pub severity: LocalBugSeverity,
    pub physical_program: Option<String>,
    pub diagnostic_text: Option<String>,
    pub reproduction_steps: Option<String>,
    pub ai_suggestion_summary: Option<String>,
    pub applied_fix_summary: Option<String>,
    pub rollback_reference: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl LocalBugEntry {
    pub fn user_entered(
        id: impl Into<String>,
        title: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let created_at = created_at.into();
        Self {
            id: id.into(),
            title: title.into(),
            source: LocalBugSource::UserEntered,
            status: LocalBugStatus::Open,
            severity: LocalBugSeverity::Medium,
            physical_program: None,
            diagnostic_text: None,
            reproduction_steps: None,
            ai_suggestion_summary: None,
            applied_fix_summary: None,
            rollback_reference: None,
            updated_at: created_at.clone(),
            created_at,
        }
    }

    pub fn from_debugging_cycle(
        id: impl Into<String>,
        title: impl Into<String>,
        physical_program: impl Into<String>,
        diagnostic_text: impl Into<String>,
        ai_suggestion_summary: impl Into<String>,
        rollback_reference: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let created_at = created_at.into();
        Self {
            id: id.into(),
            title: title.into(),
            source: LocalBugSource::CompilerDebugger,
            status: LocalBugStatus::InProgress,
            severity: LocalBugSeverity::High,
            physical_program: Some(physical_program.into()),
            diagnostic_text: Some(diagnostic_text.into()),
            reproduction_steps: Some(
                "Reproduce by rerunning the captured debugging cycle.".to_string(),
            ),
            ai_suggestion_summary: Some(ai_suggestion_summary.into()),
            applied_fix_summary: None,
            rollback_reference: Some(rollback_reference.into()),
            updated_at: created_at.clone(),
            created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalBugRepository {
    pub storage_path: &'static str,
    pub sqlite_table: &'static str,
    pub create_bug_action: &'static str,
    pub debugger_create_bug_action: &'static str,
    pub update_status_action: &'static str,
    pub export_action: &'static str,
    pub import_action: &'static str,
    pub consolidated_report_action: &'static str,
    pub sharing_mode: BugRepositorySharingMode,
}

impl LocalBugRepository {
    pub fn default_repository() -> Self {
        Self {
            storage_path: ".meditor/local-bugs/local-bugs.sqlite",
            sqlite_table: "local_bug",
            create_bug_action: "tools.localBugRepository.createBug",
            debugger_create_bug_action: "debugger.compiler.createLocalBugEntry",
            update_status_action: "tools.localBugRepository.updateStatus",
            export_action: "tools.localBugRepository.export",
            import_action: "tools.localBugRepository.import",
            consolidated_report_action: "report.localBugRepository.consolidated",
            sharing_mode: BugRepositorySharingMode::LocalOnlyNeverAutoShip,
        }
    }

    pub fn never_auto_ships_user_program_bugs(&self) -> bool {
        self.sharing_mode == BugRepositorySharingMode::LocalOnlyNeverAutoShip
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BugRepositoryArchive {
    pub archive_format: &'static str,
    pub default_extension: &'static str,
    pub includes_attachments: bool,
    pub includes_debug_reports: bool,
    pub includes_rollback_metadata: bool,
    pub created_by_user_action_only: bool,
}

impl BugRepositoryArchive {
    pub fn default_archive() -> Self {
        Self {
            archive_format: "mEditor-local-bug-archive",
            default_extension: "meditor-bugs.zip",
            includes_attachments: true,
            includes_debug_reports: true,
            includes_rollback_metadata: true,
            created_by_user_action_only: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportKind {
    DebuggingReport,
    LocalBugReport,
    WorkSummary,
    DeploymentReport,
    DocumentationReport,
    SecurityReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportMenuSurface {
    pub menu: &'static str,
    pub label: &'static str,
    pub open_command: &'static str,
    pub same_window: bool,
    pub report_kinds: Vec<ReportKind>,
}

impl ReportMenuSurface {
    pub fn default_surface() -> Self {
        Self {
            menu: "Report",
            label: "Reports",
            open_command: "report.center.open",
            same_window: true,
            report_kinds: vec![
                ReportKind::DebuggingReport,
                ReportKind::LocalBugReport,
                ReportKind::WorkSummary,
                ReportKind::DeploymentReport,
                ReportKind::DocumentationReport,
                ReportKind::SecurityReport,
            ],
        }
    }
}

pub fn local_bug_repository_fields() -> &'static [&'static str] {
    &[
        "bug_id",
        "title",
        "source",
        "status",
        "severity",
        "physical_program",
        "diagnostic_text",
        "reproduction_steps",
        "ai_suggestion_summary",
        "applied_fix_summary",
        "rollback_reference",
        "created_at",
        "updated_at",
    ]
}

pub fn report_generation_steps() -> &'static [&'static str] {
    &[
        "Open Report from the main menu in the same window.",
        "Select debugging, local bug, work summary, deployment, documentation, or security report.",
        "Read local bug repository, debugging audit documents, generated documentation records, and security scan records.",
        "Let the user filter by project, program, date range, status, severity, or source.",
        "Generate the report as an editor tab and save it under the mEditor working folder.",
        "Preserve traceability to physical programs, diagnostics, AI suggestions, fixes, rollback records, and timestamps.",
        "Generate consolidated reports from local bugs plus any user-imported bug archives.",
    ]
}

pub fn bug_repository_import_export_steps() -> &'static [&'static str] {
    &[
        "Keep user-program bugs local and never send them to the mEditor APEX feedback endpoint.",
        "Export only when the user explicitly chooses export.",
        "Package bugs, diagnostics, attachments, debug reports, status history, and rollback metadata into an archive.",
        "Let the user email or transfer the archive outside mEditor by their own chosen channel.",
        "Import an archive selected by the target user into their local mEditor bug repository.",
        "Merge imported bugs with local bugs using source archive identity and duplicate checks.",
        "Publish a consolidated report from local and imported bug records under the Report menu.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_bug_repository_supports_user_and_debugger_entries() {
        let repository = LocalBugRepository::default_repository();
        assert_eq!(repository.sqlite_table, "local_bug");
        assert!(repository.storage_path.ends_with("local-bugs.sqlite"));
        assert!(repository.never_auto_ships_user_program_bugs());
        let bug = LocalBugEntry::user_entered("bug-1", "Login fails", "2026-05-19T10:00:00Z");
        assert_eq!(bug.source, LocalBugSource::UserEntered);
        let debugger_bug = LocalBugEntry::from_debugging_cycle(
            "bug-2",
            "Compiler failure",
            "/workspace/src/main.rs",
            "missing semicolon",
            "Add semicolon at end of expression",
            "rollback-1",
            "2026-05-19T10:01:00Z",
        );
        assert_eq!(debugger_bug.source, LocalBugSource::CompilerDebugger);
        assert!(debugger_bug.physical_program.is_some());
        assert!(debugger_bug.rollback_reference.is_some());
    }

    #[test]
    fn report_menu_publishes_debugging_and_work_reports() {
        let surface = ReportMenuSurface::default_surface();
        assert_eq!(surface.menu, "Report");
        assert!(surface.same_window);
        assert!(surface.report_kinds.contains(&ReportKind::DebuggingReport));
        assert!(surface.report_kinds.contains(&ReportKind::LocalBugReport));
        assert!(surface.report_kinds.contains(&ReportKind::WorkSummary));
        assert!(report_generation_steps()
            .iter()
            .any(|step| step.contains("physical programs")));
        assert!(report_generation_steps()
            .iter()
            .any(|step| step.contains("consolidated reports")));
    }

    #[test]
    fn local_bug_fields_capture_ai_debugging_details() {
        for field in [
            "physical_program",
            "diagnostic_text",
            "ai_suggestion_summary",
            "rollback_reference",
        ] {
            assert!(local_bug_repository_fields().contains(&field));
        }
    }

    #[test]
    fn bug_repository_exports_and_imports_only_by_user_action() {
        let repository = LocalBugRepository::default_repository();
        let archive = BugRepositoryArchive::default_archive();
        assert_eq!(repository.export_action, "tools.localBugRepository.export");
        assert_eq!(repository.import_action, "tools.localBugRepository.import");
        assert!(archive.created_by_user_action_only);
        assert!(archive.includes_debug_reports);
        assert!(bug_repository_import_export_steps()
            .iter()
            .any(|step| step.contains("never send them to the mEditor APEX feedback endpoint")));
        assert!(bug_repository_import_export_steps()
            .iter()
            .any(|step| step.contains("consolidated report")));
    }
}
