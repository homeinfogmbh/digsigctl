use crate::rpc::beep::beep;
use crate::rpc::Result;

#[cfg(target_family = "unix")]
pub fn identify() -> Result {
    beep(None) + unix::display_hostname()
}

#[cfg(target_family = "windows")]
pub fn identify() -> Result {
    beep(None)
}

#[cfg(target_family = "unix")]
mod unix {
    use std::fs::read_to_string;
    use subprocess::{Popen, PopenConfig, Redirection};

    const ETC_HOSTNAME: &str = "/etc/hostname";
    const XMESSAGE_TIMEOUT_SEC: u8 = 15;

    fn display_hostname() -> Result {
        read_to_string(ETC_HOSTNAME).map_or_else(
            |error| Result::Error(error.to_string().into()),
            |hostname| {
                xmessage(hostname.trim(), XMESSAGE_TIMEOUT_SEC)
                    .map_or_else(std::convert::Into::into, |_| Result::Success(Box::new(())))
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
}
