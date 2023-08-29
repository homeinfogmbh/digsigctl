use digsigctl::{default_preferences_file, ChromiumPreferences};
use std::process::exit;

fn main() {
    default_preferences_file().map_or_else(
        || {
            eprintln!("Could not find default preferences file.");
            exit(1);
        },
        |file| {
            if let Ok(mut preferences) = ChromiumPreferences::load(file) {
                if let Err(error) = preferences.update_or_init_sessions() {
                    eprintln!("Could not update or init sessions: {error}");
                }

                if let Err(error) = preferences.update_or_init_profile() {
                    eprintln!("Could not update or init profile: {error}");
                }
            }
        },
    );
}
