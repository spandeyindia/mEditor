#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupTaskKind {
    CheckSqlite,
    InstallSqliteIfMissing,
    CreateSecurityDirectory,
    InitializeCvssRepositorySchema,
    DetectLanguageTooling,
    PresentLanguageSupportCatalog,
    InstallMissingLanguageTooling,
    ValidateToolingNetworkOrProxy,
    RefreshEditorLanguageServices,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetupTask {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: SetupTaskKind,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetupPlan {
    pub id: &'static str,
    pub label: &'static str,
    pub one_click_entrypoints: Vec<&'static str>,
    pub tasks: Vec<SetupTask>,
}

impl SetupPlan {
    pub fn sqlite_one_click() -> Self {
        Self {
            id: "setup.sqlite.oneClick",
            label: "One-click SQLite setup",
            one_click_entrypoints: vec![
                meditor_cvss_repository::SQLITE_TOP_LEVEL_UNIX_SETUP,
                meditor_cvss_repository::SQLITE_TOP_LEVEL_MAC_SETUP,
                meditor_cvss_repository::SQLITE_TOP_LEVEL_WINDOWS_SETUP,
            ],
            tasks: vec![
                task(
                    "setup.sqlite.check",
                    "Check SQLite",
                    SetupTaskKind::CheckSqlite,
                ),
                task(
                    "setup.sqlite.installIfMissing",
                    "Install SQLite if missing",
                    SetupTaskKind::InstallSqliteIfMissing,
                ),
                task(
                    "setup.sqlite.createSecurityDirectory",
                    "Create security data folder",
                    SetupTaskKind::CreateSecurityDirectory,
                ),
                task(
                    "setup.sqlite.initializeSchema",
                    "Initialize CVSS repository schema",
                    SetupTaskKind::InitializeCvssRepositorySchema,
                ),
            ],
        }
    }

    pub fn creates_required_data_structures(&self) -> bool {
        self.tasks
            .iter()
            .any(|task| task.kind == SetupTaskKind::InitializeCvssRepositorySchema)
    }

    pub fn language_support() -> Self {
        Self {
            id: "setup.languageSupport",
            label: "Programming Language Support",
            one_click_entrypoints: Vec::new(),
            tasks: vec![
                task(
                    "setup.languageSupport.detectTooling",
                    "Detect installed language tooling",
                    SetupTaskKind::DetectLanguageTooling,
                ),
                task(
                    "setup.languageSupport.presentCatalog",
                    "Show supported languages and active/missing state",
                    SetupTaskKind::PresentLanguageSupportCatalog,
                ),
                task(
                    "setup.languageSupport.installMissing",
                    "Install missing open-source tooling",
                    SetupTaskKind::InstallMissingLanguageTooling,
                ),
                task(
                    "setup.languageSupport.validateNetworkOrProxy",
                    "Validate network or proxy for tooling downloads",
                    SetupTaskKind::ValidateToolingNetworkOrProxy,
                ),
                task(
                    "setup.languageSupport.refreshServices",
                    "Refresh editor language services",
                    SetupTaskKind::RefreshEditorLanguageServices,
                ),
            ],
        }
    }

    pub fn supports_language_tooling_installation(&self) -> bool {
        self.tasks
            .iter()
            .any(|task| task.kind == SetupTaskKind::InstallMissingLanguageTooling)
            && self
                .tasks
                .iter()
                .any(|task| task.kind == SetupTaskKind::ValidateToolingNetworkOrProxy)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageSupportSetupSurface {
    pub menu_path: &'static str,
    pub open_command: &'static str,
    pub catalog_size: usize,
    pub executable_probe_count: usize,
    pub install_flow_steps: usize,
    pub network_dialog: meditor_toolchains::ToolingNetworkDialog,
}

impl LanguageSupportSetupSurface {
    pub fn default_surface() -> Self {
        Self {
            menu_path: "Setup > Programming Language Support",
            open_command: "setup.languageSupport.open",
            catalog_size: meditor_toolchains::supported_language_catalog().len(),
            executable_probe_count: meditor_toolchains::supported_executable_ids().len(),
            install_flow_steps: meditor_toolchains::language_tooling_install_flow_steps().len(),
            network_dialog: meditor_toolchains::default_tooling_network_dialog(),
        }
    }
}

fn task(id: &'static str, label: &'static str, kind: SetupTaskKind) -> SetupTask {
    SetupTask {
        id,
        label,
        kind,
        required: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_click_setup_has_cross_platform_entrypoints() {
        let plan = SetupPlan::sqlite_one_click();
        assert!(plan.one_click_entrypoints.contains(&"setup/setup.sh"));
        assert!(plan.one_click_entrypoints.contains(&"setup/setup.command"));
        assert!(plan.one_click_entrypoints.contains(&"setup/setup.cmd"));
    }

    #[test]
    fn one_click_setup_creates_cvss_data_structures() {
        let plan = SetupPlan::sqlite_one_click();
        assert!(plan.creates_required_data_structures());
        assert_eq!(plan.tasks.len(), 4);
    }

    #[test]
    fn language_support_setup_installs_tooling_and_handles_proxy_validation() {
        let plan = SetupPlan::language_support();
        assert_eq!(plan.id, "setup.languageSupport");
        assert_eq!(plan.tasks.len(), 5);
        assert!(plan.supports_language_tooling_installation());
        let surface = LanguageSupportSetupSurface::default_surface();
        assert_eq!(surface.menu_path, "Setup > Programming Language Support");
        assert!(surface.catalog_size >= 60);
        assert!(surface.executable_probe_count >= 50);
        assert!(surface.network_dialog.fields.contains(&"proxy_url"));
    }
}
