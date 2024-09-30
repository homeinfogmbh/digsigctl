use std::process::Command;

const SUDO: &str = "/usr/bin/sudo";

/// Returns a command that will be run with `sudo`.
pub fn sudo(command: impl AsRef<str>) -> Command {
    let mut cmd = Command::new(SUDO);
    cmd.arg(command.as_ref());
    cmd
}
