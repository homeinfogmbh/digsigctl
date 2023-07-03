mod cpu_info;

use crate::sysinfo::cpu_info::CpuInfo;
use procfs::cmdline;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemInformation {
    cmdline: Option<Vec<String>>,
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
