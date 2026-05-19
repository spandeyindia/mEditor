#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContributionPoint {
    Language,
    Grammar,
    LspServer,
    DapAdapter,
    SdlcAction,
    ProjectTemplate,
    DatabaseDialect,
    JdbcDriver,
    TestAdapter,
    CoverageAdapter,
    DependencyProvider,
    SecurityScanner,
    DocumentationBundle,
    Theme,
    Keymap,
}

pub fn contribution_points() -> Vec<ContributionPoint> {
    vec![
        ContributionPoint::Language,
        ContributionPoint::Grammar,
        ContributionPoint::LspServer,
        ContributionPoint::DapAdapter,
        ContributionPoint::SdlcAction,
        ContributionPoint::ProjectTemplate,
        ContributionPoint::DatabaseDialect,
        ContributionPoint::JdbcDriver,
        ContributionPoint::TestAdapter,
        ContributionPoint::CoverageAdapter,
        ContributionPoint::DependencyProvider,
        ContributionPoint::SecurityScanner,
        ContributionPoint::DocumentationBundle,
        ContributionPoint::Theme,
        ContributionPoint::Keymap,
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub contribution_count: usize,
}

impl PluginManifest {
    pub fn is_valid_minimum(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.name.trim().is_empty()
            && !self.version.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contribution_points_cover_extension_surface() {
        let points = contribution_points();
        assert!(points.contains(&ContributionPoint::Language));
        assert!(points.contains(&ContributionPoint::SecurityScanner));
        assert!(points.contains(&ContributionPoint::JdbcDriver));
    }
}
