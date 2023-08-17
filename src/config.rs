#[cfg(target_family = "unix")]
use home::home_dir;
use rocket::serde::json::serde_json::Map;
use rocket::serde::json::{serde_json, Value};
use serde::Deserialize;
#[cfg(target_family = "windows")]
use std::env::var;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use subprocess::{Popen, PopenConfig, Redirection};

#[cfg(target_family = "unix")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[cfg(target_family = "windows")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default";

#[cfg(target_family = "unix")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
}

#[cfg(target_family = "windows")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    var("%LOCALAPPDATA%")
        .map(PathBuf::from)
        .map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
        .ok()
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    ChromiumDefaultPreferencesNotFound,
    NotAJsonObject(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(error) => <serde_json::Error as Display>::fmt(error, f),
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::ChromiumDefaultPreferencesNotFound => {
                write!(f, "Chrome / Chromium default preferences not found")
            }
            Self::NotAJsonObject(key) => write!(f, "not a JSON object: {key}"),
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
        let filename =
            chromium_default_preferences().ok_or(Error::ChromiumDefaultPreferencesNotFound)?;
        let mut value = load(&filename)?;
        let preferences = value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))?;

        let default_session = Map::from_iter([
            (
                "startup_urls".to_string(),
                Value::Array(vec![Value::String(self.url.clone())]),
            ),
            ("restore_on_startup".to_string(), Value::Number(4.into())),
        ]);

        if let Some(session) = preferences.get_mut("session") {
            let session = session
                .as_object_mut()
                .ok_or(Error::NotAJsonObject("session"))?;
            session.extend(default_session);
        } else {
            preferences.insert("session".to_string(), Value::Object(default_session));
        }

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

fn load(filename: impl AsRef<Path>) -> Result<Value, Error> {
    Ok(serde_json::from_str::<Value>(&read_to_string(filename)?)?)
}

fn save(filename: impl AsRef<Path>, value: &Value) -> Result<(), Error> {
    Ok(OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?
        .write_all(serde_json::to_string(value)?.as_bytes())?)
}
