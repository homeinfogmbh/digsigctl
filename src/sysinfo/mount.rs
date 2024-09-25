use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::anyhow;
use serde::Serialize;

const MOUNTS: &str = "/proc/mounts";

/// Information about a mounted file system.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Mount {
    #[serde(rename = "what")]
    device: String,
    #[serde(rename = "where")]
    mountpoint: PathBuf,
    #[serde(rename = "type")]
    filesystem: String,
    flags: HashMap<String, Option<String>>,
    freq: u32,
    #[serde(rename = "passno")]
    pass_no: u32,
}

impl TryFrom<[&str; 6]> for Mount {
    type Error = ParseIntError;

    fn try_from(
        [device, mountpoint, filesystem, flags, freq, pass_no]: [&str; 6],
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            device: device.into(),
            mountpoint: mountpoint.into(),
            filesystem: filesystem.into(),
            flags: flags
                .split(',')
                .filter_map(|flag| {
                    let mut items = flag.splitn(2, '=');
                    items
                        .next()
                        .map(|key| (key.to_string(), items.next().map(String::from)))
                })
                .collect(),
            freq: freq.parse()?,
            pass_no: pass_no.parse()?,
        })
    }
}

impl FromStr for Mount {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: [&str; 6] = s
            .split_whitespace()
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| anyhow!("could not convert vec to array"))?;
        Ok(Self::try_from(fields)?)
    }
}

/// Collect mounts of local file systems.
///
/// # Error
///
/// This function will return an [`std::io::Error`] if `/proc/mounts` could not be read.
pub fn mounts() -> std::io::Result<Vec<Mount>> {
    read_to_string(MOUNTS).map(|mounts| {
        mounts
            .lines()
            .filter_map(|line| Mount::from_str(line).ok())
            .collect()
    })
}

/// Determines whether the root partition (`/`) is mounted read-only.
///
/// # Error
///
/// This function will return an [`std::io::Error`] if `/proc/mounts` could not be read.
pub fn root_mounted_ro() -> std::io::Result<bool> {
    mounts().and_then(|mounts| {
        mounts
            .iter()
            .find(|mount| mount.mountpoint == PathBuf::from("/"))
            .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "root partition not found"))
            .map(|mount| mount.flags.contains_key("ro"))
    })
}
