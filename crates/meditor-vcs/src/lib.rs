#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcsProvider {
    Git,
    Svn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VcsActionKind {
    Status,
    Diff,
    Commit,
    BranchOrTag,
    MergeOrUpdate,
    Revert,
    History,
    BlameOrAnnotate,
    ResolveConflict,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VcsAction {
    pub provider: VcsProvider,
    pub kind: VcsActionKind,
    pub label: &'static str,
    pub destructive: bool,
}

pub fn default_actions(provider: VcsProvider) -> Vec<VcsAction> {
    vec![
        action(provider, VcsActionKind::Status, "Status", false),
        action(provider, VcsActionKind::Diff, "Diff", false),
        action(provider, VcsActionKind::Commit, "Commit", false),
        action(provider, VcsActionKind::BranchOrTag, "Branch/Tag", false),
        action(
            provider,
            VcsActionKind::MergeOrUpdate,
            "Merge/Update",
            false,
        ),
        action(provider, VcsActionKind::Revert, "Revert", true),
        action(provider, VcsActionKind::History, "History", false),
        action(
            provider,
            VcsActionKind::BlameOrAnnotate,
            "Blame/Annotate",
            false,
        ),
        action(
            provider,
            VcsActionKind::ResolveConflict,
            "Resolve Conflict",
            false,
        ),
    ]
}

fn action(
    provider: VcsProvider,
    kind: VcsActionKind,
    label: &'static str,
    destructive: bool,
) -> VcsAction {
    VcsAction {
        provider,
        kind,
        label,
        destructive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_and_svn_cover_core_actions() {
        assert_eq!(default_actions(VcsProvider::Git).len(), 9);
        assert_eq!(default_actions(VcsProvider::Svn).len(), 9);
    }

    #[test]
    fn revert_is_marked_destructive() {
        let actions = default_actions(VcsProvider::Git);
        assert!(actions
            .iter()
            .any(|action| action.kind == VcsActionKind::Revert && action.destructive));
    }
}
