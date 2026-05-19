#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Nightly,
    OfflinePackage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateChannelManagerPlan {
    pub menu_path: &'static str,
    pub settings_file: &'static str,
    pub channels: Vec<UpdateChannel>,
    pub shows_release_notes_before_install: bool,
    pub supports_update_rollback: bool,
    pub preserves_user_data: bool,
    pub same_window: bool,
}

impl UpdateChannelManagerPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Help > Update Channel Manager",
            settings_file: ".meditor/settings/update-channel.toml",
            channels: vec![
                UpdateChannel::Stable,
                UpdateChannel::Beta,
                UpdateChannel::Nightly,
                UpdateChannel::OfflinePackage,
            ],
            shows_release_notes_before_install: true,
            supports_update_rollback: true,
            preserves_user_data: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticBundleItem {
    Logs,
    Environment,
    Configuration,
    RecentErrors,
    PluginList,
    ToolchainPaths,
    WorkspaceTrustState,
    RedactedConnections,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticsBundlePlan {
    pub menu_path: &'static str,
    pub output_extension: &'static str,
    pub included_items: Vec<DiagnosticBundleItem>,
    pub user_reviews_before_export: bool,
    pub redacts_secrets: bool,
    pub same_window: bool,
}

impl DiagnosticsBundlePlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Help > Diagnostics Bundle",
            output_extension: ".meditor-diagnostics.zip",
            included_items: vec![
                DiagnosticBundleItem::Logs,
                DiagnosticBundleItem::Environment,
                DiagnosticBundleItem::Configuration,
                DiagnosticBundleItem::RecentErrors,
                DiagnosticBundleItem::PluginList,
                DiagnosticBundleItem::ToolchainPaths,
                DiagnosticBundleItem::WorkspaceTrustState,
                DiagnosticBundleItem::RedactedConnections,
            ],
            user_reviews_before_export: true,
            redacts_secrets: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataFlowKind {
    InstallationRecord,
    InstallationEvent,
    ManualFeedback,
    AutomaticErrorReport,
    UpdateCheck,
    DiagnosticsBundleExport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrivacyCenterPlan {
    pub menu_path: &'static str,
    pub policy_file: &'static str,
    pub visible_data_flows: Vec<DataFlowKind>,
    pub explains_install_base_use: bool,
    pub shows_no_license_obligation: bool,
    pub lets_user_view_local_payloads: bool,
    pub same_window: bool,
}

impl PrivacyCenterPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Help > Privacy Center",
            policy_file: ".meditor/settings/privacy-center.toml",
            visible_data_flows: vec![
                DataFlowKind::InstallationRecord,
                DataFlowKind::InstallationEvent,
                DataFlowKind::ManualFeedback,
                DataFlowKind::AutomaticErrorReport,
                DataFlowKind::UpdateCheck,
                DataFlowKind::DiagnosticsBundleExport,
            ],
            explains_install_base_use: true,
            shows_no_license_obligation: true,
            lets_user_view_local_payloads: true,
            same_window: true,
        }
    }
}

pub fn pilot_hardening_capability_names() -> &'static [&'static str] {
    &[
        "Update Channel Manager",
        "Diagnostics Bundle",
        "Privacy Center",
    ]
}

pub fn pilot_hardening_workflow_steps() -> &'static [&'static str] {
    &[
        "Let the user choose stable, beta, nightly, or offline update packages.",
        "Show release notes and rollback plan before update installation.",
        "Collect diagnostics into a bundle only after user review.",
        "Redact secrets and credentials from every diagnostics export.",
        "Show exactly which install, feedback, error, update, and diagnostics data flows exist.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_channel_manager_supports_channels_and_rollback() {
        let plan = UpdateChannelManagerPlan::default_plan();
        assert!(plan.channels.contains(&UpdateChannel::Stable));
        assert!(plan.channels.contains(&UpdateChannel::Beta));
        assert!(plan.channels.contains(&UpdateChannel::Nightly));
        assert!(plan.shows_release_notes_before_install);
        assert!(plan.supports_update_rollback);
        assert!(plan.preserves_user_data);
    }

    #[test]
    fn diagnostics_bundle_is_reviewed_and_redacted() {
        let plan = DiagnosticsBundlePlan::default_plan();
        assert!(plan.user_reviews_before_export);
        assert!(plan.redacts_secrets);
        assert!(plan.included_items.contains(&DiagnosticBundleItem::Logs));
        assert!(plan
            .included_items
            .contains(&DiagnosticBundleItem::ToolchainPaths));
    }

    #[test]
    fn privacy_center_exposes_data_flows() {
        let plan = PrivacyCenterPlan::default_plan();
        assert!(plan
            .visible_data_flows
            .contains(&DataFlowKind::InstallationRecord));
        assert!(plan
            .visible_data_flows
            .contains(&DataFlowKind::AutomaticErrorReport));
        assert!(plan.explains_install_base_use);
        assert!(plan.shows_no_license_obligation);
        assert!(plan.lets_user_view_local_payloads);
    }
}
