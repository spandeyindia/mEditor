#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtensionContributionKind {
    Language,
    Compiler,
    Formatter,
    Debugger,
    Snippet,
    Template,
    Documentation,
    Theme,
    ToolWindow,
    ProjectWizard,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtensionSdkPlan {
    pub menu_path: &'static str,
    pub sdk_folder: &'static str,
    pub manifest_file_name: &'static str,
    pub contribution_kinds: Vec<ExtensionContributionKind>,
    pub validates_permissions: bool,
    pub supports_offline_packs: bool,
    pub same_window: bool,
}

impl ExtensionSdkPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Tools > Extension SDK",
            sdk_folder: ".meditor/extensions/sdk",
            manifest_file_name: "meditor-extension.toml",
            contribution_kinds: vec![
                ExtensionContributionKind::Language,
                ExtensionContributionKind::Compiler,
                ExtensionContributionKind::Formatter,
                ExtensionContributionKind::Debugger,
                ExtensionContributionKind::Snippet,
                ExtensionContributionKind::Template,
                ExtensionContributionKind::Documentation,
                ExtensionContributionKind::Theme,
                ExtensionContributionKind::ToolWindow,
                ExtensionContributionKind::ProjectWizard,
            ],
            validates_permissions: true,
            supports_offline_packs: true,
            same_window: true,
        }
    }
}

pub fn extension_sdk_steps() -> &'static [&'static str] {
    &[
        "Create an extension skeleton with meditor-extension.toml.",
        "Declare languages, compilers, formatters, debuggers, snippets, templates, docs, themes, tools, or project wizards.",
        "Validate declared permissions before enabling the extension.",
        "Package extensions as local/offline bundles for installation without internet.",
        "Load extension contributions into the same-window workbench after user approval.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_sdk_covers_language_and_tool_contributions() {
        let plan = ExtensionSdkPlan::default_plan();
        assert_eq!(plan.menu_path, "Tools > Extension SDK");
        assert!(plan
            .contribution_kinds
            .contains(&ExtensionContributionKind::Language));
        assert!(plan
            .contribution_kinds
            .contains(&ExtensionContributionKind::Debugger));
        assert!(plan
            .contribution_kinds
            .contains(&ExtensionContributionKind::ProjectWizard));
        assert!(plan.validates_permissions);
        assert!(plan.supports_offline_packs);
    }
}
