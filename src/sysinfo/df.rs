use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

use anyhow::anyhow;
use serde::Serialize;

use crate::from_io::TryFromIo;

const DF: &str = "/usr/bin/df";
const POSIX_FORMAT: &str = "-P";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    filesystem: String,
    blocks: u64,
    used: u64,
    available: u64,
    use_pct: u8,
    mountpoint: PathBuf,
}

impl TryFrom<[&str; 6]> for Entry {
    type Error = anyhow::Error;

    fn try_from(
        [filesystem, blocks, used, available, use_pct, mountpoint]: [&str; 6],
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            filesystem: filesystem.to_string(),
            blocks: blocks.parse()?,
            used: used.parse()?,
            available: available.parse()?,
            use_pct: use_pct.replace('%', "").parse()?,
            mountpoint: mountpoint.into(),
        })
    }
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let fields: [&str; 6] = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| anyhow!("could not convert vec to array"))?;
        Self::try_from(fields)
    }
}

pub fn df() -> std::io::Result<Vec<Entry>> {
    String::try_from_io(
        Command::new(DF)
            .arg(POSIX_FORMAT)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
            .stdout,
    )
    .map(|text| {
        text.lines()
            .skip(1)
            .filter_map(|line| Entry::from_str(line).ok())
            .collect()
    })
}
