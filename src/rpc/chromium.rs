#[cfg(target_family = "unix")]
pub use unix::{default_preferences_file, is_running, start, stop};
#[cfg(target_family = "windows")]
pub use windows::{default_preferences_file, is_running, start, stop};

pub fn await_shutdown() {
    stop();
    while is_running() {}
}

pub fn restart() -> bool {
    await_shutdown();
    start()
}

#[cfg(target_family = "unix")]
mod unix {
    use crate::constants::CHROMIUM_SERVICE;
    use crate::systemctl;
    use home::home_dir;
    use std::path::PathBuf;
    use subprocess::ExitStatus;

    const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

    /// Returns the path to te default Chromium preferences file.
    ///
    /// # Errors
    ///
    /// If the `$HOME` directory is not set, this will return `None`.
    #[must_use]
    pub fn default_preferences_file() -> Option<PathBuf> {
        home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
    }

    pub fn stop() -> bool {
        systemctl::stop(CHROMIUM_SERVICE)
            .map_or(false, |exit_status| exit_status == ExitStatus::Exited(0))
    }

    pub fn is_running() -> bool {
        systemctl::status(CHROMIUM_SERVICE)
            .map_or(false, |exit_status| exit_status == ExitStatus::Exited(0))
    }

    pub fn start() -> bool {
        systemctl::start(CHROMIUM_SERVICE)
            .map_or(false, |exit_status| exit_status == ExitStatus::Exited(0))
    }
}

#[cfg(target_family = "windows")]
mod windows {
    use std::env::var;
    use std::ffi::OsStr;
    use std::path::PathBuf;
    use sysinfo::{Process, System};

    const CHROME_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default\Preferences";

    pub fn default_preferences_file() -> Option<PathBuf> {
        var("%LOCALAPPDATA%")
            .map(PathBuf::from)
            .map(|home| home.join(CHROME_DEFAULT_PREFERENCES))
            .ok()
    }

    pub fn stop() -> bool {
        for process in System::new().processes_by_name(OsStr::new("Google Chrome")) {
            process.kill();
        }

        true
    }

    pub fn is_running() -> bool {
        !System::new()
            .processes_by_name(OsStr::new("Google Chrome"))
            .collect::<Vec<_>>()
            .is_empty()
    }

    pub fn start() -> bool {
        todo!()
    }
}
