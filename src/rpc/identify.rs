use crate::rpc::beep::beep;
use crate::rpc::Result;
use std::fs::read_to_string;
use subprocess::{Popen, PopenConfig, Redirection};

const HOSTNAME: &str = "/etc/hostname";
const XMESSAGE_TIMEOUT: u32 = 15;

pub fn identify() -> Result {
    let beep_result = beep(None);
    let xmessage_result = read_to_string(HOSTNAME).map_or_else(
        |error| Result::Error(error.to_string().into()),
        |hostname| {
            xmessage(hostname.as_str(), XMESSAGE_TIMEOUT).map_or_else(
                |error| Result::Error(error.to_string().into()),
                |_| Result::Success(None),
            )
        },
    );
    beep_result + xmessage_result
}

fn xmessage(text: &str, timeout: u32) -> subprocess::Result<Popen> {
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
            ..Default::default()
        },
    )
}
