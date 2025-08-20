//! Digital signage client library.
//!
//! This library exposes functions and datastructures as used
//! by the programs `digsigctl` and `fix-chromium-preferences`.
mod config;
mod constants;
mod net;
mod pacman;
pub mod portal;
mod rpc;
mod screenshot;
mod sudo;
mod sysinfo;
mod systemctl;
mod try_from_io;

pub use crate::sysinfo::SystemInformation;
pub use config::{ChromiumPreferences, Config};
pub use net::discover_address_or_exit;
pub use portal::{apply_portal_config_if_needed, apply_portal_config_on_startup, verify_startup_page};
pub use rpc::default_preferences_file;
pub use rpc::{Command, Result};
pub use screenshot::{take_screenshot, ScreenshotResponse};
