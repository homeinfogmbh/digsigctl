use pnet::datalink::interfaces;
use pnet::ipnetwork::{IpNetwork, IpNetworkError, Ipv6Network};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv6Addr};

const VPN: (Ipv6Addr, u8) = (
    Ipv6Addr::new(0xfd56, 0x1dda, 0x8794, 0xcb90, 0, 0, 0, 0),
    64,
);

#[derive(Debug, Eq, PartialEq)]
pub enum VpnDiscoveryError {
    InvalidNetwork(IpNetworkError),
    NoAddressFound,
}

impl Display for VpnDiscoveryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNetwork(ip_network_error) => ip_network_error.fmt(f),
            Self::NoAddressFound => write!(f, "No VPN address found to bind to"),
        }
    }
}

impl Error for VpnDiscoveryError {}

pub fn discover_address() -> Result<IpAddr, VpnDiscoveryError> {
    Ipv6Network::new(VPN.0, VPN.1)
        .map_err(VpnDiscoveryError::InvalidNetwork)
        .and_then(|ipv6network| {
            let network = IpNetwork::V6(ipv6network);

            for iface in interfaces() {
                for ip in iface.ips {
                    if network.contains(ip.ip()) {
                        return Ok(ip.ip());
                    }
                }
            }

            Err(VpnDiscoveryError::NoAddressFound)
        })
}
