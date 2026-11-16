use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::net::ipaddr::{RyIpAddr, RyIpv4Addr, RyIpv6Addr};
use crate::net::socketaddr::{RySocketAddr, RySocketAddrV4, RySocketAddrV6};

// ==========================================================================
// RyIpv4Addr
// ==========================================================================
impl From<&Ipv4Addr> for RyIpv4Addr {
    fn from(addr: &Ipv4Addr) -> Self {
        Self(*addr)
    }
}

impl From<Ipv4Addr> for RyIpv4Addr {
    fn from(addr: Ipv4Addr) -> Self {
        Self(addr)
    }
}

impl From<RyIpv4Addr> for IpAddr {
    fn from(addr: RyIpv4Addr) -> Self {
        Self::V4(addr.0)
    }
}

impl From<RySocketAddrV4> for RyIpv4Addr {
    fn from(addr: RySocketAddrV4) -> Self {
        Self(*addr.0.ip())
    }
}

// ==========================================================================
// RyIpv6Addr
// ==========================================================================
impl From<&Ipv6Addr> for RyIpv6Addr {
    fn from(addr: &Ipv6Addr) -> Self {
        Self(*addr)
    }
}

impl From<Ipv6Addr> for RyIpv6Addr {
    fn from(addr: Ipv6Addr) -> Self {
        Self(addr)
    }
}

impl From<RySocketAddrV6> for RyIpv6Addr {
    fn from(addr: RySocketAddrV6) -> Self {
        Self(*addr.0.ip())
    }
}

impl From<RyIpv6Addr> for IpAddr {
    fn from(addr: RyIpv6Addr) -> Self {
        Self::V6(addr.0)
    }
}

// ==========================================================================
// RyIpAddr
// ==========================================================================
impl From<&IpAddr> for RyIpAddr {
    fn from(addr: &IpAddr) -> Self {
        Self(*addr)
    }
}

impl From<IpAddr> for RyIpAddr {
    fn from(addr: IpAddr) -> Self {
        Self(addr)
    }
}

impl From<Ipv4Addr> for RyIpAddr {
    fn from(addr: Ipv4Addr) -> Self {
        Self(IpAddr::V4(addr))
    }
}

impl From<Ipv6Addr> for RyIpAddr {
    fn from(addr: Ipv6Addr) -> Self {
        Self(IpAddr::V6(addr))
    }
}

impl From<SocketAddrV4> for RyIpAddr {
    fn from(addr: SocketAddrV4) -> Self {
        Self(IpAddr::V4(*addr.ip()))
    }
}

impl From<&RySocketAddr> for RyIpAddr {
    fn from(addr: &RySocketAddr) -> Self {
        Self(addr.0.ip())
    }
}
impl From<RySocketAddr> for RyIpAddr {
    fn from(addr: RySocketAddr) -> Self {
        Self(addr.0.ip())
    }
}

impl From<RySocketAddrV4> for RyIpAddr {
    fn from(addr: RySocketAddrV4) -> Self {
        Self::from(addr.0)
    }
}
impl From<RySocketAddrV6> for RyIpAddr {
    fn from(addr: RySocketAddrV6) -> Self {
        Self(IpAddr::V6(*addr.0.ip()))
    }
}

// ==========================================================================
// Ipv4Addr ~ Ipv6Addr ~ IpAddr
// ==========================================================================
impl From<&RyIpv4Addr> for Ipv4Addr {
    fn from(addr: &RyIpv4Addr) -> Self {
        addr.0
    }
}
impl From<&RyIpv6Addr> for Ipv6Addr {
    fn from(addr: &RyIpv6Addr) -> Self {
        addr.0
    }
}
impl From<&RyIpAddr> for IpAddr {
    fn from(addr: &RyIpAddr) -> Self {
        addr.0
    }
}

impl From<RyIpv4Addr> for Ipv4Addr {
    fn from(addr: RyIpv4Addr) -> Self {
        addr.0
    }
}

impl From<RyIpv6Addr> for Ipv6Addr {
    fn from(addr: RyIpv6Addr) -> Self {
        addr.0
    }
}

// ==========================================================================
// IpAddr
// ==========================================================================
impl From<RyIpAddr> for IpAddr {
    fn from(addr: RyIpAddr) -> Self {
        addr.0
    }
}

// ==========================================================================
// RySocketAddrV4
// ==========================================================================
impl From<SocketAddrV4> for RySocketAddrV4 {
    fn from(addr: SocketAddrV4) -> Self {
        Self(addr)
    }
}

// ==========================================================================
// RySocketAddrV6
// ==========================================================================
impl From<SocketAddrV6> for RySocketAddrV6 {
    fn from(addr: SocketAddrV6) -> Self {
        Self(addr)
    }
}

// ==========================================================================
// RySocketAddr
// ==========================================================================
impl From<SocketAddr> for RySocketAddr {
    fn from(addr: SocketAddr) -> Self {
        Self(addr)
    }
}

impl From<&SocketAddr> for RySocketAddr {
    fn from(addr: &SocketAddr) -> Self {
        Self(*addr)
    }
}

impl From<&RySocketAddr> for SocketAddr {
    fn from(addr: &RySocketAddr) -> Self {
        addr.0
    }
}

impl From<&RySocketAddrV4> for SocketAddr {
    fn from(addr: &RySocketAddrV4) -> Self {
        addr.0.into()
    }
}

impl From<&RySocketAddrV6> for SocketAddr {
    fn from(addr: &RySocketAddrV6) -> Self {
        addr.0.into()
    }
}
