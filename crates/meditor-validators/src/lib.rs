#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidatorKind {
    Xml,
    Json,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationCapability {
    Syntax,
    Schema,
    Formatting,
    Diagnostics,
    QuickFixHints,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatorToolPlan {
    pub kind: ValidatorKind,
    pub menu_path: &'static str,
    pub supported_file_extensions: Vec<&'static str>,
    pub capabilities: Vec<ValidationCapability>,
    pub can_validate_open_file: bool,
    pub can_validate_selected_text: bool,
    pub same_window: bool,
}

impl ValidatorToolPlan {
    pub fn xml() -> Self {
        Self {
            kind: ValidatorKind::Xml,
            menu_path: "Tools > XML Validator",
            supported_file_extensions: vec!["xml", "xsd", "xsl", "xslt", "wsdl", "pom"],
            capabilities: default_capabilities(),
            can_validate_open_file: true,
            can_validate_selected_text: true,
            same_window: true,
        }
    }

    pub fn json() -> Self {
        Self {
            kind: ValidatorKind::Json,
            menu_path: "Tools > JSON Validator",
            supported_file_extensions: vec!["json", "jsonc", "schema.json"],
            capabilities: default_capabilities(),
            can_validate_open_file: true,
            can_validate_selected_text: true,
            same_window: true,
        }
    }
}

fn default_capabilities() -> Vec<ValidationCapability> {
    vec![
        ValidationCapability::Syntax,
        ValidationCapability::Schema,
        ValidationCapability::Formatting,
        ValidationCapability::Diagnostics,
        ValidationCapability::QuickFixHints,
    ]
}

pub fn validator_tools() -> Vec<ValidatorToolPlan> {
    vec![ValidatorToolPlan::xml(), ValidatorToolPlan::json()]
}

pub fn validator_workflow_steps() -> &'static [&'static str] {
    &[
        "Open XML Validator and JSON Validator from Tools.",
        "Validate the active editor tab, a selected text range, or a chosen file.",
        "Show syntax and schema diagnostics in the same-window Problems panel.",
        "Offer formatting and quick-fix hints without changing files until the user approves.",
        "Write validation reports into the project documentation/report surfaces when requested.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_and_json_validators_are_separate_tools() {
        let tools = validator_tools();
        assert_eq!(tools.len(), 2);
        assert!(tools
            .iter()
            .any(|tool| tool.menu_path == "Tools > XML Validator"));
        assert!(tools
            .iter()
            .any(|tool| tool.menu_path == "Tools > JSON Validator"));
        assert!(tools.iter().all(|tool| tool.same_window));
    }

    #[test]
    fn validators_support_schema_diagnostics_and_approval_gated_fixes() {
        let xml = ValidatorToolPlan::xml();
        let json = ValidatorToolPlan::json();
        assert!(xml.capabilities.contains(&ValidationCapability::Schema));
        assert!(json
            .capabilities
            .contains(&ValidationCapability::Diagnostics));
        assert!(xml.can_validate_open_file);
        assert!(json.can_validate_selected_text);
        assert!(validator_workflow_steps()
            .iter()
            .any(|step| step.contains("until the user approves")));
    }
}
