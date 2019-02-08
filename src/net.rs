use std::net::{IpAddr, Ipv6Addr, SocketAddr, SocketAddrV6};

pub trait IpAddrExt {
    fn to_ipv6_compatible(&self) -> Ipv6Addr;
}

impl IpAddrExt for IpAddr {
    fn to_ipv6_compatible(&self) -> Ipv6Addr {
        match self {
            IpAddr::V4(ipv4_addr) => ipv4_addr.to_ipv6_compatible(),
            IpAddr::V6(ipv6_addr) => *ipv6_addr,
        }
    }
}

pub trait SocketAddrExt {
    fn to_ipv6_compatible(&self) -> SocketAddrV6;
}

impl SocketAddrExt for SocketAddr {
    fn to_ipv6_compatible(&self) -> SocketAddrV6 {
        SocketAddrV6::new(self.ip().to_ipv6_compatible(), self.port(), 0, 0)
    }
}

