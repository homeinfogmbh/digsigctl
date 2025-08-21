mod beep;
pub mod chromium;
mod identify;
pub(crate) mod operation_mode;
mod reboot;
mod result;

use beep::beep;
pub use chromium::default_preferences_file;
use identify::identify;
use operation_mode::OperationMode;
use reboot::reboot;
pub use result::Result;
use serde::Deserialize;
use std::fmt::Debug;
use std::time::Duration;

/// Available RPC commands.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Command {
    /// Beep the PC speaker of the system.
    ///
    /// This is used to identify the system on-site for technicians.
    #[serde(rename = "beep")]
    Beep,
    /// Reboot the system.
    ///
    /// The command takes an optional delay in seconds.
    /// The reboot will be deferred for the given amount of seconds, if provided
    /// or will be executed immediately if `None` is passed.
    #[serde(rename = "reboot")]
    Reboot(Option<u64>),
    /// Identify the system.
    ///
    /// This is used to identify the system on-site for technicians.
    /// In addition to beeping the system (see [`Command::Beep`]) this will also display a
    /// message on the system's screen presenting its hostname, which is also its ID.
    #[serde(rename = "identify")]
    Identify,
    /// This will return the path to the default preferences file in use.
    #[serde(rename = "configFile")]
    ConfigFile,
    /// This will restart the web browser, i.e. Chromium, by
    /// restarting the appropriate systemd service.
    ///
    /// This is used to restart the digital signage presentation on the system,
    /// without rebooting the entire system.
    #[serde(rename = "restartWebBrowser")]
    RestartWebBrowser,
    /// Get or set the operation mode of the system.
    ///
    /// If this is `None` it will query and return information
    /// about the current operation mode of the system.
    ///
    /// If this is `Some(OperationMode)` it will set the system to the provided operation mode.
    /// See [`OperationMode`] for further details.
    #[serde(rename = "operationMode")]
    OperationMode(Option<OperationMode>),
}

impl Command {
    /// Runs the RPC command.
    ///
    /// This will return a [`Result`], that will either represent success
    /// or a list of errors that occurred while executing the RPC command.
    #[must_use]
    pub fn run(&self) -> Result {
        match self {
            Self::Beep => beep(None),
            Self::Reboot(delay) => reboot(delay.map(Duration::from_secs)),
            Self::Identify => identify(),
            Self::ConfigFile => Result::Success(Box::new(
                default_preferences_file().and_then(|path| path.to_str().map(ToString::to_string)),
            )),
            Self::RestartWebBrowser => {
                if chromium::restart() {
                    Result::Success(Box::new("Web browser restarted.".to_string()))
                } else {
                    Result::Error("Could not restart web browser.".into())
                }
            }
            Self::OperationMode(operation_mode) => operation_mode.as_ref().map_or_else(
                || Result::Success(Box::new(OperationMode::get())),
                |operation_mode| {
                    if operation_mode.set() {
                        Result::Success(Box::new("Operation mode set"))
                    } else {
                        Result::Error("Could not set operation mode.".into())
                    }
                },
            ),
        }
    }
}
