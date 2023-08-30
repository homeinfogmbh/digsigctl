mod chromium_preferences;
mod error;

pub use crate::config::chromium_preferences::ChromiumPreferences;
use crate::rpc::chromium;
pub use error::Error;
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
        chromium::await_shutdown();
        self.update_chromium_preferences()?;

        if chromium::start() {
            return Ok(());
        }

        Err(Error::SubprocessFailed.into())
    }

    fn update_chromium_preferences(&self) -> Result<(), Error> {
        let filename =
            chromium::default_preferences_file().ok_or(Error::DefaultPreferencesNotFound)?;
        let mut preferences = ChromiumPreferences::load(&filename)?;
        preferences.update_or_init_session(self.url.as_str())?;
        preferences.update_or_init_profile()?;
        preferences.update_or_init_sessions()?;
        preferences.save(filename)
    }
}
