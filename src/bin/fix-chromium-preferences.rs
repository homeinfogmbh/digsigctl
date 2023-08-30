use clap::Parser;
use digsigctl::{default_preferences_file, ChromiumPreferences};
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long)]
    filename: Option<PathBuf>,
}

fn main() {
    let file = Args::parse().filename.unwrap_or_else(|| {
        default_preferences_file().unwrap_or_else(|| {
            eprintln!("Could not find default preferences file.");
            exit(1);
        })
    });

    if let Ok(mut preferences) = ChromiumPreferences::load(&file) {
        if let Err(error) = preferences.update_or_init_sessions() {
            eprintln!("Could not update or init sessions: {error}");
        }

        if let Err(error) = preferences.update_or_init_profile() {
            eprintln!("Could not update or init profile: {error}");
        }

        if let Err(error) = preferences.save(&file) {
            eprintln!("Could not save file: {error}");
        }
    }
}
