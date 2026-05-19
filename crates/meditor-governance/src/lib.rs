#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluginPermission {
    FileSystemRead,
    FileSystemWrite,
    Terminal,
    Network,
    Database,
    Ssh,
    AiAccess,
    CredentialAccess,
    WorkspaceMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginPermissionModel {
    pub menu_path: &'static str,
    pub manifest_key: &'static str,
    pub supported_permissions: Vec<PluginPermission>,
    pub deny_by_default: bool,
    pub prompts_user_before_grant: bool,
    pub logs_decisions_to_audit_trail: bool,
    pub same_window: bool,
}

impl PluginPermissionModel {
    pub fn default_model() -> Self {
        Self {
            menu_path: "Settings > Plugin Permissions",
            manifest_key: "permissions",
            supported_permissions: vec![
                PluginPermission::FileSystemRead,
                PluginPermission::FileSystemWrite,
                PluginPermission::Terminal,
                PluginPermission::Network,
                PluginPermission::Database,
                PluginPermission::Ssh,
                PluginPermission::AiAccess,
                PluginPermission::CredentialAccess,
                PluginPermission::WorkspaceMetadata,
            ],
            deny_by_default: true,
            prompts_user_before_grant: true,
            logs_decisions_to_audit_trail: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessibilityFeature {
    HighContrastThemes,
    ScreenReaderLabels,
    FullKeyboardNavigation,
    ReducedMotion,
    FontScaling,
    FocusVisible,
    CommandPaletteAccess,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeymapCompatibility {
    NetBeans,
    Eclipse,
    VsCode,
    JDeveloper,
    Intellij,
    Vim,
    Emacs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessibilityKeyboardPlan {
    pub menu_path: &'static str,
    pub settings_file: &'static str,
    pub accessibility_features: Vec<AccessibilityFeature>,
    pub keymap_profiles: Vec<KeymapCompatibility>,
    pub full_keyboard_first_run_check: bool,
    pub same_window: bool,
}

impl AccessibilityKeyboardPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Settings > Accessibility And Keyboard",
            settings_file: ".meditor/settings/accessibility-keyboard.toml",
            accessibility_features: vec![
                AccessibilityFeature::HighContrastThemes,
                AccessibilityFeature::ScreenReaderLabels,
                AccessibilityFeature::FullKeyboardNavigation,
                AccessibilityFeature::ReducedMotion,
                AccessibilityFeature::FontScaling,
                AccessibilityFeature::FocusVisible,
                AccessibilityFeature::CommandPaletteAccess,
            ],
            keymap_profiles: vec![
                KeymapCompatibility::NetBeans,
                KeymapCompatibility::Eclipse,
                KeymapCompatibility::VsCode,
                KeymapCompatibility::JDeveloper,
                KeymapCompatibility::Intellij,
                KeymapCompatibility::Vim,
                KeymapCompatibility::Emacs,
            ],
            full_keyboard_first_run_check: true,
            same_window: true,
        }
    }
}

pub fn governance_capability_names() -> &'static [&'static str] {
    &[
        "Plugin Permission Model",
        "Accessibility And Keyboard Compatibility",
    ]
}

pub fn governance_workflow_steps() -> &'static [&'static str] {
    &[
        "Require every plugin to declare permissions in its manifest.",
        "Deny sensitive plugin access by default until the user grants it.",
        "Record plugin permission decisions in the local audit trail.",
        "Provide high-contrast themes, screen-reader labels, focus visibility, reduced motion, and keyboard-only navigation.",
        "Offer keymap compatibility profiles for NetBeans, Eclipse, VS Code, JDeveloper, IntelliJ, Vim, and Emacs.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_permissions_are_deny_by_default_and_audited() {
        let model = PluginPermissionModel::default_model();
        assert!(model.deny_by_default);
        assert!(model.prompts_user_before_grant);
        assert!(model.logs_decisions_to_audit_trail);
        assert!(model
            .supported_permissions
            .contains(&PluginPermission::CredentialAccess));
    }

    #[test]
    fn accessibility_plan_supports_keyboard_and_screen_reader_use() {
        let plan = AccessibilityKeyboardPlan::default_plan();
        assert!(plan
            .accessibility_features
            .contains(&AccessibilityFeature::ScreenReaderLabels));
        assert!(plan
            .accessibility_features
            .contains(&AccessibilityFeature::FullKeyboardNavigation));
        assert!(plan
            .keymap_profiles
            .contains(&KeymapCompatibility::NetBeans));
        assert!(plan.keymap_profiles.contains(&KeymapCompatibility::VsCode));
        assert!(plan.full_keyboard_first_run_check);
    }
}
