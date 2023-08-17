#[cfg(target_family = "unix")]
use unix::{chromium_is_running, stop_chromium};
#[cfg(target_family = "windows")]
use windows::{chromium_is_running, stop_chromium};

pub fn await_chromium_shutdown() -> subprocess::Result<()> {
    stop_chromium()?;
    while chromium_is_running()? {}
    Ok(())
}

#[cfg(target_family = "unix")]
pub mod unix {
    use home::home_dir;
    use std::path::PathBuf;
    use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

    const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

    pub fn chromium_default_preferences() -> Option<PathBuf> {
        home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
    }
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
    pub fn chromium_is_running() -> subprocess::Result<bool> {
        Popen::create(
            &["systemctl", "status", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        )
        .map(|popen| {
            popen.exit_status().unwrap_or(ExitStatus::Exited(255)) == ExitStatus::Exited(0)
        })
    }

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
}

#[cfg(target_family = "windows")]
pub mod windows {
    use std::env::var;
    use std::path::PathBuf;
    use subprocess::Popen;

    const CHROMIUM_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default\Preferences";

    pub fn chromium_default_preferences() -> Option<PathBuf> {
        var("%LOCALAPPDATA%")
            .map(PathBuf::from)
            .map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
            .ok()
    }

    pub fn stop_chromium() -> subprocess::Result<Popen> {
        todo!()
    }

    pub fn chromium_is_running() -> subprocess::Result<bool> {
        todo!()
    }

    pub fn start_chromium() -> subprocess::Result<Popen> {
        todo!()
    }
}
