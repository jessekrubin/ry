use crate::net::ipaddr::{PyIpAddr, PyIpv4Addr, PyIpv6Addr};
use crate::net::socketaddr::{PySocketAddr, PySocketAddrV4, PySocketAddrV6};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// ==========================================================================
// PyIpv4Addr
// ==========================================================================
impl From<&Ipv4Addr> for PyIpv4Addr {
    fn from(addr: &Ipv4Addr) -> Self {
        Self(*addr)
    }
}

impl From<Ipv4Addr> for PyIpv4Addr {
    fn from(addr: Ipv4Addr) -> Self {
        Self(addr)
    }
}

impl From<PySocketAddrV4> for PyIpv4Addr {
    fn from(addr: PySocketAddrV4) -> Self {
        Self(*addr.0.ip())
    }
}

impl From<&Self> for PySocketAddrV4 {
    fn from(addr: &Self) -> Self {
        Self(addr.0)
    }
}

// ==========================================================================
// PyIpv6Addr
// ==========================================================================
impl From<&Ipv6Addr> for PyIpv6Addr {
    fn from(addr: &Ipv6Addr) -> Self {
        Self(*addr)
    }
}
impl From<Ipv6Addr> for PyIpv6Addr {
    fn from(addr: Ipv6Addr) -> Self {
        Self(addr)
    }
}

impl From<PySocketAddrV6> for PyIpv6Addr {
    fn from(addr: PySocketAddrV6) -> Self {
        Self(*addr.0.ip())
    }
}

// ==========================================================================
// PyIpAddr
// ==========================================================================
impl From<&IpAddr> for PyIpAddr {
    fn from(addr: &IpAddr) -> Self {
        Self(*addr)
    }
}

impl From<IpAddr> for PyIpAddr {
    fn from(addr: IpAddr) -> Self {
        Self(addr)
    }
}

impl From<Ipv4Addr> for PyIpAddr {
    fn from(addr: Ipv4Addr) -> Self {
        Self(IpAddr::V4(addr))
    }
}

impl From<Ipv6Addr> for PyIpAddr {
    fn from(addr: Ipv6Addr) -> Self {
        Self(IpAddr::V6(addr))
    }
}

impl From<SocketAddrV4> for PyIpAddr {
    fn from(addr: SocketAddrV4) -> Self {
        Self(IpAddr::V4(*addr.ip()))
    }
}

impl From<&PySocketAddr> for PyIpAddr {
    fn from(addr: &PySocketAddr) -> Self {
        Self(addr.0.ip())
    }
}
impl From<PySocketAddr> for PyIpAddr {
    fn from(addr: PySocketAddr) -> Self {
        Self(addr.0.ip())
    }
}

impl From<PySocketAddrV4> for PyIpAddr {
    fn from(addr: PySocketAddrV4) -> Self {
        Self::from(addr.0)
    }
}
impl From<PySocketAddrV6> for PyIpAddr {
    fn from(addr: PySocketAddrV6) -> Self {
        Self(IpAddr::V6(*addr.0.ip()))
    }
}

// ==========================================================================
// Ipv4Addr ~ Ipv6Addr ~ IpAddr
// ==========================================================================
impl From<&PyIpv4Addr> for Ipv4Addr {
    fn from(addr: &PyIpv4Addr) -> Self {
        addr.0
    }
}
impl From<&PyIpv6Addr> for Ipv6Addr {
    fn from(addr: &PyIpv6Addr) -> Self {
        addr.0
    }
}
impl From<&PyIpAddr> for IpAddr {
    fn from(addr: &PyIpAddr) -> Self {
        addr.0
    }
}

impl From<PyIpv4Addr> for Ipv4Addr {
    fn from(addr: PyIpv4Addr) -> Self {
        addr.0
    }
}

impl From<PyIpv6Addr> for Ipv6Addr {
    fn from(addr: PyIpv6Addr) -> Self {
        addr.0
    }
}

// ==========================================================================
// IpAddr
// ==========================================================================
impl From<PyIpAddr> for IpAddr {
    fn from(addr: PyIpAddr) -> Self {
        addr.0
    }
}

// ==========================================================================
// PySocketAddrV4
// ==========================================================================
impl From<SocketAddrV4> for PySocketAddrV4 {
    fn from(addr: SocketAddrV4) -> Self {
        Self(addr)
    }
}

// ==========================================================================
// PySocketAddrV6
// ==========================================================================
impl From<SocketAddrV6> for PySocketAddrV6 {
    fn from(addr: SocketAddrV6) -> Self {
        Self(addr)
    }
}

// ==========================================================================
// PySocketAddr
// ==========================================================================

impl From<SocketAddr> for PySocketAddr {
    fn from(addr: SocketAddr) -> Self {
        Self(addr)
    }
}
