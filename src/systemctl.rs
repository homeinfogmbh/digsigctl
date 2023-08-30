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
    evaluate_result(Popen::create(
        &[&["sudo", "systemctl"], command].concat(),
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    ))
}

fn systemctl(command: &[&str]) -> bool {
    evaluate_result(Popen::create(
        &[&["systemctl"], command].concat(),
        PopenConfig {
            stdout: Redirection::None,
            detached: false,
            ..Default::default()
        },
    ))
}

fn evaluate_result(result: subprocess::Result<Popen>) -> bool {
    match result {
        Ok(popen) => popen.exit_status().map_or_else(
            || {
                eprintln!("Could not determine exit status");
                false
            },
            |exit_status| {
                println!("Exit status is: {exit_status:?}");
                exit_status == ExitStatus::Exited(0)
            },
        ),
        Err(error) => {
            eprintln!("Popen failed: {error}");
            false
        }
    }
}
