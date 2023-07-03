use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SystemInformation {
    cmdline: Option<Vec<String>>,
}

impl SystemInformation {
    pub fn gather() -> Self {
        Self {
            cmdline: procfs::cmdline().ok(),
        }
    }
}
