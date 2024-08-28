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

impl From<&Disk> for Entry {
    fn from(disk: &Disk) -> Self {
        let used = disk.total_space() - disk.available_space();
        Self {
            filesystem: String::from_utf8_lossy(disk.file_system().as_encoded_bytes()).to_string(),
            used,
            available: disk.available_space(),
            use_pct: (used * 100)
                .div_euclid(disk.total_space())
                .try_into()
                .expect("percentage too high"),
            mountpoint: disk.mount_point().into(),
        }
    }
}
