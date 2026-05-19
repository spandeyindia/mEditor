#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuArchitecture {
    X64,
    Arm64,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlatformProfile {
    pub os: OperatingSystem,
    pub arch: CpuArchitecture,
}

impl PlatformProfile {
    pub fn current() -> Self {
        Self {
            os: current_os(),
            arch: current_arch(),
        }
    }

    pub fn is_ver_1_laptop_target(&self) -> bool {
        matches!(
            (self.os, self.arch),
            (OperatingSystem::Windows, CpuArchitecture::X64)
                | (OperatingSystem::Windows, CpuArchitecture::Arm64)
                | (OperatingSystem::MacOS, CpuArchitecture::X64)
                | (OperatingSystem::MacOS, CpuArchitecture::Arm64)
                | (OperatingSystem::Linux, CpuArchitecture::X64)
                | (OperatingSystem::Linux, CpuArchitecture::Arm64)
        )
    }

    pub fn default_shells(&self) -> &'static [&'static str] {
        match self.os {
            OperatingSystem::Windows => &["pwsh", "powershell", "cmd", "git-bash", "wsl"],
            OperatingSystem::MacOS => &["zsh", "bash", "fish"],
            OperatingSystem::Linux => &["bash", "zsh", "fish", "sh"],
            OperatingSystem::Other => &["sh"],
        }
    }
}

fn current_os() -> OperatingSystem {
    if cfg!(target_os = "windows") {
        OperatingSystem::Windows
    } else if cfg!(target_os = "macos") {
        OperatingSystem::MacOS
    } else if cfg!(target_os = "linux") {
        OperatingSystem::Linux
    } else {
        OperatingSystem::Other
    }
}

fn current_arch() -> CpuArchitecture {
    if cfg!(target_arch = "x86_64") {
        CpuArchitecture::X64
    } else if cfg!(target_arch = "aarch64") {
        CpuArchitecture::Arm64
    } else {
        CpuArchitecture::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_platform_has_shell_defaults() {
        assert!(!PlatformProfile::current().default_shells().is_empty());
    }

    #[test]
    fn mac_arm64_covers_apple_silicon_m_series() {
        let platform = PlatformProfile {
            os: OperatingSystem::MacOS,
            arch: CpuArchitecture::Arm64,
        };
        assert!(platform.is_ver_1_laptop_target());
    }
}
