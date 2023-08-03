use pnet::datalink::interfaces;
use pnet::ipnetwork::IpNetwork;
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
    for iface in interfaces() {
        for ip in iface.ips {
            if network.contains(ip.ip()) {
                return Some(ip.ip());
            }
        }
    }

    None
}
