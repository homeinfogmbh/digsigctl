use crate::rpc::Result;
#[cfg(target_family = "unix")]
use beep_evdev::Melody;

#[cfg(target_family = "unix")]
pub fn beep(melody: Option<Melody>) -> Result {
    melody
        .unwrap_or_default()
        .play()
        .map_or_else(Into::into, |()| Result::Success(Box::new(())))
}

#[cfg(target_family = "windows")]
pub fn beep(_: Option<()>) -> Result {
    todo!()
}
