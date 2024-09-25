use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

/// Starts the given service.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn start(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["start", service])
}

/// Stops the given service.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn stop(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["stop", service])
}

/// Stops and disables the given service.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn stop_and_disable(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["disable", "--now", service])
}

/// Enables and starts the given service.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn enable_and_start(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["enable", "--now", service])
}

/// Determines whether the given service is enabled or active.
pub fn is_enabled_or_active(service: &str) -> bool {
    if let Ok(enabled) = is_enabled(service) {
        if enabled == ExitStatus::Exited(0) {
            return true;
        }
    }

    if let Ok(active) = is_active(service) {
        if active == ExitStatus::Exited(0) {
            return true;
        }
    }

    false
}

/// Determines whether the given service is enabled.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn is_enabled(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["is-enabled", service])
}

/// Determines whether the given service is active.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn is_active(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["is-active", service])
}

/// Returns the status of the given service.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
pub fn status(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["status", service])
}

/// Runs a systemctl subcommand with administrative privileges using `sudo`.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
fn systemctl_adm(command: &[&str]) -> subprocess::Result<ExitStatus> {
    Popen::create(
        &[&["sudo", "systemctl"], command].concat(),
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            detached: false,
            ..Default::default()
        },
    )
    .and_then(|mut popen| popen.wait())
}

/// Runs a systemctl subcommand as normal user.
///
/// # Errors
///
/// This function will return a [`subprocess::PopenError`] if the subprocess fails.
fn systemctl(command: &[&str]) -> subprocess::Result<ExitStatus> {
    Popen::create(
        &[&["systemctl"], command].concat(),
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            detached: false,
            ..Default::default()
        },
    )
    .and_then(|mut popen| popen.wait())
}
