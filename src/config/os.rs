#[cfg(target_family = "unix")]
pub use unix::{default_preferences_file, start_webbrowser};
#[cfg(target_family = "unix")]
use unix::{stop_webbrowser, webbrowser_is_running};
#[cfg(target_family = "windows")]
pub use windows::{default_preferences_file, start_webbrowser};
#[cfg(target_family = "windows")]
use windows::{stop_webbrowser, webbrowser_is_running};

pub fn await_webbrowser_shutdown() -> anyhow::Result<()> {
    stop_webbrowser()?;
    while webbrowser_is_running()? {}
    Ok(())
}

#[cfg(target_family = "unix")]
mod unix {
    use home::home_dir;
    use std::path::PathBuf;
    use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

    const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

    pub fn default_preferences_file() -> Option<PathBuf> {
        home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
    }

    pub fn stop_webbrowser() -> subprocess::Result<Popen> {
        Popen::create(
            &["systemctl", "stop", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        )
    }

    pub fn webbrowser_is_running() -> subprocess::Result<bool> {
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

    pub fn start_webbrowser() -> subprocess::Result<ExitStatus> {
        if let Ok(popen) = Popen::create(
            &["systemctl", "start", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        ) {
            popen.exit_status().unwrap_or(ExitStatus::Exited(255)) == ExitStatus::Exited(0)
        } else {
            false
        }
    }
}

#[cfg(target_family = "windows")]
mod windows {
    use std::env::var;
    use std::path::PathBuf;
    use sysinfo::{ProcessExt, SystemExt};

    const CHROME_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default\Preferences";

    pub fn default_preferences_file() -> Option<PathBuf> {
        var("%LOCALAPPDATA%")
            .map(PathBuf::from)
            .map(|home| home.join(CHROME_DEFAULT_PREFERENCES))
            .ok()
    }

    pub fn stop_webbrowser() -> anyhow::Result<()> {
        for process in sysinfo::System::new().processes_by_name("Google Chrome") {
            process.kill();
        }

        Ok(())
    }

    pub fn webbrowser_is_running() -> anyhow::Result<bool> {
        Ok(!sysinfo::System::new()
            .processes_by_name("Google Chrome")
            .collect::<Vec<_>>()
            .is_empty())
    }

    pub fn start_webbrowser() -> bool {
        true
    }
}
