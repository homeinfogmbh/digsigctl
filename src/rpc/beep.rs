use crate::rpc::Result;
use beep_evdev::Melody;

pub fn beep(melody: Option<Melody>) -> Result {
    melody.unwrap_or_default().play().map_or_else(
        |error| Result::Error(error.to_string().into()),
        |_| Result::Success(None),
    )
}
