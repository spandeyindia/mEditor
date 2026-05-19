use std::path::{Path, PathBuf};

pub const VULNERABILITY_DB_FILE: &str = "vulnerability-intel.sqlite";
pub const ENCRYPTED_CACHE_FILE: &str = "vulnerability-intel-cache.enc";
pub const SCAN_HISTORY_DB_FILE: &str = "security-scan-history.sqlite";
pub const SQLITE_SETUP_DIR: &str = "setup/sqlite";
pub const SQLITE_TOP_LEVEL_UNIX_SETUP: &str = "setup/setup.sh";
pub const SQLITE_TOP_LEVEL_MAC_SETUP: &str = "setup/setup.command";
pub const SQLITE_TOP_LEVEL_WINDOWS_SETUP: &str = "setup/setup.cmd";
pub const SQLITE_SCHEMA_FILE: &str = "setup/sqlite/init_meditor_sqlite.sql";
pub const SQLITE_UNIX_SETUP_SCRIPT: &str = "setup/sqlite/setup-sqlite.sh";
pub const SQLITE_WINDOWS_SETUP_SCRIPT: &str = "setup/sqlite/setup-sqlite.ps1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryLayout {
    pub security_dir: PathBuf,
    pub vulnerability_db: PathBuf,
    pub encrypted_cache: PathBuf,
    pub scan_history_db: PathBuf,
}

impl RepositoryLayout {
    pub fn under_working_folder(working_folder: impl AsRef<Path>) -> Self {
        let security_dir = working_folder.as_ref().join(".meditor").join("security");
        Self {
            vulnerability_db: security_dir.join(VULNERABILITY_DB_FILE),
            encrypted_cache: security_dir.join(ENCRYPTED_CACHE_FILE),
            scan_history_db: security_dir.join(SCAN_HISTORY_DB_FILE),
            security_dir,
        }
    }
}

pub fn schema_sql() -> &'static str {
    include_str!("../../../setup/sqlite/init_meditor_sqlite.sql")
}

pub fn sqlite_setup_files() -> &'static [&'static str] {
    &[
        SQLITE_TOP_LEVEL_UNIX_SETUP,
        SQLITE_TOP_LEVEL_MAC_SETUP,
        SQLITE_TOP_LEVEL_WINDOWS_SETUP,
        SQLITE_SCHEMA_FILE,
        SQLITE_UNIX_SETUP_SCRIPT,
        SQLITE_WINDOWS_SETUP_SCRIPT,
    ]
}

pub fn update_sources() -> &'static [&'static str] {
    &["NVD", "CISA-KEV"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_layout_matches_frozen_working_folder_contract() {
        let layout = RepositoryLayout::under_working_folder("/workspace/project");
        assert!(layout
            .vulnerability_db
            .ends_with(".meditor/security/vulnerability-intel.sqlite"));
        assert!(layout
            .encrypted_cache
            .ends_with(".meditor/security/vulnerability-intel-cache.enc"));
        assert!(layout
            .scan_history_db
            .ends_with(".meditor/security/security-scan-history.sqlite"));
    }

    #[test]
    fn schema_includes_required_tables_and_indexes() {
        let schema = schema_sql();
        for required in [
            "CREATE TABLE IF NOT EXISTS vulnerability",
            "CREATE TABLE IF NOT EXISTS sync_state",
            "CREATE TABLE IF NOT EXISTS scan_run",
            "CREATE TABLE IF NOT EXISTS finding",
            "CREATE TABLE IF NOT EXISTS finding_suppression",
            "CREATE TABLE IF NOT EXISTS rule_pack",
            "CREATE TABLE IF NOT EXISTS rest_endpoint_config",
            "CREATE INDEX IF NOT EXISTS idx_vuln_cve",
            "CREATE INDEX IF NOT EXISTS idx_vuln_pkg",
            "CREATE INDEX IF NOT EXISTS idx_rest_endpoint_active",
            "openapi_catalog_url",
            "swagger_ui_url",
            "INSERT OR REPLACE INTO rest_endpoint_config",
            "https://oracleapex.com/ords/wksp_myhobbies/meditor/v1/installations/",
        ] {
            assert!(
                schema.contains(required),
                "missing schema fragment: {required}"
            );
        }
    }

    #[test]
    fn update_sources_include_nvd_and_cisa_kev() {
        assert_eq!(update_sources(), &["NVD", "CISA-KEV"]);
    }

    #[test]
    fn setup_files_are_registered_for_cross_platform_sqlite_setup() {
        assert_eq!(SQLITE_SETUP_DIR, "setup/sqlite");
        assert!(sqlite_setup_files().contains(&SQLITE_TOP_LEVEL_UNIX_SETUP));
        assert!(sqlite_setup_files().contains(&SQLITE_TOP_LEVEL_MAC_SETUP));
        assert!(sqlite_setup_files().contains(&SQLITE_TOP_LEVEL_WINDOWS_SETUP));
        assert!(sqlite_setup_files().contains(&SQLITE_SCHEMA_FILE));
        assert!(sqlite_setup_files().contains(&SQLITE_UNIX_SETUP_SCRIPT));
        assert!(sqlite_setup_files().contains(&SQLITE_WINDOWS_SETUP_SCRIPT));
    }
}
