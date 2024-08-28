use std::path::PathBuf;

use serde::Serialize;
use sysinfo::Disks;

const EFI_PARTITION: &str = "/dev/disk/by-label/EFI";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Efi {
    mounted: bool,
}

impl From<Disks> for Efi {
    fn from(disks: Disks) -> Self {
        Self {
            mounted: disks
                .list()
                .iter()
                .any(|disk| disk.mount_point().as_os_str() == "/boot")
                && PathBuf::from(EFI_PARTITION).exists(),
        }
    }
}

impl Default for Efi {
    fn default() -> Self {
        Self::from(Disks::new_with_refreshed_list())
    }
}
