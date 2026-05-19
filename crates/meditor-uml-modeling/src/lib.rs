#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UmlAuthoringMode {
    TextLivePreview,
    GuiModeler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UmlDiagramKind {
    Class,
    Sequence,
    UseCase,
    Activity,
    StateMachine,
    Component,
    Deployment,
    Package,
    Object,
    Communication,
    EntityRelationship,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UmlTextDialect {
    PlantUml,
    Mermaid,
    GraphvizDot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UmlModelingSurface {
    pub id: &'static str,
    pub label: &'static str,
    pub menu_path: &'static str,
    pub open_command: &'static str,
    pub default_mode: UmlAuthoringMode,
    pub same_window: bool,
    pub supported_diagrams: Vec<UmlDiagramKind>,
    pub supported_text_dialects: Vec<UmlTextDialect>,
    pub text_output_extension: &'static str,
    pub storage_folder: &'static str,
}

impl UmlModelingSurface {
    pub fn default_surface() -> Self {
        Self {
            id: "umlModeling",
            label: "UML Modeling",
            menu_path: "Tools > UML Modeling",
            open_command: "tools.umlModeling.open",
            default_mode: UmlAuthoringMode::TextLivePreview,
            same_window: true,
            supported_diagrams: supported_diagram_kinds().to_vec(),
            supported_text_dialects: supported_text_dialects().to_vec(),
            text_output_extension: "puml",
            storage_folder: ".meditor/uml-models",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UmlTextSession {
    pub model_name: String,
    pub dialect: UmlTextDialect,
    pub source_text: String,
    pub render_action: &'static str,
    pub render_as_user_types: bool,
    pub output_file_path: String,
}

impl UmlTextSession {
    pub fn new(model_name: impl Into<String>, dialect: UmlTextDialect) -> Self {
        let model_name = model_name.into();
        Self {
            output_file_path: format!(
                ".meditor/uml-models/{}.{}",
                sanitize_name(&model_name),
                extension_for_dialect(dialect)
            ),
            model_name,
            dialect,
            source_text: String::new(),
            render_action: "tools.umlModeling.renderText",
            render_as_user_types: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UmlGuiModeler {
    pub palette_items: Vec<&'static str>,
    pub canvas_actions: Vec<&'static str>,
    pub generated_text_action: &'static str,
    pub round_trip_text_storage: bool,
}

impl UmlGuiModeler {
    pub fn default_modeler() -> Self {
        Self {
            palette_items: vec![
                "Class",
                "Interface",
                "Actor",
                "Use Case",
                "Lifeline",
                "Message",
                "State",
                "Activity",
                "Component",
                "Node",
                "Package",
                "Relationship",
                "Note",
            ],
            canvas_actions: vec![
                "Add element",
                "Connect elements",
                "Edit properties",
                "Arrange layout",
                "Group package",
                "Preview generated UML text",
                "Export diagram",
            ],
            generated_text_action: "tools.umlModeling.generateTextFromGui",
            round_trip_text_storage: true,
        }
    }
}

pub fn supported_diagram_kinds() -> &'static [UmlDiagramKind] {
    &[
        UmlDiagramKind::Class,
        UmlDiagramKind::Sequence,
        UmlDiagramKind::UseCase,
        UmlDiagramKind::Activity,
        UmlDiagramKind::StateMachine,
        UmlDiagramKind::Component,
        UmlDiagramKind::Deployment,
        UmlDiagramKind::Package,
        UmlDiagramKind::Object,
        UmlDiagramKind::Communication,
        UmlDiagramKind::EntityRelationship,
    ]
}

pub fn supported_text_dialects() -> &'static [UmlTextDialect] {
    &[
        UmlTextDialect::PlantUml,
        UmlTextDialect::Mermaid,
        UmlTextDialect::GraphvizDot,
    ]
}

pub fn uml_modeling_workflow_steps() -> &'static [&'static str] {
    &[
        "Open UML Modeling from the Tools menu in the same window.",
        "Let the user choose text authoring or GUI modeling.",
        "In text mode, render the UML diagram live as the user types.",
        "In GUI mode, let the user create UML diagrams with a palette, canvas, properties, and connectors.",
        "Generate UML text from GUI diagrams for reference, storage, diff, and version control.",
        "Store UML source text under the mEditor working folder.",
        "Allow exported diagrams to be embedded into generated documentation.",
    ]
}

fn extension_for_dialect(dialect: UmlTextDialect) -> &'static str {
    match dialect {
        UmlTextDialect::PlantUml => "puml",
        UmlTextDialect::Mermaid => "mmd",
        UmlTextDialect::GraphvizDot => "dot",
    }
}

fn sanitize_name(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    sanitized.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uml_surface_opens_under_tools_and_same_window() {
        let surface = UmlModelingSurface::default_surface();
        assert_eq!(surface.menu_path, "Tools > UML Modeling");
        assert_eq!(surface.open_command, "tools.umlModeling.open");
        assert!(surface.same_window);
        assert!(surface
            .supported_text_dialects
            .contains(&UmlTextDialect::PlantUml));
    }

    #[test]
    fn text_mode_renders_as_user_types_and_stores_source() {
        let session = UmlTextSession::new("Order Flow", UmlTextDialect::PlantUml);
        assert!(session.render_as_user_types);
        assert_eq!(
            session.output_file_path,
            ".meditor/uml-models/order-flow.puml"
        );
    }

    #[test]
    fn gui_mode_round_trips_to_uml_text() {
        let modeler = UmlGuiModeler::default_modeler();
        assert!(modeler.palette_items.contains(&"Class"));
        assert!(modeler.canvas_actions.contains(&"Connect elements"));
        assert_eq!(
            modeler.generated_text_action,
            "tools.umlModeling.generateTextFromGui"
        );
        assert!(modeler.round_trip_text_storage);
    }

    #[test]
    fn workflows_cover_text_gui_and_documentation_export() {
        assert!(supported_diagram_kinds().contains(&UmlDiagramKind::Sequence));
        assert!(supported_diagram_kinds().contains(&UmlDiagramKind::EntityRelationship));
        assert!(uml_modeling_workflow_steps()
            .iter()
            .any(|step| step.contains("render the UML diagram live")));
        assert!(uml_modeling_workflow_steps()
            .iter()
            .any(|step| step.contains("Generate UML text from GUI")));
    }
}
