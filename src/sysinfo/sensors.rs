use rocket::serde::json::serde_json;
use std::process::{Command, Stdio};

const SENSORS: &str = "/usr/bin/sensors";
const JSON: &str = "-j";

/// Collect information about local sensors.
///
/// # Errors
///
/// This function returns an [`anyhow::Error`] if the values could not be read
/// from the subcommand or could not be parsed from its output.
pub fn sensors() -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::from_slice(
        Command::new(SENSORS)
            .arg(JSON)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?
            .wait_with_output()?
            .stdout
            .as_slice(),
    )?)
}
