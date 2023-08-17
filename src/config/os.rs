#[cfg(target_family = "unix")]
use home::home_dir;
#[cfg(target_family = "windows")]
use std::env::var;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = ".config/chromium/Default/Preferences";

#[cfg(target_family = "windows")]
const CHROMIUM_DEFAULT_PREFERENCES: &str = r"Google\Chrome\User Data\Default";

#[cfg(target_family = "unix")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    home_dir().map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
}

#[cfg(target_family = "windows")]
pub fn chromium_default_preferences() -> Option<PathBuf> {
    var("%LOCALAPPDATA%")
        .map(PathBuf::from)
        .map(|home| home.join(CHROMIUM_DEFAULT_PREFERENCES))
        .ok()
}
