pub const FREEWARE_REGISTRATION_PROMPT: &str = "mEditor is freeware. Registration is optional and is only used to make this installation your personal copy and to inform you about future updates. There is no licensing or financial obligation to you or to your organisation. You can continue with the full product without registration. If you continue without registration, mEditor will send only this installation's network domain, machine name, and IP address to record an unregistered fully functional copy.";
pub const INSTALL_BASE_IDENTITY_USE_TEXT: &str = "Network domain, machine name, and IP address are used only to identify the mEditor install base and are not used in bug reports or any other reporting.";
pub const ACTIVE_LICENSE_CODE: &str = "LicenseRef-mEditor-Freeware-EULA";
pub const ACTIVE_LICENSE_NAME: &str = "mEditor Freeware EULA";
pub const ACTIVE_LICENSE_VERSION: &str = "1.0";
pub const ACTIVE_LICENSE_TEXT: &str = include_str!("../../../LICENSE.md");
pub const OWNER_CONTACT_EMAIL: &str = "s.pandey.india@gmail.com";
pub const UNREGISTERED_FEEDBACK_EMAIL_PROMPT: &str = "Please provide your personal email id for this issue. It will be used only to inform you about the bugfix or status update for this bug report or enhancement request.";
pub const APEX_OPENAPI_CATALOG_URL: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/";
pub const APEX_SWAGGER_UI_URL: &str =
    "https://oracleapex.com/swagger/index.html?url=https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/";
pub const DEFAULT_APEX_BASE_URL: &str = "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1";
pub const DEFAULT_INSTALLATION_URL: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/";
pub const DEFAULT_INSTALL_EVENT_URL_TEMPLATE: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/events/";
pub const DEFAULT_FEEDBACK_URL: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/";
pub const DEFAULT_FEEDBACK_LIST_URL_TEMPLATE: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/";
pub const DEFAULT_FEEDBACK_SUMMARY_URL_TEMPLATE: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/summary/";
pub const DEFAULT_FEEDBACK_STATUS_URL_TEMPLATE: &str =
    "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/:feedback_id/status/";

