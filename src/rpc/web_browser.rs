use subprocess::ExitStatus;
#[cfg(target_family = "unix")]
pub use unix::{default_preferences_file, is_running, start, stop};
#[cfg(target_family = "windows")]
pub use windows::{default_preferences_file, is_running, start, stop};

pub fn await_shutdown() -> anyhow::Result<()> {
    stop()?;
    while is_running()? {}
    Ok(())
}

pub fn restart() -> anyhow::Result<bool> {
    await_shutdown()?;
    Ok(start())
}

#[cfg(target_family = "unix")]
mod unix {
    use home::home_dir;
    use std::path::PathBuf;
    use subprocess::{Popen, PopenConfig, Redirection};

    const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

    pub fn default_preferences_file() -> Option<PathBuf> {
        home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
    }

    pub fn stop() -> subprocess::Result<Popen> {
        Popen::create(
            &["sudo", "systemctl", "stop", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        )
    }

    pub fn is_running() -> subprocess::Result<bool> {
        Popen::create(
            &["systemctl", "status", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        )
        .map(|popen| super::evaluate_exit_status_option(popen.exit_status()))
    }

    pub fn start() -> bool {
        Popen::create(
            &["sudo", "systemctl", "start", "chromium.service"],
            PopenConfig {
                stdout: Redirection::None,
                detached: false,
                ..Default::default()
            },
        )
        .map_or(false, |popen| {
            super::evaluate_exit_status_option(popen.exit_status())
        })
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

    pub fn stop() -> anyhow::Result<()> {
        for process in sysinfo::System::new().processes_by_name("Google Chrome") {
            process.kill();
        }

        Ok(())
    }

    pub fn is_running() -> anyhow::Result<bool> {
        Ok(!sysinfo::System::new()
            .processes_by_name("Google Chrome")
            .collect::<Vec<_>>()
            .is_empty())
    }

    pub fn start() -> bool {
        todo!()
    }
}

fn evaluate_exit_status_option(exit_status: Option<ExitStatus>) -> bool {
    exit_status.map_or_else(
        || {
            eprintln!("Subprocess did not exit.");
            false
        },
        evaluate_exit_status,
    )
}

fn evaluate_exit_status(exit_status: ExitStatus) -> bool {
    match exit_status {
        ExitStatus::Exited(exit_code) => {
            eprintln!("Subprocess exited with: {exit_code}");
        }
        ExitStatus::Signaled(exit_code) => {
            eprintln!("Subprocess signalled: {exit_code}");
        }
        ExitStatus::Other(exit_code) => {
            eprintln!("Subprocess other: {exit_code}");
        }
        ExitStatus::Undetermined => {
            eprintln!("Undetermined exit status.");
        }
    }

    exit_status == ExitStatus::Exited(0)
}
