use crate::rpc::CommandResult;
use rocket::serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_FREQ: u16 = 440;
const DEFAULT_LEN: u64 = 200;
const DEFAULT_REPEATS: u64 = 1;
const DEFAULT_DELAY: u64 = 100;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Args {
    melody: Option<Vec<Note>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Note {
    freq: Option<u16>,
    len: Option<u64>,
    repeats: Option<u64>,
    delay: Option<u64>,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            freq: Some(DEFAULT_FREQ),
            len: Some(DEFAULT_LEN),
            repeats: Some(DEFAULT_REPEATS),
            delay: Some(DEFAULT_DELAY),
        }
    }
}

pub fn beep(args: &Args) -> CommandResult {
    args.melody
        .as_ref()
        .map_or_else(
            || play_melody(&[Note::default()]),
            |melody| play_melody(melody),
        )
        .map_or_else(
            |error| CommandResult::Error(Some(error.to_string()), None),
            |_| CommandResult::Success(None),
        )
}

fn play_melody(melody: &[Note]) -> Result<(), std::io::Error> {
    for note in melody {
        if note.repeats.unwrap_or(DEFAULT_REPEATS) > 0 {
            beep_evdev::beep(note.freq.unwrap_or(DEFAULT_FREQ))?;
            sleep(Duration::from_millis(note.len.unwrap_or(DEFAULT_LEN)));
            beep_evdev::beep(0)?;
        }

        for _ in 1..note.repeats.unwrap_or(DEFAULT_REPEATS) {
            sleep(Duration::from_millis(note.delay.unwrap_or(DEFAULT_DELAY)));
            beep_evdev::beep(note.freq.unwrap_or(DEFAULT_FREQ))?;
            sleep(Duration::from_millis(note.len.unwrap_or(DEFAULT_LEN)));
            beep_evdev::beep(0)?;
        }
    }

    Ok(())
}
