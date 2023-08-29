use serde::{Deserialize, Serialize};
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

const CHROMIUM_SERVICE: &str = "chromium.service";
const INSTALLATION_INSTRUCTIONS_SERVICE: &str = "installation-instructions.service";
const UNCONFIGURED_WARNING_SERVICE: &str = "unconfigured-warning.service";
const CONFLICTING_SERVICES: [&str; 3] = [
    CHROMIUM_SERVICE,
    INSTALLATION_INSTRUCTIONS_SERVICE,
    UNCONFIGURED_WARNING_SERVICE,
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationMode {
    #[serde(rename = "chromium")]
    Chromium,
    #[serde(rename = "installationInstructions")]
    InstallationInstructions,
    #[serde(rename = "unconfiguredWarning")]
    UnconfiguredWarning,
    #[serde(rename = "blackScreen")]
    BlackScreen,
}

impl OperationMode {
    pub fn get() -> Self {
        if is_enabled_or_active(CHROMIUM_SERVICE) {
            Self::Chromium
        } else if is_enabled_or_active(INSTALLATION_INSTRUCTIONS_SERVICE) {
            Self::InstallationInstructions
        } else if is_enabled_or_active(UNCONFIGURED_WARNING_SERVICE) {
            Self::UnconfiguredWarning
        } else {
            Self::BlackScreen
        }
    }

    pub fn set(&self) -> bool {
        match self {
            Self::Chromium => activate_exclusive(Some(CHROMIUM_SERVICE)),
            Self::InstallationInstructions => {
                activate_exclusive(Some(INSTALLATION_INSTRUCTIONS_SERVICE))
            }
            Self::UnconfiguredWarning => activate_exclusive(Some(UNCONFIGURED_WARNING_SERVICE)),
            Self::BlackScreen => activate_exclusive(None),
        }
    }
}

fn activate_exclusive(service: Option<&str>) -> bool {
    for conflicting_service in CONFLICTING_SERVICES {
        stop_and_disable(conflicting_service);
    }

    service.map_or(true, enable_and_start)
}

fn stop_and_disable(service: &str) -> bool {
    Popen::create(
        &["systemctl", "disable", "--now", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

fn enable_and_start(service: &str) -> bool {
    Popen::create(
        &["systemctl", "enable", "--now", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

fn is_enabled_or_active(service: &str) -> bool {
    is_enabled(service) | is_active(service)
}

fn is_enabled(service: &str) -> bool {
    Popen::create(
        &["systemctl", "is-enabled", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

fn is_active(service: &str) -> bool {
    Popen::create(
        &["systemctl", "is-active", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}
