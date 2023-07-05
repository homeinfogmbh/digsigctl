mod beep;
mod reboot;

use crate::rpc::beep::beep;
use crate::rpc::reboot::reboot;
use beep_evdev::Melody;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Command {
    #[serde(rename = "beep")]
    Beep(Option<Melody>),
    #[serde(rename = "reboot")]
    Reboot(Option<u64>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success(Option<String>),
    #[serde(rename = "error")]
    Error(String),
}

impl Command {
    #[must_use]
    pub fn run(&self) -> Result {
        match self {
            Self::Beep(melody) => beep(melody.as_ref().cloned()),
            Self::Reboot(delay) => reboot(*delay),
        }
    }
}
