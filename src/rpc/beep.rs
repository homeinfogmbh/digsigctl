use crate::rpc::Result;
use beep_evdev::Melody;

pub fn beep(melody: Option<Melody>) -> Result {
    melody
        .unwrap_or_default()
        .play()
        .map_or_else(std::convert::Into::into, |_| Result::Success(Box::new(())))
}
