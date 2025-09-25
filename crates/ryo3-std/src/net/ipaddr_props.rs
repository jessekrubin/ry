//! traits for IP address like thing properties
use crate::net::{PyIpAddr, PyIpv4Addr, PyIpv6Addr, PySocketAddr, PySocketAddrV4, PySocketAddrV6};
use pyo3::prelude::*;
use ryo3_macro_rules::pytodo;

pub(crate) trait AsIpAddr {
    fn as_ipaddr(&self) -> std::net::IpAddr;
}

pub(crate) trait IpAddrProps {
    fn version(&self) -> u8;
    fn is_ipv4(&self) -> bool {
        self.version() == 4
    }
    fn is_ipv6(&self) -> bool {
        self.version() == 6
    }

    fn is_broadcast(&self) -> bool;
    fn is_documentation(&self) -> bool;
    fn is_ipv4_mapped(&self) -> bool;
    fn is_link_local(&self) -> bool;
    fn is_loopback(&self) -> bool;
    fn is_multicast(&self) -> bool;
    fn is_private(&self) -> bool;
    fn is_unicast(&self) -> bool;
    fn is_unicast_global(&self) -> bool;
    fn is_unicast_link_local(&self) -> bool;
    fn is_unique_local(&self) -> bool;
    fn is_unspecified(&self) -> bool;
    // ========================================================================
    // UNSTABLE COPY PASTA-ED
    // ========================================================================
    fn is_benchmarking(&self) -> bool;
    fn is_reserved(&self) -> bool;
    fn is_shared(&self) -> bool;

    fn is_global(&self) -> PyResult<bool> {
        pytodo!()
    }
}

impl<T> IpAddrProps for T
where
    T: AsIpAddr,
{
    #[inline]
    fn version(&self) -> u8 {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(_) => 4,
            std::net::IpAddr::V6(_) => 6,
        }
    }

    #[inline]
    fn is_broadcast(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(ipv4) => ipv4.is_broadcast(),
            std::net::IpAddr::V6(_) => false,
        }
    }

    #[inline]
    fn is_documentation(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => addr.is_documentation(),
            std::net::IpAddr::V6(addr) => {
                matches!(
                    addr.segments(),
                    [0x2001, 0xdb8, ..] | [0x3fff, 0..=0x0fff, ..]
                )
            }
        }
    }

    #[inline]
    fn is_ipv4_mapped(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V6(addr) => {
                matches!(
                    addr.octets(),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, _, _, _, _]
                )
            }
            std::net::IpAddr::V4(_) => false,
        }
    }

    #[inline]
    fn is_link_local(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => addr.is_link_local(),
            // maybe should be is_unicast_link_local?
            std::net::IpAddr::V6(_) => false,
        }
    }

    #[inline]
    fn is_loopback(&self) -> bool {
        self.as_ipaddr().is_loopback()
    }

    #[inline]
    fn is_multicast(&self) -> bool {
        self.as_ipaddr().is_multicast()
    }

    #[inline]
    fn is_private(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => addr.is_private(),
            std::net::IpAddr::V6(_addr) => false,
        }
    }

    #[inline]
    fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }

    #[inline]
    fn is_unicast_global(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(_) => false,
            std::net::IpAddr::V6(_) => {
                self.is_unicast()
                    && !self.is_loopback()
                    && !self.is_unicast_link_local()
                    && !self.is_unique_local()
                    && !self.is_unspecified()
                    && !self.is_documentation()
                    && !self.is_benchmarking()
            }
        }
    }

    #[inline]
    fn is_unicast_link_local(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(_) => false,
            std::net::IpAddr::V6(addr) => addr.is_unicast_link_local(),
        }
    }

    #[inline]
    fn is_unique_local(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(_) => false,
            std::net::IpAddr::V6(addr) => addr.is_unique_local(),
        }
    }

    #[inline]
    fn is_unspecified(&self) -> bool {
        self.as_ipaddr().is_unspecified()
    }

    // ========================================================================
    // UNSTABLE COPY PASTA-ED
    // ========================================================================
    #[inline]
    fn is_benchmarking(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => {
                (addr.octets()[0] == 200) && (addr.octets()[1] == 0) && (addr.octets()[2] == 0)
            }
            std::net::IpAddr::V6(addr) => {
                (addr.segments()[0] == 0x2001)
                    && (addr.segments()[1] == 0x2)
                    && (addr.segments()[2] == 0)
            }
        }
    }

    #[inline]
    fn is_reserved(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => addr.octets()[0] >= 240,
            std::net::IpAddr::V6(_addr) => false,
        }
    }

    #[inline]
    fn is_shared(&self) -> bool {
        match self.as_ipaddr() {
            std::net::IpAddr::V4(addr) => {
                addr.octets()[0] == 100 && (addr.octets()[1] & 0b1100_0000 == 0b0100_0000)
            }
            std::net::IpAddr::V6(_) => false,
        }
    }
}

impl<T: AsIpAddr + ?Sized> AsIpAddr for &T {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        (*self).as_ipaddr()
    }
}

impl AsIpAddr for PyIpv4Addr {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        std::net::IpAddr::V4(self.0)
    }
}

impl AsIpAddr for PySocketAddrV4 {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        std::net::IpAddr::V4(*self.0.ip())
    }
}

impl AsIpAddr for PyIpv6Addr {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        std::net::IpAddr::V6(self.0)
    }
}

impl AsIpAddr for PySocketAddrV6 {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        std::net::IpAddr::V6(*self.0.ip())
    }
}

impl AsIpAddr for PyIpAddr {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        self.0
    }
}

impl AsIpAddr for PySocketAddr {
    #[inline]
    fn as_ipaddr(&self) -> std::net::IpAddr {
        self.0.ip()
    }
}
