use crate::rpc::Result;
use std::thread;
use std::time::Duration;

#[cfg(target_family = "unix")]
pub fn reboot(delay: Option<u64>) -> Result {
    let _ = thread::spawn(move || {
        if let Some(delay) = delay {
            thread::sleep(Duration::from_secs(delay));
        }

        system_shutdown::reboot().unwrap_or_else(|error| eprintln!("Could not reboot: {error}"));
    });

    Result::Success(Box::new(()))
}

#[cfg(target_family = "windows")]
pub fn reboot(delay: Option<u64>) -> Result {
    todo!()
}
