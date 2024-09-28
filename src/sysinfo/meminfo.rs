use std::collections::HashMap;
use std::fs::read_to_string;

const PROC_MEMINFO: &str = "/proc/meminfo";

/// Collects a hash map of entries in `/proc/meminfo`.
///
/// # Errors
///
/// This function will return an [`std::io::Error`] if `/proc/meminfo` could not be read.
pub fn meminfo() -> std::io::Result<HashMap<String, usize>> {
    read_to_string(PROC_MEMINFO).map(|text| {
        text.lines()
            .filter_map(|line| line.split_once(':'))
            .filter_map(|(key, value)| {
                value
                    .trim()
                    .split_once(' ')
                    .map_or(value.trim().parse::<usize>().ok(), |(value, unit)| {
                        value.trim().parse::<usize>().ok().and_then(|value| {
                            match unit.trim() {
                                "kB" => Some(1024),
                                _ => None,
                            }
                            .map(|factor| factor * value)
                        })
                    })
                    .map(|value| (key.trim().to_string(), value))
            })
            .collect()
    })
}
