create table meditor_installation (
    installation_id varchar2(64) not null,
    product_name varchar2(32) default 'mEditor' not null,
    app_version varchar2(32) not null,
    registration_status varchar2(40) not null,
    registered_flag char(1) default 'N' not null,
    user_display_name varchar2(200),
    user_email_or_id varchar2(320),
    personal_license_text varchar2(500),
    license_code varchar2(100) default 'LicenseRef-mEditor-Freeware-EULA' not null,
    license_name varchar2(200) default 'mEditor Freeware EULA' not null,
    license_version varchar2(40) default '1.0' not null,
    license_accepted_flag char(1) default 'N' not null,
    license_accepted_at timestamp with time zone,
    license_accepted_by varchar2(320),
    network_domain varchar2(255),
    machine_name varchar2(255),
    ip_address varchar2(64),
    install_base_identity_use varchar2(40) default 'INSTALL_BASE_ONLY' not null,
    os_family varchar2(64),
    cpu_arch varchar2(64),
    payload_json clob,
    created_at timestamp with time zone default systimestamp not null,
    updated_at timestamp with time zone default systimestamp not null,
    constraint meditor_installation_pk primary key (installation_id),
    constraint meditor_installation_product_ck check (product_name = 'mEditor'),
    constraint meditor_installation_reg_ck check (
        registration_status in (
            'REGISTERED',
            'UNREGISTERED_FULLY_FUNCTIONAL'
        )
    ),
    constraint meditor_installation_reg_flag_ck check (registered_flag in ('Y', 'N')),
    constraint meditor_installation_license_ck check (license_accepted_flag in ('Y', 'N')),
    constraint meditor_installation_identity_use_ck check (install_base_identity_use = 'INSTALL_BASE_ONLY'),
    constraint meditor_installation_payload_json_ck check (payload_json is json)
);

create index meditor_installation_email_i on meditor_installation (user_email_or_id);
create index meditor_installation_status_i on meditor_installation (registration_status);
create index meditor_installation_seen_i on meditor_installation (updated_at);

create table meditor_install_event (
    event_id number generated always as identity,
    installation_id varchar2(64) not null,
    event_type varchar2(64) not null,
    event_ts timestamp with time zone default systimestamp not null,
    app_version varchar2(32),
    os_family varchar2(64),
    cpu_arch varchar2(64),
    metric_name varchar2(128),
    metric_value_number number,
    metric_value_text varchar2(1000),
    payload_json clob,
    constraint meditor_install_event_pk primary key (event_id),
    constraint meditor_install_event_install_fk foreign key (installation_id)
        references meditor_installation (installation_id),
    constraint meditor_install_event_type_ck check (
        event_type in (
            'FIRST_LAUNCH',
            'LICENSE_ACCEPTED',
            'REGISTRATION_SUBMITTED',
            'REGISTRATION_DECLINED',
            'UPDATE_CHECK',
            'LAUNCH',
            'FEEDBACK_SUBMITTED',
            'AUTOMATIC_ERROR_CAPTURED'
        )
    ),
    constraint meditor_install_event_payload_json_ck check (payload_json is json)
);

create index meditor_install_event_install_i on meditor_install_event (installation_id);
create index meditor_install_event_type_i on meditor_install_event (event_type, event_ts);

