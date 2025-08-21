use crate::constants::{
    CHROMIUM_SERVICE, CONFLICTING_SERVICES, INSTALLATION_INSTRUCTIONS_SERVICE,
    UNCONFIGURED_WARNING_SERVICE,
};
use crate::systemctl::{enable_and_start, is_enabled_or_active, stop_and_disable};
use serde::{Deserialize, Serialize};
use subprocess::ExitStatus;

/// Operation mode of the system.
///
/// This determines what will be shown on the system's display.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationMode {
    /// The Chromium web browser will be started.
    ///
    /// This is used for normal digital signage operations.
    #[serde(rename = "chromium")]
    Chromium,
    /// Installation instructions will be shown on the screen.
    ///
    /// This is the default for systems that have not yet been set to production mode.
    #[serde(rename = "installationInstructions")]
    InstallationInstructions,
    /// A warning message that the system is not configured will be shown.
    ///
    /// This is the default after installing a `HIDSL` image onto the system.
    #[serde(rename = "unconfiguredWarning")]
    UnconfiguredWarning,
    /// The screen is turned black.
    ///
    /// This may be done to temporarily disable digital signage.
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

pub fn activate_exclusive(service: Option<&str>) -> bool {
    for conflicting_service in CONFLICTING_SERVICES {
        let _ = stop_and_disable(conflicting_service);
    }

    service.map_or(true, |service| {
        enable_and_start(service).map_or(false, |exit_status| exit_status == ExitStatus::Exited(0))
    })
}
