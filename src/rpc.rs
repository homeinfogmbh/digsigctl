mod beep;
pub mod chromium;
mod identify;
mod operation_mode;
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Command {
    #[serde(rename = "beep")]
    Beep,
    #[serde(rename = "reboot")]
    Reboot(Option<u64>),
    #[serde(rename = "identify")]
    Identify,
    #[serde(rename = "configFile")]
    ConfigFile,
    #[serde(rename = "restartWebBrowser")]
    RestartWebBrowser,
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
            Self::Reboot(delay) => reboot(*delay),
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
