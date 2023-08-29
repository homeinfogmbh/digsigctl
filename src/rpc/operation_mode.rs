use crate::constants::{
    CHROMIUM_SERVICE, CONFLICTING_SERVICES, INSTALLATION_INSTRUCTIONS_SERVICE,
    UNCONFIGURED_WARNING_SERVICE,
};
use crate::systemctl::{enable_and_start, is_enabled_or_active, stop_and_disable};
use serde::{Deserialize, Serialize};

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
