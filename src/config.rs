use configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

const HTML5DS_CONFIG: &str = "/etc/html5ds.conf";

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ConfigurationFailed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigurationFailed(_) => write!(f, "configuration failed"),
        }
    }
}

impl std::error::Error for Error {}

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
        let mut html5ds_config = Ini::new();
        html5ds_config
            .load(HTML5DS_CONFIG)
            .map_err(Error::ConfigurationFailed)?;
        html5ds_config.set("application", "url", Some(self.url.clone()));
        html5ds_config
            .write(HTML5DS_CONFIG)
            .map_err(|error| Error::ConfigurationFailed(error.to_string()))?;
        Ok(())
    }
}
