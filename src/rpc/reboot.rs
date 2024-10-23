use crate::rpc::Result;
use std::thread;
use std::time::Duration;

/// Reboots the system.
///
/// If `delay` is `None` the system will be rebooted immediately.
/// Otherwise, the reboot is delayed for the given duration.
pub fn reboot(delay: Option<Duration>) -> Result {
    let _ = thread::spawn(move || {
        if let Some(delay) = delay {
            thread::sleep(delay);
        }

        system_shutdown::reboot().unwrap_or_else(|error| eprintln!("Could not reboot: {error}"));
    });

    Result::Success(Box::new(()))
}
