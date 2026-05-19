#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserCommand {
    Back,
    Forward,
    Reload,
    Stop,
    Home,
    FindInPage,
    ZoomIn,
    ZoomOut,
    Bookmark,
    ShowHistory,
    ShowDownloads,
    OpenExternal,
    ClearBrowsingData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserPolicy {
    pub same_window_tabs: bool,
    pub allow_external_handoff: bool,
    pub require_download_confirmation: bool,
    pub allow_local_files: bool,
}

impl Default for BrowserPolicy {
    fn default() -> Self {
        Self {
            same_window_tabs: true,
            allow_external_handoff: true,
            require_download_confirmation: true,
            allow_local_files: true,
        }
    }
}

pub fn default_toolbar_commands() -> Vec<BrowserCommand> {
    vec![
        BrowserCommand::Back,
        BrowserCommand::Forward,
        BrowserCommand::Reload,
        BrowserCommand::Stop,
        BrowserCommand::Home,
        BrowserCommand::FindInPage,
        BrowserCommand::Bookmark,
        BrowserCommand::ShowHistory,
        BrowserCommand::ShowDownloads,
        BrowserCommand::OpenExternal,
        BrowserCommand::ClearBrowsingData,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_defaults_to_same_window_tabs_and_safe_downloads() {
        let policy = BrowserPolicy::default();
        assert!(policy.same_window_tabs);
        assert!(policy.require_download_confirmation);
    }

    #[test]
    fn toolbar_contains_navigation_and_downloads() {
        let commands = default_toolbar_commands();
        assert!(commands.contains(&BrowserCommand::Back));
        assert!(commands.contains(&BrowserCommand::ShowDownloads));
    }
}
