mod reboot;

use crate::rpc::reboot::reboot;
use beep_evdev::Melody;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Command {
    #[serde(rename = "beep")]
    Beep(Option<Melody>),
    #[serde(rename = "reboot")]
    Reboot,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CommandResult {
    #[serde(rename = "success")]
    Success(Option<String>),
    #[serde(rename = "error")]
    Error(Option<String>, Option<u16>),
}

impl Command {
    #[must_use]
    pub fn run(&self) -> CommandResult {
        match self {
            Self::Beep(melody) => melody.clone().unwrap_or_default().play().map_or_else(
                |error| CommandResult::Error(Some(error.to_string()), None),
                |_| CommandResult::Success(None),
            ),
            Self::Reboot => reboot(),
        }
    }
}
