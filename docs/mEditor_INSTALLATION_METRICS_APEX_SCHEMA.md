# mEditor Installation Data, License Acceptance, And Bug Reporting

## Purpose

mEditor is freeware. Registration is optional and is only used to make an installation a personal copy and to inform the user about future updates. The full product remains available without registration and without licensing or financial obligation to the user or to the user's organisation.

## First-Run Flow

On first launch, mEditor should show a registration panel with this message:

```text
mEditor is freeware. Registration is optional and is only used to make this installation your personal copy and to inform you about future updates. There is no licensing or financial obligation to you or to your organisation. You can continue with the full product without registration. If you continue without registration, mEditor will send only this installation's network domain, machine name, and IP address to record an unregistered fully functional copy.
```

Registration choices:

- Register: user confirms or edits the OS account hint, email/id, and display name. The About window shows `Personal License to USERNAME`.
- Continue unregistered: the About window shows `UnRegistered but fully functional copy with no obligation`.
- License acceptance: the Registration dialog displays the active `mEditor Freeware EULA` and requires acceptance before the product opens.
- About dialog: includes a license button that displays the same active EULA and the accepted license code/version.
- Install-base identification: network domain, machine name, and IP address are captured only to identify the install base.
- Feedback and bug reporting: the top-right Feedback icon opens a same-window form for bug reports and enhancement requests. Automatic error mode captures diagnostics and then asks the user to submit.
- Direct owner contact: the feedback UI displays `s.pandey.india@gmail.com` for users who prefer direct correspondence.

## Install-Base Boundary

IP address, network domain, and machine name are stored on `meditor_installation` only. They are used only to identify the mEditor install base and must not be copied into bug reports, launch events, feature reporting, or any other reporting table.

Registration remains optional. EULA acceptance is required for both registered and unregistered use because it defines the freeware use terms.

## APEX Tables

The backend needs five tables:

- `meditor_installation`: one row per installation id and the current registration state.
- `meditor_install_event`: append-only launch, registration, update-check, feedback-submission, and automatic-error-capture events.
- `meditor_feedback_report`: user-submitted bug reports, automatic error reports, and enhancement requests.
- `meditor_feedback_status_history`: status timeline shown back to the user.
- `meditor_rest_ingest_log`: optional REST ingest audit and troubleshooting log.

The SQL file is provided at:

```text
setup/apex/meditor_installation_metrics_schema.sql
```

The ORDS REST endpoint script is provided at:

```text
setup/apex/meditor_ords_rest_endpoints.sql
```

Run order:

```sql
@setup/apex/meditor_installation_metrics_schema.sql
@setup/apex/meditor_ords_rest_endpoints.sql
```

## Configured REST Endpoints

The APEX REST modules are created and the mEditor client baseline is configured for these live URLs:

- Base URL: `https://oracleapex.com/ords/wksp_myhobbies/meditor/v1`
- `POST https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/`
- `POST https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/events/`
- `POST https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/`
- `GET https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/`
- `GET https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/summary/`
- `GET https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/:feedback_id/status/`

OpenAPI and Swagger references:

- OpenAPI catalog: `https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/`
- Swagger UI: `https://oracleapex.com/swagger/index.html?url=https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/`

The desktop client keeps these URLs in an mEditor settings file and in local SQLite table `rest_endpoint_config`. The one-click SQLite setup seeds row `oracleapex-wksp-myhobbies-meditor-v1` with the URLs above.

## Feedback Interface

The menu bar includes a top-right Feedback icon. It opens a same-window feedback panel with two modes:

- Automatic error capture: opened when mEditor catches an error. It captures relevant diagnostic fields needed for bug fixing, including app version, OS family, CPU architecture, error message, error kind, stack trace, active module, active file, active command, and recent log excerpt.
- Manual feedback: opened from the Feedback icon. The user chooses `Bug Report` or `Enhancement Request`, then enters title, details, severity where applicable, category, steps, expected behavior, and actual behavior.

