mod beep;
mod reboot;

use crate::rpc::beep::{beep, Args};
use crate::rpc::reboot::reboot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Command {
    #[serde(rename = "beep")]
    Beep(Option<Args>),
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
            Self::Beep(args) => beep(args.as_ref().unwrap_or(&Args::default())),
            Self::Reboot => reboot(),
        }
    }
}
