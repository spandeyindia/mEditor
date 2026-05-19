#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    NetBeans,
    Eclipse,
    JDeveloper,
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
    } else {
        ProjectKind::Unknown
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
    }
}
