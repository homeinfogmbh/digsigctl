use crate::sudo::sudo;
use crate::try_from_io::TryFromIo;
use rocket::log::private::error;
use std::collections::HashMap;
use std::process::{Child, Stdio};

const SMARTCTL: &str = "/usr/bin/smartctl";
const SMART_STATUS_PREFIX: &str = "SMART overall-health self-assessment test result:";

/// Returns a child process running `smartctl`.
///
/// # Errors
///
/// This functions will return an [`std::io::Error`] if it fails to spawn the subcommand.
pub fn smartctl(args: &[&str]) -> std::io::Result<Child> {
    let mut command = sudo(SMARTCTL);
    command.args(args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.spawn()
}

/// Returns an iterator of scanned device names.
///
/// # Errors
///
/// This functions will return an [`std::io::Error`] if it fails to spawn the subcommand.
pub fn get_devices() -> std::io::Result<impl Iterator<Item = String>> {
    smartctl(&["--scan-open"])
        .and_then(Child::wait_with_output)
        .and_then(|output| String::try_from_io(output.stdout))
        .map(|text| {
            text.lines()
                .map(String::from)
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() {
                        None
                    } else {
                        Some(line.to_string())
                    }
                })
                .filter_map(|line| line.split_whitespace().next().map(String::from))
        })
}

/// Returns the S.M.A.R.T. status of the given device as a string.
///
/// # Errors
///
/// This functions will return an [`std::io::Error`] if it fails to spawn the subcommand.
pub fn check_device(device: &str) -> std::io::Result<Option<String>> {
    smartctl(&["-H", device])
        .and_then(Child::wait_with_output)
        .and_then(|output| String::try_from_io(output.stdout))
        .map(check_device_from_text)
}

fn check_device_from_text(text: impl AsRef<str>) -> Option<String> {
    text.as_ref()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
        .into_iter()
        .find_map(|line| {
            if line.trim().starts_with(SMART_STATUS_PREFIX) {
                Some(line)
            } else {
                None
            }
            .and_then(|line| line.split(':').nth(1).map(String::from))
        })
}

/// Returns a hash map of the S.M.A.R.T. devices and their states as string.
///
/// # Errors
///
/// This functions will return an [`std::io::Error`] if it fails to spawn the subcommand.
pub fn device_states() -> std::io::Result<HashMap<String, Option<String>>> {
    get_devices().map(|devices| {
        devices
            .filter_map(|device| {
                check_device(&device)
                    .inspect_err(|error| error!("{error}"))
                    .ok()
                    .map(|status| (device, status))
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::check_device_from_text;

    const SMARTCTL_H: &str =
        "smartctl 7.4 2023-08-01 r5530 [x86_64-linux-6.10.10-arch1-1] (local build)
Copyright (C) 2002-23, Bruce Allen, Christian Franke, www.smartmontools.org

=== START OF READ SMART DATA SECTION ===
SMART overall-health self-assessment test result: PASSED

";

    #[test]
    fn test_check_device() {
        let status = check_device_from_text(SMARTCTL_H);

        assert_eq!(status.as_deref(), Some(" PASSED"));
    }
}
