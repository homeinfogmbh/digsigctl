mod net;

use crate::net::{discover_address, VpnDiscoveryError};
use rocket::{figment::Figment, Config};

/// Returns a custom config for Rocket
/// # Errors
/// Returns a `[VpnDiscoveryError]` in case a VPN address could not be discovered
pub fn get_config() -> Result<Figment, VpnDiscoveryError> {
    discover_address().map(|address| {
        Config::figment()
            .merge(("port", 5000))
            .merge(("address", address))
    })
}
