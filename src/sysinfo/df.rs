use std::num::TryFromIntError;
use std::path::PathBuf;

use serde::Serialize;
use sysinfo::Disk;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Entry {
    filesystem: String,
    used: u64,
    available: u64,
    use_pct: u8,
    mountpoint: PathBuf,
}

impl TryFrom<&Disk> for Entry {
    type Error = TryFromIntError;

    fn try_from(disk: &Disk) -> Result<Self, Self::Error> {
        let used = disk.total_space() - disk.available_space();
        (used * 100)
            .div_euclid(disk.total_space())
            .try_into()
            .map(|use_pct| Self {
                filesystem: String::from_utf8_lossy(disk.file_system().as_encoded_bytes())
                    .to_string(),
                used,
                available: disk.available_space(),
                use_pct,
                mountpoint: disk.mount_point().into(),
            })
    }
}
