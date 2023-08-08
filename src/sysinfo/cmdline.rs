use std::collections::HashMap;
use std::fs::read_to_string;

const PROC_CMDLINE: &str = "/proc/cmdline";

#[cfg(target_family = "unix")]
pub fn cmdline() -> Result<HashMap<String, Option<String>>, std::io::Error> {
    Ok(read_to_string(PROC_CMDLINE)?
        .split_whitespace()
        .map(|item| {
            item.split_once('=')
                .map_or((item.to_string(), None), |(key, value)| {
                    (key.to_string(), Some(value.to_string()))
                })
        })
        .collect())
}
