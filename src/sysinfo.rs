use std::collections::HashMap;

use rocket::log::private::error;
use serde::Serialize;

use application::Metadata;
use cmdline::cmdline;
use cpuinfo::CpuInfo;
use df::{df, Entry};
use meminfo::meminfo;
use uptime::Uptime;

mod application;
mod cmdline;
mod cpuinfo;
mod df;
mod meminfo;
mod uptime;

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
    application: Metadata,
    df: Option<Vec<Entry>>,
    uptime: Uptime,
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
            application: application::status(),
            df: df().map_err(|error| error!("{error}")).ok(),
            uptime: Uptime::default(),
        }
    }
}
