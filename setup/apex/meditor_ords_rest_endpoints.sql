set define off

prompt mEditor ORDS REST endpoints
prompt Run this as the APEX/ORDS parsing schema after meditor_installation_metrics_schema.sql.

begin
    ords.enable_schema(
        p_enabled             => true,
        p_schema              => user,
        p_url_mapping_type    => 'BASE_PATH',
        p_url_mapping_pattern => lower(user),
        p_auto_rest_auth      => false
    );
    commit;
end;
/

create or replace package meditor_rest_api authid definer as
    procedure post_installation(
        p_body        in clob,
        p_status_code out number
    );

    procedure post_install_event(
        p_installation_id in varchar2,
        p_body            in clob,
        p_status_code     out number
    );

    procedure post_feedback(
        p_body        in clob,
        p_status_code out number
    );
end meditor_rest_api;
/

create or replace package body meditor_rest_api as
    c_owner_email constant varchar2(320) := 's.pandey.india@gmail.com';
    c_email_purpose constant varchar2(80) := 'ISSUE_FIX_OR_STATUS_NOTIFICATION_ONLY';

    function jstr(
        p_json    in json_object_t,
        p_name    in varchar2,
        p_default in varchar2 default null
    ) return varchar2 is
    begin
        return coalesce(p_json.get_string(p_name), p_default);
    exception
        when others then
            return p_default;
    end jstr;

    function jclob(
        p_json in json_object_t,
        p_name in varchar2
    ) return clob is
    begin
        return p_json.get_clob(p_name);
    exception
        when others then
            return to_clob(jstr(p_json, p_name));
    end jclob;

    function jts(
        p_json in json_object_t,
        p_name in varchar2
    ) return timestamp with time zone is
        l_value varchar2(1000);
    begin
        l_value := jstr(p_json, p_name);
        if l_value is null then
            return null;
        end if;

        if substr(l_value, -1) = 'Z' then
            l_value := substr(l_value, 1, length(l_value) - 1) || '+00:00';
        end if;

        return to_timestamp_tz(l_value, 'YYYY-MM-DD"T"HH24:MI:SSTZH:TZM');
    exception
        when others then
            return null;
    end jts;

    procedure emit_json(
        p_doc in clob
    ) is
    begin
        owa_util.mime_header('application/json', false);
        owa_util.http_header_close;
        htp.p(p_doc);
    end emit_json;

    procedure emit_error(
        p_status_code out number,
        p_http_status in number,
        p_message     in varchar2
    ) is
        l_doc clob;
    begin
        p_status_code := p_http_status;
        select json_object(
                   'ok' value 'false' format json,
                   'error' value p_message
                   returning clob
               )
          into l_doc
          from dual;
        emit_json(l_doc);
    end emit_error;

    procedure post_installation(
        p_body        in clob,
        p_status_code out number
    ) is
        l_body            clob := p_body;
        l_json            json_object_t;
        l_installation_id varchar2(64);
        l_registered_flag char(1);
        l_status          varchar2(40);
        l_doc             clob;
    begin
        l_json := json_object_t.parse(l_body);
        l_installation_id := jstr(l_json, 'installation_id');

        if l_installation_id is null then
            emit_error(p_status_code, 400, 'installation_id is required');
            return;
        end if;

        l_status := jstr(l_json, 'registration_status', 'UNREGISTERED_FULLY_FUNCTIONAL');
        l_registered_flag := jstr(
            l_json,
            'registered_flag',
            case when l_status = 'REGISTERED' then 'Y' else 'N' end
        );

        merge into meditor_installation d
        using (select l_installation_id installation_id from dual) s
           on (d.installation_id = s.installation_id)
         when matched then update set
              d.product_name              = jstr(l_json, 'product_name', 'mEditor'),
              d.app_version               = jstr(l_json, 'app_version'),
              d.registration_status       = l_status,
              d.registered_flag           = l_registered_flag,
              d.user_display_name         = jstr(l_json, 'user_display_name'),
              d.user_email_or_id          = jstr(l_json, 'user_email_or_id'),
              d.personal_license_text     = jstr(l_json, 'personal_license_text'),
              d.license_code              = jstr(l_json, 'license_code', 'LicenseRef-mEditor-Freeware-EULA'),
              d.license_name              = jstr(l_json, 'license_name', 'mEditor Freeware EULA'),
              d.license_version           = jstr(l_json, 'license_version', '1.0'),
              d.license_accepted_flag     = jstr(l_json, 'license_accepted_flag', 'N'),
              d.license_accepted_at       = jts(l_json, 'license_accepted_at'),
              d.license_accepted_by       = jstr(l_json, 'license_accepted_by'),
              d.network_domain            = jstr(l_json, 'network_domain'),
              d.machine_name              = jstr(l_json, 'machine_name'),
              d.ip_address                = jstr(l_json, 'ip_address'),
              d.install_base_identity_use = 'INSTALL_BASE_ONLY',
              d.os_family                 = jstr(l_json, 'os_family'),
              d.cpu_arch                  = jstr(l_json, 'cpu_arch'),
              d.payload_json              = l_body,
              d.updated_at                = systimestamp
         when not matched then insert (
              installation_id,
              product_name,
              app_version,
              registration_status,
              registered_flag,
              user_display_name,
              user_email_or_id,
              personal_license_text,
              license_code,
              license_name,
              license_version,
              license_accepted_flag,
              license_accepted_at,
              license_accepted_by,
              network_domain,
              machine_name,
              ip_address,
              install_base_identity_use,
              os_family,
              cpu_arch,
              payload_json
         ) values (
              l_installation_id,
              jstr(l_json, 'product_name', 'mEditor'),
              jstr(l_json, 'app_version'),
              l_status,
              l_registered_flag,
              jstr(l_json, 'user_display_name'),
              jstr(l_json, 'user_email_or_id'),
              jstr(l_json, 'personal_license_text'),
              jstr(l_json, 'license_code', 'LicenseRef-mEditor-Freeware-EULA'),
              jstr(l_json, 'license_name', 'mEditor Freeware EULA'),
              jstr(l_json, 'license_version', '1.0'),
              jstr(l_json, 'license_accepted_flag', 'N'),
              jts(l_json, 'license_accepted_at'),
              jstr(l_json, 'license_accepted_by'),
              jstr(l_json, 'network_domain'),
              jstr(l_json, 'machine_name'),
              jstr(l_json, 'ip_address'),
              'INSTALL_BASE_ONLY',
              jstr(l_json, 'os_family'),
              jstr(l_json, 'cpu_arch'),
              l_body
         );

        insert into meditor_install_event (
            installation_id,
            event_type,
            app_version,
            os_family,
            cpu_arch,
            payload_json
        ) values (
            l_installation_id,
            case when l_status = 'REGISTERED' then 'REGISTRATION_SUBMITTED' else 'REGISTRATION_DECLINED' end,
            jstr(l_json, 'app_version'),
            jstr(l_json, 'os_family'),
            jstr(l_json, 'cpu_arch'),
            l_body
        );

        commit;
        p_status_code := 201;
        select json_object(
                   'ok' value 'true' format json,
                   'installation_id' value l_installation_id,
                   'registration_status' value l_status
                   returning clob
               )
          into l_doc
          from dual;
        emit_json(l_doc);
    exception
        when others then
            rollback;
            emit_error(p_status_code, 400, sqlerrm);
    end post_installation;

    procedure post_install_event(
        p_installation_id in varchar2,
        p_body            in clob,
        p_status_code     out number
    ) is
        l_body clob := p_body;
        l_json json_object_t;
        l_doc  clob;
    begin
        l_json := json_object_t.parse(l_body);

        if p_installation_id is null then
            emit_error(p_status_code, 400, 'installation_id is required');
            return;
        end if;

        insert into meditor_install_event (
            installation_id,
            event_type,
            app_version,
            os_family,
            cpu_arch,
            metric_name,
            metric_value_number,
            metric_value_text,
            payload_json
        ) values (
            p_installation_id,
            jstr(l_json, 'event_type'),
            jstr(l_json, 'app_version'),
            jstr(l_json, 'os_family'),
            jstr(l_json, 'cpu_arch'),
            jstr(l_json, 'metric_name'),
            to_number(jstr(l_json, 'metric_value_number')),
            jstr(l_json, 'metric_value_text'),
            l_body
        );

        commit;
        p_status_code := 201;
        select json_object(
                   'ok' value 'true' format json,
                   'installation_id' value p_installation_id
                   returning clob
               )
          into l_doc
          from dual;
        emit_json(l_doc);
    exception
        when others then
            rollback;
            emit_error(p_status_code, 400, sqlerrm);
    end post_install_event;

    procedure post_feedback(
        p_body        in clob,
        p_status_code out number
    ) is
        l_body            clob := p_body;
        l_json            json_object_t;
        l_feedback_id     number;
        l_installation_id varchar2(64);
        l_entry_mode      varchar2(40);
        l_event_type      varchar2(40);
        l_doc             clob;
    begin
        l_json := json_object_t.parse(l_body);
        l_installation_id := jstr(l_json, 'installation_id');
        l_entry_mode := jstr(l_json, 'entry_mode', 'MANUAL_USER_FEEDBACK');

        if l_installation_id is null then
            emit_error(p_status_code, 400, 'installation_id is required');
            return;
        end if;

        if jstr(l_json, 'reporter_email_or_id') is null then
            emit_error(p_status_code, 400, 'reporter_email_or_id is required for feedback follow-up');
            return;
        end if;

        insert into meditor_feedback_report (
            installation_id,
            app_version,
            os_family,
            cpu_arch,
            feedback_type,
            entry_mode,
            registered_user_flag,
            reporter_email_or_id,
            reporter_email_purpose,
            owner_contact_email,
            severity,
            category,
            title,
            details,
            steps_to_reproduce,
            expected_behavior,
            actual_behavior,
            error_message,
            error_kind,
            stack_trace,
            active_module,
            active_file,
            active_command,
            log_excerpt,
            open_file_path,
            status,
            status_note,
            client_feedback_ref,
            payload_json
        ) values (
            l_installation_id,
            jstr(l_json, 'app_version'),
            jstr(l_json, 'os_family'),
            jstr(l_json, 'cpu_arch'),
            jstr(l_json, 'feedback_type', 'BUG_REPORT'),
            l_entry_mode,
            jstr(l_json, 'registered_user_flag', 'N'),
            jstr(l_json, 'reporter_email_or_id'),
            c_email_purpose,
            c_owner_email,
            jstr(l_json, 'severity'),
            jstr(l_json, 'category'),
            jstr(l_json, 'title'),
            jclob(l_json, 'details'),
            jclob(l_json, 'steps_to_reproduce'),
            jclob(l_json, 'expected_behavior'),
            jclob(l_json, 'actual_behavior'),
            jclob(l_json, 'error_message'),
            jstr(l_json, 'error_kind'),
            jclob(l_json, 'stack_trace'),
            jstr(l_json, 'active_module'),
            jstr(l_json, 'active_file'),
            jstr(l_json, 'active_command'),
            jclob(l_json, 'log_excerpt'),
            jstr(l_json, 'open_file_path'),
            'SUBMITTED',
            'Submitted from mEditor',
            jstr(l_json, 'client_feedback_ref'),
            l_body
        )
        returning feedback_id into l_feedback_id;

        insert into meditor_feedback_status_history (
            feedback_id,
            status,
            status_note,
            visible_to_user_flag,
            updated_by
        ) values (
            l_feedback_id,
            'SUBMITTED',
            'Submitted from mEditor',
            'Y',
            'mEditor client'
        );

        l_event_type := case
                            when l_entry_mode = 'AUTOMATIC_ERROR_CAPTURE' then 'AUTOMATIC_ERROR_CAPTURED'
                            else 'FEEDBACK_SUBMITTED'
                        end;

        insert into meditor_install_event (
            installation_id,
            event_type,
            app_version,
            os_family,
            cpu_arch,
            payload_json
        ) values (
            l_installation_id,
            l_event_type,
            jstr(l_json, 'app_version'),
            jstr(l_json, 'os_family'),
            jstr(l_json, 'cpu_arch'),
            l_body
        );

        commit;
        p_status_code := 201;
        select json_object(
                   'ok' value 'true' format json,
                   'feedback_id' value l_feedback_id,
                   'status' value 'SUBMITTED',
                   'owner_contact_email' value c_owner_email
                   returning clob
               )
          into l_doc
          from dual;
        emit_json(l_doc);
    exception
        when others then
            rollback;
            emit_error(p_status_code, 400, sqlerrm);
    end post_feedback;
