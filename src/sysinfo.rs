mod cmdline;
mod cpu_info;
mod meminfo;

use crate::sysinfo::cpu_info::CpuInfo;
use crate::sysinfo::meminfo::meminfo;
use cmdline::cmdline;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SystemInformation {
    cmdline: Option<HashMap<String, Option<String>>>,
    cpu_info: Option<CpuInfo>,
    mem_info: Option<HashMap<String, usize>>,
}

impl SystemInformation {
    #[must_use]
    pub fn gather() -> Self {
        Self {
            cmdline: cmdline().ok(),
            cpu_info: CpuInfo::new().ok(),
            mem_info: meminfo().ok(),
        }
    }
}
