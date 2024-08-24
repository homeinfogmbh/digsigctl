use std::process::Command;

const SUDO: &str = "/usr/bin/sudo";

pub fn sudo(command: impl AsRef<str>) -> Command {
    let mut cmd = Command::new(SUDO);
    cmd.arg(command.as_ref());
    cmd
}
