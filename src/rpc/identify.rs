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
pub mod unix {
    use crate::rpc::Result;
    use std::fs::read_to_string;
    use subprocess::{Popen, PopenConfig, Redirection};

    const ETC_HOSTNAME: &str = "/etc/hostname";
    const X_MESSAGE_TIMEOUT_SEC: u8 = 15;

    pub fn display_hostname() -> Result {
        read_to_string(ETC_HOSTNAME).map_or_else(
            |error| Result::Error(error.to_string().into()),
            |hostname| {
                x_message(hostname.trim(), X_MESSAGE_TIMEOUT_SEC)
                    .map_or_else(std::convert::Into::into, |_| Result::Success(Box::new(())))
            },
        )
    }

    fn x_message(text: &str, timeout: u8) -> subprocess::Result<Popen> {
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
