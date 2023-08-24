mod chromium_preferences;
mod error;
mod os;

use crate::config::chromium_preferences::ChromiumPreferences;
pub use error::Error;
pub use os::{await_webbrowser_shutdown, default_preferences_file, start_webbrowser};
use serde::Deserialize;
use std::fmt::Debug;

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
        let mut preferences = ChromiumPreferences::load(&filename)?;
        preferences.update_or_init_session(self.url.as_str())?;
        preferences.update_or_init_profile()?;
        preferences.update_or_init_sessions()?;
        preferences.save(filename)
    }
}
