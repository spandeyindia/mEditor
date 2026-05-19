#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CiProvider {
    GitHubActions,
    GitLabCi,
    Jenkins,
    AzurePipelines,
    CircleCi,
    LocalScript,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CicdGeneratorPlan {
    pub menu_path: &'static str,
    pub output_folder: &'static str,
    pub providers: Vec<CiProvider>,
    pub generates_build_test_package_deploy: bool,
    pub uses_project_metadata: bool,
    pub asks_before_overwrite: bool,
    pub same_window: bool,
}

impl CicdGeneratorPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Tools > CI/CD Generator",
            output_folder: ".meditor/generated/cicd",
            providers: vec![
                CiProvider::GitHubActions,
                CiProvider::GitLabCi,
                CiProvider::Jenkins,
                CiProvider::AzurePipelines,
                CiProvider::CircleCi,
                CiProvider::LocalScript,
            ],
            generates_build_test_package_deploy: true,
            uses_project_metadata: true,
            asks_before_overwrite: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiProtocol {
    Rest,
    Soap,
    GraphQl,
    Grpc,
    WebSocket,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiWorkbenchPlan {
    pub menu_path: &'static str,
    pub collection_store: &'static str,
    pub protocols: Vec<ApiProtocol>,
    pub supports_environment_variables: bool,
    pub supports_generated_clients: bool,
    pub links_to_project_docs: bool,
    pub same_window: bool,
}

impl ApiWorkbenchPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Tools > API Workbench",
            collection_store: ".meditor/api-workbench/collections",
            protocols: vec![
                ApiProtocol::Rest,
                ApiProtocol::Soap,
                ApiProtocol::GraphQl,
                ApiProtocol::Grpc,
                ApiProtocol::WebSocket,
            ],
            supports_environment_variables: true,
            supports_generated_clients: true,
            links_to_project_docs: true,
            same_window: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MigrationAction {
    VersionSchema,
    GenerateMigration,
    DryRun,
    Apply,
    Rollback,
    DriftDetect,
    CompareSchema,
    ExportDdl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatabaseMigrationPlan {
    pub menu_path: &'static str,
    pub metadata_folder: &'static str,
    pub actions: Vec<MigrationAction>,
    pub supports_flyway_style: bool,
    pub supports_liquibase_style: bool,
    pub requires_backup_prompt_before_apply: bool,
    pub same_window: bool,
}

impl DatabaseMigrationPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "Tools > Database Migration",
            metadata_folder: ".meditor/database-migrations",
            actions: vec![
                MigrationAction::VersionSchema,
                MigrationAction::GenerateMigration,
                MigrationAction::DryRun,
                MigrationAction::Apply,
                MigrationAction::Rollback,
                MigrationAction::DriftDetect,
                MigrationAction::CompareSchema,
                MigrationAction::ExportDdl,
            ],
            supports_flyway_style: true,
            supports_liquibase_style: true,
            requires_backup_prompt_before_apply: true,
            same_window: true,
        }
    }
}

pub fn delivery_automation_capability_names() -> &'static [&'static str] {
    &[
        "CI/CD Generator",
        "API Workbench",
        "Database Migration Module",
    ]
}

pub fn delivery_automation_workflow_steps() -> &'static [&'static str] {
    &[
        "Read project metadata, deliverables, language tooling, tests, and deploy targets.",
        "Generate CI/CD files only after preview and user approval.",
        "Run API calls from a same-window API Workbench with environments and generated client links.",
        "Version database schemas, generate migrations, dry-run changes, and require backup prompt before apply.",
        "Provide rollback scripts and schema drift reports for database changes.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cicd_generator_covers_common_ci_targets() {
        let plan = CicdGeneratorPlan::default_plan();
        assert!(plan.providers.contains(&CiProvider::GitHubActions));
        assert!(plan.providers.contains(&CiProvider::GitLabCi));
        assert!(plan.providers.contains(&CiProvider::Jenkins));
        assert!(plan.generates_build_test_package_deploy);
        assert!(plan.asks_before_overwrite);
    }

    #[test]
    fn api_workbench_supports_modern_api_protocols() {
        let plan = ApiWorkbenchPlan::default_plan();
        assert!(plan.protocols.contains(&ApiProtocol::Rest));
        assert!(plan.protocols.contains(&ApiProtocol::GraphQl));
        assert!(plan.protocols.contains(&ApiProtocol::Grpc));
        assert!(plan.supports_environment_variables);
        assert!(plan.links_to_project_docs);
    }

    #[test]
    fn database_migration_requires_dry_run_backup_and_rollback() {
        let plan = DatabaseMigrationPlan::default_plan();
        assert!(plan.actions.contains(&MigrationAction::DryRun));
        assert!(plan.actions.contains(&MigrationAction::Rollback));
        assert!(plan.actions.contains(&MigrationAction::DriftDetect));
        assert!(plan.supports_flyway_style);
        assert!(plan.supports_liquibase_style);
        assert!(plan.requires_backup_prompt_before_apply);
    }
}
