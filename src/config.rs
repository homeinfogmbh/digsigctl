use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ConfigurationFailed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigurationFailed => write!(f, "configuration failed"),
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

    pub fn apply(&self) -> Result<(), Error> {
        todo!("Where do we store the URL on the system?")
    }
}
