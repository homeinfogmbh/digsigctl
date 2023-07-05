mod beep;
mod error;
mod identify;
mod reboot;

use crate::rpc::beep::beep;
use crate::rpc::error::Errors;
use crate::rpc::identify::identify;
use crate::rpc::reboot::reboot;
use beep_evdev::Melody;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Deserialize;
use std::fmt::Debug;
use std::io::Cursor;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Command {
    #[serde(rename = "beep")]
    Beep(Option<Melody>),
    #[serde(rename = "reboot")]
    Reboot(Option<u64>),
    #[serde(rename = "identify")]
    Identify,
}

impl Command {
    #[must_use]
    pub fn run(&self) -> Result {
        match self {
            Self::Beep(melody) => beep(melody.as_ref().cloned()),
            Self::Reboot(delay) => reboot(*delay),
            Self::Identify => identify(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Result {
    Success(Option<String>),
    Error(Errors),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Result {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let json;
        let status;

        match self {
            Self::Success(message) => {
                json = rocket::serde::json::to_string(&message)
                    .map_err(|_| Status::InternalServerError)?;
                status = Status::Accepted;
            }
            Self::Error(errors) => {
                json = rocket::serde::json::to_string(errors.errors())
                    .map_err(|_| Status::InternalServerError)?;
                status = errors.status();
            }
        }

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
