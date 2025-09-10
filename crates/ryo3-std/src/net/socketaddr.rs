use crate::net::PyIpv6Addr;
use crate::net::ipaddr::{IpAddrLike, PyIpAddr, PyIpv4Addr};
use pyo3::prelude::*;
use ryo3_macro_rules::{pytodo, pytodo_err};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddrV4", module = "ry.ryo3", frozen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PySocketAddrV4(pub(crate) SocketAddrV4);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddrV6", module = "ry.ryo3", frozen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PySocketAddrV6(pub(crate) SocketAddrV6);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddr", module = "ry.ryo3", frozen)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PySocketAddr(pub(crate) SocketAddr);

#[pymethods]
#[expect(clippy::trivially_copy_pass_by_ref)]
impl PySocketAddrV4 {
    #[new]
    #[expect(clippy::needless_pass_by_value)]
    pub(crate) fn py_new(ip: IpAddrLike, port: u16) -> PyResult<Self> {
        let ipv4 = ip.get_ipv4()?;
        let sa = SocketAddrV4::new(ipv4, port);
        Ok(Self(sa))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        let py_ip = PyIpv4Addr::from(self.0.ip());
        format!("SocketAddrV4({}, {})", py_ip.__repr__(), self.port())
    }

    fn __richcmp__(&self, other: Self, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.0 == other.0,
            pyo3::basic::CompareOp::Ne => self.0 != other.0,
            pyo3::basic::CompareOp::Lt => self.0 < other.0,
            pyo3::basic::CompareOp::Le => self.0 <= other.0,
            pyo3::basic::CompareOp::Gt => self.0 > other.0,
            pyo3::basic::CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddrv4(&self) -> PyIpv4Addr {
        PyIpv4Addr::from(self.0.ip())
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> Ipv4Addr {
        *self.0.ip()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }

    #[classattr]
    fn version() -> u8 {
        4
    }

    // ip getter
    #[getter]
    fn ip(&self) -> PyIpv4Addr {
        PyIpv4Addr::from(self.0.ip())
    }
    // ========================================================================
    // Ipv4 forwarded
    // ========================================================================
    #[expect(clippy::unused_self)]
    #[getter]
    fn is_benchmarking(&self) -> PyResult<bool> {
        pytodo_err!()
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        self.0.ip().is_broadcast()
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        self.0.ip().is_documentation()
    }

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_global(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_link_local(&self) -> bool {
        self.0.ip().is_link_local()
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.ip().is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.ip().is_multicast()
    }

    #[getter]
    fn is_private(&self) -> bool {
        self.0.ip().is_private()
    }

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_reserved(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_shared(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        self.0.ip().is_unspecified()
    }
}

#[pymethods]
impl PySocketAddrV6 {
    #[new]
    #[pyo3(signature = (ip, port, flowinfo = 0, scope_id = 0))]
    #[expect(clippy::needless_pass_by_value)]
    fn py_new(ip: IpAddrLike, port: u16, flowinfo: u32, scope_id: u32) -> PyResult<Self> {
        let ipv6 = ip.get_ipv6()?;
        let sa = SocketAddrV6::new(ipv6, port, flowinfo, scope_id);
        Ok(Self(sa))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        let py_ip = PyIpv6Addr::from(self.0.ip());
        format!("SocketAddrV6({}, {})", py_ip.__repr__(), self.port())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.0 == other.0,
            pyo3::basic::CompareOp::Ne => self.0 != other.0,
            pyo3::basic::CompareOp::Lt => self.0 < other.0,
            pyo3::basic::CompareOp::Le => self.0 <= other.0,
            pyo3::basic::CompareOp::Gt => self.0 > other.0,
            pyo3::basic::CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> Ipv6Addr {
        *self.0.ip()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddrv6(&self) -> PyIpv6Addr {
        PyIpv6Addr::from(self.0.ip())
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(IpAddr::V6(*self.0.ip()))
    }

    #[getter]
    fn ip(&self) -> PyIpv6Addr {
        PyIpv6Addr::from(self.0.ip())
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }

    #[classattr]
    fn version() -> u8 {
        6
    }

    // ========================================================================
    // IpAddr forwarded
    // ========================================================================
    #[getter]
    #[expect(clippy::unused_self)]
    fn is_benchmarking(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_documentation(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_global(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.ip().is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.ip().is_multicast()
    }

    #[getter]
    fn is_ipv4_mapped(&self) -> bool {
        self.to_ipaddrv6().is_ipv4_mapped()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_unicast(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_unicast_global(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_unicast_link_local(&self) -> bool {
        self.0.ip().is_unicast_link_local()
    }

    #[getter]
    fn is_unique_local(&self) -> bool {
        self.0.ip().is_unique_local()
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        self.0.ip().is_unspecified()
    }
}

#[pymethods]
impl PySocketAddr {
    #[new]
    #[pyo3(signature = (ip, port, flowinfo = None, scope_id = None))]
    #[expect(clippy::needless_pass_by_value)]
    pub(crate) fn py_new(
        ip: IpAddrLike,
        port: u16,
        flowinfo: Option<u32>,
        scope_id: Option<u32>,
    ) -> PyResult<Self> {
        let ip = ip.get_ip()?;
        match ip {
            IpAddr::V4(ipv4) => {
                let sa = SocketAddr::V4(SocketAddrV4::new(ipv4, port));
                Ok(Self(sa))
            }
            IpAddr::V6(ipv6) => {
                let flowinfo = flowinfo.unwrap_or(0);
                let scope_id = scope_id.unwrap_or(0);
                let sa = SocketAddr::V6(SocketAddrV6::new(ipv6, port, flowinfo, scope_id));
                Ok(Self(sa))
            }
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        let py_str = match self.0.ip() {
            IpAddr::V4(ipv4) => PyIpv4Addr::from(ipv4).__repr__(),
            IpAddr::V6(ipv6) => PyIpv6Addr::from(ipv6).__repr__(),
        };
        format!("SocketAddr({}, {})", py_str, self.port())
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> bool {
        match op {
            pyo3::basic::CompareOp::Eq => self.0 == other.0,
            pyo3::basic::CompareOp::Ne => self.0 != other.0,
            pyo3::basic::CompareOp::Lt => self.0 < other.0,
            pyo3::basic::CompareOp::Le => self.0 <= other.0,
            pyo3::basic::CompareOp::Gt => self.0 > other.0,
            pyo3::basic::CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        let sock = s.parse()?;
        Ok(Self(sock))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0.ip())
    }

    #[getter]
    fn ip(&self) -> PyIpAddr {
        PyIpAddr::from(self.0.ip())
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> IpAddr {
        match self.0.ip() {
            IpAddr::V4(addr) => IpAddr::V4(addr),
            IpAddr::V6(addr) => IpAddr::V6(addr),
        }
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[getter]
    fn version(&self) -> u8 {
        match self.0.ip() {
            IpAddr::V4(_) => 4,
            IpAddr::V6(_) => 6,
        }
    }

    // ========================================================================
    // IpAddr forwarded
    // ========================================================================
    #[expect(clippy::unused_self)]
    #[getter]
    fn is_benchmarking(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_ipv4(&self) -> bool {
        self.0.ip().is_ipv4()
    }

    #[getter]
    fn is_ipv6(&self) -> bool {
        self.0.ip().is_ipv6()
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(addr) => addr.is_broadcast(),
            IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(addr) => addr.is_documentation(),
            IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.ip().is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.ip().is_multicast()
    }

    #[getter]
    fn is_private(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(addr) => addr.is_private(),
            IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        self.0.ip().is_unspecified()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_global(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_ipv4_mapped(&self) -> bool {
        match self.0.ip() {
            IpAddr::V6(addr) => {
                matches!(
                    addr.octets(),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, _, _, _, _]
                )
            }
            IpAddr::V4(_) => false,
        }
    }

    #[getter]
    fn is_link_local(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(addr) => addr.is_link_local(),
            IpAddr::V6(_) => false,
        }
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_reserved(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_shared(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_unicast(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(addr) => !addr.is_multicast(),
            IpAddr::V6(addr) => !addr.is_multicast(),
        }
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_unicast_global(&self) -> PyResult<bool> {
        pytodo!()
    }

    #[getter]
    fn is_unicast_link_local(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.is_unicast_link_local(),
        }
    }

    #[getter]
    fn is_unique_local(&self) -> bool {
        match self.0.ip() {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.is_unique_local(),
        }
    }
}
