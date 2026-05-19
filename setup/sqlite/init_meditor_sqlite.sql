PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;

CREATE TABLE IF NOT EXISTS vulnerability (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    cve_id TEXT,
    cwe TEXT,
    package_name TEXT,
    affected_version TEXT,
    title TEXT NOT NULL,
    description TEXT,
    severity TEXT,
    cvss_vector TEXT,
    cvss_score REAL NOT NULL DEFAULT 0,
    remediation TEXT,
    reference_url TEXT,
    search_terms TEXT,
    published_at TEXT,
    updated_at TEXT,
    UNIQUE(source, cve_id, package_name, affected_version)
);

CREATE INDEX IF NOT EXISTS idx_vuln_cve ON vulnerability(cve_id);
CREATE INDEX IF NOT EXISTS idx_vuln_pkg ON vulnerability(package_name);
CREATE INDEX IF NOT EXISTS idx_vuln_score ON vulnerability(cvss_score DESC);
CREATE INDEX IF NOT EXISTS idx_vuln_terms ON vulnerability(search_terms);

CREATE TABLE IF NOT EXISTS sync_state (
    source TEXT PRIMARY KEY,
    last_success TEXT,
    last_attempt TEXT,
    status TEXT NOT NULL,
    message TEXT
);

CREATE TABLE IF NOT EXISTS rule_pack (
    id TEXT PRIMARY KEY,
    version TEXT NOT NULL,
    source TEXT NOT NULL,
    installed_at TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS scan_run (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scope TEXT NOT NULL,
    target TEXT NOT NULL,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    status TEXT NOT NULL,
    finding_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS finding (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_run_id INTEGER NOT NULL,
    rule_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    severity TEXT NOT NULL,
    cvss_score REAL NOT NULL DEFAULT 0,
    cwe TEXT,
    summary TEXT NOT NULL,
    remediation TEXT,
    snippet TEXT,
    FOREIGN KEY(scan_run_id) REFERENCES scan_run(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS finding_suppression (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_id TEXT NOT NULL,
    file_path TEXT,
    reason TEXT NOT NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT
);

CREATE TABLE IF NOT EXISTS rest_endpoint_config (
    id TEXT PRIMARY KEY,
    base_url TEXT NOT NULL,
    installation_url TEXT NOT NULL,
    install_event_url_template TEXT NOT NULL,
    feedback_url TEXT NOT NULL,
    feedback_list_url_template TEXT NOT NULL,
    feedback_summary_url_template TEXT NOT NULL,
    feedback_status_url_template TEXT NOT NULL,
    openapi_catalog_url TEXT,
    swagger_ui_url TEXT,
    owner_contact_email TEXT NOT NULL DEFAULT 's.pandey.india@gmail.com',
    active INTEGER NOT NULL DEFAULT 1,
    configured_at TEXT NOT NULL,
    updated_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_rest_endpoint_active ON rest_endpoint_config(active);

INSERT OR REPLACE INTO rest_endpoint_config (
    id,
    base_url,
    installation_url,
    install_event_url_template,
    feedback_url,
    feedback_list_url_template,
    feedback_summary_url_template,
    feedback_status_url_template,
    openapi_catalog_url,
    swagger_ui_url,
    owner_contact_email,
    active,
    configured_at,
    updated_at
) VALUES (
    'oracleapex-wksp-myhobbies-meditor-v1',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/events/',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/:installation_id/feedback/summary/',
    'https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/feedback/:feedback_id/status/',
    'https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/',
    'https://oracleapex.com/swagger/index.html?url=https://oracleapex.com/ords/wksp_myhobbies/open-api-catalog/meditor/v1/',
    's.pandey.india@gmail.com',
    1,
    '2026-05-19T00:00:00+05:30',
    '2026-05-19T00:00:00+05:30'
);
