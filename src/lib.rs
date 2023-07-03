mod config;
mod net;

use crate::net::{discover_address, VpnDiscoveryError, PORT};
pub use config::Config;

/// Returns a custom config for Rocket
/// # Errors
/// Returns a `[VpnDiscoveryError]` in case a VPN address could not be discovered
pub fn get_config() -> Result<rocket::figment::Figment, VpnDiscoveryError> {
    discover_address().map(|address| {
        rocket::Config::figment()
            .merge(("port", PORT))
            .merge(("address", address))
    })
}
