#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkbenchArea {
    LeftNavigator,
    CentralTabs,
    RightSidebar,
    BottomPanel,
    ToolsMenu,
    WindowMenu,
    HelpMenu,
    SetupMenu,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkbenchModule {
    pub id: &'static str,
    pub label: &'static str,
    pub area: WorkbenchArea,
    pub same_window: bool,
}

pub fn pre_pilot_ver_2_modules() -> Vec<WorkbenchModule> {
    vec![
        module("editor", "Editor", WorkbenchArea::CentralTabs),
        module(
            "projectWorkspace",
            "Project Workspace",
            WorkbenchArea::CentralTabs,
        ),
        module(
            "languageSupportSetup",
            "Programming Language Support",
            WorkbenchArea::SetupMenu,
        ),
        module(
            "fileExplorer",
            "File Explorer",
            WorkbenchArea::LeftNavigator,
        ),
        module("webBrowser", "Web Browser", WorkbenchArea::CentralTabs),
        module("dbaWorkshop", "DBA Workshop", WorkbenchArea::ToolsMenu),
        module("sshTerminus", "SSH Terminus", WorkbenchArea::ToolsMenu),
        module(
            "sftpScpTransfer",
            "SFTP/SCP Transfer",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "commandPalette",
            "Command Palette",
            WorkbenchArea::CentralTabs,
        ),
        module("perspectives", "Perspectives", WorkbenchArea::WindowMenu),
        module("profiler", "Profiler", WorkbenchArea::ToolsMenu),
        module(
            "templatesSnippets",
            "Templates And Snippets",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "keymapMigration",
            "Keymaps And Imports",
            WorkbenchArea::Settings,
        ),
        module(
            "localTerminals",
            "Local Terminals",
            WorkbenchArea::BottomPanel,
        ),
        module(
            "codeSecurityAnalyzer",
            "Code Security Analyzer",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "compilerDebugger",
            "Compiler Error Debugger",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "cvssRepository",
            "CVSS Repository",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "aimlAssistant",
            "AI/ML Assistant",
            WorkbenchArea::CentralTabs,
        ),
        module(
            "aiKnowledgeBase",
            "AI Knowledge Base",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "aiTrainingStudio",
            "AI Training Studio",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "specToSystemWorkbench",
            "Spec-to-System Workbench",
            WorkbenchArea::ToolsMenu,
        ),
        module("dependencies", "Dependencies", WorkbenchArea::ToolsMenu),
        module(
            "databaseModeling",
            "Database Modeling",
            WorkbenchArea::ToolsMenu,
        ),
        module("testExplorer", "Test Explorer", WorkbenchArea::BottomPanel),
        module("coverage", "Coverage", WorkbenchArea::BottomPanel),
        module("diffMerge", "Diff/Merge", WorkbenchArea::ToolsMenu),
        module(
            "offlineDocs",
            "Offline Documentation",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "projectDocumentation",
            "Project Documentation",
            WorkbenchArea::ToolsMenu,
        ),
        module(
            "projectPlanner",
            "Project Planner",
            WorkbenchArea::ToolsMenu,
        ),
        module("umlModeling", "UML Modeling", WorkbenchArea::ToolsMenu),
        module(
            "largeFileViewer",
            "Large File Viewer",
            WorkbenchArea::CentralTabs,
        ),
        module(
            "workspaceDashboard",
            "Workspace Dashboard",
            WorkbenchArea::CentralTabs,
        ),
        module(
            "installationMetrics",
            "Installation Metrics",
            WorkbenchArea::Settings,
        ),
        module(
            "feedbackAndBugs",
            "Feedback And Bugs",
            WorkbenchArea::ToolsMenu,
        ),
        module("projectAbout", "About Project", WorkbenchArea::HelpMenu),
        module("projectHelp", "Project Help", WorkbenchArea::HelpMenu),
        module(
            "localBugRepository",
            "Local Bug Repository",
            WorkbenchArea::ToolsMenu,
        ),
        module("reports", "Reports", WorkbenchArea::ToolsMenu),
        module("settings", "Settings", WorkbenchArea::Settings),
    ]
}

pub fn frozen_ver_1_modules() -> Vec<WorkbenchModule> {
    pre_pilot_ver_2_modules()
}

