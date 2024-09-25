use ipnetwork::IpNetwork;
use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;

/// Discovers a local IP address of the system within the network as specified by the `network`.
///
/// This will exit the program, if the passed-in string is not a valid IP network
/// or of no appropriate IP within that IP network can be found on the system.
#[must_use]
pub fn discover_address_or_exit(network: &str) -> IpAddr {
    discover_address(IpNetwork::from_str(network).unwrap_or_else(|error| {
        eprintln!("{error}");
        exit(1)
    }))
    .unwrap_or_else(|| {
        eprintln!("No address found");
        exit(2);
    })
}

/// Discovers a local IP address within the given network.
///
/// This is used to find the system's VPN IP address on which we want to listen,
/// since we don't want to expose `digsigctl` to the entire internet.
///
/// # Errors
/// This function will return `None` if no appropriate IP address could be found on the system.
#[must_use]
pub fn discover_address(network: IpNetwork) -> Option<IpAddr> {
    if let Ok(network_interfaces) = list_afinet_netifas() {
        for (_, ip) in network_interfaces {
            if network.contains(ip) {
                return Some(ip);
            }
        }
    }

    None
}