end meditor_rest_api;
/

begin
    begin
        ords.delete_module(p_module_name => 'meditor.v1');
    exception
        when others then
            null;
    end;

    ords.define_module(
        p_module_name    => 'meditor.v1',
        p_base_path      => '/meditor/v1/',
        p_items_per_page => 25,
        p_status         => 'PUBLISHED',
        p_comments       => 'mEditor installation, license, event, feedback, and bug-report endpoints.'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/',
        p_etag_type   => 'NONE',
        p_comments    => 'Create or update an mEditor installation record.'
    );

    ords.define_handler(
        p_module_name   => 'meditor.v1',
        p_pattern       => 'installations/',
        p_method        => 'POST',
        p_source_type   => ords.source_type_plsql,
        p_mimes_allowed => 'application/json',
        p_source        => q'[
declare
    l_body        clob := :body_text;
    l_status_code number;
begin
    meditor_rest_api.post_installation(l_body, l_status_code);
    :status_code := l_status_code;
end;
]'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/:installation_id/events/',
        p_etag_type   => 'NONE',
        p_comments    => 'Append an mEditor installation event.'
    );

    ords.define_handler(
        p_module_name   => 'meditor.v1',
        p_pattern       => 'installations/:installation_id/events/',
        p_method        => 'POST',
        p_source_type   => ords.source_type_plsql,
        p_mimes_allowed => 'application/json',
        p_source        => q'[
declare
    l_body        clob := :body_text;
    l_status_code number;
begin
    meditor_rest_api.post_install_event(:installation_id, l_body, l_status_code);
    :status_code := l_status_code;
end;
]'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'feedback/',
        p_etag_type   => 'NONE',
        p_comments    => 'Submit an mEditor bug report, automatic error report, or enhancement request.'
    );

    ords.define_handler(
        p_module_name   => 'meditor.v1',
        p_pattern       => 'feedback/',
        p_method        => 'POST',
        p_source_type   => ords.source_type_plsql,
        p_mimes_allowed => 'application/json',
        p_source        => q'[
declare
    l_body        clob := :body_text;
    l_status_code number;
begin
    meditor_rest_api.post_feedback(l_body, l_status_code);
    :status_code := l_status_code;
end;
]'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/:installation_id/feedback/',
        p_etag_type   => 'NONE',
        p_comments    => 'List feedback and bug reports for one mEditor installation.'
    );

    ords.define_handler(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/:installation_id/feedback/',
        p_method      => 'GET',
        p_source_type => ords.source_type_query,
        p_source      => q'[
select feedback_id,
       feedback_type,
       entry_mode,
       severity,
       category,
       title,
       status,
       status_note,
       apex_issue_url,
       owner_contact_email,
       created_at,
       updated_at
  from meditor_feedback_report
 where installation_id = :installation_id
 order by created_at desc
]'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/:installation_id/feedback/summary/',
        p_etag_type   => 'NONE',
        p_comments    => 'Show user-visible feedback metrics for one mEditor installation.'
    );

    ords.define_handler(
        p_module_name => 'meditor.v1',
        p_pattern     => 'installations/:installation_id/feedback/summary/',
        p_method      => 'GET',
        p_source_type => ords.source_type_query,
        p_source      => q'[
select count(*) feedback_given,
       sum(case when feedback_type = 'BUG_REPORT' then 1 else 0 end) bugs_reported,
       sum(case when feedback_type = 'ENHANCEMENT_REQUEST' then 1 else 0 end) enhancements_requested,
       sum(case when status in ('SUBMITTED', 'TRIAGED', 'IN_PROGRESS', 'NEEDS_INFO', 'DEFERRED') then 1 else 0 end) open_items,
       sum(case when status in ('FIXED', 'RELEASED', 'CLOSED') then 1 else 0 end) fixed_items,
       max(status) keep (dense_rank last order by updated_at) latest_status,
       max(updated_at) latest_status_at,
       max(owner_contact_email) owner_contact_email
  from meditor_feedback_report
 where installation_id = :installation_id
]'
    );

    ords.define_template(
        p_module_name => 'meditor.v1',
        p_pattern     => 'feedback/:feedback_id/status/',
        p_etag_type   => 'NONE',
        p_comments    => 'Show visible status history for one mEditor feedback item.'
    );

    ords.define_handler(
        p_module_name => 'meditor.v1',
        p_pattern     => 'feedback/:feedback_id/status/',
        p_method      => 'GET',
        p_source_type => ords.source_type_query,
        p_source      => q'[
select h.feedback_status_id,
       h.feedback_id,
       h.status,
       h.status_note,
       h.updated_by,
       h.created_at
  from meditor_feedback_status_history h
 where h.feedback_id = :feedback_id
   and h.visible_to_user_flag = 'Y'
 order by h.created_at
]'
    );

    commit;
end;
/

prompt Endpoints created under /ords/<schema-alias>/meditor/v1/
prompt Share the final base URL with mEditor, for example:
prompt https://your-host.example.com/ords/<schema-alias>/meditor/v1/
