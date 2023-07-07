use home::home_dir;
use rocket::serde::json::{serde_json, Value};
use serde::{Deserialize, Serialize};
use std::env::join_paths;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

const CHROMIUM_DEFAULTS_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    PathError(std::env::JoinPathsError),
    HomeNotFound,
    NotAJsonObject(&'static str),
    SessionNotFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(error) => <serde_json::Error as Display>::fmt(error, f),
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::PathError(error) => <std::env::JoinPathsError as Display>::fmt(error, f),
            Self::HomeNotFound => write!(f, "home directory not found"),
            Self::NotAJsonObject(key) => write!(f, "not a JSON object: {key}"),
            Self::SessionNotFound => write!(f, "session object not found"),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    url: String,
}

impl Config {
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Applies the configuration to the system
    /// # Errors
    /// Returns an [`digsigctl::config::Error`] if the configuration could not be applied
    pub fn apply(&self) -> Result<(), Error> {
        let filename = join_paths([
            home_dir().ok_or(Error::HomeNotFound)?,
            CHROMIUM_DEFAULTS_PREFERENCES.into(),
        ])
        .map_err(Error::PathError)?;
        let mut value = load(&filename)?;
        value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))?
            .get_mut("session")
            .ok_or(Error::SessionNotFound)?
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("session"))
            .map(|session| {
                match session
                    .get_mut("startup_urls")
                    .and_then(|startup_urls| startup_urls.as_array_mut())
                {
                    Some(startup_urls) => {
                        startup_urls.clear();
                        startup_urls.push(self.url.clone().into());
                    }
                    None => {
                        session.insert("startup_urls".to_string(), vec![self.url.clone()].into());
                    }
                }
            })?;

        save(&filename, &value)?;
        Ok(())
    }
}

fn load(filename: impl AsRef<Path>) -> Result<Value, Error> {
    serde_json::from_str::<Value>(&read_to_string(filename).map_err(Error::IoError)?)
        .map_err(Error::SerdeError)
}

fn save(filename: impl AsRef<Path>, value: &Value) -> Result<(), Error> {
    File::create(filename)
        .map_err(Error::IoError)?
        .write_all(
            serde_json::to_string(value)
                .map_err(Error::SerdeError)?
                .as_bytes(),
        )
        .map_err(Error::IoError)
}
