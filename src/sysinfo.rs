use std::collections::HashMap;

use serde::Serialize;
use sysinfo::Disks;

use crate::sysinfo::mount::root_mounted_ro;
use application::Metadata;
use cmdline::cmdline;
use cpuinfo::CpuInfo;
use df::Entry;
use efi::Efi;
use meminfo::meminfo;
use uptime::Uptime;

mod application;
mod cmdline;
mod cpuinfo;
mod df;
mod efi;
mod meminfo;
mod mount;
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
    application: Metadata,
    baytrail: Option<bool>,
    efi: Efi,
    #[serde(rename = "cmdline")]
    cmd_line: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "cpuinfo")]
    cpu_info: Option<CpuInfo>,
    df: Vec<Entry>,
    #[serde(rename = "meminfo")]
    mem_info: Option<HashMap<String, usize>>,
    root_ro: Option<bool>,
    uptime: Uptime,
    disks: Disks,
}

impl Default for SystemInformation {
    fn default() -> Self {
        Self {
            #[cfg(target_family = "unix")]
            os: Os::Unix,
            #[cfg(target_family = "windows")]
            os: Os::Windows,
            application: application::status(),
            baytrail: CpuInfo::read().map(|cpu_info| cpu_info.is_bay_trail()).ok(),
            efi: Efi::default(),
            cmd_line: cmdline().ok(),
            cpu_info: CpuInfo::read().ok(),
            df: Disks::new_with_refreshed_list()
                .list()
                .iter()
                .map(Entry::from)
                .collect(),
            mem_info: meminfo().ok(),
            root_ro: root_mounted_ro().ok(),
            uptime: Uptime::default(),
            disks: Disks::new_with_refreshed_list(),
        }
    }
}
