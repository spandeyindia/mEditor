#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileKind {
    Directory,
    TextOrSource,
    Binary,
    Large,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplorerAction {
    OpenInEditorTab,
    OpenToSide,
    OpenWith,
    Preview,
    SecurityScanFile,
    RunRelatedTask,
    CompareWith,
    ShowFileHistory,
    OpenTerminalHere,
    RevealInSystemFileManager,
}

pub fn context_menu_for(kind: FileKind) -> Vec<ExplorerAction> {
    match kind {
        FileKind::Directory => vec![
            ExplorerAction::OpenTerminalHere,
            ExplorerAction::OpenWith,
            ExplorerAction::RevealInSystemFileManager,
            ExplorerAction::ShowFileHistory,
        ],
        FileKind::TextOrSource => vec![
            ExplorerAction::OpenInEditorTab,
            ExplorerAction::OpenToSide,
            ExplorerAction::Preview,
            ExplorerAction::OpenWith,
            ExplorerAction::SecurityScanFile,
            ExplorerAction::RunRelatedTask,
            ExplorerAction::CompareWith,
            ExplorerAction::ShowFileHistory,
            ExplorerAction::RevealInSystemFileManager,
        ],
        FileKind::Binary => vec![
            ExplorerAction::OpenWith,
            ExplorerAction::CompareWith,
            ExplorerAction::ShowFileHistory,
            ExplorerAction::RevealInSystemFileManager,
        ],
        FileKind::Large => vec![
            ExplorerAction::Preview,
            ExplorerAction::OpenWith,
            ExplorerAction::SecurityScanFile,
            ExplorerAction::CompareWith,
            ExplorerAction::ShowFileHistory,
            ExplorerAction::RevealInSystemFileManager,
        ],
    }
}

pub fn default_file_action(kind: FileKind) -> ExplorerAction {
    match kind {
        FileKind::Directory => ExplorerAction::OpenTerminalHere,
        FileKind::TextOrSource => ExplorerAction::OpenInEditorTab,
        FileKind::Binary => ExplorerAction::OpenWith,
        FileKind::Large => ExplorerAction::Preview,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_file_context_menu_opens_in_editor_tab() {
        let actions = context_menu_for(FileKind::TextOrSource);
        assert_eq!(actions.first(), Some(&ExplorerAction::OpenInEditorTab));
        assert_eq!(
            default_file_action(FileKind::TextOrSource),
            ExplorerAction::OpenInEditorTab
        );
    }

    #[test]
    fn large_files_route_to_preview_instead_of_forced_editor() {
        assert_eq!(
            default_file_action(FileKind::Large),
            ExplorerAction::Preview
        );
    }
}
