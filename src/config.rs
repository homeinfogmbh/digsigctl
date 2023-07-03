use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigError {
    ConfigurationFailed,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigurationFailed => write!(f, "configuration failed"),
        }
    }
}

impl Error for ConfigError {}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    url: String,
}

impl Config {
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn apply(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}
