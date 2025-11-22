use std::str::FromStr;

use crate::net::{PyIpAddr, PyIpv4Addr, PyIpv6Addr, PySocketAddr, PySocketAddrV4, PySocketAddrV6};

impl FromStr for PyIpAddr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for PyIpv4Addr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for PyIpv6Addr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip = s.parse()?;
        Ok(Self(ip))
    }
}

impl FromStr for PySocketAddr {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}

impl FromStr for PySocketAddrV4 {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}

impl FromStr for PySocketAddrV6 {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }
}
