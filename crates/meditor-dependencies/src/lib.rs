#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManifestKind {
    CargoToml,
    MavenPom,
    Gradle,
    NpmPackageJson,
    PnpmLock,
    YarnLock,
    BunLock,
    ComposerJson,
    PythonRequirements,
    PyProjectToml,
    GoMod,
    Nuget,
    Sbom,
}

pub fn supported_manifest_kinds() -> Vec<ManifestKind> {
    vec![
        ManifestKind::CargoToml,
        ManifestKind::MavenPom,
        ManifestKind::Gradle,
        ManifestKind::NpmPackageJson,
        ManifestKind::PnpmLock,
        ManifestKind::YarnLock,
        ManifestKind::BunLock,
        ManifestKind::ComposerJson,
        ManifestKind::PythonRequirements,
        ManifestKind::PyProjectToml,
        ManifestKind::GoMod,
        ManifestKind::Nuget,
        ManifestKind::Sbom,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DependencyAction {
    Inventory,
    Graph,
    LicenseScan,
    CveScan,
    SbomGenerate,
    UpdateSuggestion,
}

pub fn dependency_actions() -> Vec<DependencyAction> {
    vec![
        DependencyAction::Inventory,
        DependencyAction::Graph,
        DependencyAction::LicenseScan,
        DependencyAction::CveScan,
        DependencyAction::SbomGenerate,
        DependencyAction::UpdateSuggestion,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifests_cover_major_package_managers() {
        let manifests = supported_manifest_kinds();
        assert!(manifests.contains(&ManifestKind::CargoToml));
        assert!(manifests.contains(&ManifestKind::NpmPackageJson));
        assert!(manifests.contains(&ManifestKind::ComposerJson));
        assert!(manifests.contains(&ManifestKind::PyProjectToml));
    }
}
