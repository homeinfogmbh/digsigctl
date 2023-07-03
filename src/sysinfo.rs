use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SystemInformation {
    cmdline: Option<Vec<String>>,
}

impl SystemInformation {
    #[must_use]
    pub fn gather() -> Self {
        Self {
            cmdline: procfs::cmdline().ok(),
        }
    }
}
