use rocket::serde::json::serde_json;
use std::process::{Command, Stdio};

const SENSORS: &str = "/usr/bin/sensors";
const JSON: &str = "-j";

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
