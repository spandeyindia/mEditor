#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceProduct {
    NetBeans,
    Eclipse,
    JDeveloper,
    VisualStudio,
    VsCode,
    SqlDeveloper,
    SqlDataModeler,
    Termius,
    FileZilla,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceFeatureArea {
    Workspace,
    Navigation,
    Editing,
    Refactoring,
    Debugging,
    Profiling,
    VisualDesign,
    Database,
    DataModeling,
    TerminalSsh,
    Automation,
    Documentation,
    Reports,
    Extensions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReferenceFeature {
    pub id: &'static str,
    pub label: &'static str,
    pub area: ReferenceFeatureArea,
    pub inspired_by: Vec<ReferenceProduct>,
    pub meditor_surface: &'static str,
    pub same_window: bool,
}

pub fn reference_feature_catalog() -> Vec<ReferenceFeature> {
    vec![
        feature(
            "perspectives-layouts",
            "Perspectives, layouts, and workbench views",
            ReferenceFeatureArea::Workspace,
            &[
                ReferenceProduct::Eclipse,
                ReferenceProduct::NetBeans,
                ReferenceProduct::JDeveloper,
            ],
            "Window > Perspectives",
        ),
        feature(
            "visual-studio-project-import",
            "Visual Studio solution and MSBuild project import",
            ReferenceFeatureArea::Workspace,
            &[ReferenceProduct::VisualStudio],
            "File > Import Project",
        ),
        feature(
            "command-palette",
            "Command palette and quick actions",
            ReferenceFeatureArea::Navigation,
            &[ReferenceProduct::VsCode, ReferenceProduct::Eclipse],
            "Command Palette",
        ),
        feature(
            "global-search-symbols",
            "Global search, symbols, types, and files",
            ReferenceFeatureArea::Navigation,
            &[
                ReferenceProduct::VsCode,
                ReferenceProduct::Eclipse,
                ReferenceProduct::NetBeans,
            ],
            "Search",
        ),
        feature(
            "refactoring-preview",
            "Refactoring with preview, undo, and source-safe review",
            ReferenceFeatureArea::Refactoring,
            &[
                ReferenceProduct::NetBeans,
                ReferenceProduct::Eclipse,
                ReferenceProduct::JDeveloper,
            ],
            "Refactor",
        ),
        feature(
            "code-templates-snippets",
            "Code templates, snippets, live templates, and postfix completions",
            ReferenceFeatureArea::Editing,
            &[
                ReferenceProduct::NetBeans,
                ReferenceProduct::VsCode,
                ReferenceProduct::Eclipse,
            ],
            "Tools > Templates And Snippets",
        ),
        feature(
            "profiler-diagnostics",
            "CPU, memory, thread, and allocation profiling",
            ReferenceFeatureArea::Profiling,
            &[ReferenceProduct::NetBeans, ReferenceProduct::Eclipse],
            "Tools > Profiler",
        ),
        feature(
            "visual-ui-designer",
            "Visual UI designer and form/editor binding",
            ReferenceFeatureArea::VisualDesign,
            &[ReferenceProduct::NetBeans, ReferenceProduct::JDeveloper],
            "Tools > Visual Designer",
        ),
        feature(
            "task-runner",
            "Task runner, problem matchers, and saved run configurations",
            ReferenceFeatureArea::Automation,
            &[ReferenceProduct::VsCode, ReferenceProduct::Eclipse],
            "Run > Tasks",
        ),
        feature(
            "database-reports",
            "Database reports, object search, explain plans, and data grids",
            ReferenceFeatureArea::Database,
            &[ReferenceProduct::SqlDeveloper],
            "Tools > DBA Workshop",
        ),
        feature(
            "schema-compare-migration",
            "Schema compare, data compare, migration, and DDL generation",
            ReferenceFeatureArea::Database,
            &[
                ReferenceProduct::SqlDeveloper,
                ReferenceProduct::SqlDataModeler,
            ],
            "Tools > DBA Workshop",
        ),
        feature(
            "logical-relational-physical-modeling",
            "Logical, relational, and physical data models",
            ReferenceFeatureArea::DataModeling,
            &[ReferenceProduct::SqlDataModeler],
            "Tools > Data Modeler",
        ),
        feature(
            "ssh-groups-snippets-forwarding",
            "SSH groups, snippets, port forwarding, and jump-host profiles",
            ReferenceFeatureArea::TerminalSsh,
            &[ReferenceProduct::Termius],
            "Tools > SSH Terminus",
        ),
        feature(
            "sftp-scp-transfer",
            "SFTP/SCP transfer queue, site manager, bookmarks, resume, and compare",
            ReferenceFeatureArea::TerminalSsh,
            &[ReferenceProduct::FileZilla, ReferenceProduct::Termius],
            "Tools > SFTP/SCP Transfer",
        ),
        feature(
            "extension-marketplace",
            "Extension catalog, contribution points, and activation events",
            ReferenceFeatureArea::Extensions,
            &[ReferenceProduct::VsCode, ReferenceProduct::Eclipse],
            "Tools > Extensions",
        ),
        feature(
            "documentation-center",
            "Generated docs, report center, and project knowledge exports",
            ReferenceFeatureArea::Documentation,
            &[ReferenceProduct::NetBeans, ReferenceProduct::VsCode],
            "Tools > Project Documentation",
        ),
        feature(
            "report-center",
            "Report center for debugging, local bugs, work, deployment, docs, and security",
            ReferenceFeatureArea::Reports,
            &[ReferenceProduct::SqlDeveloper, ReferenceProduct::NetBeans],
            "Report > Reports",
        ),
    ]
}

pub fn reference_source_links() -> &'static [(&'static str, &'static str)] {
    &[
        ("NetBeans", "https://netbeans.apache.org/"),
        ("Eclipse", "https://www.eclipse.org/ide/"),
        ("Visual Studio", "https://visualstudio.microsoft.com/"),
        ("VS Code", "https://code.visualstudio.com/docs"),
        (
            "SQL Developer",
            "https://www.oracle.com/database/sqldeveloper/",
        ),
        (
            "SQL Developer Data Modeler",
            "https://www.oracle.com/database/sqldeveloper/technologies/sql-data-modeler/",
        ),
        ("Termius", "https://termius.com/"),
        ("FileZilla Client", "https://filezilla-project.org/"),
    ]
}

