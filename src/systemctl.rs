use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

pub fn stop_and_disable(service: &str) -> bool {
    Popen::create(
        &["sudo", "systemctl", "disable", "--now", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

pub fn enable_and_start(service: &str) -> bool {
    Popen::create(
        &["sudo", "systemctl", "enable", "--now", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

pub fn is_enabled_or_active(service: &str) -> bool {
    is_enabled(service) | is_active(service)
}

pub fn is_enabled(service: &str) -> bool {
    Popen::create(
        &["systemctl", "is-enabled", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}

pub fn is_active(service: &str) -> bool {
    Popen::create(
        &["systemctl", "is-active", service],
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    )
    .map_or(false, |popen| {
        popen.exit_status().unwrap_or(ExitStatus::Undetermined) == ExitStatus::Exited(0)
    })
}
