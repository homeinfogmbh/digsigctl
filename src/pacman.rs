use std::process::{Command, Stdio};

use crate::from_io::TryFromIo;

const PACMAN: &str = "/usr/bin/pacman";

/// Create pacman commands.
pub fn pacman(args: &[&str]) -> Command {
    let mut command = Command::new(PACMAN);

    for arg in args {
        command.arg(arg);
    }

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command
}

/// Retrieve the version of a package using pacman.
pub fn package_version(package: &str) -> std::io::Result<String> {
    String::try_from_io(pacman(&["-Q", package]).spawn()?.wait_with_output()?.stdout)
        .and_then(|string| {
            string
                .split_whitespace()
                .collect::<Vec<_>>()
                .get(1)
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "missing version field")
                })
                .map(std::string::ToString::to_string)
        })
}
