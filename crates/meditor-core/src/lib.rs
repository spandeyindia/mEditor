use std::fmt;
use std::str::FromStr;

pub const PRODUCT_NAME: &str = "mEditor";
pub const PRODUCT_EXPANSION: &str = "Multi Editor";
pub const FROZEN_VER_1: &str = "1.5.0.0-frozen";
pub const PRE_PILOT_VER_2: &str = "2.0.0.0-pre-pilot";
pub const VER_2_1_BASELINE: &str = "2.1.0.0";
pub const CURRENT_BASELINE_VERSION: &str = VER_2_1_BASELINE;
pub const VERSION_SCHEMA: &str = "Major release.Minor release.Bugfix or enhancement.Build";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppVersion {
    pub major_release: u32,
    pub minor_release: u32,
    pub bugfix_or_enhancement: u32,
    pub build: u32,
    pub label: Option<String>,
}

impl AppVersion {
    pub fn bump_build(&self) -> Self {
        let mut next = self.clone();
        next.build += 1;
        next.label = None;
        next
    }

    pub fn with_imported_feature_minor_bumps(&self, imported_feature_count: u32) -> Self {
        let mut next = self.clone();
        next.minor_release += imported_feature_count;
        next.build = 0;
        next
    }

    pub fn validate_next_build(&self, next: &Self) -> Result<(), VersionError> {
        let same_release = self.major_release == next.major_release
            && self.minor_release == next.minor_release
            && self.bugfix_or_enhancement == next.bugfix_or_enhancement;
        if same_release && next.build == self.build + 1 {
            Ok(())
        } else {
            Err(VersionError::BuildNotIncremented {
                current: self.to_string(),
                next: next.to_string(),
            })
        }
    }
}

impl fmt::Display for AppVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major_release, self.minor_release, self.bugfix_or_enhancement, self.build
        )?;
        if let Some(label) = &self.label {
            write!(f, "-{label}")?;
        }
        Ok(())
    }
}

impl FromStr for AppVersion {
    type Err = VersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (core, label) = match value.split_once('-') {
            Some((core, label)) => (core, Some(label.to_string())),
            None => (value, None),
        };
        let parts = core.split('.').collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(VersionError::InvalidShape(value.to_string()));
        }
        Ok(Self {
            major_release: parse_component(parts[0], "major release")?,
            minor_release: parse_component(parts[1], "minor release")?,
            bugfix_or_enhancement: parse_component(parts[2], "bugfix or enhancement")?,
            build: parse_component(parts[3], "build")?,
            label,
        })
    }
}

fn parse_component(value: &str, name: &'static str) -> Result<u32, VersionError> {
    value
        .parse::<u32>()
        .map_err(|_| VersionError::InvalidComponent {
            name,
            value: value.to_string(),
        })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionError {
    InvalidShape(String),
    InvalidComponent { name: &'static str, value: String },
    BuildNotIncremented { current: String, next: String },
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionError::InvalidShape(value) => {
                write!(f, "version '{value}' must match {VERSION_SCHEMA}")
            }
            VersionError::InvalidComponent { name, value } => {
                write!(
                    f,
                    "version component '{name}' must be numeric, got '{value}'"
                )
            }
            VersionError::BuildNotIncremented { current, next } => {
                write!(
                    f,
                    "next build '{next}' must increment build from '{current}' by exactly 1"
                )
            }
        }
    }
}

impl std::error::Error for VersionError {}

pub fn imported_ver_1_feature_count() -> u32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_four_part_frozen_version() {
        let version = AppVersion::from_str(FROZEN_VER_1).expect("frozen version should parse");
        assert_eq!(version.major_release, 1);
        assert_eq!(version.minor_release, 5);
        assert_eq!(version.bugfix_or_enhancement, 0);
        assert_eq!(version.build, 0);
        assert_eq!(version.label.as_deref(), Some("frozen"));
    }

    #[test]
    fn parses_ver_2_pre_pilot_baseline_version() {
        let version =
            AppVersion::from_str(PRE_PILOT_VER_2).expect("pre-pilot version should parse");
        assert_eq!(version.major_release, 2);
        assert_eq!(version.minor_release, 0);
        assert_eq!(version.bugfix_or_enhancement, 0);
        assert_eq!(version.build, 0);
        assert_eq!(version.label.as_deref(), Some("pre-pilot"));
    }

    #[test]
    fn parses_ver_2_1_baseline_version() {
        let version = AppVersion::from_str(VER_2_1_BASELINE).expect("Ver 2.1 should parse");
        assert_eq!(version.major_release, 2);
        assert_eq!(version.minor_release, 1);
        assert_eq!(version.bugfix_or_enhancement, 0);
        assert_eq!(version.build, 0);
        assert_eq!(version.label, None);
        assert_eq!(CURRENT_BASELINE_VERSION, VER_2_1_BASELINE);
    }

    #[test]
    fn build_bump_increments_fourth_component_and_clears_label() {
        let version = AppVersion::from_str(FROZEN_VER_1).expect("frozen version should parse");
        assert_eq!(version.bump_build().to_string(), "1.5.0.1");
    }

    #[test]
    fn validates_each_build_bumps_by_one() {
        let current = AppVersion::from_str("1.5.0.7").unwrap();
        let next = AppVersion::from_str("1.5.0.8").unwrap();
        assert!(current.validate_next_build(&next).is_ok());

        let skipped = AppVersion::from_str("1.5.0.9").unwrap();
        assert!(current.validate_next_build(&skipped).is_err());
    }

    #[test]
    fn imported_feature_count_matches_frozen_minor_release() {
        let base = AppVersion {
            major_release: 1,
            minor_release: 0,
            bugfix_or_enhancement: 0,
            build: 0,
            label: None,
        };
        let adjusted = base.with_imported_feature_minor_bumps(imported_ver_1_feature_count());
        assert_eq!(adjusted.to_string(), "1.5.0.0");
    }
}
