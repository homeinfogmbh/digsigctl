use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

pub fn start(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["start", service])
}

pub fn stop(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["stop", service])
}

pub fn stop_and_disable(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["disable", "--now", service])
}

pub fn enable_and_start(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl_adm(&["enable", "--now", service])
}

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

pub fn is_enabled(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["is-enabled", service])
}

pub fn is_active(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["is-active", service])
}

pub fn status(service: &str) -> subprocess::Result<ExitStatus> {
    systemctl(&["status", service])
}

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
