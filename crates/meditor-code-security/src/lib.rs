use std::fmt;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageFamily {
    C,
    Java,
    JavaScript,
    Php,
    Python,
    Perl,
    Shell,
    Sql,
    Unknown,
}

impl LanguageFamily {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        match path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "c" | "h" | "cc" | "cpp" | "cxx" | "hpp" => Self::C,
            "java" | "kt" | "groovy" | "scala" => Self::Java,
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => Self::JavaScript,
            "php" | "phtml" => Self::Php,
            "py" | "pyw" => Self::Python,
            "pl" | "pm" | "t" => Self::Perl,
            "sh" | "bash" | "zsh" | "ksh" => Self::Shell,
            "sql" | "pls" | "plsql" | "tsql" => Self::Sql,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchMode {
    AnyPattern,
    AllPatterns,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SecurityRule {
    pub id: &'static str,
    pub language: LanguageFamily,
    pub patterns: &'static [&'static str],
    pub match_mode: MatchMode,
    pub severity: Severity,
    pub cvss_score: f32,
    pub cwe: &'static str,
    pub summary: &'static str,
    pub remediation: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SecurityFinding {
    pub rule_id: String,
    pub file_path: String,
    pub line_number: usize,
    pub severity: Severity,
    pub cvss_score: f32,
    pub cwe: String,
    pub summary: String,
    pub remediation: String,
    pub snippet: String,
}

#[derive(Clone, Debug)]
pub struct CodeSecurityScanner {
    rules: Vec<SecurityRule>,
}

impl CodeSecurityScanner {
    pub fn with_seed_rules() -> Self {
        Self {
            rules: seed_rules(),
        }
    }

    pub fn rules(&self) -> &[SecurityRule] {
        &self.rules
    }

    pub fn scan_text(&self, file_path: impl AsRef<Path>, source: &str) -> Vec<SecurityFinding> {
        let path = file_path.as_ref();
        let language = LanguageFamily::from_path(path);
        let display_path = path.display().to_string();
        source
            .lines()
            .enumerate()
            .flat_map(|(line_index, line)| {
                self.rules
                    .iter()
                    .filter(move |rule| rule.language == language)
                    .filter(move |rule| matches_rule(line, rule))
                    .map({
                        let display_path = display_path.clone();
                        move |rule| SecurityFinding {
                            rule_id: rule.id.to_string(),
                            file_path: display_path.clone(),
                            line_number: line_index + 1,
                            severity: rule.severity,
                            cvss_score: rule.cvss_score,
                            cwe: rule.cwe.to_string(),
                            summary: rule.summary.to_string(),
                            remediation: rule.remediation.to_string(),
                            snippet: line.trim().to_string(),
                        }
                    })
            })
            .collect()
    }
}

fn matches_rule(line: &str, rule: &SecurityRule) -> bool {
    let line = line.to_ascii_lowercase();
    match rule.match_mode {
        MatchMode::AnyPattern => rule
            .patterns
            .iter()
            .any(|pattern| line.contains(&pattern.to_ascii_lowercase())),
        MatchMode::AllPatterns => rule
            .patterns
            .iter()
            .all(|pattern| line.contains(&pattern.to_ascii_lowercase())),
    }
}

pub fn seed_rules() -> Vec<SecurityRule> {
    vec![
        SecurityRule {
            id: "C-UNSAFE-STRING-COPY",
            language: LanguageFamily::C,
            patterns: &["strcpy(", "strcat(", "gets("],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::High,
            cvss_score: 8.1,
            cwe: "CWE-120",
            summary: "Unsafe C string/buffer function may allow memory corruption.",
            remediation: "Use bounded APIs, validate lengths, and prefer safer abstractions.",
        },
        SecurityRule {
            id: "JAVA-RUNTIME-EXEC",
            language: LanguageFamily::Java,
            patterns: &["runtime.getruntime().exec", "processbuilder("],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::High,
            cvss_score: 8.0,
            cwe: "CWE-78",
            summary: "Command execution from Java code can become OS command injection.",
            remediation: "Avoid shell execution or strictly validate command and arguments.",
        },
        SecurityRule {
            id: "JS-EVAL",
            language: LanguageFamily::JavaScript,
            patterns: &["eval(", "new function("],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::High,
            cvss_score: 8.8,
            cwe: "CWE-95",
            summary: "Dynamic JavaScript evaluation can execute untrusted code.",
            remediation: "Replace dynamic evaluation with explicit parsing or safe dispatch.",
        },
        SecurityRule {
            id: "PHP-DYNAMIC-EVAL",
            language: LanguageFamily::Php,
            patterns: &["eval(", "assert($_", "system($_", "shell_exec($_"],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::Critical,
            cvss_score: 9.3,
            cwe: "CWE-95",
            summary: "Dynamic PHP execution with user-controlled input can execute arbitrary code.",
            remediation: "Remove dynamic execution and use safe command/query abstractions.",
        },
        SecurityRule {
            id: "PYTHON-SHELL-TRUE",
            language: LanguageFamily::Python,
            patterns: &[
                "shell=true",
                "eval(",
                "exec(",
                "os.system(",
                "subprocess.popen(",
            ],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::High,
            cvss_score: 8.2,
            cwe: "CWE-78",
            summary: "Python dynamic execution or shell invocation may execute untrusted input.",
            remediation: "Use argument arrays, disable shell mode, and avoid eval/exec.",
        },
        SecurityRule {
            id: "PYTHON-SQL-CONCAT",
            language: LanguageFamily::Python,
            patterns: &["execute(\"select", " + "],
            match_mode: MatchMode::AllPatterns,
            severity: Severity::High,
            cvss_score: 8.1,
            cwe: "CWE-89",
            summary: "Python SQL construction may concatenate untrusted input.",
            remediation: "Use parameterized queries and pass bind values separately.",
        },
        SecurityRule {
            id: "PERL-SYSTEM-USER-INPUT",
            language: LanguageFamily::Perl,
            patterns: &["system($", "exec($", "eval $"],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::High,
            cvss_score: 8.0,
            cwe: "CWE-78",
            summary: "Perl dynamic execution may execute user-controlled commands.",
            remediation: "Use list-form execution and validate all arguments.",
        },
        SecurityRule {
            id: "SHELL-DANGEROUS-DELETE",
            language: LanguageFamily::Shell,
            patterns: &["rm -rf /", "rm -rf $", "chmod 777"],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::Critical,
            cvss_score: 9.0,
            cwe: "CWE-732",
            summary: "Dangerous shell command may destroy files or weaken permissions.",
            remediation: "Add path guards, confirmations, and least-privilege permissions.",
        },
        SecurityRule {
            id: "SQL-STRING-CONCAT",
            language: LanguageFamily::Sql,
            patterns: &["||", "execute immediate", "sp_executesql"],
            match_mode: MatchMode::AnyPattern,
            severity: Severity::Medium,
            cvss_score: 6.5,
            cwe: "CWE-89",
            summary: "Dynamic SQL should be reviewed for injection risk.",
            remediation:
                "Use bind variables, parameterized APIs, and strict identifier allowlists.",
        },
    ]
}

pub fn repository_schema_sql() -> &'static str {
    meditor_cvss_repository::schema_sql()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_language_from_path() {
        assert_eq!(LanguageFamily::from_path("sample.php"), LanguageFamily::Php);
        assert_eq!(LanguageFamily::from_path("build.sh"), LanguageFamily::Shell);
        assert_eq!(LanguageFamily::from_path("query.sql"), LanguageFamily::Sql);
    }

    #[test]
    fn seed_rule_pack_covers_core_imported_analyzer_languages() {
        let rules = seed_rules();
        for language in [
            LanguageFamily::C,
            LanguageFamily::Java,
            LanguageFamily::JavaScript,
            LanguageFamily::Php,
            LanguageFamily::Python,
            LanguageFamily::Perl,
            LanguageFamily::Shell,
            LanguageFamily::Sql,
        ] {
            assert!(rules.iter().any(|rule| rule.language == language));
        }
    }

    #[test]
    fn scans_open_file_text_and_returns_editor_findings() {
        let scanner = CodeSecurityScanner::with_seed_rules();
        let source = r#"
<?php
eval($_GET["cmd"]);
"#;
        let findings = scanner.scan_text("index.php", source);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "PHP-DYNAMIC-EVAL");
        assert_eq!(findings[0].line_number, 3);
        assert_eq!(findings[0].severity, Severity::Critical);
    }

    #[test]
    fn exposes_cvss_repository_schema() {
        assert!(repository_schema_sql().contains("CREATE TABLE IF NOT EXISTS vulnerability"));
    }
}
