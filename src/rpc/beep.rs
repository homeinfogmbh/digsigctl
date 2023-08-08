use crate::rpc::Result;
use beep_evdev::Melody;

#[cfg(target_family = "unix")]
pub fn beep(melody: Option<Melody>) -> Result {
    melody
        .unwrap_or_default()
        .play()
        .map_or_else(std::convert::Into::into, |_| Result::Success(Box::new(())))
}

#[cfg(target_family = "windows")]
pub fn beep(melody: Option<Melody>) -> Result {
    todo!()
}
