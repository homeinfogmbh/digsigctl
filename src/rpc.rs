mod beep;
mod identify;
mod reboot;
mod result;

use crate::config::chromium_default_preferences;
use crate::rpc::beep::beep;
use crate::rpc::identify::identify;
use crate::rpc::reboot::reboot;
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
}

impl Command {
    #[must_use]
    pub fn run(&self) -> Result {
        match self {
            Self::Beep => beep(None),
            Self::Reboot(delay) => reboot(*delay),
            Self::Identify => identify(),
            Self::ConfigFile => Result::Success(Box::new(
                chromium_default_preferences()
                    .and_then(|path| path.to_str().map(ToString::to_string)),
            )),
        }
    }
}
