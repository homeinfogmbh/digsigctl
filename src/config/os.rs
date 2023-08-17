#[cfg(target_family = "unix")]
use home::home_dir;
#[cfg(target_family = "windows")]
use std::env::var;
use std::path::PathBuf;
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

#[cfg(target_family = "unix")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[cfg(target_family = "windows")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default\Preferences";

#[cfg(target_family = "unix")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
}

#[cfg(target_family = "windows")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    var("%LOCALAPPDATA%")
        .map(PathBuf::from)
        .map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
        .ok()
}

pub fn await_chromium_shutdown() -> subprocess::Result<()> {
    stop_chromium()?;
    while chromium_is_running()? {}
    Ok(())
}

#[cfg(target_family = "unix")]
pub fn stop_chromium() -> subprocess::Result<Popen> {
    Popen::create(
        &["systemctl", "stop", "chromium.service"],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
}

#[cfg(target_family = "windows")]
pub fn stop_chromium() -> subprocess::Result<Popen> {
    todo!()
}

#[cfg(target_family = "unix")]
pub fn chromium_is_running() -> subprocess::Result<bool> {
    Popen::create(
        &["systemctl", "status", "chromium.service"],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map(|popen| popen.exit_status().unwrap_or(ExitStatus::Exited(255)) == ExitStatus::Exited(0))
}

#[cfg(target_family = "windows")]
pub fn chromium_is_running() -> subprocess::Result<bool> {
    todo!()
}

#[cfg(target_family = "unix")]
pub fn start_chromium() -> subprocess::Result<Popen> {
    Popen::create(
        &["systemctl", "start", "chromium.service"],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
}

#[cfg(target_family = "windows")]
pub fn start_chromium() -> subprocess::Result<Popen> {
    todo!()
}
