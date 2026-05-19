#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SshGroup {
    pub id: String,
    pub name: String,
    pub profile_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SshProfile {
    pub id: String,
    pub group_id: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalAction {
    OpenTab,
    OpenGroup,
    CloseGroup,
    ReconnectTab,
    ReconnectGroup,
    PortForward,
    CommandRunner,
}

pub fn terminal_actions() -> Vec<TerminalAction> {
    vec![
        TerminalAction::OpenTab,
        TerminalAction::OpenGroup,
        TerminalAction::CloseGroup,
        TerminalAction::ReconnectTab,
        TerminalAction::ReconnectGroup,
        TerminalAction::PortForward,
        TerminalAction::CommandRunner,
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalTab {
    pub id: String,
    pub profile_id: String,
    pub group_id: String,
    pub title: String,
    pub pinned: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_actions_cover_grouped_tabbed_use() {
        let actions = terminal_actions();
        assert!(actions.contains(&TerminalAction::OpenGroup));
        assert!(actions.contains(&TerminalAction::ReconnectTab));
        assert!(actions.contains(&TerminalAction::PortForward));
    }
}
