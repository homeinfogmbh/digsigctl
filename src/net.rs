use ipnetwork::IpNetwork;
use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;

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
