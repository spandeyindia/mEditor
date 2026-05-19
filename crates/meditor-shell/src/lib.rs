#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenBehavior {
    SameWindowTab,
    DetachedWhenAllowed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub menu: &'static str,
    pub label: &'static str,
    pub command: &'static str,
    pub behavior: OpenBehavior,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopRightMenuBarItem {
    pub id: &'static str,
    pub icon: &'static str,
    pub label: &'static str,
    pub tooltip: &'static str,
    pub command: &'static str,
    pub behavior: OpenBehavior,
}

pub fn default_menu_items() -> Vec<MenuItem> {
    vec![
        item("File", "New Project", "project.create"),
        item(
            "File",
            "Local History And Recovery",
            "file.localHistoryRecovery.open",
        ),
        item(
            "File",
            "Workspace Backup And Restore",
            "file.workspaceBackupRestore.open",
        ),
        item("File", "Import Project", "file.importProject.open"),
        item("Source", "Refactor", "source.refactor.open"),
        item("Source", "Reformat", "source.reformat"),
        item("Run", "Tasks", "run.tasks.open"),
        item(
            "Setup",
            "Programming Language Support",
            "setup.languageSupport.open",
        ),
        item("Tools", "DBA Workshop", "tools.dbaWorkshop.open"),
        item("Tools", "SSH Terminus", "tools.sshTerminus.open"),
        item("Tools", "SFTP/SCP Transfer", "tools.sftpScpTransfer.open"),
        item("Tools", "Web Browser", "tools.webBrowser.open"),
        item("Tools", "Profiler", "tools.profiler.open"),
        item(
            "Tools",
            "Templates And Snippets",
            "tools.templatesSnippets.open",
        ),
        item("Tools", "CI/CD Generator", "tools.cicdGenerator.open"),
        item("Tools", "API Workbench", "tools.apiWorkbench.open"),
        item(
            "Tools",
            "Database Migration",
            "tools.databaseMigration.open",
        ),
        item("Tools", "Extension SDK", "tools.extensionSdk.open"),
        item("Tools", "Workspace Indexer", "tools.workspaceIndexer.open"),
        item("Tools", "XML Validator", "tools.xmlValidator.open"),
        item("Tools", "JSON Validator", "tools.jsonValidator.open"),
        item(
            "Tools",
            "Code Security Analyzer",
            "tools.codeSecurityAnalyzer.open",
        ),
        item("Tools", "CVSS Repository", "tools.cvssRepository.open"),
        item("Tools", "AI/ML Assistant", "tools.aimlAssistant.open"),
        item("Tools", "AI Knowledge Base", "tools.aiKnowledgeBase.open"),
        item("Tools", "AI Training Studio", "tools.aiTrainingStudio.open"),
        item(
            "Tools",
            "Spec-to-System Workbench",
            "tools.specToSystem.open",
        ),
        item(
            "Tools",
            "Project Documentation",
            "tools.projectDocumentation.open",
        ),
        item("Tools", "Project Planner", "tools.projectPlanner.open"),
        item("Tools", "UML Modeling", "tools.umlModeling.open"),
        item("Window", "File Explorer", "window.fileExplorer.focus"),
        item("Window", "Perspectives", "window.perspectives.open"),
        item(
            "Window",
            "Workspace Dashboard",
            "window.workspaceDashboard.open",
        ),
        item(
            "Settings",
            "Keymaps And Imports",
            "settings.keymapsImports.open",
        ),
        item(
            "Settings",
            "Workspace Trust",
            "settings.workspaceTrust.open",
        ),
        item(
            "Settings",
            "Secrets And Credentials",
            "settings.secretsCredentials.open",
        ),
        item(
            "Settings",
            "Plugin Permissions",
            "settings.pluginPermissions.open",
        ),
        item(
            "Settings",
            "Accessibility And Keyboard",
            "settings.accessibilityKeyboard.open",
        ),
        item("Help", "Register mEditor", "help.registration.open"),
        item("Help", "Feedback And Bugs", "help.feedback.open"),
        item("Help", "About Project", "help.projectAbout.open"),
        item("Help", "Project Help", "help.projectHelp.open"),
        item("Help", "About mEditor", "help.about.open"),
        item("Help", "Check For Updates", "help.about.checkForUpdates"),
        item(
            "Help",
            "Update Channel Manager",
            "help.updateChannelManager.open",
        ),
        item("Help", "Diagnostics Bundle", "help.diagnosticsBundle.open"),
        item("Help", "Privacy Center", "help.privacyCenter.open"),
        item("Report", "Reports", "report.center.open"),
        item("Report", "Audit Trail", "report.auditTrail.open"),
    ]
}

pub fn top_right_menu_bar_items() -> Vec<TopRightMenuBarItem> {
    vec![TopRightMenuBarItem {
        id: "feedback",
        icon: "message-circle-warning",
        label: "Feedback",
        tooltip: "Send bug report or enhancement request",
        command: "feedback.open",
        behavior: OpenBehavior::SameWindowTab,
    }]
}

fn item(menu: &'static str, label: &'static str, command: &'static str) -> MenuItem {
    MenuItem {
        menu,
        label,
        command,
        behavior: OpenBehavior::SameWindowTab,
    }
}

pub fn product_title() -> &'static str {
    meditor_core::PRODUCT_NAME
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_title_preserves_required_case() {
        assert_eq!(product_title(), "mEditor");
    }

    #[test]
    fn default_menu_contains_browser_and_file_explorer() {
        let menu = default_menu_items();
        assert!(menu
            .iter()
            .any(|item| item.menu == "File" && item.label == "New Project"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "File" && item.label == "Local History And Recovery"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "File" && item.label == "Workspace Backup And Restore"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "File" && item.label == "Import Project"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Source" && item.label == "Refactor"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Source" && item.label == "Reformat"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Run" && item.label == "Tasks"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Project Planner"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "SFTP/SCP Transfer"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Profiler"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Templates And Snippets"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "CI/CD Generator"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "API Workbench"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Database Migration"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Extension SDK"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Workspace Indexer"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "XML Validator"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "JSON Validator"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Window" && item.label == "Perspectives"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Keymaps And Imports"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Workspace Trust"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Secrets And Credentials"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Plugin Permissions"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Accessibility And Keyboard"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Setup" && item.label == "Programming Language Support"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Web Browser"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "Project Documentation"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Tools" && item.label == "UML Modeling"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Window" && item.label == "File Explorer"));
        assert!(menu
            .iter()
            .all(|item| item.behavior == OpenBehavior::SameWindowTab));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "About mEditor"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "About Project"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Project Help"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Check For Updates"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Update Channel Manager"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Diagnostics Bundle"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Privacy Center"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Report" && item.label == "Reports"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Report" && item.label == "Audit Trail"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Feedback And Bugs"));
        let top_right = top_right_menu_bar_items();
        assert!(top_right
            .iter()
            .any(|item| item.id == "feedback" && item.command == "feedback.open"));
    }
}
