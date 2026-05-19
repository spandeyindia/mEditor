use meditor_platform::{CpuArchitecture, OperatingSystem, PlatformProfile};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LauncherKind {
    WindowsExeOrScript,
    MacAppBundle,
    LinuxShellOrAppImage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliverableScope {
    MonoPlatform,
    MultiPlatform,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliverableKind {
    SourceArchive,
    GenericJar,
    NativeInstaller,
    PortableBundle,
    WebStaticBundle,
    ContainerImage,
    DatabaseMigrationPackage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectDeliverableDefinition {
    pub name: String,
    pub scope: DeliverableScope,
    pub kind: DeliverableKind,
    pub target_platforms: Vec<PlatformProfile>,
    pub build_action: &'static str,
}

impl ProjectDeliverableDefinition {
    pub fn multi_platform(
        name: impl Into<String>,
        kind: DeliverableKind,
        target_platforms: Vec<PlatformProfile>,
    ) -> Self {
        Self {
            name: name.into(),
            scope: DeliverableScope::MultiPlatform,
            kind,
            target_platforms,
            build_action: "sdlc.package.buildDistributable",
        }
    }

    pub fn mono_platform(
        name: impl Into<String>,
        kind: DeliverableKind,
        target_platform: PlatformProfile,
    ) -> Self {
        Self {
            name: name.into(),
            scope: DeliverableScope::MonoPlatform,
            kind,
            target_platforms: vec![target_platform],
            build_action: "sdlc.package.buildDistributable",
        }
    }
}

pub fn distributable_build_steps() -> &'static [&'static str] {
    &[
        "Ask the user whether the deliverable is mono-platform or multi-platform.",
        "Collect target operating systems, CPU architectures, packaging kind, runtime requirements, and signing needs.",
        "Build source archive, generic JAR, native installer, portable bundle, web static bundle, container image, or database migration package as requested.",
        "For multi-platform builds, create one artifact per selected platform and keep a manifest tying them to the same build.",
        "For mono-platform builds, optimize the artifact for the selected platform only.",
        "Preserve project metadata and generated deliverable evidence under the mEditor working folder.",
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LauncherSpec {
    pub kind: LauncherKind,
    pub display_name: &'static str,
    pub platform: PlatformProfile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericJarPackagingPlan {
    pub shared_core_artifact: &'static str,
    pub launchers: Vec<LauncherSpec>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateSourceConfig {
    pub id: &'static str,
    pub public_git_url: Option<&'static str>,
    pub check_action: &'static str,
    pub prompt_before_install: bool,
    pub preserve_local_data: bool,
    pub preserve_user_setup: bool,
    pub rollback_supported: bool,
}

impl UpdateSourceConfig {
    pub fn awaiting_public_git_url() -> Self {
        Self {
            id: "updates.publicGit",
            public_git_url: None,
            check_action: "help.about.checkForUpdates",
            prompt_before_install: true,
            preserve_local_data: true,
            preserve_user_setup: true,
            rollback_supported: true,
        }
    }

    pub fn is_safe_update_contract(&self) -> bool {
        self.prompt_before_install
            && self.preserve_local_data
            && self.preserve_user_setup
            && self.rollback_supported
    }
}

pub fn update_preserved_paths() -> &'static [&'static str] {
    &[
        ".meditor",
        ".meditor/security",
        ".meditor/debug-staging",
        ".meditor/debug-reports",
        ".meditor/deployment-docs",
        ".meditor/project-docs",
        "connections",
        "setup",
        "settings",
    ]
}

pub fn update_flow_steps() -> &'static [&'static str] {
    &[
        "Open update check from About mEditor.",
        "Read the configured public Git release or build source.",
        "Compare the latest available version against the installed version.",
        "Show release notes and ask the user before downloading or installing.",
        "Back up local data, setup, connections, SQLite repositories, settings, and generated documentation.",
        "Install the new application bits without overwriting user data.",
        "Validate the updated application starts and can read existing setup.",
        "Offer rollback if validation or user acceptance fails.",
    ]
}

impl GenericJarPackagingPlan {
    pub fn ver_1_default() -> Self {
        Self {
            shared_core_artifact: "mEditor.jar",
            launchers: vec![
                launcher(
                    LauncherKind::WindowsExeOrScript,
                    "Windows launcher",
                    OperatingSystem::Windows,
                    CpuArchitecture::X64,
                ),
                launcher(
                    LauncherKind::WindowsExeOrScript,
                    "Windows ARM64 launcher",
                    OperatingSystem::Windows,
                    CpuArchitecture::Arm64,
                ),
                launcher(
                    LauncherKind::MacAppBundle,
                    "macOS Intel launcher",
                    OperatingSystem::MacOS,
                    CpuArchitecture::X64,
                ),
                launcher(
                    LauncherKind::MacAppBundle,
                    "macOS Apple Silicon launcher",
                    OperatingSystem::MacOS,
                    CpuArchitecture::Arm64,
                ),
                launcher(
                    LauncherKind::LinuxShellOrAppImage,
                    "Linux x64 launcher",
                    OperatingSystem::Linux,
                    CpuArchitecture::X64,
                ),
                launcher(
                    LauncherKind::LinuxShellOrAppImage,
                    "Linux ARM64 launcher",
                    OperatingSystem::Linux,
                    CpuArchitecture::Arm64,
                ),
            ],
        }
    }
}

fn launcher(
    kind: LauncherKind,
    display_name: &'static str,
    os: OperatingSystem,
    arch: CpuArchitecture,
) -> LauncherSpec {
    LauncherSpec {
        kind,
        display_name,
        platform: PlatformProfile { os, arch },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_jar_plan_keeps_one_shared_core() {
        let plan = GenericJarPackagingPlan::ver_1_default();
        assert_eq!(plan.shared_core_artifact, "mEditor.jar");
        assert_eq!(plan.launchers.len(), 6);
    }

    #[test]
    fn generic_jar_plan_includes_core_laptop_platforms() {
        let plan = GenericJarPackagingPlan::ver_1_default();
        assert!(plan
            .launchers
            .iter()
            .any(|launcher| launcher.platform.os == OperatingSystem::MacOS
                && launcher.platform.arch == CpuArchitecture::Arm64));
        assert!(plan
            .launchers
            .iter()
            .all(|launcher| launcher.platform.is_ver_1_laptop_target()));
    }

    #[test]
    fn about_update_contract_waits_for_public_git_and_preserves_user_data() {
        let config = UpdateSourceConfig::awaiting_public_git_url();
        assert_eq!(config.check_action, "help.about.checkForUpdates");
        assert!(config.public_git_url.is_none());
        assert!(config.is_safe_update_contract());
        assert!(update_preserved_paths().contains(&".meditor"));
        assert!(update_preserved_paths().contains(&"connections"));
        assert!(update_flow_steps()
            .iter()
            .any(|step| step.contains("without overwriting user data")));
    }

    #[test]
    fn deliverables_support_mono_and_multi_platform_builds() {
        let mac = PlatformProfile {
            os: OperatingSystem::MacOS,
            arch: CpuArchitecture::Arm64,
        };
        let linux = PlatformProfile {
            os: OperatingSystem::Linux,
            arch: CpuArchitecture::X64,
        };
        let mono = ProjectDeliverableDefinition::mono_platform(
            "mac-app",
            DeliverableKind::NativeInstaller,
            mac,
        );
        let multi = ProjectDeliverableDefinition::multi_platform(
            "desktop-suite",
            DeliverableKind::PortableBundle,
            vec![mac, linux],
        );
        assert_eq!(mono.scope, DeliverableScope::MonoPlatform);
        assert_eq!(mono.target_platforms.len(), 1);
        assert_eq!(multi.scope, DeliverableScope::MultiPlatform);
        assert_eq!(multi.target_platforms.len(), 2);
        assert!(distributable_build_steps()
            .iter()
            .any(|step| step.contains("mono-platform or multi-platform")));
    }
}