fn module(id: &'static str, label: &'static str, area: WorkbenchArea) -> WorkbenchModule {
    WorkbenchModule {
        id,
        label,
        area,
        same_window: true,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkbenchBaseline {
    pub modules: Vec<WorkbenchModule>,
    pub menu_items: usize,
    pub top_right_menu_bar_items: usize,
    pub editor_buffer_routes: usize,
    pub file_context_actions: usize,
    pub browser_commands: usize,
    pub db_worksheet_actions: usize,
    pub db_dashboard_sections: usize,
    pub ssh_terminal_actions: usize,
    pub ai_knowledge_formats: usize,
    pub ai_capabilities: usize,
    pub project_import_modes: usize,
    pub project_node_kinds: usize,
    pub planning_item_kinds: usize,
    pub project_workspace_steps: usize,
    pub project_planning_steps: usize,
    pub embedded_project_documentation_sections: usize,
    pub project_help_sources: usize,
    pub test_actions: usize,
    pub coverage_formats: usize,
    pub dependency_manifest_kinds: usize,
    pub dependency_actions: usize,
    pub plugin_contribution_points: usize,
    pub registration_statuses: usize,
    pub license_acceptance_fields: usize,
    pub installation_event_types: usize,
    pub bug_report_fields: usize,
    pub feedback_form_fields: usize,
    pub automatic_error_capture_fields: usize,
    pub feedback_statuses: usize,
    pub user_feedback_metrics: usize,
    pub apex_metric_tables: usize,
    pub first_run_notice_points: usize,
    pub vcs_actions: usize,
    pub setup_tasks: usize,
    pub update_flow_steps: usize,
    pub update_preserved_paths: usize,
    pub language_support_catalog_entries: usize,
    pub language_tooling_install_steps: usize,
    pub language_tooling_network_dialog_fields: usize,
    pub sdlc_actions: usize,
    pub executable_requirements: usize,
    pub compiler_debugger_cycle_steps: usize,
    pub compiler_debugger_audit_sections: usize,
    pub deployment_doc_targets: usize,
    pub deployment_doc_steps: usize,
    pub project_documentation_sections: usize,
    pub flowchart_generation_steps: usize,
    pub erd_supported_source_kinds: usize,
    pub erd_generation_steps: usize,
    pub uml_diagram_kinds: usize,
    pub uml_text_dialects: usize,
    pub uml_workflow_steps: usize,
    pub local_bug_repository_fields: usize,
    pub bug_import_export_steps: usize,
    pub report_generation_steps: usize,
    pub report_kinds: usize,
    pub reference_feature_count: usize,
    pub inbuilt_professional_capabilities: usize,
    pub self_learning_ai_service_enabled: bool,
    pub security_rules: usize,
    pub cvss_update_sources: usize,
}

impl WorkbenchBaseline {
    pub fn pre_pilot_ver_2() -> Self {
        Self::baseline(pre_pilot_ver_2_modules())
    }

    pub fn frozen_ver_1() -> Self {
        Self::pre_pilot_ver_2()
    }

    fn baseline(modules: Vec<WorkbenchModule>) -> Self {
        Self {
            modules,
            menu_items: meditor_shell::default_menu_items().len(),
            top_right_menu_bar_items: meditor_shell::top_right_menu_bar_items().len(),
            editor_buffer_routes: 4,
            file_context_actions: meditor_file_explorer::context_menu_for(
                meditor_file_explorer::FileKind::TextOrSource,
            )
            .len(),
            browser_commands: meditor_browser::default_toolbar_commands().len(),
            db_worksheet_actions: meditor_db_workbench::worksheet_actions().len(),
            db_dashboard_sections: meditor_db_workbench::dba_dashboard_sections().len(),
            ssh_terminal_actions: meditor_ssh_terminus::terminal_actions().len(),
            ai_knowledge_formats: meditor_ai::supported_knowledge_formats().len(),
            ai_capabilities: meditor_ai::assistant_capabilities().len(),
            project_import_modes: meditor_project_importers::import_modes().len(),
            project_node_kinds: meditor_project_workspace::supported_project_node_kinds().len(),
            planning_item_kinds: meditor_project_workspace::supported_planning_items().len(),
            project_workspace_steps: meditor_project_workspace::project_workspace_steps().len(),
            project_planning_steps: meditor_project_workspace::project_planning_steps().len(),
            embedded_project_documentation_sections:
                meditor_project_workspace::embedded_project_documentation_sections().len(),
            project_help_sources: meditor_project_workspace::project_help_sources().len(),
            test_actions: meditor_tests::test_actions().len(),
            coverage_formats: meditor_tests::coverage_formats().len(),
            dependency_manifest_kinds: meditor_dependencies::supported_manifest_kinds().len(),
            dependency_actions: meditor_dependencies::dependency_actions().len(),
            plugin_contribution_points: meditor_plugins::contribution_points().len(),
            registration_statuses: meditor_installation_metrics::registration_statuses().len(),
            license_acceptance_fields: meditor_installation_metrics::license_acceptance_fields()
                .len(),
            installation_event_types: meditor_installation_metrics::installation_event_types()
                .len(),
            bug_report_fields: meditor_installation_metrics::bug_report_fields().len(),
            feedback_form_fields: meditor_installation_metrics::feedback_form_fields().len(),
            automatic_error_capture_fields:
                meditor_installation_metrics::automatic_error_capture_fields().len(),
            feedback_statuses: meditor_installation_metrics::feedback_statuses().len(),
            user_feedback_metrics: meditor_installation_metrics::user_feedback_metrics().len(),
            apex_metric_tables: meditor_installation_metrics::apex_table_names().len(),
            first_run_notice_points: meditor_installation_metrics::first_run_notice_points().len(),
            vcs_actions: meditor_vcs::default_actions(meditor_vcs::VcsProvider::Git).len()
                + meditor_vcs::default_actions(meditor_vcs::VcsProvider::Svn).len(),
            setup_tasks: meditor_setup::SetupPlan::sqlite_one_click().tasks.len()
                + meditor_setup::SetupPlan::language_support().tasks.len(),
            update_flow_steps: meditor_packaging::update_flow_steps().len(),
            update_preserved_paths: meditor_packaging::update_preserved_paths().len(),
            language_support_catalog_entries: meditor_toolchains::supported_language_catalog()
                .len(),
            language_tooling_install_steps:
                meditor_toolchains::language_tooling_install_flow_steps().len(),
            language_tooling_network_dialog_fields:
                meditor_toolchains::default_tooling_network_dialog()
                    .fields
                    .len(),
            sdlc_actions: meditor_sdlc::standard_actions().len(),
            executable_requirements: meditor_toolchains::seed_executable_requirements().len(),
            compiler_debugger_cycle_steps:
                meditor_compiler_debugger::compiler_debugger_cycle_steps().len(),
            compiler_debugger_audit_sections:
                meditor_compiler_debugger::debugging_audit_document_sections().len(),
            deployment_doc_targets: meditor_deployment_docs::supported_deployment_targets().len(),
            deployment_doc_steps: meditor_deployment_docs::deployment_doc_generation_steps().len(),
            project_documentation_sections:
                meditor_deployment_docs::project_documentation_sections().len(),
            flowchart_generation_steps: meditor_deployment_docs::flowchart_generation_steps().len(),
            erd_supported_source_kinds: meditor_deployment_docs::erd_supported_source_kinds().len(),
            erd_generation_steps: meditor_deployment_docs::erd_generation_steps().len(),
            uml_diagram_kinds: meditor_uml_modeling::supported_diagram_kinds().len(),
            uml_text_dialects: meditor_uml_modeling::supported_text_dialects().len(),
            uml_workflow_steps: meditor_uml_modeling::uml_modeling_workflow_steps().len(),
            local_bug_repository_fields: meditor_local_bug_repository::local_bug_repository_fields(
            )
            .len(),
            bug_import_export_steps:
                meditor_local_bug_repository::bug_repository_import_export_steps().len(),
            report_generation_steps: meditor_local_bug_repository::report_generation_steps().len(),
            report_kinds: meditor_local_bug_repository::ReportMenuSurface::default_surface()
                .report_kinds
                .len(),
            reference_feature_count: meditor_reference_features::reference_feature_catalog().len(),
            inbuilt_professional_capabilities:
                meditor_reference_features::inbuilt_professional_ide_capabilities().len(),
            self_learning_ai_service_enabled:
                meditor_ai::SelfLearningAiServicePlan::ver_1_contract().embedded_service,
            security_rules: meditor_code_security::seed_rules().len(),
            cvss_update_sources: meditor_cvss_repository::update_sources().len(),
        }
    }

    pub fn all_modules_same_window(&self) -> bool {
        self.modules.iter().all(|module| module.same_window)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_pilot_workbench_includes_key_ver_2_modules() {
        let baseline = WorkbenchBaseline::pre_pilot_ver_2();
        for id in [
            "fileExplorer",
            "projectWorkspace",
            "webBrowser",
            "dbaWorkshop",
            "sshTerminus",
            "sftpScpTransfer",
            "commandPalette",
            "perspectives",
            "profiler",
            "templatesSnippets",
            "keymapMigration",
            "codeSecurityAnalyzer",
            "compilerDebugger",
            "cvssRepository",
            "aimlAssistant",
            "specToSystemWorkbench",
            "languageSupportSetup",
            "projectDocumentation",
            "projectPlanner",
            "umlModeling",
            "localBugRepository",
            "reports",
            "installationMetrics",
            "feedbackAndBugs",
            "projectAbout",
            "projectHelp",
        ] {
            assert!(baseline.modules.iter().any(|module| module.id == id));
        }
    }

    #[test]
    fn pre_pilot_workbench_defaults_to_same_window_modules() {
        assert!(WorkbenchBaseline::pre_pilot_ver_2().all_modules_same_window());
    }

    #[test]
    fn pre_pilot_workbench_counts_core_models() {
        let baseline = WorkbenchBaseline::pre_pilot_ver_2();
        assert_eq!(baseline.menu_items, 31);
        assert_eq!(baseline.top_right_menu_bar_items, 1);
        assert!(baseline.project_node_kinds >= 7);
        assert!(baseline.planning_item_kinds >= 12);
        assert!(baseline.project_workspace_steps >= 8);
        assert!(baseline.project_planning_steps >= 6);
        assert_eq!(baseline.embedded_project_documentation_sections, 8);
        assert_eq!(baseline.project_help_sources, 7);
        assert_eq!(baseline.setup_tasks, 9);
        assert_eq!(baseline.update_flow_steps, 8);
        assert!(baseline.update_preserved_paths >= 8);
        assert!(baseline.language_support_catalog_entries >= 60);
        assert_eq!(baseline.language_tooling_install_steps, 9);
        assert!(baseline.language_tooling_network_dialog_fields >= 5);
        assert_eq!(baseline.sdlc_actions, 11);
        assert!(baseline.executable_requirements >= 55);
        assert!(baseline.compiler_debugger_cycle_steps >= 8);
        assert!(baseline.compiler_debugger_audit_sections >= 10);
        assert!(baseline.deployment_doc_targets >= 10);
        assert!(baseline.deployment_doc_steps >= 8);
        assert!(baseline.project_documentation_sections >= 12);
        assert!(baseline.flowchart_generation_steps >= 5);
        assert!(baseline.erd_supported_source_kinds >= 10);
        assert!(baseline.erd_generation_steps >= 6);
        assert!(baseline.uml_diagram_kinds >= 10);
        assert_eq!(baseline.uml_text_dialects, 3);
        assert!(baseline.uml_workflow_steps >= 7);
        assert!(baseline.local_bug_repository_fields >= 13);
        assert!(baseline.bug_import_export_steps >= 7);
        assert!(baseline.report_generation_steps >= 7);
        assert!(baseline.report_kinds >= 6);
        assert!(baseline.reference_feature_count >= 15);
        assert_eq!(baseline.inbuilt_professional_capabilities, 9);
        assert!(baseline.self_learning_ai_service_enabled);
        assert_eq!(baseline.security_rules, 9);
        assert_eq!(baseline.db_dashboard_sections, 7);
        assert_eq!(baseline.ai_knowledge_formats, 11);
        assert_eq!(baseline.vcs_actions, 18);
        assert_eq!(baseline.registration_statuses, 2);
        assert_eq!(baseline.license_acceptance_fields, 6);
        assert_eq!(baseline.installation_event_types, 8);
        assert_eq!(baseline.bug_report_fields, 17);
        assert_eq!(baseline.feedback_form_fields, 9);
        assert_eq!(baseline.automatic_error_capture_fields, 10);
        assert_eq!(baseline.feedback_statuses, 8);
        assert_eq!(baseline.user_feedback_metrics, 6);
        assert_eq!(baseline.apex_metric_tables, 5);
    }
}
