use home::home_dir;
use rocket::serde::json::{serde_json, Value};
use serde::Deserialize;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use subprocess::{Popen, PopenConfig, Redirection};

const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    HomeNotFound,
    NotAJsonObject(&'static str),
    KeyNotFound(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(error) => <serde_json::Error as Display>::fmt(error, f),
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::HomeNotFound => write!(f, "home directory not found"),
            Self::NotAJsonObject(key) => write!(f, "not a JSON object: {key}"),
            Self::KeyNotFound(key) => write!(f, "JSON key not found: {key}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SerdeError(error) => Some(error),
            Self::IoError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
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
    pub fn apply(&self) -> Result<(), anyhow::Error> {
        self.update()?;
        reload()?;
        Ok(())
    }

    fn update(&self) -> Result<(), Error> {
        let filename = filename().ok_or(Error::HomeNotFound)?;
        let mut value = load(&filename)?;
        value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))?
            .get_mut("session")
            .ok_or(Error::KeyNotFound("session"))?
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("session"))?
            .insert("startup_urls".to_string(), vec![self.url.clone()].into());
        save(&filename, &value)
    }
}

fn reload() -> subprocess::Result<Popen> {
    Popen::create(
        &["systemctl", "restart", "chromium.service"],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
}

pub fn filename() -> Option<PathBuf> {
    home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
}

fn load(filename: impl AsRef<Path>) -> Result<Value, Error> {
    Ok(serde_json::from_str::<Value>(&read_to_string(filename)?)?)
}

fn save(filename: impl AsRef<Path>, value: &Value) -> Result<(), Error> {
    Ok(OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?
        .write_all(serde_json::to_string(value)?.as_bytes())?)
}
