use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

pub fn start(service: &str) -> bool {
    systemctl_adm(&["start", service])
}

pub fn stop(service: &str) -> bool {
    systemctl_adm(&["stop", service])
}

pub fn stop_and_disable(service: &str) -> bool {
    systemctl_adm(&["disable", "--now", service])
}

pub fn enable_and_start(service: &str) -> bool {
    systemctl_adm(&["enable", "--now", service])
}

pub fn is_enabled_or_active(service: &str) -> bool {
    is_enabled(service) | is_active(service)
}

pub fn is_enabled(service: &str) -> bool {
    systemctl(&["is-enabled", service])
}

pub fn is_active(service: &str) -> bool {
    systemctl(&["is-active", service])
}

pub fn status(service: &str) -> bool {
    systemctl(&["status", service])
}

fn systemctl_adm(command: &[&str]) -> bool {
    Popen::create(
        &[&["sudo", "systemctl"], command].concat(),
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

fn systemctl(command: &[&str]) -> bool {
    Popen::create(
        &[&["systemctl"], command].concat(),
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
