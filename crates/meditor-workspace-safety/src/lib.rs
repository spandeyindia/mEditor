#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrustLevel {
    Restricted,
    Trusted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrustCapability {
    RunScripts,
    AutoBuild,
    TerminalAccess,
    NetworkAccess,
    DatabaseConnections,
    SshConnections,
    AiCodeExecution,
    PluginActivation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceTrustPolicy {
    pub menu_path: &'static str,
    pub default_level_for_unknown_projects: TrustLevel,
    pub restricted_capabilities: Vec<TrustCapability>,
    pub trust_file: &'static str,
    pub requires_user_decision: bool,
    pub same_window: bool,
}

impl WorkspaceTrustPolicy {
    pub fn default_policy() -> Self {
        Self {
            menu_path: "Settings > Workspace Trust",
            default_level_for_unknown_projects: TrustLevel::Restricted,
            restricted_capabilities: vec![
                TrustCapability::RunScripts,
                TrustCapability::AutoBuild,
                TrustCapability::TerminalAccess,
                TrustCapability::NetworkAccess,
                TrustCapability::DatabaseConnections,
                TrustCapability::SshConnections,
                TrustCapability::AiCodeExecution,
                TrustCapability::PluginActivation,
            ],
            trust_file: ".meditor/workspace/trust.toml",
            requires_user_decision: true,
            same_window: true,
        }
    }

    pub fn can_use(&self, level: TrustLevel, capability: TrustCapability) -> bool {
        match level {
            TrustLevel::Trusted => true,
            TrustLevel::Restricted => !self.restricted_capabilities.contains(&capability),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalHistoryRecoveryPlan {
    pub menu_path: &'static str,
    pub snapshot_folder: &'static str,
    pub crash_recovery_folder: &'static str,
    pub captures_file_saves: bool,
    pub captures_project_metadata: bool,
    pub supports_session_undo: bool,
    pub supports_restore_previous_project_state: bool,
    pub same_window: bool,
}

impl LocalHistoryRecoveryPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "File > Local History And Recovery",
            snapshot_folder: ".meditor/history/snapshots",
            crash_recovery_folder: ".meditor/history/crash-recovery",
            captures_file_saves: true,
            captures_project_metadata: true,
            supports_session_undo: true,
            supports_restore_previous_project_state: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretKind {
    DatabasePassword,
    SshPrivateKey,
    ApiToken,
    ProxyCredential,
    GitCredential,
    SigningKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialVaultPlan {
    pub menu_path: &'static str,
    pub metadata_file: &'static str,
    pub uses_os_keychain: bool,
    pub stores_secret_values_in_json: bool,
    pub supported_secret_kinds: Vec<SecretKind>,
    pub export_requires_user_unlock: bool,
    pub same_window: bool,
}

impl CredentialVaultPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Settings > Secrets And Credentials",
            metadata_file: ".meditor/security/credential-vault.toml",
            uses_os_keychain: true,
            stores_secret_values_in_json: false,
            supported_secret_kinds: vec![
                SecretKind::DatabasePassword,
                SecretKind::SshPrivateKey,
                SecretKind::ApiToken,
                SecretKind::ProxyCredential,
                SecretKind::GitCredential,
                SecretKind::SigningKey,
            ],
            export_requires_user_unlock: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackupItem {
    Settings,
    ProjectMetadata,
    EncryptedConnections,
    Snippets,
    LocalBugRepository,
    Reports,
    PlannerData,
    EmbeddedProjectDocumentation,
    AuditTrail,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceBackupRestorePlan {
    pub menu_path: &'static str,
    pub export_extension: &'static str,
    pub includes: Vec<BackupItem>,
    pub excludes_raw_secrets: bool,
    pub restore_uses_preview: bool,
    pub restore_creates_rollback_point: bool,
    pub same_window: bool,
}

impl WorkspaceBackupRestorePlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "File > Workspace Backup And Restore",
            export_extension: ".meditor-backup.zip",
            includes: vec![
                BackupItem::Settings,
                BackupItem::ProjectMetadata,
                BackupItem::EncryptedConnections,
                BackupItem::Snippets,
                BackupItem::LocalBugRepository,
                BackupItem::Reports,
                BackupItem::PlannerData,
                BackupItem::EmbeddedProjectDocumentation,
                BackupItem::AuditTrail,
            ],
            excludes_raw_secrets: true,
            restore_uses_preview: true,
            restore_creates_rollback_point: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditEventKind {
    Build,
    DebugCycle,
    AiSuggestion,
    AppliedChange,
    Rollback,
    DeployDocument,
    DbaAction,
    SecretAccess,
    PluginPermissionDecision,
    WorkspaceTrustDecision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditTrailPlan {
    pub menu_path: &'static str,
    pub storage_path: &'static str,
    pub append_only: bool,
    pub user_exportable: bool,
    pub supported_event_kinds: Vec<AuditEventKind>,
    pub same_window: bool,
}

impl AuditTrailPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Report > Audit Trail",
            storage_path: ".meditor/audit/audit-trail.sqlite",
            append_only: true,
            user_exportable: true,
            supported_event_kinds: vec![
                AuditEventKind::Build,
                AuditEventKind::DebugCycle,
                AuditEventKind::AiSuggestion,
                AuditEventKind::AppliedChange,
                AuditEventKind::Rollback,
                AuditEventKind::DeployDocument,
                AuditEventKind::DbaAction,
                AuditEventKind::SecretAccess,
                AuditEventKind::PluginPermissionDecision,
                AuditEventKind::WorkspaceTrustDecision,
            ],
            same_window: true,
        }
    }
}

pub fn workspace_safety_capability_names() -> &'static [&'static str] {
    &[
        "Workspace Trust Mode",
        "Local History And Recovery",
        "Secrets And Credential Vault",
        "Workspace Backup And Restore",
        "Audit Trail",
    ]
}

pub fn workspace_safety_workflow_steps() -> &'static [&'static str] {
    &[
        "Open unknown workspaces in Restricted trust until the user decides.",
        "Disable scripts, auto-builds, terminals, network calls, database/SSH connections, AI code execution, and plugins while restricted.",
        "Snapshot file saves and project metadata before risky actions.",
        "Store secret references in mEditor metadata while keeping actual secret values in the OS keychain or equivalent secure store.",
        "Export workspace backups only after previewing included data and excluding raw secret values.",
        "Record build, debug, AI, rollback, deploy, DBA, trust, secret, and plugin decisions in an append-only local audit trail.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_workspaces_start_restricted() {
        let policy = WorkspaceTrustPolicy::default_policy();
        assert_eq!(
            policy.default_level_for_unknown_projects,
            TrustLevel::Restricted
        );
        assert!(!policy.can_use(TrustLevel::Restricted, TrustCapability::RunScripts));
        assert!(policy.can_use(TrustLevel::Trusted, TrustCapability::RunScripts));
        assert!(policy.requires_user_decision);
    }

    #[test]
    fn local_history_supports_restore_and_crash_recovery() {
        let plan = LocalHistoryRecoveryPlan::default_plan();
        assert!(plan.captures_file_saves);
        assert!(plan.captures_project_metadata);
        assert!(plan.supports_session_undo);
        assert!(plan.supports_restore_previous_project_state);
    }

    #[test]
    fn vault_uses_keychain_and_never_json_secret_values() {
        let vault = CredentialVaultPlan::default_plan();
        assert!(vault.uses_os_keychain);
        assert!(!vault.stores_secret_values_in_json);
        assert!(vault
            .supported_secret_kinds
            .contains(&SecretKind::SshPrivateKey));
        assert!(vault.export_requires_user_unlock);
    }

    #[test]
    fn backup_restore_includes_project_state_but_not_raw_secrets() {
        let backup = WorkspaceBackupRestorePlan::default_plan();
        assert!(backup.includes.contains(&BackupItem::ProjectMetadata));
        assert!(backup.includes.contains(&BackupItem::LocalBugRepository));
        assert!(backup
            .includes
            .contains(&BackupItem::EmbeddedProjectDocumentation));
        assert!(backup.excludes_raw_secrets);
        assert!(backup.restore_creates_rollback_point);
    }

    #[test]
    fn audit_trail_is_append_only_and_covers_sensitive_events() {
        let audit = AuditTrailPlan::default_plan();
        assert!(audit.append_only);
        assert!(audit.user_exportable);
        assert!(audit
            .supported_event_kinds
            .contains(&AuditEventKind::DbaAction));
        assert!(audit
            .supported_event_kinds
            .contains(&AuditEventKind::PluginPermissionDecision));
    }
}
