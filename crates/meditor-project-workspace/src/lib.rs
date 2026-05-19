#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectNodeKind {
    Portfolio,
    MajorProject,
    SubProject,
    Module,
    SourceRoot,
    ResourceRoot,
    TestRoot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanningItemKind {
    Portfolio,
    Program,
    Project,
    Deliverable,
    Milestone,
    Epic,
    Story,
    Task,
    SubTask,
    Risk,
    Issue,
    Dependency,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanningStatus {
    NotStarted,
    InProgress,
    Blocked,
    Done,
    Deferred,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorProjectCreationPath {
    DirectNewProjectWizard,
    FromProjectPlanner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlannerCreationPath {
    ManualUiEntry,
    ExcelTemplateImport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanningMethodology {
    ObjectiveMilestoneTask,
    ScrumEpicStoryTask,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlannerHealth {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectCostTracking {
    pub currency_code: String,
    pub planned_cost: f64,
    pub actual_cost: f64,
    pub forecast_cost: f64,
}

impl ProjectCostTracking {
    pub fn health(&self) -> PlannerHealth {
        if self.forecast_cost > self.planned_cost * 1.15 {
            PlannerHealth::Red
        } else if self.forecast_cost > self.planned_cost * 1.05 {
            PlannerHealth::Yellow
        } else {
            PlannerHealth::Green
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectWorkspaceModel {
    pub id: &'static str,
    pub label: &'static str,
    pub menu_path: &'static str,
    pub project_about_menu_path: &'static str,
    pub project_help_menu_path: &'static str,
    pub create_project_action: &'static str,
    pub create_subproject_action: &'static str,
    pub project_metadata_file: &'static str,
    pub project_metadata_folder: &'static str,
    pub planning_metadata_file: &'static str,
    pub embedded_documentation_folder: &'static str,
    pub supports_major_project_with_subprojects: bool,
    pub supports_netbeans_style_metadata: bool,
    pub embeds_about_and_help_documentation: bool,
    pub creation_paths: Vec<EditorProjectCreationPath>,
    pub same_window: bool,
}

impl ProjectWorkspaceModel {
    pub fn default_model() -> Self {
        Self {
            id: "projectWorkspace",
            label: "Project Workspace",
            menu_path: "File > New Project",
            project_about_menu_path: "Help > About Project",
            project_help_menu_path: "Help > Project Help",
            create_project_action: "project.create",
            create_subproject_action: "project.createSubProject",
            project_metadata_file: ".meditor/project-workspace.toml",
            project_metadata_folder: ".meditor/project",
            planning_metadata_file: ".meditor/project-planning/project-plan.toml",
            embedded_documentation_folder: ".meditor/project/docs",
            supports_major_project_with_subprojects: true,
            supports_netbeans_style_metadata: true,
            embeds_about_and_help_documentation: true,
            creation_paths: vec![
                EditorProjectCreationPath::DirectNewProjectWizard,
                EditorProjectCreationPath::FromProjectPlanner,
            ],
            same_window: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectMetadataPlan {
    pub metadata_folder: &'static str,
    pub project_file: &'static str,
    pub modules_file: &'static str,
    pub planning_file: &'static str,
    pub about_file: &'static str,
    pub help_index_file: &'static str,
    pub help_topics_file: &'static str,
    pub private_user_file: &'static str,
    pub generated_from_import_file: &'static str,
    pub supports_major_project: bool,
    pub supports_subprojects: bool,
    pub supports_portfolio_planning: bool,
    pub requires_embedded_about_help: bool,
}

impl ProjectMetadataPlan {
    pub fn netbeans_style_for_meditor() -> Self {
        Self {
            metadata_folder: ".meditor/project",
            project_file: ".meditor/project/project.toml",
            modules_file: ".meditor/project/modules.toml",
            planning_file: ".meditor/project/planning.toml",
            about_file: ".meditor/project/docs/about.md",
            help_index_file: ".meditor/project/docs/help-index.md",
            help_topics_file: ".meditor/project/docs/help-topics.toml",
            private_user_file: ".meditor/project/private-user.toml",
            generated_from_import_file: ".meditor/project/import-report.toml",
            supports_major_project: true,
            supports_subprojects: true,
            supports_portfolio_planning: true,
            requires_embedded_about_help: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedProjectDocumentationModel {
    pub id: &'static str,
    pub label: &'static str,
    pub about_menu_path: &'static str,
    pub help_menu_path: &'static str,
    pub docs_folder: &'static str,
    pub about_file: &'static str,
    pub help_index_file: &'static str,
    pub help_topics_file: &'static str,
    pub opens_in_same_window_tab: bool,
    pub generated_for_every_project: bool,
    pub editable_by_user: bool,
}

impl EmbeddedProjectDocumentationModel {
    pub fn default_model() -> Self {
        Self {
            id: "embeddedProjectDocumentation",
            label: "Embedded Project About And Help",
            about_menu_path: "Help > About Project",
            help_menu_path: "Help > Project Help",
            docs_folder: ".meditor/project/docs",
            about_file: ".meditor/project/docs/about.md",
            help_index_file: ".meditor/project/docs/help-index.md",
            help_topics_file: ".meditor/project/docs/help-topics.toml",
            opens_in_same_window_tab: true,
            generated_for_every_project: true,
            editable_by_user: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectPlanningItem {
    pub id: String,
    pub title: String,
    pub kind: PlanningItemKind,
    pub status: PlanningStatus,
    pub owner: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub progress_percent: u8,
    pub parent_id: Option<String>,
}

impl ProjectPlanningItem {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        kind: PlanningItemKind,
        parent_id: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            kind,
            status: PlanningStatus::NotStarted,
            owner: None,
            start_date: None,
            due_date: None,
            progress_percent: 0,
            parent_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectPlanningModel {
    pub id: &'static str,
    pub label: &'static str,
    pub menu_path: &'static str,
    pub storage_path: &'static str,
    pub linked_project_metadata_file: &'static str,
    pub supported_items: Vec<PlanningItemKind>,
    pub ai_background_update_action: &'static str,
    pub create_editor_project_action: &'static str,
    pub export_excel_template_action: &'static str,
    pub import_excel_template_action: &'static str,
    pub timezone_required_before_timeline_calculation: bool,
    pub asks_user_for_methodology: bool,
    pub tracks_cost: bool,
    pub dashboard_enabled: bool,
    pub supported_methodologies: Vec<PlanningMethodology>,
    pub creation_paths: Vec<PlannerCreationPath>,
    pub ai_updates_progress_from_user_activity: bool,
}

impl ProjectPlanningModel {
    pub fn default_model() -> Self {
        Self {
            id: "projectPlanning",
            label: "Project Planner",
            menu_path: "Tools > Project Planner",
            storage_path: ".meditor/project-planning/project-plan.sqlite",
            linked_project_metadata_file: ".meditor/project/project.toml",
            supported_items: supported_planning_items().to_vec(),
            ai_background_update_action: "project.planning.aiUpdateProgress",
            create_editor_project_action: "project.planning.createEditorProject",
            export_excel_template_action: "project.planning.exportExcelTemplate",
            import_excel_template_action: "project.planning.importExcelTemplate",
            timezone_required_before_timeline_calculation: true,
            asks_user_for_methodology: true,
            tracks_cost: true,
            dashboard_enabled: true,
            supported_methodologies: vec![
                PlanningMethodology::ObjectiveMilestoneTask,
                PlanningMethodology::ScrumEpicStoryTask,
            ],
            creation_paths: vec![
                PlannerCreationPath::ManualUiEntry,
                PlannerCreationPath::ExcelTemplateImport,
            ],
            ai_updates_progress_from_user_activity: true,
        }
    }
}

pub fn planning_hierarchy_for(methodology: PlanningMethodology) -> &'static [PlanningItemKind] {
    match methodology {
        PlanningMethodology::ObjectiveMilestoneTask => &[
            PlanningItemKind::Project,
            PlanningItemKind::Deliverable,
            PlanningItemKind::Milestone,
            PlanningItemKind::Task,
            PlanningItemKind::SubTask,
        ],
        PlanningMethodology::ScrumEpicStoryTask => &[
            PlanningItemKind::Project,
            PlanningItemKind::Epic,
            PlanningItemKind::Story,
            PlanningItemKind::Task,
            PlanningItemKind::SubTask,
        ],
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannerExcelTemplate {
    pub file_name: &'static str,
    pub sheets: Vec<&'static str>,
    pub required_columns: Vec<&'static str>,
    pub timezone_column: &'static str,
}

impl PlannerExcelTemplate {
    pub fn default_template() -> Self {
        Self {
            file_name: "mEditor-project-plan-template.xlsx",
            sheets: vec![
                "Portfolio",
                "Projects",
                "Deliverables",
                "Milestones",
                "Epics",
                "Stories",
                "Tasks",
                "Risks",
                "Dependencies",
            ],
            required_columns: vec![
                "item_id",
                "parent_id",
                "item_type",
                "title",
                "owner",
                "start_datetime",
                "end_datetime",
                "timezone",
                "status",
                "progress_percent",
            ],
            timezone_column: "timezone",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannerProjectCreationRequest {
    pub planning_item_id: String,
    pub portfolio_or_project_name: String,
    pub tech_stack: String,
    pub required_functionality: Vec<String>,
    pub deliverables: Vec<meditor_packaging::ProjectDeliverableDefinition>,
    pub create_major_project: bool,
    pub create_subprojects: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannerProjectCreationPlan {
    pub editor_project_name: String,
    pub linked_planning_item_id: String,
    pub project_metadata_file: &'static str,
    pub create_project_action: &'static str,
    pub create_subproject_action: &'static str,
    pub tech_stack: String,
    pub required_functionality: Vec<String>,
    pub deliverables: Vec<meditor_packaging::ProjectDeliverableDefinition>,
}

impl PlannerProjectCreationPlan {
    pub fn from_request(request: PlannerProjectCreationRequest) -> Self {
        Self {
            editor_project_name: request.portfolio_or_project_name,
            linked_planning_item_id: request.planning_item_id,
            project_metadata_file: ".meditor/project/project.toml",
            create_project_action: "project.create",
            create_subproject_action: "project.createSubProject",
            tech_stack: request.tech_stack,
            required_functionality: request.required_functionality,
            deliverables: request.deliverables,
        }
    }
}

pub fn supported_project_node_kinds() -> &'static [ProjectNodeKind] {
    &[
        ProjectNodeKind::Portfolio,
        ProjectNodeKind::MajorProject,
        ProjectNodeKind::SubProject,
        ProjectNodeKind::Module,
        ProjectNodeKind::SourceRoot,
        ProjectNodeKind::ResourceRoot,
        ProjectNodeKind::TestRoot,
    ]
}

pub fn supported_planning_items() -> &'static [PlanningItemKind] {
    &[
        PlanningItemKind::Portfolio,
        PlanningItemKind::Program,
        PlanningItemKind::Project,
        PlanningItemKind::Deliverable,
        PlanningItemKind::Milestone,
        PlanningItemKind::Epic,
        PlanningItemKind::Story,
        PlanningItemKind::Task,
        PlanningItemKind::SubTask,
        PlanningItemKind::Risk,
        PlanningItemKind::Issue,
        PlanningItemKind::Dependency,
    ]
}

pub fn project_workspace_steps() -> &'static [&'static str] {
    &[
        "Create a major project from File > New Project.",
        "Create a project from Project Planner after the user provides tech stack and required functionality.",
        "During project creation, ask the user to define mono-platform or multi-platform deliverables.",
        "Allow sub-projects under the major project, similar to NetBeans and JDeveloper project trees.",
        "Store project metadata in a NetBeans-inspired mEditor metadata folder without rewriting source folders unexpectedly.",
        "Attach source roots, resource roots, test roots, build profiles, run profiles, deployment profiles, and documentation outputs.",
        "Generate embedded About Project and Project Help documentation for every project.",
        "Show the whole project and sub-project tree in the same-window workspace navigator.",
    ]
}

pub fn project_planning_steps() -> &'static [&'static str] {
    &[
        "Open Project Planner from Tools and link it to the active mEditor project metadata.",
        "Ask the user whether to follow Objective -> Milestone -> Task or Scrum Epic -> Story -> Task.",
        "Ask the user for timezone before calculating or importing project timelines.",
        "Let the user create plans manually through the standard UI, including subtasks, assignees, ETA, timeline, and status tracking.",
        "Let the user export an Excel planning template, fill it externally, upload it, and consume it into the planner.",
        "Create portfolio, program, project, deliverable, milestone, epic, story, task, subtask, risk, issue, and dependency records.",
        "Let the user assign owners, start dates, due dates, status, and progress.",
        "Track planned cost, actual cost, forecast cost, and cost health for the project.",
        "Show a planner dashboard with Gantt chart, progress line charts, progress bar charts, and yellow/red risk indicators.",
        "Render timelines and milestone views for the project.",
        "Link planning items to physical source files, bugs, debug reports, deployment docs, and tests.",
        "For portfolio or project planning items, offer Create Project In Editor after collecting tech stack and required functionality from the user.",
        "Require deliverable definitions so project plans can track what will be produced, packaged, and delivered.",
        "Let the AI update progress in the background from user activity, commits, completed tasks, tests, and debugging outcomes.",
        "Keep AI updates reviewable so the user can accept, edit, or reject progress changes.",
    ]
}

pub fn planner_dashboard_widgets() -> &'static [&'static str] {
    &[
        "Gantt chart",
        "Progress line chart",
        "Progress bar chart",
        "Budget versus actual cost",
        "Forecast cost",
        "Schedule health",
        "Cost health",
        "Yellow/red risk indicator",
        "Milestone status",
        "Task burndown",
    ]
}

pub fn embedded_project_documentation_sections() -> &'static [&'static str] {
    &[
        "Project identity and purpose",
        "Owner, contacts, and support channel",
        "Technology stack and deliverables",
        "Build, run, debug, deploy, and rollback notes",
        "Architecture, ERD, UML, flowchart, and generated program documentation links",
        "Local bug repository and report links",
        "Project planner status, milestones, risks, and cost summary",
        "User-maintained help topics and operational notes",
    ]
}

pub fn project_help_sources() -> &'static [&'static str] {
    &[
        "Generated project metadata",
        "User-authored project help topics",
        "Project documentation generator output",
        "Compiler debugger handover reports",
        "Deployment instruction documents",
        "UML, ERD, and flowchart artifacts",
        "Local-only bug repository exports and reports",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_supports_major_project_with_subprojects() {
        let model = ProjectWorkspaceModel::default_model();
        assert!(model.supports_major_project_with_subprojects);
        assert!(model.supports_netbeans_style_metadata);
        assert_eq!(model.creation_paths.len(), 2);
        assert!(model
            .creation_paths
            .contains(&EditorProjectCreationPath::DirectNewProjectWizard));
        assert!(model
            .creation_paths
            .contains(&EditorProjectCreationPath::FromProjectPlanner));
        assert_eq!(model.menu_path, "File > New Project");
        assert_eq!(model.project_about_menu_path, "Help > About Project");
        assert_eq!(model.project_help_menu_path, "Help > Project Help");
        assert_eq!(model.project_metadata_folder, ".meditor/project");
        assert!(model.embeds_about_and_help_documentation);
        assert!(supported_project_node_kinds().contains(&ProjectNodeKind::MajorProject));
        assert!(supported_project_node_kinds().contains(&ProjectNodeKind::SubProject));
    }

    #[test]
    fn planning_model_covers_portfolio_project_management_items() {
        let model = ProjectPlanningModel::default_model();
        assert_eq!(model.label, "Project Planner");
        assert_eq!(model.menu_path, "Tools > Project Planner");
        assert_eq!(
            model.linked_project_metadata_file,
            ".meditor/project/project.toml"
        );
        for item in [
            PlanningItemKind::Deliverable,
            PlanningItemKind::Milestone,
            PlanningItemKind::Epic,
            PlanningItemKind::Story,
            PlanningItemKind::Task,
            PlanningItemKind::Risk,
            PlanningItemKind::Dependency,
        ] {
            assert!(model.supported_items.contains(&item));
        }
        assert!(model.ai_updates_progress_from_user_activity);
        assert!(model.timezone_required_before_timeline_calculation);
        assert!(model.asks_user_for_methodology);
        assert!(model.tracks_cost);
        assert!(model.dashboard_enabled);
        assert!(model
            .supported_methodologies
            .contains(&PlanningMethodology::ObjectiveMilestoneTask));
        assert!(model
            .supported_methodologies
            .contains(&PlanningMethodology::ScrumEpicStoryTask));
        assert_eq!(model.creation_paths.len(), 2);
        assert!(model
            .creation_paths
            .contains(&PlannerCreationPath::ManualUiEntry));
        assert!(model
            .creation_paths
            .contains(&PlannerCreationPath::ExcelTemplateImport));
        assert_eq!(
            model.create_editor_project_action,
            "project.planning.createEditorProject"
        );
    }

    #[test]
    fn planning_items_have_timeline_and_progress_fields() {
        let item = ProjectPlanningItem::new(
            "story-1",
            "Create login API",
            PlanningItemKind::Story,
            Some("epic-1".to_string()),
        );
        assert_eq!(item.status, PlanningStatus::NotStarted);
        assert_eq!(item.progress_percent, 0);
        assert_eq!(item.parent_id.as_deref(), Some("epic-1"));
    }

    #[test]
    fn workflow_steps_link_project_tree_and_ai_progress_updates() {
        assert!(project_workspace_steps()
            .iter()
            .any(|step| step.contains("sub-projects")));
        assert!(project_workspace_steps()
            .iter()
            .any(|step| step.contains("Project Planner")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("active mEditor project metadata")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("Objective -> Milestone -> Task")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("timezone")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("Excel planning template")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("Create Project In Editor")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("planned cost")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("Gantt chart")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("AI update progress")));
        assert!(project_planning_steps()
            .iter()
            .any(|step| step.contains("accept, edit, or reject")));
    }

    #[test]
    fn metadata_plan_is_netbeans_inspired_and_portfolio_ready() {
        let metadata = ProjectMetadataPlan::netbeans_style_for_meditor();
        assert_eq!(metadata.metadata_folder, ".meditor/project");
        assert!(metadata.supports_major_project);
        assert!(metadata.supports_subprojects);
        assert!(metadata.supports_portfolio_planning);
        assert!(metadata.requires_embedded_about_help);
        assert!(metadata.project_file.ends_with("project.toml"));
        assert!(metadata.about_file.ends_with("about.md"));
        assert!(metadata.help_index_file.ends_with("help-index.md"));
        assert!(metadata.private_user_file.ends_with("private-user.toml"));
    }

    #[test]
    fn every_project_embeds_about_and_help_documentation() {
        let docs = EmbeddedProjectDocumentationModel::default_model();
        assert_eq!(docs.about_menu_path, "Help > About Project");
        assert_eq!(docs.help_menu_path, "Help > Project Help");
        assert!(docs.opens_in_same_window_tab);
        assert!(docs.generated_for_every_project);
        assert!(docs.editable_by_user);
        assert!(embedded_project_documentation_sections()
            .iter()
            .any(|section| section.contains("Build, run, debug, deploy")));
        assert!(project_help_sources()
            .iter()
            .any(|source| source.contains("Compiler debugger")));
    }

    #[test]
    fn planner_can_create_editor_project_from_portfolio_or_project() {
        let request = PlannerProjectCreationRequest {
            planning_item_id: "project-1".to_string(),
            portfolio_or_project_name: "Customer Portal".to_string(),
            tech_stack: "Rust backend, React frontend, PostgreSQL".to_string(),
            required_functionality: vec![
                "Login".to_string(),
                "Customer dashboard".to_string(),
                "REST API".to_string(),
            ],
            deliverables: vec![
                meditor_packaging::ProjectDeliverableDefinition::multi_platform(
                    "Customer Portal Desktop",
                    meditor_packaging::DeliverableKind::PortableBundle,
                    vec![
                        meditor_platform_profile("mac"),
                        meditor_platform_profile("linux"),
                    ],
                ),
            ],
            create_major_project: true,
            create_subprojects: true,
        };
        let plan = PlannerProjectCreationPlan::from_request(request);
        assert_eq!(plan.editor_project_name, "Customer Portal");
        assert_eq!(plan.linked_planning_item_id, "project-1");
        assert_eq!(plan.create_project_action, "project.create");
        assert!(plan.tech_stack.contains("React"));
        assert_eq!(plan.required_functionality.len(), 3);
        assert_eq!(plan.deliverables.len(), 1);
        assert_eq!(
            plan.deliverables[0].scope,
            meditor_packaging::DeliverableScope::MultiPlatform
        );
    }

    #[test]
    fn planner_excel_template_has_required_timeline_and_timezone_columns() {
        let template = PlannerExcelTemplate::default_template();
        assert_eq!(template.file_name, "mEditor-project-plan-template.xlsx");
        assert!(template.sheets.contains(&"Tasks"));
        assert!(template.required_columns.contains(&"timezone"));
        assert!(template.required_columns.contains(&"start_datetime"));
        assert!(template.required_columns.contains(&"end_datetime"));
        assert_eq!(template.timezone_column, "timezone");
    }

    #[test]
    fn planning_methodology_changes_ui_hierarchy() {
        assert_eq!(
            planning_hierarchy_for(PlanningMethodology::ObjectiveMilestoneTask),
            &[
                PlanningItemKind::Project,
                PlanningItemKind::Deliverable,
                PlanningItemKind::Milestone,
                PlanningItemKind::Task,
                PlanningItemKind::SubTask,
            ]
        );
        assert_eq!(
            planning_hierarchy_for(PlanningMethodology::ScrumEpicStoryTask),
            &[
                PlanningItemKind::Project,
                PlanningItemKind::Epic,
                PlanningItemKind::Story,
                PlanningItemKind::Task,
                PlanningItemKind::SubTask,
            ]
        );
    }

    #[test]
    fn planner_tracks_cost_and_dashboard_health() {
        let green = ProjectCostTracking {
            currency_code: "USD".to_string(),
            planned_cost: 1000.0,
            actual_cost: 400.0,
            forecast_cost: 1030.0,
        };
        let yellow = ProjectCostTracking {
            forecast_cost: 1080.0,
            ..green.clone()
        };
        let red = ProjectCostTracking {
            forecast_cost: 1200.0,
            ..green.clone()
        };
        assert_eq!(green.health(), PlannerHealth::Green);
        assert_eq!(yellow.health(), PlannerHealth::Yellow);
        assert_eq!(red.health(), PlannerHealth::Red);
        assert!(planner_dashboard_widgets().contains(&"Gantt chart"));
        assert!(planner_dashboard_widgets().contains(&"Yellow/red risk indicator"));
    }
}

#[cfg(test)]
fn meditor_platform_profile(id: &str) -> meditor_platform::PlatformProfile {
    use meditor_platform::{CpuArchitecture, OperatingSystem};

    match id {
        "mac" => meditor_platform::PlatformProfile {
            os: OperatingSystem::MacOS,
            arch: CpuArchitecture::Arm64,
        },
        _ => meditor_platform::PlatformProfile {
            os: OperatingSystem::Linux,
            arch: CpuArchitecture::X64,
        },
    }
}
