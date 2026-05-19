#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseDriverKind {
    BuiltIn,
    Jdbc,
    Odbc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionProfile {
    pub name: String,
    pub driver: DatabaseDriverKind,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub admin_capable: bool,
}

impl ConnectionProfile {
    pub fn can_activate_dba_panel(&self) -> bool {
        self.admin_capable
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JdbcDriverDefinition {
    pub id: String,
    pub name: String,
    pub jar_paths: Vec<String>,
    pub driver_class: Option<String>,
    pub url_template: String,
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorksheetAction {
    RunStatement,
    RunScript,
    ExplainPlan,
    Commit,
    Rollback,
    Cancel,
}

pub fn worksheet_actions() -> Vec<WorksheetAction> {
    vec![
        WorksheetAction::RunStatement,
        WorksheetAction::RunScript,
        WorksheetAction::ExplainPlan,
        WorksheetAction::Commit,
        WorksheetAction::Rollback,
        WorksheetAction::Cancel,
    ]
}

pub fn dba_dashboard_sections() -> &'static [&'static str] {
    &[
        "Health",
        "Sessions",
        "Storage",
        "Waits and Locks",
        "High Availability",
        "Patch History",
        "Backup/RMAN History",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_connection_can_activate_dba_panel() {
        let profile = ConnectionProfile {
            name: "admin".to_string(),
            driver: DatabaseDriverKind::Jdbc,
            host: "localhost".to_string(),
            port: 1521,
            database: "demo".to_string(),
            user: "sys".to_string(),
            admin_capable: true,
        };
        assert!(profile.can_activate_dba_panel());
    }

    #[test]
    fn worksheet_and_dashboard_cover_required_surfaces() {
        assert_eq!(worksheet_actions().len(), 6);
        assert!(dba_dashboard_sections().contains(&"Backup/RMAN History"));
    }
}
