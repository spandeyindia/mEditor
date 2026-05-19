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
        item("Help", "Register mEditor", "help.registration.open"),
        item("Help", "Feedback And Bugs", "help.feedback.open"),
        item("Help", "About Project", "help.projectAbout.open"),
        item("Help", "Project Help", "help.projectHelp.open"),
        item("Help", "About mEditor", "help.about.open"),
        item("Help", "Check For Updates", "help.about.checkForUpdates"),
        item("Report", "Reports", "report.center.open"),
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
            .any(|item| item.menu == "Window" && item.label == "Perspectives"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Settings" && item.label == "Keymaps And Imports"));
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
            .any(|item| item.menu == "Report" && item.label == "Reports"));
        assert!(menu
            .iter()
            .any(|item| item.menu == "Help" && item.label == "Feedback And Bugs"));
        let top_right = top_right_menu_bar_items();
        assert!(top_right
            .iter()
            .any(|item| item.id == "feedback" && item.command == "feedback.open"));
    }
}
