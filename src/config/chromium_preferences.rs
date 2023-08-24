use super::error::Error;
use rocket::serde::json::serde_json::Map;
use rocket::serde::json::{serde_json, Value};
use std::convert::Into;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string::ToString;

pub struct ChromiumPreferences {
    filename: PathBuf,
    value: Value,
}

impl ChromiumPreferences {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self {
            value: serde_json::from_str::<Value>(&read_to_string(filename.as_ref())?)?,
            filename: filename.as_ref().into(),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?
            .write_all(serde_json::to_string(&self.value)?.as_bytes())?)
    }

    pub fn update_or_init_session(&mut self, url: &str) -> Result<(), Error> {
        #[allow(clippy::option_if_let_else)]
        if let Some(session) = self
            .preferences()?
            .get_mut("session")
            .and_then(Value::as_object_mut)
        {
            session.extend(default_session(url));
        } else {
            self.preferences()?
                .insert("session".to_string(), Value::Object(default_session(url)));
        }

        Ok(())
    }

    pub fn update_or_init_profile(&mut self) -> Result<(), Error> {
        #[allow(clippy::option_if_let_else)]
        if let Some(profile) = self
            .preferences()?
            .get_mut("profile")
            .and_then(Value::as_object_mut)
        {
            profile.extend(default_profile());
        } else {
            self.preferences()?
                .insert("profile".to_string(), Value::Object(default_profile()));
        }

        Ok(())
    }

    pub fn update_or_init_sessions(&mut self) -> Result<(), Error> {
        #[allow(clippy::option_if_let_else)]
        if let Some(sessions) = self
            .preferences()?
            .get_mut("sessions")
            .and_then(Value::as_object_mut)
        {
            sessions.extend(default_sessions());
        } else {
            self.preferences()?
                .insert("sessions".to_string(), Value::Object(default_sessions()));
        }

        Ok(())
    }

    fn preferences(&mut self) -> Result<&mut Map<String, Value>, Error> {
        self.value
            .as_object_mut()
            .ok_or(Error::NotAJsonObject("preferences"))
    }
}

fn default_session(url: &str) -> Map<String, Value> {
    Map::from_iter([
        (
            "startup_urls".to_string(),
            Value::Array(vec![Value::String(url.to_string())]),
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
