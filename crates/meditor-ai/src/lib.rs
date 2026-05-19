#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnowledgeSourceFormat {
    Pdf,
    Epub,
    Mobi,
    Doc,
    Docx,
    Txt,
    Xls,
    Xlsx,
    Csv,
    Markdown,
    SourceRepository,
}

pub fn supported_knowledge_formats() -> Vec<KnowledgeSourceFormat> {
    vec![
        KnowledgeSourceFormat::Pdf,
        KnowledgeSourceFormat::Epub,
        KnowledgeSourceFormat::Mobi,
        KnowledgeSourceFormat::Doc,
        KnowledgeSourceFormat::Docx,
        KnowledgeSourceFormat::Txt,
        KnowledgeSourceFormat::Xls,
        KnowledgeSourceFormat::Xlsx,
        KnowledgeSourceFormat::Csv,
        KnowledgeSourceFormat::Markdown,
        KnowledgeSourceFormat::SourceRepository,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LearningMode {
    RetrievalOnly,
    AdapterOrFineTune,
    FullRetraining,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssistantCapability {
    ExplainCode,
    SuggestFix,
    GenerateTests,
    RefactorPreview,
    SecurityRemediation,
    CompilerErrorDebugging,
    SpecToArchitecture,
    SpecToBacklog,
    AskAgainstSpecification,
}

pub fn assistant_capabilities() -> Vec<AssistantCapability> {
    vec![
        AssistantCapability::ExplainCode,
        AssistantCapability::SuggestFix,
        AssistantCapability::GenerateTests,
        AssistantCapability::RefactorPreview,
        AssistantCapability::SecurityRemediation,
        AssistantCapability::CompilerErrorDebugging,
        AssistantCapability::SpecToArchitecture,
        AssistantCapability::SpecToBacklog,
        AssistantCapability::AskAgainstSpecification,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiLearningEventKind {
    KnowledgeIngestion,
    CompilerDebugCycle,
    SecurityFixCycle,
    UserAcceptedSuggestion,
    UserRejectedSuggestion,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiLearningEvent {
    pub id: String,
    pub kind: AiLearningEventKind,
    pub domain: String,
    pub error_count: u32,
    pub warning_count: u32,
    pub suggestion_count: u32,
    pub accepted_fix_count: u32,
    pub retraining_required: bool,
}

impl AiLearningEvent {
    pub fn debug_cycle(
        id: impl Into<String>,
        domain: impl Into<String>,
        error_count: u32,
        warning_count: u32,
        retraining_required: bool,
    ) -> Self {
        Self {
            id: id.into(),
            kind: AiLearningEventKind::CompilerDebugCycle,
            domain: domain.into(),
            error_count,
            warning_count,
            suggestion_count: error_count + warning_count,
            accepted_fix_count: 0,
            retraining_required,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfLearningAiServicePlan {
    pub id: &'static str,
    pub label: &'static str,
    pub embedded_service: bool,
    pub stores_training_events_locally: bool,
    pub supports_retrieval_updates: bool,
    pub supports_adapter_or_fine_tune_jobs: bool,
    pub supports_full_retraining_jobs: bool,
    pub retrain_after_debug_cycle: bool,
    pub human_review_required_for_code_writes: bool,
}

impl SelfLearningAiServicePlan {
    pub fn ver_1_contract() -> Self {
        Self {
            id: "ai.selfLearningService",
            label: "Self-Learning AI/ML Service",
            embedded_service: true,
            stores_training_events_locally: true,
            supports_retrieval_updates: true,
            supports_adapter_or_fine_tune_jobs: true,
            supports_full_retraining_jobs: true,
            retrain_after_debug_cycle: true,
            human_review_required_for_code_writes: true,
        }
    }

    pub fn is_contract_not_runtime(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knowledge_formats_cover_requested_documents() {
        let formats = supported_knowledge_formats();
        for format in [
            KnowledgeSourceFormat::Pdf,
            KnowledgeSourceFormat::Epub,
            KnowledgeSourceFormat::Mobi,
            KnowledgeSourceFormat::Doc,
            KnowledgeSourceFormat::Txt,
            KnowledgeSourceFormat::Xls,
        ] {
            assert!(formats.contains(&format));
        }
    }

    #[test]
    fn assistant_supports_spec_to_system_flow() {
        let capabilities = assistant_capabilities();
        assert!(capabilities.contains(&AssistantCapability::SpecToArchitecture));
        assert!(capabilities.contains(&AssistantCapability::AskAgainstSpecification));
        assert!(capabilities.contains(&AssistantCapability::CompilerErrorDebugging));
    }

    #[test]
    fn self_learning_service_contract_requires_debug_cycle_retraining() {
        let plan = SelfLearningAiServicePlan::ver_1_contract();
        assert!(plan.embedded_service);
        assert!(plan.retrain_after_debug_cycle);
        assert!(plan.stores_training_events_locally);
        assert!(plan.is_contract_not_runtime());
        let event = AiLearningEvent::debug_cycle("cycle-1", "rust", 2, 1, true);
        assert_eq!(event.kind, AiLearningEventKind::CompilerDebugCycle);
        assert_eq!(event.suggestion_count, 3);
        assert!(event.retraining_required);
    }
}
