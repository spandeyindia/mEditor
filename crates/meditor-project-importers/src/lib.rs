#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    NetBeans,
    Eclipse,
    JDeveloper,
    VisualStudio,
    Unknown,
}

pub fn detect_project_kind(files: &[&str]) -> ProjectKind {
    if files.iter().any(|file| *file == "nbproject/project.xml") {
        ProjectKind::NetBeans
    } else if files
        .iter()
        .any(|file| *file == ".project" || *file == ".classpath")
    {
        ProjectKind::Eclipse
    } else if files
        .iter()
        .any(|file| file.ends_with(".jws") || file.ends_with(".jpr"))
    {
        ProjectKind::JDeveloper
    } else if files
        .iter()
        .any(|file| classify_visual_studio_file(file).is_some())
    {
        ProjectKind::VisualStudio
    } else {
        ProjectKind::Unknown
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualStudioProjectFileKind {
    Solution,
    CSharpProject,
    VisualBasicProject,
    FSharpProject,
    CppProject,
    LegacyCppProject,
    SqlProject,
    SharedProject,
    JavaScriptProject,
    WixProject,
    Props,
    Targets,
    Filters,
}

pub fn classify_visual_studio_file(file: &str) -> Option<VisualStudioProjectFileKind> {
    let lower = file.to_ascii_lowercase();
    if lower.ends_with(".sln") {
        Some(VisualStudioProjectFileKind::Solution)
    } else if lower.ends_with(".csproj") {
        Some(VisualStudioProjectFileKind::CSharpProject)
    } else if lower.ends_with(".vbproj") {
        Some(VisualStudioProjectFileKind::VisualBasicProject)
    } else if lower.ends_with(".fsproj") {
        Some(VisualStudioProjectFileKind::FSharpProject)
    } else if lower.ends_with(".vcxproj") {
        Some(VisualStudioProjectFileKind::CppProject)
    } else if lower.ends_with(".vcproj") {
        Some(VisualStudioProjectFileKind::LegacyCppProject)
    } else if lower.ends_with(".sqlproj") || lower.ends_with(".dbproj") {
        Some(VisualStudioProjectFileKind::SqlProject)
    } else if lower.ends_with(".shproj") {
        Some(VisualStudioProjectFileKind::SharedProject)
    } else if lower.ends_with(".esproj") || lower.ends_with(".njsproj") {
        Some(VisualStudioProjectFileKind::JavaScriptProject)
    } else if lower.ends_with(".wixproj") {
        Some(VisualStudioProjectFileKind::WixProject)
    } else if lower.ends_with(".props") {
        Some(VisualStudioProjectFileKind::Props)
    } else if lower.ends_with(".targets") {
        Some(VisualStudioProjectFileKind::Targets)
    } else if lower.ends_with(".filters") {
        Some(VisualStudioProjectFileKind::Filters)
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportMode {
    ImportAsIs,
    ImportWithLocalMetadata,
    ConvertToWorkspace,
}

pub fn import_modes() -> Vec<ImportMode> {
    vec![
        ImportMode::ImportAsIs,
        ImportMode::ImportWithLocalMetadata,
        ImportMode::ConvertToWorkspace,
    ]
}

pub fn supported_project_kinds() -> &'static [ProjectKind] {
    &[
        ProjectKind::NetBeans,
        ProjectKind::Eclipse,
        ProjectKind::JDeveloper,
        ProjectKind::VisualStudio,
    ]
}

pub fn visual_studio_project_file_kinds() -> &'static [VisualStudioProjectFileKind] {
    &[
        VisualStudioProjectFileKind::Solution,
        VisualStudioProjectFileKind::CSharpProject,
        VisualStudioProjectFileKind::VisualBasicProject,
        VisualStudioProjectFileKind::FSharpProject,
        VisualStudioProjectFileKind::CppProject,
        VisualStudioProjectFileKind::LegacyCppProject,
        VisualStudioProjectFileKind::SqlProject,
        VisualStudioProjectFileKind::SharedProject,
        VisualStudioProjectFileKind::JavaScriptProject,
        VisualStudioProjectFileKind::WixProject,
        VisualStudioProjectFileKind::Props,
        VisualStudioProjectFileKind::Targets,
        VisualStudioProjectFileKind::Filters,
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualStudioImportPlan {
    pub menu_path: &'static str,
    pub detects_solution_files: bool,
    pub detects_msbuild_project_files: bool,
    pub maps_solution_projects_to_subprojects: bool,
    pub maps_msbuild_properties: bool,
    pub maps_configurations_and_platforms: bool,
    pub maps_references_and_packages: bool,
    pub maps_filters_as_virtual_folders: bool,
    pub creates_import_report: bool,
    pub same_window: bool,
}

impl VisualStudioImportPlan {
    pub fn default_plan() -> Self {
        Self {
            menu_path: "File > Import Project",
            detects_solution_files: true,
            detects_msbuild_project_files: true,
            maps_solution_projects_to_subprojects: true,
            maps_msbuild_properties: true,
            maps_configurations_and_platforms: true,
            maps_references_and_packages: true,
            maps_filters_as_virtual_folders: true,
            creates_import_report: true,
            same_window: true,
        }
    }
}

pub fn visual_studio_import_steps() -> &'static [&'static str] {
    &[
        "Detect Visual Studio solution and project files from File > Import Project.",
        "Parse solution entries into an mEditor major project with sub-projects.",
        "Map MSBuild configurations, platforms, properties, imports, references, package references, source roots, and generated output folders.",
        "Map C++ .filters files as virtual folders without moving physical files.",
        "Create build, clean, test, run, debug, package, and deploy task profiles where toolchains are detected or configured.",
        "Write an import report listing converted settings, unresolved SDKs, missing workloads, package restores, and manual follow-up items.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_project_metadata() {
        assert_eq!(
            detect_project_kind(&["nbproject/project.xml"]),
            ProjectKind::NetBeans
        );
        assert_eq!(detect_project_kind(&[".project"]), ProjectKind::Eclipse);
        assert_eq!(detect_project_kind(&["app.jws"]), ProjectKind::JDeveloper);
        assert_eq!(
            detect_project_kind(&["CustomerPortal.sln"]),
            ProjectKind::VisualStudio
        );
        assert_eq!(
            detect_project_kind(&["src/App/App.csproj"]),
            ProjectKind::VisualStudio
        );
        assert_eq!(
            detect_project_kind(&["native/App.vcxproj"]),
            ProjectKind::VisualStudio
        );
    }

    #[test]
    fn classifies_visual_studio_project_files_case_insensitively() {
        assert_eq!(
            classify_visual_studio_file("Demo.SLN"),
            Some(VisualStudioProjectFileKind::Solution)
        );
        assert_eq!(
            classify_visual_studio_file("web/App.esproj"),
            Some(VisualStudioProjectFileKind::JavaScriptProject)
        );
        assert_eq!(
            classify_visual_studio_file("native/App.vcxproj.filters"),
            Some(VisualStudioProjectFileKind::Filters)
        );
        assert_eq!(classify_visual_studio_file("README.md"), None);
    }

    #[test]
    fn visual_studio_import_plan_preserves_project_structure() {
        let plan = VisualStudioImportPlan::default_plan();
        assert_eq!(plan.menu_path, "File > Import Project");
        assert!(plan.detects_solution_files);
        assert!(plan.detects_msbuild_project_files);
        assert!(plan.maps_solution_projects_to_subprojects);
        assert!(plan.maps_msbuild_properties);
        assert!(plan.maps_configurations_and_platforms);
        assert!(plan.maps_references_and_packages);
        assert!(plan.maps_filters_as_virtual_folders);
        assert!(plan.creates_import_report);
        assert!(plan.same_window);
        assert!(supported_project_kinds().contains(&ProjectKind::VisualStudio));
        assert!(visual_studio_project_file_kinds()
            .contains(&VisualStudioProjectFileKind::CSharpProject));
        assert!(visual_studio_import_steps()
            .iter()
            .any(|step| step.contains("MSBuild configurations")));
    }
}
