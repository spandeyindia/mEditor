#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeploymentTargetKind {
    HttpServer,
    AppServer,
    Container,
    CloudPlatform,
    StaticHosting,
    DesktopInstaller,
    DatabaseMigration,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeploymentResearchMode {
    OfflineTemplatesOnly,
    OfficialDocsAndInstallerResearch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectDocumentKind {
    DeploymentGuide,
    ProductRequirementsDocument,
    TechnicalDesignDocument,
    ArchitectureDecisionRecord,
    Runbook,
    UserGuide,
    ApiReference,
    ProgramReference,
    ErdDocument,
    ReleaseNotes,
    TestPlan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowchartKind {
    ProcessFlow,
    ProgramControlFlow,
    DataFlow,
    DeploymentFlow,
    ErrorHandlingFlow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErdSourceKind {
    DatabaseSchema,
    SqlDdl,
    OrmModel,
    ClassModel,
    StructModel,
    JsonSchema,
    XmlSchema,
    Spreadsheet,
    CobolCopybook,
    UserDescribedDataStructure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentationOutputFormat {
    Markdown,
    Html,
    Pdf,
    Docx,
    Mermaid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentTarget {
    pub id: &'static str,
    pub display_name: &'static str,
    pub kind: DeploymentTargetKind,
    pub official_docs_url: &'static str,
    pub installer_url: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentDocRequest {
    pub project_name: String,
    pub implementation_stack: String,
    pub target_platforms: Vec<String>,
    pub target_servers: Vec<String>,
    pub include_install_steps: bool,
    pub include_configuration_steps: bool,
    pub include_validation_steps: bool,
    pub include_rollback_steps: bool,
    pub research_mode: DeploymentResearchMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentDocPlan {
    pub title: String,
    pub sections: Vec<&'static str>,
    pub target_servers: Vec<String>,
    pub research_sources: Vec<DeploymentTarget>,
    pub output_path: String,
    pub handover_to_user: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectDocumentationRequest {
    pub project_name: String,
    pub document_kinds: Vec<ProjectDocumentKind>,
    pub output_formats: Vec<DocumentationOutputFormat>,
    pub include_flowcharts: bool,
    pub include_program_reference: bool,
    pub include_source_link_back: bool,
    pub research_mode: DeploymentResearchMode,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowchartRequest {
    pub project_name: String,
    pub flowchart_kind: FlowchartKind,
    pub source_scope: String,
    pub output_format: DocumentationOutputFormat,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErdDocumentRequest {
    pub project_name: String,
    pub source_kind: ErdSourceKind,
    pub source_scope: String,
    pub output_formats: Vec<DocumentationOutputFormat>,
    pub include_attribute_dictionary: bool,
    pub include_relationship_rules: bool,
    pub include_source_traceability: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErdDocumentPlan {
    pub title: String,
    pub source_kind: ErdSourceKind,
    pub sections: Vec<&'static str>,
    pub output_formats: Vec<DocumentationOutputFormat>,
    pub output_path: String,
    pub handover_to_user: bool,
}

impl ErdDocumentPlan {
    pub fn from_request(request: &ErdDocumentRequest) -> Self {
        Self {
            title: format!("{} ERD Documentation", request.project_name),
            source_kind: request.source_kind,
            sections: erd_document_sections().to_vec(),
            output_formats: request.output_formats.clone(),
            output_path: format!(
                ".meditor/project-docs/{}/erd-documentation.md",
                sanitize_name(&request.project_name)
            ),
            handover_to_user: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectDocumentationPlan {
    pub title: String,
    pub document_kinds: Vec<ProjectDocumentKind>,
    pub output_formats: Vec<DocumentationOutputFormat>,
    pub sections: Vec<&'static str>,
    pub flowchart_formats: Vec<DocumentationOutputFormat>,
    pub output_folder: String,
    pub handover_to_user: bool,
}

impl ProjectDocumentationPlan {
    pub fn from_request(request: &ProjectDocumentationRequest) -> Self {
        Self {
            title: format!("{} Project Documentation", request.project_name),
            document_kinds: request.document_kinds.clone(),
            output_formats: request.output_formats.clone(),
            sections: project_documentation_sections().to_vec(),
            flowchart_formats: vec![
                DocumentationOutputFormat::Mermaid,
                DocumentationOutputFormat::Html,
                DocumentationOutputFormat::Pdf,
            ],
            output_folder: format!(
                ".meditor/project-docs/{}",
                sanitize_name(&request.project_name)
            ),
            handover_to_user: true,
        }
    }

    pub fn supports_javadoc_style_program_docs(&self) -> bool {
        self.document_kinds
            .contains(&ProjectDocumentKind::ProgramReference)
            || self
                .document_kinds
                .contains(&ProjectDocumentKind::ApiReference)
    }
}

impl DeploymentDocPlan {
    pub fn from_request(request: &DeploymentDocRequest) -> Self {
        let research_sources = request
            .target_servers
            .iter()
            .filter_map(|server| deployment_target_by_id(server))
            .collect::<Vec<_>>();
        Self {
            title: format!("{} Deployment Guide", request.project_name),
            sections: deployment_document_sections().to_vec(),
            target_servers: request.target_servers.clone(),
            research_sources,
            output_path: format!(
                ".meditor/deployment-docs/{}-deployment-guide.md",
                sanitize_name(&request.project_name)
            ),
            handover_to_user: true,
        }
    }

    pub fn uses_official_references_when_requested(&self, request: &DeploymentDocRequest) -> bool {
        request.research_mode == DeploymentResearchMode::OfficialDocsAndInstallerResearch
            && self
                .research_sources
                .iter()
                .all(|source| !source.official_docs_url.is_empty())
    }
}

pub fn deployment_document_sections() -> &'static [&'static str] {
    &[
        "Deployment scope and assumptions",
        "Target architecture",
        "Prerequisites and supported operating systems",
        "Runtime, SDK, compiler, and package-manager installation",
        "HTTP server or platform installation",
        "Application build and artifact creation",
        "Server configuration",
        "Environment variables and secrets",
        "Database migration and seed data",
        "Security hardening",
        "Smoke tests and health checks",
        "Troubleshooting",
        "Rollback and recovery",
        "References used",
    ]
}

pub fn project_documentation_sections() -> &'static [&'static str] {
    &[
        "Project overview",
        "Business goals and PRD requirements",
        "Technical design and architecture",
        "Module and package map",
        "Program/class/function reference",
        "Public API and endpoint reference",
        "Process flowcharts",
        "Program control-flow charts",
        "Data flow and integration diagrams",
        "ERD and data-structure documentation",
        "Build, test, deployment, and operations notes",
        "Known limitations and assumptions",
        "References and generated-source traceability",
    ]
}

pub fn erd_document_sections() -> &'static [&'static str] {
    &[
        "ERD scope and source inventory",
        "Entities and data structures",
        "Attributes, fields, columns, and data types",
        "Primary keys, natural keys, and unique constraints",
        "Relationships and cardinality",
        "Optionality and validation rules",
        "Indexes, lookup values, and derived fields",
        "Data lineage and source traceability",
        "Mermaid ER diagram",
        "Implementation notes and assumptions",
    ]
}

pub fn erd_supported_source_kinds() -> &'static [ErdSourceKind] {
    &[
        ErdSourceKind::DatabaseSchema,
        ErdSourceKind::SqlDdl,
        ErdSourceKind::OrmModel,
        ErdSourceKind::ClassModel,
        ErdSourceKind::StructModel,
        ErdSourceKind::JsonSchema,
        ErdSourceKind::XmlSchema,
        ErdSourceKind::Spreadsheet,
        ErdSourceKind::CobolCopybook,
        ErdSourceKind::UserDescribedDataStructure,
    ]
}

pub fn erd_generation_steps() -> &'static [&'static str] {
    &[
        "Accept any user-selected data structure source: database schema, DDL, ORM model, class, struct, JSON/XML schema, spreadsheet, COBOL copybook, or user-described structure.",
        "Extract entities, attributes, keys, constraints, and relationships.",
        "Infer cardinality and optionality where possible and flag uncertain relationships for user confirmation.",
        "Generate a Mermaid ER diagram plus attribute dictionary.",
        "Link every entity and relationship back to the physical source where possible.",
        "Open the ERD document in an editor tab and save it under the mEditor working folder.",
    ]
}

pub fn javadoc_style_language_coverage() -> &'static [&'static str] {
    &[
        "Assembly",
        "C",
        "C++",
        "Objective-C",
        "Ada",
        "Fortran",
        "COBOL",
        "BASIC",
        "Clipper/xBase",
        "Rust",
        "Python",
        "PHP",
        "Perl",
        "Java/JVM",
        "JavaScript",
        "TypeScript",
        "Julia",
        "R",
        "Prolog",
        "Lisp/Scheme",
        "SQL",
        "Go",
        ".NET",
        "Ruby",
        "Lua",
        "Shell",
    ]
}

pub fn documentation_generation_steps() -> &'static [&'static str] {
    &[
        "Accept the document type requested by the user, such as PRD, TDD, deployment guide, runbook, API reference, or program reference.",
        "Inspect project metadata, source files, comments, symbols, endpoints, build files, tests, and runtime configuration.",
        "Create process, program, data-flow, deployment, or error-handling flowcharts when requested.",
        "Generate language-agnostic Javadoc-style program documentation for supported languages.",
        "Link generated documentation back to the physical source files and symbols where possible.",
        "When requested and network is available, research official documentation before producing install, deploy, or platform-specific sections.",
        "Open the generated document in an editor tab and save it under the mEditor working folder.",
    ]
}

pub fn flowchart_generation_steps() -> &'static [&'static str] {
    &[
        "Detect the requested flowchart scope: process, program, data, deployment, or error handling.",
        "Extract nodes and edges from source, project metadata, routes, tests, configuration, or user-described steps.",
        "Generate Mermaid first so the user can edit it as text.",
        "Render HTML, PDF, or document-friendly images when requested.",
        "Keep source traceability from each diagram node back to the physical file, symbol, endpoint, or user requirement.",
    ]
}

pub fn supported_deployment_targets() -> Vec<DeploymentTarget> {
    vec![
        target(
            "apache-httpd",
            "Apache HTTP Server",
            DeploymentTargetKind::HttpServer,
            "https://httpd.apache.org/docs/",
            "https://httpd.apache.org/download.cgi",
        ),
        target(
            "nginx",
            "Nginx",
            DeploymentTargetKind::HttpServer,
            "https://nginx.org/en/docs/",
            "https://nginx.org/en/download.html",
        ),
        target(
            "caddy",
            "Caddy",
            DeploymentTargetKind::HttpServer,
            "https://caddyserver.com/docs/",
            "https://caddyserver.com/docs/install",
        ),
        target(
            "lighttpd",
            "lighttpd",
            DeploymentTargetKind::HttpServer,
            "https://redmine.lighttpd.net/projects/lighttpd/wiki",
            "https://www.lighttpd.net/download/",
        ),
        target(
            "iis",
            "Microsoft IIS",
            DeploymentTargetKind::HttpServer,
            "https://learn.microsoft.com/iis/",
            "https://learn.microsoft.com/iis/install/",
        ),
        target(
            "tomcat",
            "Apache Tomcat",
            DeploymentTargetKind::AppServer,
            "https://tomcat.apache.org/tomcat-10.1-doc/",
            "https://tomcat.apache.org/download-10.cgi",
        ),
        target(
            "docker",
            "Docker",
            DeploymentTargetKind::Container,
            "https://docs.docker.com/",
            "https://docs.docker.com/get-docker/",
        ),
        target(
            "kubernetes",
            "Kubernetes",
            DeploymentTargetKind::CloudPlatform,
            "https://kubernetes.io/docs/",
            "https://kubernetes.io/docs/tasks/tools/",
        ),
        target(
            "nodejs",
            "Node.js Runtime",
            DeploymentTargetKind::AppServer,
            "https://nodejs.org/api/",
            "https://nodejs.org/",
        ),
        target(
            "php",
            "PHP Runtime",
            DeploymentTargetKind::AppServer,
            "https://www.php.net/docs.php",
            "https://www.php.net/downloads.php",
        ),
        target(
            "static-site",
            "Static Site Hosting",
            DeploymentTargetKind::StaticHosting,
            "https://developer.mozilla.org/docs/Learn/Common_questions/Tools_and_setup/How_do_you_upload_your_files_to_a_web_server",
            "",
        ),
    ]
}

pub fn seed_stack_templates() -> &'static [&'static str] {
    &[
        "PHP REST API on Apache HTTP Server with PHP-FPM or mod_php",
        "PHP REST API on Nginx with PHP-FPM",
        "Node.js REST API behind Nginx reverse proxy",
        "React/Vite static frontend on Nginx, Apache, Caddy, or static hosting",
        "Java/JVM application on Tomcat or systemd service behind a reverse proxy",
        "Python ASGI/WSGI application behind Nginx or Apache",
        "Rust web service behind Nginx, Caddy, Docker, or Kubernetes",
        "Containerized application with Docker Compose",
        "Kubernetes deployment with service, ingress, config maps, and secrets",
    ]
}

pub fn deployment_doc_generation_steps() -> &'static [&'static str] {
    &[
        "Accept the user's implementation stack and target deployment platform as input.",
        "Inspect project metadata, build artifacts, environment needs, ports, routes, and database requirements.",
        "Select matching deployment templates and official documentation sources.",
        "When requested and network is available, research official docs and installer pages before writing the guide.",
        "Generate installation instructions for required runtimes, package managers, HTTP servers, app servers, or platform tools.",
        "Generate configuration steps for the selected server or platform.",
        "Add smoke tests, health checks, troubleshooting, security hardening, and rollback steps.",
        "Write the deployment guide into the editor as a document tab and save it under the mEditor working folder.",
    ]
}

fn deployment_target_by_id(id: &str) -> Option<DeploymentTarget> {
    supported_deployment_targets()
        .into_iter()
        .find(|target| target.id == id)
}

fn target(
    id: &'static str,
    display_name: &'static str,
    kind: DeploymentTargetKind,
    official_docs_url: &'static str,
    installer_url: &'static str,
) -> DeploymentTarget {
    DeploymentTarget {
        id,
        display_name,
        kind,
        official_docs_url,
        installer_url,
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
    fn targets_include_php_http_server_examples_and_generic_platforms() {
        let targets = supported_deployment_targets();
        for id in [
            "apache-httpd",
            "nginx",
            "caddy",
            "lighttpd",
            "iis",
            "docker",
            "kubernetes",
            "php",
            "nodejs",
        ] {
            assert!(targets.iter().any(|target| target.id == id), "{id}");
        }
    }

    #[test]
    fn plan_uses_user_stack_and_official_reference_mode() {
        let request = DeploymentDocRequest {
            project_name: "Billing REST API".to_string(),
            implementation_stack: "PHP REST API".to_string(),
            target_platforms: vec!["Linux".to_string()],
            target_servers: vec!["apache-httpd".to_string(), "nginx".to_string()],
            include_install_steps: true,
            include_configuration_steps: true,
            include_validation_steps: true,
            include_rollback_steps: true,
            research_mode: DeploymentResearchMode::OfficialDocsAndInstallerResearch,
        };
        let plan = DeploymentDocPlan::from_request(&request);
        assert_eq!(plan.title, "Billing REST API Deployment Guide");
        assert!(plan.handover_to_user);
        assert!(plan
            .sections
            .contains(&"HTTP server or platform installation"));
        assert!(plan.sections.contains(&"Rollback and recovery"));
        assert_eq!(
            plan.output_path,
            ".meditor/deployment-docs/billing-rest-api-deployment-guide.md"
        );
        assert!(plan.uses_official_references_when_requested(&request));
    }

    #[test]
    fn templates_are_examples_but_generation_is_generic() {
        assert!(seed_stack_templates()
            .iter()
            .any(|template| template.contains("PHP REST API")));
        assert!(seed_stack_templates()
            .iter()
            .any(|template| template.contains("Rust web service")));
        assert!(deployment_doc_generation_steps()
            .iter()
            .any(|step| step.contains("user's implementation stack")));
    }

    #[test]
    fn project_documentation_supports_prd_tdd_flowcharts_and_program_reference() {
        let request = ProjectDocumentationRequest {
            project_name: "Trading System".to_string(),
            document_kinds: vec![
                ProjectDocumentKind::ProductRequirementsDocument,
                ProjectDocumentKind::TechnicalDesignDocument,
                ProjectDocumentKind::ProgramReference,
                ProjectDocumentKind::ErdDocument,
            ],
            output_formats: vec![
                DocumentationOutputFormat::Markdown,
                DocumentationOutputFormat::Html,
            ],
            include_flowcharts: true,
            include_program_reference: true,
            include_source_link_back: true,
            research_mode: DeploymentResearchMode::OfflineTemplatesOnly,
        };
        let plan = ProjectDocumentationPlan::from_request(&request);
        assert!(plan
            .document_kinds
            .contains(&ProjectDocumentKind::ProductRequirementsDocument));
        assert!(plan
            .document_kinds
            .contains(&ProjectDocumentKind::TechnicalDesignDocument));
        assert!(plan.supports_javadoc_style_program_docs());
        assert!(project_documentation_sections().contains(&"Program/class/function reference"));
        assert!(project_documentation_sections().contains(&"ERD and data-structure documentation"));
        assert!(flowchart_generation_steps()
            .iter()
            .any(|step| step.contains("Mermaid")));
    }

    #[test]
    fn javadoc_style_coverage_spans_supported_languages() {
        for language in ["Java/JVM", "Rust", "PHP", "COBOL", "Prolog", "SQL"] {
            assert!(javadoc_style_language_coverage().contains(&language));
        }
        assert!(documentation_generation_steps()
            .iter()
            .any(|step| step.contains("language-agnostic Javadoc-style")));
    }

    #[test]
    fn erd_documentation_accepts_any_data_structure_source() {
        let request = ErdDocumentRequest {
            project_name: "Legacy Billing".to_string(),
            source_kind: ErdSourceKind::CobolCopybook,
            source_scope: "copybooks/customer.cpy".to_string(),
            output_formats: vec![
                DocumentationOutputFormat::Markdown,
                DocumentationOutputFormat::Mermaid,
            ],
            include_attribute_dictionary: true,
            include_relationship_rules: true,
            include_source_traceability: true,
        };
        let plan = ErdDocumentPlan::from_request(&request);
        assert_eq!(plan.source_kind, ErdSourceKind::CobolCopybook);
        assert!(plan.sections.contains(&"Mermaid ER diagram"));
        assert!(erd_supported_source_kinds().contains(&ErdSourceKind::JsonSchema));
        assert!(erd_supported_source_kinds().contains(&ErdSourceKind::StructModel));
        assert!(erd_generation_steps()
            .iter()
            .any(|step| step.contains("any user-selected data structure")));
    }
}
