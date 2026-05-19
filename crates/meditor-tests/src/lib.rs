#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestAction {
    RunAll,
    RunSelected,
    RerunFailed,
    DebugSelected,
    Stop,
    ShowOutput,
}

pub fn test_actions() -> Vec<TestAction> {
    vec![
        TestAction::RunAll,
        TestAction::RunSelected,
        TestAction::RerunFailed,
        TestAction::DebugSelected,
        TestAction::Stop,
        TestAction::ShowOutput,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverageFormat {
    Lcov,
    CoberturaXml,
    JacocoXml,
    LlvmCoverage,
    CoveragePy,
    Istanbul,
    PhpUnit,
}

pub fn coverage_formats() -> Vec<CoverageFormat> {
    vec![
        CoverageFormat::Lcov,
        CoverageFormat::CoberturaXml,
        CoverageFormat::JacocoXml,
        CoverageFormat::LlvmCoverage,
        CoverageFormat::CoveragePy,
        CoverageFormat::Istanbul,
        CoverageFormat::PhpUnit,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_coverage_actions_cover_frozen_scope() {
        assert!(test_actions().contains(&TestAction::DebugSelected));
        assert!(coverage_formats().contains(&CoverageFormat::Lcov));
        assert!(coverage_formats().contains(&CoverageFormat::JacocoXml));
    }
}