create table meditor_feedback_report (
    feedback_id number generated always as identity,
    installation_id varchar2(64) not null,
    app_version varchar2(32),
    os_family varchar2(64),
    cpu_arch varchar2(64),
    feedback_type varchar2(40) not null,
    entry_mode varchar2(40) not null,
    registered_user_flag char(1) default 'N' not null,
    reporter_email_or_id varchar2(320) not null,
    reporter_email_purpose varchar2(80) default 'ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY' not null,
    owner_contact_email varchar2(320) default 's.pandey.india@gmail.com' not null,
    severity varchar2(16),
    category varchar2(100),
    title varchar2(500) not null,
    details clob not null,
    steps_to_reproduce clob,
    expected_behavior clob,
    actual_behavior clob,
    error_message clob,
    error_kind varchar2(100),
    stack_trace clob,
    active_module varchar2(200),
    active_file varchar2(1000),
    active_command varchar2(200),
    log_excerpt clob,
    open_file_path varchar2(1000),
    status varchar2(40) default 'SUBMITTED' not null,
    status_note varchar2(1000),
    apex_issue_url varchar2(1000),
    client_feedback_ref varchar2(100),
    payload_json clob,
    created_at timestamp with time zone default systimestamp not null,
    updated_at timestamp with time zone default systimestamp not null,
    constraint meditor_feedback_report_pk primary key (feedback_id),
    constraint meditor_feedback_report_install_fk foreign key (installation_id)
        references meditor_installation (installation_id),
    constraint meditor_feedback_type_ck check (
        feedback_type in ('BUG_REPORT', 'ENHANCEMENT_REQUEST')
    ),
    constraint meditor_feedback_entry_ck check (
        entry_mode in ('AUTOMATIC_ERROR_CAPTURE', 'MANUAL_USER_FEEDBACK')
    ),
    constraint meditor_feedback_registered_ck check (registered_user_flag in ('Y', 'N')),
    constraint meditor_feedback_email_use_ck check (
        reporter_email_purpose = 'ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY'
    ),
    constraint meditor_feedback_owner_email_ck check (owner_contact_email = 's.pandey.india@gmail.com'),
    constraint meditor_feedback_sev_ck check (
        severity is null or severity in ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')
    ),
    constraint meditor_feedback_status_ck check (
        status in ('SUBMITTED', 'TRIAGED', 'IN_PROGRESS', 'FIXED', 'RELEASED', 'NEEDS_INFO', 'DEFERRED', 'CLOSED')
    ),
    constraint meditor_feedback_payload_json_ck check (payload_json is json)
);

create index meditor_feedback_install_i on meditor_feedback_report (installation_id);
create index meditor_feedback_status_i on meditor_feedback_report (status, created_at);
create index meditor_feedback_type_i on meditor_feedback_report (feedback_type, entry_mode);
create index meditor_feedback_email_i on meditor_feedback_report (reporter_email_or_id);

create table meditor_feedback_status_history (
    feedback_status_id number generated always as identity,
    feedback_id number not null,
    status varchar2(40) not null,
    status_note varchar2(1000),
    visible_to_user_flag char(1) default 'Y' not null,
    updated_by varchar2(320),
    created_at timestamp with time zone default systimestamp not null,
    constraint meditor_feedback_status_pk primary key (feedback_status_id),
    constraint meditor_feedback_status_report_fk foreign key (feedback_id)
        references meditor_feedback_report (feedback_id),
    constraint meditor_feedback_status_val_ck check (
        status in ('SUBMITTED', 'TRIAGED', 'IN_PROGRESS', 'FIXED', 'RELEASED', 'NEEDS_INFO', 'DEFERRED', 'CLOSED')
    ),
    constraint meditor_feedback_status_visible_ck check (visible_to_user_flag in ('Y', 'N'))
);

create index meditor_feedback_status_hist_i on meditor_feedback_status_history (feedback_id, created_at);

create table meditor_rest_ingest_log (
    ingest_id number generated always as identity,
    endpoint_name varchar2(100) not null,
    installation_id varchar2(64),
    http_status number,
    payload_json clob,
    error_message varchar2(4000),
    received_at timestamp with time zone default systimestamp not null,
    constraint meditor_rest_ingest_log_pk primary key (ingest_id),
    constraint meditor_rest_ingest_log_payload_json_ck check (payload_json is json)
);

create index meditor_rest_ingest_log_install_i on meditor_rest_ingest_log (installation_id);
create index meditor_rest_ingest_log_endpoint_i on meditor_rest_ingest_log (endpoint_name, received_at);
