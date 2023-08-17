mod error;
mod os;

pub use error::Error;
pub use os::{chromium_default_preferences, reload};
use rocket::serde::json::serde_json::Map;
use rocket::serde::json::{serde_json, Value};
use serde::Deserialize;
use std::fmt::Debug;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;
use subprocess::ExitStatus;

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

        if reload()?.exit_status().unwrap_or(ExitStatus::Exited(1)) != ExitStatus::Exited(0) {
            return Err(Error::SubprocessFailed.into());
        }

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
            session
                .as_object_mut()
                .ok_or(Error::NotAJsonObject("session"))?
                .extend(default_session);
        } else {
            preferences.insert("session".to_string(), Value::Object(default_session));
        }

        save(&filename, &value)
    }
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