Email rule:

- Registered users use their registered email/id.
- Unregistered users are asked for a personal email id only when they submit feedback.
- The prompt must say: `Please provide your personal email id for this issue. It will be used only to inform you about the bugfix or status update for this bug report or enhancement request.`

The user-facing feedback panel should show:

- Feedback given.
- Bugs reported.
- Enhancements requested.
- Open items.
- Fixed items.
- Latest visible status.

The UI must display owner contact email `s.pandey.india@gmail.com` for direct correspondence.

## Feedback Payload

```json
{
  "installation_id": "local-generated-id",
  "app_version": "2.0.0.0-pre-pilot",
  "os_family": "MacOS",
  "cpu_arch": "Arm64",
  "feedback_type": "BUG_REPORT",
  "entry_mode": "AUTOMATIC_ERROR_CAPTURE",
  "registered_user_flag": "N",
  "reporter_email_or_id": "user.personal@example.com",
  "reporter_email_purpose": "ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY",
  "owner_contact_email": "s.pandey.india@gmail.com",
  "severity": "HIGH",
  "category": "Runtime Error",
  "title": "Crash while saving file",
  "details": "mEditor captured a save failure.",
  "steps_to_reproduce": "Captured automatically from the error context.",
  "actual_behavior": "panic while saving file",
  "error_message": "panic while saving file",
  "error_kind": "panic",
  "stack_trace": "stack frame 1\nstack frame 2",
  "active_module": "editor",
  "active_file": "/workspace/demo.rs",
  "active_command": "file.save",
  "status": "SUBMITTED"
}
```

## Registered Installation Payload

```json
{
  "installation_id": "local-generated-id",
  "product_name": "mEditor",
  "app_version": "2.0.0.0-pre-pilot",
  "registration_status": "REGISTERED",
  "registered_flag": "Y",
  "user_display_name": "Sanjay Pandey",
  "user_email_or_id": "s.pandey.india@gmail.com",
  "personal_license_text": "Personal License to Sanjay Pandey",
  "license_code": "LicenseRef-mEditor-Freeware-EULA",
  "license_name": "mEditor Freeware EULA",
  "license_version": "1.0",
  "license_accepted_flag": "Y",
  "license_accepted_at": "2026-05-19T00:00:00Z",
  "license_accepted_by": "s.pandey.india@gmail.com",
  "network_domain": "example.local",
  "machine_name": "developer-laptop",
  "ip_address": "192.0.2.10",
  "install_base_identity_use": "INSTALL_BASE_ONLY",
  "os_family": "MacOS",
  "cpu_arch": "Arm64"
}
```

## Unregistered Installation Payload

```json
{
  "installation_id": "local-generated-id",
  "product_name": "mEditor",
  "app_version": "2.0.0.0-pre-pilot",
  "registration_status": "UNREGISTERED_FULLY_FUNCTIONAL",
  "registered_flag": "N",
  "personal_license_text": "UnRegistered but fully functional copy with no obligation",
  "license_code": "LicenseRef-mEditor-Freeware-EULA",
  "license_name": "mEditor Freeware EULA",
  "license_version": "1.0",
  "license_accepted_flag": "Y",
  "license_accepted_at": "2026-05-19T00:00:00Z",
  "network_domain": "example.local",
  "machine_name": "developer-laptop",
  "ip_address": "192.0.2.10",
  "install_base_identity_use": "INSTALL_BASE_ONLY",
  "os_family": "MacOS",
  "cpu_arch": "Arm64"
}
```

## Active License

The active license file is:

```text
mEditor/LICENSE.md
```

The active license metadata is:

- License code: `LicenseRef-mEditor-Freeware-EULA`
- License name: `mEditor Freeware EULA`
- License version: `1.0`

The product may later switch to `GPL-3.0-only` or `GPL-3.0-or-later` if the owner chooses an open-source release.
