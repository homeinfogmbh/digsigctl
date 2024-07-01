use std::path::PathBuf;

use crate::pacman::package_version;
use crate::systemctl::{is_active, is_enabled};
use serde::Serialize;
use subprocess::ExitStatus;

const SERVICES_DIR: &str = "/usr/lib/systemd/system";
const APPLICATION_PREFERENCE: [Application; 5] = [
    Application::Html,
    Application::Air,
    Application::NotConfiguredWarning,
    Application::InstallationInstructions,
    Application::Off,
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Mode {
    #[serde(rename(serialize = "PRODUCTIVE"))]
    Productive,
    #[serde(rename(serialize = "INSTALLATION_INSTRUCTIONS"))]
    InstallationInstructions,
    #[serde(rename(serialize = "NOT_CONFIGURED"))]
    NotConfigured,
    #[serde(rename(serialize = "OFF"))]
    Off,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Metadata {
    name: &'static str,
    mode: Mode,
    unit: Option<&'static str>,
    package: Option<&'static str>,
}

impl Metadata {
    pub const fn new(
        name: &'static str,
        mode: Mode,
        unit: Option<&'static str>,
        package: Option<&'static str>,
    ) -> Self {
        Self {
            name,
            mode,
            unit,
            package,
        }
    }

    pub fn is_productive(&self) -> bool {
        self.mode == Mode::Productive
    }

    pub fn version(&self) -> Option<String> {
        self.package
            .as_deref()
            .and_then(|package| package_version(package).ok())
    }
}

pub enum Application {
    Html,
    Air,
    NotConfiguredWarning,
    InstallationInstructions,
    Off,
}

impl From<&Application> for Metadata {
    fn from(application: &Application) -> Self {
        match application {
            Application::Html => Metadata::new(
                "html",
                Mode::Productive,
                Some("html5ds.service"),
                Some("application-html"),
            ),
            Application::Air => Metadata::new(
                "air",
                Mode::Productive,
                Some("application.service"),
                Some("application-air"),
            ),
            Application::NotConfiguredWarning => Metadata::new(
                "not configured",
                Mode::NotConfigured,
                Some("unconfigured-warning.service"),
                None,
            ),
            Application::InstallationInstructions => Metadata::new(
                "installation instructions",
                Mode::InstallationInstructions,
                Some("installation-instructions.service"),
                None,
            ),
            Application::Off => Metadata::new("off", Mode::Off, None, None),
        }
    }
}

impl From<Application> for Metadata {
    fn from(application: Application) -> Self {
        (&application).into()
    }
}

impl From<Mode> for Option<Application> {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Productive => get_preferred_application(),
            Mode::InstallationInstructions => Some(Application::InstallationInstructions),
            Mode::NotConfigured => Some(Application::NotConfiguredWarning),
            Mode::Off => Some(Application::Off),
        }
    }
}

pub fn get_preferred_application() -> Option<Application> {
    for application in APPLICATION_PREFERENCE {
        let metadata = Metadata::from(&application);

        if metadata.is_productive() {
            if let Some(unit) = metadata.unit {
                if PathBuf::from(SERVICES_DIR).join(unit).is_file() {
                    return Some(application);
                }
            }
        }
    }

    None
}

pub fn status() -> Metadata {
    for metadata in APPLICATION_PREFERENCE.map(Metadata::from) {
        if let Some(unit) = metadata.unit {
            if is_enabled(unit).unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
                && is_active(unit).unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
            {
                return metadata;
            }
        }
    }

    Application::Off.into()
}