pub fn recommended_reference_additions() -> &'static [&'static str] {
    &[
        "Add Perspective/Layout Manager for DBA, coding, debugging, modeling, terminal, and planning workspaces.",
        "Add Command Palette with searchable commands, settings, files, symbols, and recent actions.",
        "Add Refactor Preview with user approval, rollback, and cross-language symbol awareness.",
        "Add Profiler for CPU, memory, thread, allocation, database wait, and web request diagnostics.",
        "Add Visual Designer for UI forms, REST endpoint maps, workflow screens, and generated code bindings.",
        "Add Extensions catalog with local/offline plugin bundles and contribution-point validation.",
        "Add Task Runner with problem matchers and saved run/debug/deploy configurations.",
        "Add Data Modeler for logical, relational, and physical modeling with DDL generation.",
        "Add SSH productivity features: snippets, jump hosts, tunnels, saved groups, and local-only exports.",
        "Add FileZilla-style SFTP/SCP transfer manager with site profiles, queue, resume, bookmarks, filters, and directory compare.",
        "Add NetBeans-style Refactor and Reformat source menu actions with preview, formatter profile, and rollback.",
        "Add Report Center with generated documentation and consolidated local bug/work reports.",
    ]
}

pub fn inbuilt_professional_ide_capabilities() -> &'static [&'static str] {
    &[
        "Perspectives and workspaces",
        "Command palette",
        "Refactoring and navigation",
        "Visual designers and modelers",
        "Profiler and diagnostics",
        "Remote and terminal quality",
        "Task automation",
        "Templates and snippets",
        "Migration and keymap compatibility",
    ]
}

pub fn inbuilt_capability_surfaces() -> &'static [(&'static str, &'static str)] {
    &[
        ("Perspectives and workspaces", "Window > Perspectives"),
        ("Command palette", "Command Palette"),
        ("Refactoring and navigation", "Source > Refactor"),
        ("Visual designers and modelers", "Tools > UML Modeling"),
        ("Profiler and diagnostics", "Tools > Profiler"),
        ("Remote and terminal quality", "Tools > SSH Terminus"),
        ("Task automation", "Run > Tasks"),
        ("Templates and snippets", "Tools > Templates And Snippets"),
        (
            "Migration and keymap compatibility",
            "Settings > Keymaps And Imports",
        ),
    ]
}

fn feature(
    id: &'static str,
    label: &'static str,
    area: ReferenceFeatureArea,
    inspired_by: &[ReferenceProduct],
    meditor_surface: &'static str,
) -> ReferenceFeature {
    ReferenceFeature {
        id,
        label,
        area,
        inspired_by: inspired_by.to_vec(),
        meditor_surface,
        same_window: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_requested_reference_products() {
        let catalog = reference_feature_catalog();
        for product in [
            ReferenceProduct::NetBeans,
            ReferenceProduct::Eclipse,
            ReferenceProduct::JDeveloper,
            ReferenceProduct::VisualStudio,
            ReferenceProduct::VsCode,
            ReferenceProduct::SqlDeveloper,
            ReferenceProduct::SqlDataModeler,
            ReferenceProduct::Termius,
            ReferenceProduct::FileZilla,
        ] {
            assert!(
                catalog
                    .iter()
                    .any(|feature| feature.inspired_by.contains(&product)),
                "{product:?}"
            );
        }
    }

    #[test]
    fn catalog_adds_professional_ide_polish_areas() {
        let catalog = reference_feature_catalog();
        for area in [
            ReferenceFeatureArea::Workspace,
            ReferenceFeatureArea::Refactoring,
            ReferenceFeatureArea::Profiling,
            ReferenceFeatureArea::DataModeling,
            ReferenceFeatureArea::TerminalSsh,
            ReferenceFeatureArea::Reports,
        ] {
            assert!(
                catalog.iter().any(|feature| feature.area == area),
                "{area:?}"
            );
        }
        assert!(catalog.iter().all(|feature| feature.same_window));
    }

    #[test]
    fn additions_include_command_palette_data_modeler_and_report_center() {
        assert!(recommended_reference_additions()
            .iter()
            .any(|addition| addition.contains("Command Palette")));
        assert!(recommended_reference_additions()
            .iter()
            .any(|addition| addition.contains("Data Modeler")));
        assert!(recommended_reference_additions()
            .iter()
            .any(|addition| addition.contains("Report Center")));
        assert!(reference_source_links()
            .iter()
            .any(|(name, _)| *name == "Termius"));
    }

    #[test]
    fn requested_professional_ide_capabilities_are_inbuilt() {
        for capability in [
            "Perspectives and workspaces",
            "Command palette",
            "Refactoring and navigation",
            "Visual designers and modelers",
            "Profiler and diagnostics",
            "Remote and terminal quality",
            "Task automation",
            "Templates and snippets",
            "Migration and keymap compatibility",
        ] {
            assert!(inbuilt_professional_ide_capabilities().contains(&capability));
            assert!(inbuilt_capability_surfaces()
                .iter()
                .any(|(name, surface)| *name == capability && !surface.is_empty()));
        }
    }
}
