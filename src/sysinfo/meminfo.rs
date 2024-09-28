use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

const PROC_MEMINFO: &str = "/proc/meminfo";
const KIB: usize = 1024; // Kibibytes.

/// Collects a hash map of entries in `/proc/meminfo`.
///
/// # Errors
///
/// This function will return an [`std::io::Error`] if `/proc/meminfo` could not be read.
pub fn meminfo() -> std::io::Result<HashMap<String, usize>> {
    meminfo_from_file(PROC_MEMINFO)
}

fn meminfo_from_file(filename: impl AsRef<Path>) -> std::io::Result<HashMap<String, usize>> {
    read_to_string(filename.as_ref()).map(meminfo_from_text)
}

fn meminfo_from_text(text: impl AsRef<str>) -> HashMap<String, usize> {
    text.as_ref()
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(key, value)| {
            value
                .trim()
                .split_once(' ')
                .map_or(value.trim().parse::<usize>().ok(), |(value, unit)| {
                    value.trim().parse::<usize>().ok().and_then(|value| {
                        match unit.trim() {
                            "kB" => Some(KIB),
                            _ => None,
                        }
                        .map(|factor| factor * value)
                    })
                })
                .map(|value| (key.trim().to_string(), value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{meminfo_from_text, KIB};

    const MEMINFO_TEXT: &str = r"MemTotal:        8001412 kB
MemFree:          136304 kB
MemAvailable:     109900 kB
Buffers:            1280 kB
Cached:          1176668 kB
SwapCached:            0 kB
Active:          4266976 kB
Inactive:        2303744 kB
Active(anon):    4200280 kB
Inactive(anon):  2247412 kB
Active(file):      66696 kB
Inactive(file):    56332 kB
Unevictable:      951772 kB
Mlocked:             240 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:              7808 kB
Writeback:             0 kB
AnonPages:       6343520 kB
Mapped:           177572 kB
Shmem:           1054964 kB
KReclaimable:      92876 kB
Slab:             193904 kB
SReclaimable:      92876 kB
SUnreclaim:       101028 kB
KernelStack:       15280 kB
PageTables:        43684 kB
SecPageTables:      2056 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     4000704 kB
Committed_AS:   13562732 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       52252 kB
VmallocChunk:          0 kB
Percpu:             2832 kB
HardwareCorrupted:     0 kB
AnonHugePages:   2168832 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
CmaTotal:              0 kB
CmaFree:               0 kB
Unaccepted:            0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      173940 kB
DirectMap2M:     5986304 kB
DirectMap1G:     2097152 kB
";

    #[test]
    fn test_meminfo_from_text() {
        let meminfo = meminfo_from_text(MEMINFO_TEXT);

        // Test some samples of the above /proc/meninfo dump.
        assert_eq!(meminfo.get("MemFree").copied(), Some(136_304 * KIB));
        assert_eq!(meminfo.get("MemAvailable").copied(), Some(109_900 * KIB));
        assert_eq!(meminfo.get("Buffers").copied(), Some(1280 * KIB));
        assert_eq!(meminfo.get("Cached").copied(), Some(1_176_668 * KIB));
        assert_eq!(meminfo.get("SwapCached").copied(), Some(0));
        assert_eq!(meminfo.get("HugePages_Total").copied(), Some(0));
        assert_eq!(meminfo.get("DirectMap1G").copied(), Some(2_097_152 * KIB));
        assert_eq!(meminfo.get("NoSuchKey").copied(), None);
    }
}
