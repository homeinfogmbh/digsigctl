use pnet::datalink::interfaces;
use pnet::ipnetwork::IpNetwork;
use std::net::IpAddr;

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
