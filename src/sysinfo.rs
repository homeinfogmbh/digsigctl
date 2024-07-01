mod cmdline;
mod cpuinfo;
mod df;
mod meminfo;

use crate::sysinfo::cpuinfo::CpuInfo;
use crate::sysinfo::meminfo::meminfo;
use cmdline::cmdline;
use serde::Serialize;
use std::collections::HashMap;
use rocket::log::private::error;
use crate::sysinfo::df::{df, Entry};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum Os {
    Unix,
    Windows,
}

#[derive(Debug, Serialize)]
pub struct SystemInformation {
    os: Os,
    cmd_line: Option<HashMap<String, Option<String>>>,
    cpu_info: Option<CpuInfo>,
    mem_info: Option<HashMap<String, usize>>,
    df: Option<Vec<Entry>>,
}

impl SystemInformation {
    #[must_use]
    pub fn gather() -> Self {
        Self {
            #[cfg(target_family = "unix")]
            os: Os::Unix,
            #[cfg(target_family = "windows")]
            os: Os::Windows,
            cmd_line: cmdline().ok(),
            cpu_info: CpuInfo::read().ok(),
            mem_info: meminfo().ok(),
            df: df().map_err(|error| error!("{error}")).ok(),
        }
    }
}
