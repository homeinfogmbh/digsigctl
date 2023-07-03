mod config;
mod net;
mod sysinfo;

use crate::net::{discover_address, VpnDiscoveryError, PORT};
pub use config::Config;
pub use sysinfo::SystemInformation;

/// Returns a custom config for Rocket
/// # Errors
/// Returns a `[VpnDiscoveryError]` in case a VPN address could not be discovered
pub fn get_config() -> Result<rocket::figment::Figment, VpnDiscoveryError> {
    match rocket::Config::figment().profile().as_str().as_str() {
        "debug" => Ok(rocket::Config::figment().merge(("port", PORT))),
        _ => discover_address().map(|address| {
            rocket::Config::figment()
                .merge(("port", PORT))
                .merge(("address", address))
        }),
    }
}
