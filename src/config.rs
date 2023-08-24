mod error;
mod os;

pub use error::Error;
pub use os::{await_webbrowser_shutdown, default_preferences_file, start_webbrowser};
use rocket::serde::json::serde_json::Map;
use rocket::serde::json::{serde_json, Value};
use serde::Deserialize;
use std::fmt::Debug;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::Path;

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
        await_webbrowser_shutdown()?;
        self.update_chromium_preferences()?;

        if start_webbrowser() {
            return Ok(());
        }

        Err(Error::SubprocessFailed.into())
    }

    fn update_chromium_preferences(&self) -> Result<(), Error> {
        let filename = default_preferences_file().ok_or(Error::DefaultPreferencesNotFound)?;
        let mut value = load(&filename)?;
        let preferences = value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))?;
        self.update_or_init_session(preferences);
        Self::update_or_init_profile(preferences);
        Self::update_or_init_sessions(preferences);
        save(&filename, &value)
    }

    fn update_or_init_session(&self, preferences: &mut Map<String, Value>) {
        if let Some(session) = preferences
            .get_mut("session")
            .and_then(Value::as_object_mut)
        {
            session.extend(self.default_session());
        } else {
            preferences.insert("session".to_string(), Value::Object(self.default_session()));
        }
    }

    fn update_or_init_profile(preferences: &mut Map<String, Value>) {
        if let Some(profile) = preferences
            .get_mut("profile")
            .and_then(Value::as_object_mut)
        {
            profile.extend(Self::default_profile());
        } else {
            preferences.insert(
                "profile".to_string(),
                Value::Object(Self::default_profile()),
            );
        }
    }

    fn update_or_init_sessions(preferences: &mut Map<String, Value>) {
        if let Some(sessions) = preferences
            .get_mut("sessions")
            .and_then(Value::as_object_mut)
        {
            sessions.extend(Self::default_sessions());
        } else {
            preferences.insert(
                "sessions".to_string(),
                Value::Object(Self::default_sessions()),
            );
        }
    }

    fn default_session(&self) -> Map<String, Value> {
        Map::from_iter([
            (
                "startup_urls".to_string(),
                Value::Array(vec![Value::String(self.url.clone())]),
            ),
            ("restore_on_startup".to_string(), Value::Number(4.into())),
        ])
    }

    fn default_profile() -> Map<String, Value> {
        Map::from_iter([("exit_type".to_string(), "Normal".into())])
    }

    fn default_sessions() -> Map<String, Value> {
        Map::from_iter([("session_data_status".to_string(), 3.into())])
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
