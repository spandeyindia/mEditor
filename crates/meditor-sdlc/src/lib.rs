#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdlcActionKind {
    Build,
    Clean,
    Rebuild,
    Test,
    Deploy,
    Launch,
    Debug,
    Package,
    Publish,
    Stop,
    Rerun,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SdlcAction {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: SdlcActionKind,
    pub executable_id: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionReadiness {
    Ready,
    NeedsExecutableConfiguration {
        executable_id: String,
        configure_action: &'static str,
    },
}

pub fn standard_actions() -> Vec<SdlcAction> {
    vec![
        action("sdlc.build", "Build", SdlcActionKind::Build, Some("cargo")),
        action("sdlc.clean", "Clean", SdlcActionKind::Clean, Some("cargo")),
        action(
            "sdlc.rebuild",
            "Rebuild",
            SdlcActionKind::Rebuild,
            Some("cargo"),
        ),
        action("sdlc.test", "Test", SdlcActionKind::Test, Some("cargo")),
        action("sdlc.deploy", "Deploy", SdlcActionKind::Deploy, None),
        action("sdlc.launch", "Launch", SdlcActionKind::Launch, None),
        action("sdlc.debug", "Debug", SdlcActionKind::Debug, None),
        action("sdlc.package", "Package", SdlcActionKind::Package, None),
        action("sdlc.publish", "Publish", SdlcActionKind::Publish, None),
        action("sdlc.stop", "Stop", SdlcActionKind::Stop, None),
        action("sdlc.rerun", "Rerun", SdlcActionKind::Rerun, None),
    ]
}

fn action(
    id: &'static str,
    label: &'static str,
    kind: SdlcActionKind,
    executable_id: Option<&'static str>,
) -> SdlcAction {
    SdlcAction {
        id,
        label,
        kind,
        executable_id,
    }
}

pub fn readiness_for(
    action: &SdlcAction,
    profile: Option<&meditor_toolchains::ToolchainProfile>,
) -> ActionReadiness {
    match (action.executable_id, profile) {
        (Some(executable_id), Some(profile))
            if profile.executable_id == executable_id && profile.path.is_some() =>
        {
            ActionReadiness::Ready
        }
        (Some(executable_id), _) => ActionReadiness::NeedsExecutableConfiguration {
            executable_id: executable_id.to_string(),
            configure_action: "Settings > Languages & Toolchains > Configure Executable",
        },
        (None, _) => ActionReadiness::Ready,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_actions_cover_frozen_sdlc_scope() {
        let actions = standard_actions();
        for kind in [
            SdlcActionKind::Build,
            SdlcActionKind::Clean,
            SdlcActionKind::Rebuild,
            SdlcActionKind::Test,
            SdlcActionKind::Deploy,
            SdlcActionKind::Launch,
            SdlcActionKind::Debug,
            SdlcActionKind::Package,
            SdlcActionKind::Publish,
            SdlcActionKind::Stop,
            SdlcActionKind::Rerun,
        ] {
            assert!(actions.iter().any(|action| action.kind == kind));
        }
    }

    #[test]
    fn action_reports_missing_executable_configuration() {
        let build = standard_actions()
            .into_iter()
            .find(|action| action.kind == SdlcActionKind::Build)
            .unwrap();
        let readiness = readiness_for(&build, None);
        assert!(matches!(
            readiness,
            ActionReadiness::NeedsExecutableConfiguration { .. }
        ));
    }

    #[test]
    fn action_is_ready_when_executable_profile_is_configured() {
        let build = standard_actions()
            .into_iter()
            .find(|action| action.kind == SdlcActionKind::Build)
            .unwrap();
        let profile = meditor_toolchains::ToolchainProfile::configured("cargo", "/usr/bin/cargo");
        assert_eq!(
            readiness_for(&build, Some(&profile)),
            ActionReadiness::Ready
        );
    }
}
