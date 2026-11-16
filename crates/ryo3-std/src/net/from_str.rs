use std::str::FromStr;

use crate::net::{RyIpAddr, RyIpv4Addr, RyIpv6Addr, RySocketAddr, RySocketAddrV4, RySocketAddrV6};

impl FromStr for RyIpAddr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for RyIpv4Addr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for RyIpv6Addr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for RySocketAddr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}

impl FromStr for RySocketAddrV4 {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}

impl FromStr for RySocketAddrV6 {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}
