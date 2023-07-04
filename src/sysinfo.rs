mod cmdline;
mod cpu_info;

use crate::sysinfo::cpu_info::CpuInfo;
use cmdline::cmdline;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SystemInformation {
    cmdline: Option<HashMap<String, Option<String>>>,
    cpu_info: Option<CpuInfo>,
}

impl SystemInformation {
    #[must_use]
    pub fn gather() -> Self {
        Self {
            cmdline: cmdline().ok(),
            cpu_info: CpuInfo::new().ok(),
        }
    }
}
