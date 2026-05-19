#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexKind {
    Symbols,
    References,
    CallHierarchy,
    TodoComments,
    DependencyGraph,
    Files,
    Tests,
    Endpoints,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceIndexerPlan {
    pub menu_path: &'static str,
    pub index_folder: &'static str,
    pub index_kinds: Vec<IndexKind>,
    pub incremental: bool,
    pub respects_workspace_trust: bool,
    pub same_window: bool,
}

impl WorkspaceIndexerPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Tools > Workspace Indexer",
            index_folder: ".meditor/index",
            index_kinds: vec![
                IndexKind::Symbols,
                IndexKind::References,
                IndexKind::CallHierarchy,
                IndexKind::TodoComments,
                IndexKind::DependencyGraph,
                IndexKind::Files,
                IndexKind::Tests,
                IndexKind::Endpoints,
            ],
            incremental: true,
            respects_workspace_trust: true,
            same_window: true,
        }
    }
}

pub fn workspace_indexer_steps() -> &'static [&'static str] {
    &[
        "Create or refresh indexes after workspace trust permits scanning.",
        "Index files, symbols, references, call hierarchy, TODOs, dependency graph, tests, and endpoints.",
        "Use incremental refreshes after file save, Git/SVN update, import, or generated code changes.",
        "Expose project-wide navigation through search, command palette, references, and workspace dashboard.",
        "Store indexes locally under the mEditor working folder.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_indexer_covers_navigation_and_analysis_indexes() {
        let plan = WorkspaceIndexerPlan::default_plan();
        assert!(plan.index_kinds.contains(&IndexKind::Symbols));
        assert!(plan.index_kinds.contains(&IndexKind::References));
        assert!(plan.index_kinds.contains(&IndexKind::CallHierarchy));
        assert!(plan.index_kinds.contains(&IndexKind::DependencyGraph));
        assert!(plan.incremental);
        assert!(plan.respects_workspace_trust);
    }
}