pub const PERSONAL_LICENSE_PREFIX: &str = "Personal License to ";
pub const UNREGISTERED_LICENSE_TEXT: &str =
    "UnRegistered but fully functional copy with no obligation";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentifierSource {
    OsAccountHint,
    UserEntered,
    NotProvided,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistrationIdentity {
    pub display_name: String,
    pub email_or_id: Option<String>,
    pub source: IdentifierSource,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LicenseAcceptance {
    pub license_code: &'static str,
    pub license_name: &'static str,
    pub license_version: &'static str,
    pub accepted: bool,
    pub accepted_at_utc: Option<String>,
    pub accepted_by: Option<String>,
}

impl LicenseAcceptance {
    pub fn accepted_now(accepted_at_utc: impl Into<String>, accepted_by: Option<String>) -> Self {
        Self {
            license_code: ACTIVE_LICENSE_CODE,
            license_name: ACTIVE_LICENSE_NAME,
            license_version: ACTIVE_LICENSE_VERSION,
            accepted: true,
            accepted_at_utc: Some(accepted_at_utc.into()),
            accepted_by,
        }
    }

    pub fn not_accepted() -> Self {
        Self {
            license_code: ACTIVE_LICENSE_CODE,
            license_name: ACTIVE_LICENSE_NAME,
            license_version: ACTIVE_LICENSE_VERSION,
            accepted: false,
            accepted_at_utc: None,
            accepted_by: None,
        }
    }
}

impl RegistrationIdentity {
    pub fn new(
        display_name: impl Into<String>,
        email_or_id: Option<String>,
        source: IdentifierSource,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            email_or_id,
            source,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistrationStatus {
    Registered,
    Unregistered,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FirstRunChoice {
    Register {
        identity: RegistrationIdentity,
        license_acceptance: LicenseAcceptance,
    },
    ContinueUnregistered {
        license_acceptance: LicenseAcceptance,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceIdentity {
    pub network_domain: Option<String>,
    pub machine_name: Option<String>,
    pub ip_address: Option<String>,
}

impl DeviceIdentity {
    pub fn empty() -> Self {
        Self {
            network_domain: None,
            machine_name: None,
            ip_address: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstallationPayload {
    pub installation_id: String,
    pub product_name: &'static str,
    pub app_version: String,
    pub registration_status: RegistrationStatus,
    pub identity: Option<RegistrationIdentity>,
    pub device_identity: DeviceIdentity,
    pub license_acceptance: LicenseAcceptance,
    pub about_license_text: String,
}

impl InstallationPayload {
    pub fn from_first_run(
        installation_id: impl Into<String>,
        app_version: impl Into<String>,
        choice: FirstRunChoice,
        detected_device_identity: DeviceIdentity,
    ) -> Self {
        let installation_id = installation_id.into();
        let app_version = app_version.into();
        match choice {
            FirstRunChoice::Register {
                identity,
                license_acceptance,
            } => {
                let about_license_text = personal_license_text(&identity.display_name);
                Self {
                    installation_id,
                    product_name: meditor_core::PRODUCT_NAME,
                    app_version,
                    registration_status: RegistrationStatus::Registered,
                    identity: Some(identity),
                    device_identity: detected_device_identity,
                    license_acceptance,
                    about_license_text,
                }
            }
            FirstRunChoice::ContinueUnregistered { license_acceptance } => Self {
                installation_id,
                product_name: meditor_core::PRODUCT_NAME,
                app_version,
                registration_status: RegistrationStatus::Unregistered,
                identity: None,
                device_identity: detected_device_identity,
                license_acceptance,
                about_license_text: UNREGISTERED_LICENSE_TEXT.to_string(),
            },
        }
    }

    pub fn can_open_product(&self) -> bool {
        self.license_acceptance.accepted
    }
}

pub fn personal_license_text(display_name: &str) -> String {
    format!("{PERSONAL_LICENSE_PREFIX}{}", display_name.trim())
}

pub fn registration_statuses() -> Vec<RegistrationStatus> {
    vec![
        RegistrationStatus::Registered,
        RegistrationStatus::Unregistered,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstallationEventType {
    FirstLaunch,
    LicenseAccepted,
    RegistrationSubmitted,
    RegistrationDeclined,
    UpdateCheck,
    Launch,
    FeedbackSubmitted,
    AutomaticErrorCaptured,
}

pub fn installation_event_types() -> Vec<InstallationEventType> {
    vec![
        InstallationEventType::FirstLaunch,
        InstallationEventType::LicenseAccepted,
        InstallationEventType::RegistrationSubmitted,
        InstallationEventType::RegistrationDeclined,
        InstallationEventType::UpdateCheck,
        InstallationEventType::Launch,
        InstallationEventType::FeedbackSubmitted,
        InstallationEventType::AutomaticErrorCaptured,
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackKind {
    BugReport,
    EnhancementRequest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackEntryMode {
    AutomaticErrorCapture,
    ManualUserFeedback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BugSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackStatus {
    Submitted,
    Triaged,
    InProgress,
    Fixed,
    Released,
    NeedsInfo,
    Deferred,
    Closed,
}

pub fn feedback_statuses() -> Vec<FeedbackStatus> {
    vec![
        FeedbackStatus::Submitted,
        FeedbackStatus::Triaged,
        FeedbackStatus::InProgress,
        FeedbackStatus::Fixed,
        FeedbackStatus::Released,
        FeedbackStatus::NeedsInfo,
        FeedbackStatus::Deferred,
        FeedbackStatus::Closed,
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorDiagnosticSnapshot {
    pub app_version: String,
    pub os_family: String,
    pub cpu_arch: String,
    pub error_message: String,
    pub error_kind: String,
    pub stack_trace: Option<String>,
    pub active_module: Option<String>,
    pub active_file: Option<String>,
    pub active_command: Option<String>,
    pub recent_log_excerpt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeedbackDraft {
    pub kind: FeedbackKind,
    pub entry_mode: FeedbackEntryMode,
    pub title: String,
    pub severity: Option<BugSeverity>,
    pub category: String,
    pub details: String,
    pub steps_to_reproduce: String,
    pub expected_behavior: Option<String>,
    pub actual_behavior: Option<String>,
    pub diagnostic_snapshot: Option<ErrorDiagnosticSnapshot>,
    pub reporter_email_or_id: Option<String>,
    pub registered_user: bool,
    pub include_log_excerpt: bool,
    pub include_open_file_path: bool,
}

impl FeedbackDraft {
    pub fn from_error(
        title: impl Into<String>,
        details: impl Into<String>,
        diagnostic_snapshot: ErrorDiagnosticSnapshot,
        registered_user: bool,
        reporter_email_or_id: Option<String>,
    ) -> Self {
        Self {
            kind: FeedbackKind::BugReport,
            entry_mode: FeedbackEntryMode::AutomaticErrorCapture,
            title: title.into(),
            severity: Some(BugSeverity::High),
            category: "Runtime Error".to_string(),
            details: details.into(),
            steps_to_reproduce: "Captured automatically from the error context.".to_string(),
            expected_behavior: None,
            actual_behavior: Some(diagnostic_snapshot.error_message.clone()),
            diagnostic_snapshot: Some(diagnostic_snapshot),
            reporter_email_or_id,
            registered_user,
            include_log_excerpt: true,
            include_open_file_path: true,
        }
    }

    pub fn requires_user_submit(&self) -> bool {
        true
    }

    pub fn requires_unregistered_email_prompt(&self) -> bool {
        !self.registered_user && self.reporter_email_or_id.is_none()
    }

    pub fn unregistered_email_prompt(&self) -> Option<&'static str> {
        self.requires_unregistered_email_prompt()
            .then_some(UNREGISTERED_FEEDBACK_EMAIL_PROMPT)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeedbackPayload {
    pub installation_id: String,
    pub draft: FeedbackDraft,
    pub status: FeedbackStatus,
    pub owner_contact_email: &'static str,
    pub reporter_email_purpose: &'static str,
}

impl FeedbackPayload {
    pub fn new(installation_id: impl Into<String>, draft: FeedbackDraft) -> Self {
        Self {
            installation_id: installation_id.into(),
            draft,
            status: FeedbackStatus::Submitted,
            owner_contact_email: OWNER_CONTACT_EMAIL,
            reporter_email_purpose: "ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY",
        }
    }

    pub fn requires_user_submit(&self) -> bool {
        self.draft.requires_user_submit()
    }
}

pub fn bug_report_fields() -> &'static [&'static str] {
    feedback_report_fields()
}

pub fn feedback_report_fields() -> &'static [&'static str] {
    &[
        "installation_id",
        "feedback_type",
        "entry_mode",
        "reporter_email_or_id",
        "reporter_email_purpose",
        "severity",
        "category",
        "title",
        "details",
        "steps_to_reproduce",
        "expected_behavior",
        "actual_behavior",
        "error_message",
        "stack_trace",
        "owner_contact_email",
        "log_excerpt",
        "status",
    ]
}

pub fn feedback_form_fields() -> &'static [&'static str] {
    &[
        "feedback_type",
        "title",
        "details",
        "severity",
        "category",
        "steps_to_reproduce",
        "expected_behavior",
        "actual_behavior",
        "personal_email_for_issue_updates",
    ]
}

pub fn automatic_error_capture_fields() -> &'static [&'static str] {
    &[
        "app_version",
        "os_family",
        "cpu_arch",
        "error_message",
        "error_kind",
        "stack_trace",
        "active_module",
        "active_file",
        "active_command",
        "recent_log_excerpt",
    ]
}

pub fn user_feedback_metrics() -> &'static [&'static str] {
    &[
        "feedback_given",
        "bugs_reported",
        "enhancements_requested",
        "open_items",
        "fixed_items",
        "latest_status",
    ]
}

pub fn install_base_identity_fields() -> &'static [&'static str] {
    &["network_domain", "machine_name", "ip_address"]
}

pub fn license_acceptance_fields() -> &'static [&'static str] {
    &[
        "license_code",
        "license_name",
        "license_version",
        "license_accepted_flag",
        "license_accepted_at",
        "license_accepted_by",
    ]
}

pub fn apex_table_names() -> &'static [&'static str] {
    &[
        "meditor_installation",
        "meditor_install_event",
        "meditor_feedback_report",
        "meditor_feedback_status_history",
        "meditor_rest_ingest_log",
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendEndpointConfig {
    pub base_url: Option<String>,
    pub installation_url: Option<String>,
    pub install_event_url_template: Option<String>,
    pub feedback_url: Option<String>,
    pub feedback_list_url_template: Option<String>,
    pub feedback_summary_url_template: Option<String>,
    pub feedback_status_url_template: Option<String>,
    pub openapi_catalog_url: Option<String>,
    pub swagger_ui_url: Option<String>,
}

impl BackendEndpointConfig {
    pub fn oracle_apex_default() -> Self {
        Self {
            base_url: Some(DEFAULT_APEX_BASE_URL.to_string()),
            installation_url: Some(DEFAULT_INSTALLATION_URL.to_string()),
            install_event_url_template: Some(DEFAULT_INSTALL_EVENT_URL_TEMPLATE.to_string()),
            feedback_url: Some(DEFAULT_FEEDBACK_URL.to_string()),
            feedback_list_url_template: Some(DEFAULT_FEEDBACK_LIST_URL_TEMPLATE.to_string()),
            feedback_summary_url_template: Some(DEFAULT_FEEDBACK_SUMMARY_URL_TEMPLATE.to_string()),
            feedback_status_url_template: Some(DEFAULT_FEEDBACK_STATUS_URL_TEMPLATE.to_string()),
            openapi_catalog_url: Some(APEX_OPENAPI_CATALOG_URL.to_string()),
            swagger_ui_url: Some(APEX_SWAGGER_UI_URL.to_string()),
        }
    }

    pub fn ready_for_installation_ingest(&self) -> bool {
        self.installation_url.is_some()
    }

    pub fn ready_for_events(&self) -> bool {
        self.install_event_url_template.is_some()
    }

    pub fn ready_for_feedback(&self) -> bool {
        self.feedback_url.is_some()
    }

    pub fn ready_for_feedback_status(&self) -> bool {
        self.feedback_list_url_template.is_some()
            && self.feedback_summary_url_template.is_some()
            && self.feedback_status_url_template.is_some()
    }

    pub fn ready_for_openapi_reference(&self) -> bool {
        self.openapi_catalog_url.is_some() && self.swagger_ui_url.is_some()
    }
}

pub fn default_backend_endpoint_values() -> &'static [(&'static str, &'static str)] {
    &[
        ("base_url", DEFAULT_APEX_BASE_URL),
        ("installation_url", DEFAULT_INSTALLATION_URL),
        (
            "install_event_url_template",
            DEFAULT_INSTALL_EVENT_URL_TEMPLATE,
        ),
        ("feedback_url", DEFAULT_FEEDBACK_URL),
        (
            "feedback_list_url_template",
            DEFAULT_FEEDBACK_LIST_URL_TEMPLATE,
        ),
        (
            "feedback_summary_url_template",
            DEFAULT_FEEDBACK_SUMMARY_URL_TEMPLATE,
        ),
        (
            "feedback_status_url_template",
            DEFAULT_FEEDBACK_STATUS_URL_TEMPLATE,
        ),
        ("openapi_catalog_url", APEX_OPENAPI_CATALOG_URL),
        ("swagger_ui_url", APEX_SWAGGER_UI_URL),
    ]
}

pub fn first_run_notice_points() -> &'static [&'static str] {
    &[
        "mEditor remains fully functional without registration.",
        "Registration makes the installation a personal freeware copy and has no licensing or financial obligation.",
        "The active license is displayed in the Registration dialog and from the About dialog.",
        "The user must accept the active freeware EULA before the product opens.",
        "OS account information may be used only as a prefilled hint and must be confirmed by the user.",
        "Unregistered installs send only network domain, machine name, and IP address as the installation record.",
        "IP address is used only to identify install base and is not used in bug reports or other reporting.",
        "Bug reports and enhancement requests are sent only when the user submits them.",
        "Unregistered users are asked for a personal email id only to inform them about the fix or status for that issue.",
        "The top-right Feedback icon opens manual bug report and enhancement request entry.",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device_identity() -> DeviceIdentity {
        DeviceIdentity {
            network_domain: Some("example.local".to_string()),
            machine_name: Some("developer-laptop".to_string()),
            ip_address: Some("192.0.2.10".to_string()),
        }
    }

    #[test]
    fn registered_payload_displays_personal_license() {
        let identity = RegistrationIdentity::new(
            "Sanjay Pandey",
            Some("s.pandey.india@gmail.com".to_string()),
            IdentifierSource::UserEntered,
        );
        let payload = InstallationPayload::from_first_run(
            "install-1",
            meditor_core::CURRENT_BASELINE_VERSION,
            FirstRunChoice::Register {
                identity,
                license_acceptance: LicenseAcceptance::accepted_now(
                    "2026-05-19T00:00:00Z",
                    Some("Sanjay Pandey".to_string()),
                ),
            },
            device_identity(),
        );

        assert_eq!(payload.registration_status, RegistrationStatus::Registered);
        assert_eq!(
            payload.about_license_text,
            "Personal License to Sanjay Pandey"
        );
        assert!(payload.identity.is_some());
        assert!(payload.can_open_product());
        assert_eq!(payload.license_acceptance.license_code, ACTIVE_LICENSE_CODE);
        assert_eq!(
            payload.device_identity.machine_name.as_deref(),
            Some("developer-laptop")
        );
    }

    #[test]
    fn unregistered_payload_is_full_product_with_minimal_install_record() {
        let payload = InstallationPayload::from_first_run(
            "install-2",
            meditor_core::CURRENT_BASELINE_VERSION,
            FirstRunChoice::ContinueUnregistered {
                license_acceptance: LicenseAcceptance::accepted_now("2026-05-19T00:00:00Z", None),
            },
            device_identity(),
        );

        assert_eq!(
            payload.registration_status,
            RegistrationStatus::Unregistered
        );
        assert_eq!(payload.about_license_text, UNREGISTERED_LICENSE_TEXT);
        assert!(payload.identity.is_none());
        assert!(payload.can_open_product());
        assert_eq!(
            payload.device_identity.machine_name.as_deref(),
            Some("developer-laptop")
        );
    }

    #[test]
    fn registration_notice_states_freeware_and_no_obligation() {
        assert!(FREEWARE_REGISTRATION_PROMPT.contains("freeware"));
        assert!(FREEWARE_REGISTRATION_PROMPT.contains("no licensing or financial obligation"));
        assert!(FREEWARE_REGISTRATION_PROMPT.contains("full product without registration"));
        assert!(
            FREEWARE_REGISTRATION_PROMPT.contains("network domain, machine name, and IP address")
        );
        assert!(
            INSTALL_BASE_IDENTITY_USE_TEXT.contains("only to identify the mEditor install base")
        );
        assert!(ACTIVE_LICENSE_TEXT.contains("mEditor Freeware End User License Agreement"));
        assert!(license_acceptance_fields().contains(&"license_accepted_flag"));
    }

    #[test]
    fn bug_reports_are_user_submitted() {
        let report = FeedbackDraft {
            kind: FeedbackKind::BugReport,
            entry_mode: FeedbackEntryMode::ManualUserFeedback,
            title: "Crash while opening workspace".to_string(),
            severity: Some(BugSeverity::High),
            category: "Startup".to_string(),
            details: "The app exits before the workspace opens.".to_string(),
            steps_to_reproduce: "Launch mEditor and choose the workspace.".to_string(),
            expected_behavior: Some("Workspace opens".to_string()),
            actual_behavior: Some("Application exits".to_string()),
            diagnostic_snapshot: None,
            reporter_email_or_id: None,
            registered_user: false,
            include_log_excerpt: true,
            include_open_file_path: false,
        };
        let payload = FeedbackPayload::new("install-2", report);

        assert!(payload.requires_user_submit());
        assert!(payload.draft.requires_unregistered_email_prompt());
        assert_eq!(payload.owner_contact_email, OWNER_CONTACT_EMAIL);
        assert_eq!(
            payload.reporter_email_purpose,
            "ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY"
        );
        assert!(bug_report_fields().contains(&"steps_to_reproduce"));
        assert!(!bug_report_fields().contains(&"ip_address"));
        assert!(install_base_identity_fields().contains(&"ip_address"));
        assert!(feedback_form_fields().contains(&"personal_email_for_issue_updates"));
        assert!(user_feedback_metrics().contains(&"latest_status"));
    }

    #[test]
    fn automatic_error_mode_captures_diagnostics_and_prompts_unregistered_email() {
        let snapshot = ErrorDiagnosticSnapshot {
            app_version: meditor_core::CURRENT_BASELINE_VERSION.to_string(),
            os_family: "MacOS".to_string(),
            cpu_arch: "Arm64".to_string(),
            error_message: "panic while saving file".to_string(),
            error_kind: "panic".to_string(),
            stack_trace: Some("stack frame 1\nstack frame 2".to_string()),
            active_module: Some("editor".to_string()),
            active_file: Some("/workspace/demo.rs".to_string()),
            active_command: Some("file.save".to_string()),
            recent_log_excerpt: Some("save failed".to_string()),
        };
        let report = FeedbackDraft::from_error(
            "Automatic error report",
            "mEditor captured a save failure.",
            snapshot,
            false,
            None,
        );

        assert_eq!(report.kind, FeedbackKind::BugReport);
        assert_eq!(report.entry_mode, FeedbackEntryMode::AutomaticErrorCapture);
        assert!(report.diagnostic_snapshot.is_some());
        assert!(report.requires_unregistered_email_prompt());
        assert_eq!(
            report.unregistered_email_prompt(),
            Some(UNREGISTERED_FEEDBACK_EMAIL_PROMPT)
        );
        assert!(automatic_error_capture_fields().contains(&"stack_trace"));
    }

    #[test]
    fn registered_feedback_uses_existing_registration_email() {
        let report = FeedbackDraft {
            kind: FeedbackKind::EnhancementRequest,
            entry_mode: FeedbackEntryMode::ManualUserFeedback,
            title: "Add theme import".to_string(),
            severity: None,
            category: "UX".to_string(),
            details: "Support importing theme bundles.".to_string(),
            steps_to_reproduce: "Open Feedback and choose Enhancement Request.".to_string(),
            expected_behavior: None,
            actual_behavior: None,
            diagnostic_snapshot: None,
            reporter_email_or_id: Some("s.pandey.india@gmail.com".to_string()),
            registered_user: true,
            include_log_excerpt: false,
            include_open_file_path: false,
        };

        assert!(!report.requires_unregistered_email_prompt());
        assert_eq!(feedback_statuses().len(), 8);
    }

    #[test]
    fn product_requires_license_acceptance_before_opening() {
        let payload = InstallationPayload::from_first_run(
            "install-4",
            meditor_core::CURRENT_BASELINE_VERSION,
            FirstRunChoice::ContinueUnregistered {
                license_acceptance: LicenseAcceptance::not_accepted(),
            },
            device_identity(),
        );

        assert!(!payload.can_open_product());
    }

    #[test]
    fn oracle_apex_default_endpoint_config_matches_created_rest_apis() {
        let config = BackendEndpointConfig::oracle_apex_default();

        assert_eq!(config.base_url.as_deref(), Some(DEFAULT_APEX_BASE_URL));
        assert_eq!(
            config.installation_url.as_deref(),
            Some("https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/")
        );
        assert_eq!(
            config.install_event_url_template.as_deref(),
            Some("https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/events/")
        );
        assert_eq!(
            config.feedback_url.as_deref(),
            Some("https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/")
        );
        assert_eq!(
            config.feedback_status_url_template.as_deref(),
            Some("https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/:feedback_id/status/")
        );
        assert_eq!(
            config.openapi_catalog_url.as_deref(),
            Some(APEX_OPENAPI_CATALOG_URL)
        );
        assert_eq!(config.swagger_ui_url.as_deref(), Some(APEX_SWAGGER_UI_URL));
        assert!(config.ready_for_installation_ingest());
        assert!(config.ready_for_events());
        assert!(config.ready_for_feedback());
        assert!(config.ready_for_feedback_status());
        assert!(config.ready_for_openapi_reference());
    }

    #[test]
    fn default_backend_endpoint_values_cover_ords_and_swagger_contract() {
        let values = default_backend_endpoint_values();

        assert_eq!(values.len(), 9);
        assert!(values.contains(&("base_url", DEFAULT_APEX_BASE_URL)));
        assert!(values.contains(&("openapi_catalog_url", APEX_OPENAPI_CATALOG_URL)));
        assert!(values.contains(&("swagger_ui_url", APEX_SWAGGER_UI_URL)));
    }
}
