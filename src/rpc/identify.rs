use crate::rpc::beep::beep;
use crate::rpc::Result;
use std::fs::read_to_string;
use subprocess::{Popen, PopenConfig, Redirection};

const HOSTNAME: &str = "/etc/hostname";
const XMESSAGE_TIMEOUT_SEC: u8 = 15;

pub fn identify() -> Result {
    beep(None) + display_hostname()
}

fn display_hostname() -> Result {
    read_to_string(HOSTNAME).map_or_else(
        |error| Result::Error(error.to_string().into()),
        |hostname| {
            xmessage(hostname.trim(), XMESSAGE_TIMEOUT_SEC).map_or_else(
                |error| Result::Error(error.to_string().into()),
                |_| Result::Success(None),
            )
        },
    )
}

fn xmessage(text: &str, timeout: u8) -> subprocess::Result<Popen> {
    Popen::create(
        &[
            "xmessage",
            "-center",
            "-timeout",
            timeout.to_string().as_str(),
            text,
        ],
        PopenConfig {
            stdout: Redirection::None,
            detached: true,
            ..Default::default()
        },
    )
}
