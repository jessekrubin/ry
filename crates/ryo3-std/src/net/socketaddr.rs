use crate::net::ipaddr::{IpAddrLike, PyIpAddr, PyIpv4Addr};
use crate::net::PyIpv6Addr;
use pyo3::prelude::*;
use pyo3::types::PyType;
use ryo3_macro_rules::err_py_not_impl;
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

#[pyclass(name = "SocketAddrV4", module = "ry.ryo3", frozen)]
pub struct PySocketAddrV4(pub(crate) SocketAddrV4);

#[pyclass(name = "SocketAddrV6", module = "ry.ryo3", frozen)]
pub struct PySocketAddrV6(pub(crate) SocketAddrV6);

#[pyclass(name = "SocketAddr", module = "ry.ryo3", frozen)]
pub struct PySocketAddr(pub(crate) SocketAddr);

#[pymethods]
impl PySocketAddrV4 {
    #[new]
    pub(crate) fn py_new(ip: IpAddrLike, port: u16) -> PyResult<Self> {
        let ipv4 = ip.get_ipv4()?;
        let sa = SocketAddrV4::new(ipv4, port);
        Ok(PySocketAddrV4(sa))
    }

    fn __repr__(&self) -> String {
        format!("SocketAddrV4({}, {})", self.0.ip(), self.port())
    }

    fn __richcmp__(&self, other: &PySocketAddrV4, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
            pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
            pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
            pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn to_ipaddrv4(&self) -> PyIpv4Addr {
        PyIpv4Addr::from(self.0.ip())
    }

    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    fn to_pyipaddress(&self) -> Ipv4Addr {
        *self.0.ip()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        // split on ':' and parse the first part as an IPv4 address and then
        // second as port yay
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid SocketAddrV4 format, expected 'ip:port'",
            ));
        }
        let ip_part = parts[0];
        let port_part = parts[1];
        let ip = Ipv4Addr::from_str(ip_part)
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv4 address"))?;
        let port = port_part
            .parse::<u16>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid port number"))?;
        Ok(PySocketAddrV4(SocketAddrV4::new(ip, port)))
    }

    #[getter]
    fn version(&self) -> u8 {
        4
    }

    // ========================================================================
    // Ipv4 forwarded
    // ========================================================================
    #[expect(clippy::unused_self)]
    #[getter]
    fn is_benchmarking(&self) -> PyResult<bool> {
        err_py_not_impl!()
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
        err_py_not_impl!()
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
        err_py_not_impl!()
    }

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_shared(&self) -> PyResult<bool> {
        err_py_not_impl!()
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
    fn py_new(ip: IpAddrLike, port: u16, flowinfo: u32, scope_id: u32) -> PyResult<Self> {
        let ipv6 = ip.get_ipv6()?;
        let sa = SocketAddrV6::new(ipv6, port, flowinfo, scope_id);
        Ok(PySocketAddrV6(sa))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("SocketAddrV6({}, {})", self.0.ip(), self.port())
    }

    fn __richcmp__(&self, other: &PySocketAddrV6, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
            pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
            pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
            pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn to_pyipaddress(&self) -> Ipv6Addr {
        *self.0.ip()
    }

    fn to_ipaddrv6(&self) -> PyIpv6Addr {
        PyIpv6Addr::from(self.0.ip())
    }

    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(IpAddr::V6(*self.0.ip()))
    }

    #[getter]
    fn port(&self) -> u16 {
        self.0.port()
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        // split on ':' and parse the first part as an IPv6 address and then
        // second as port yay
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid SocketAddrV6 format, expected 'ip:port'",
            ));
        }
        let ip_part = parts[0];
        let port_part = parts[1];
        let ip = Ipv6Addr::from_str(ip_part)
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv6 address"))?;
        let port = port_part
            .parse::<u16>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid port number"))?;
        Ok(PySocketAddrV6(SocketAddrV6::new(ip, port, 0, 0)))
    }

    #[getter]
    fn version(&self) -> u8 {
        6
    }

    // ========================================================================
    // IpAddr forwarded
    // ========================================================================
    #[getter]
    #[expect(clippy::unused_self)]
    fn is_benchmarking(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_documentation(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_global(&self) -> PyResult<bool> {
        err_py_not_impl!()
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
    #[expect(clippy::unused_self)]
    fn is_ipv4_mapped(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_unicast(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    #[expect(clippy::unused_self)]
    fn is_unicast_global(&self) -> PyResult<bool> {
        err_py_not_impl!()
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
                Ok(PySocketAddr(sa))
            }
            IpAddr::V6(ipv6) => {
                let flowinfo = flowinfo.unwrap_or(0);
                let scope_id = scope_id.unwrap_or(0);
                let sa = SocketAddr::V6(SocketAddrV6::new(ipv6, port, flowinfo, scope_id));
                Ok(PySocketAddr(sa))
            }
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("SocketAddr({}, {})", self.0.ip(), self.port())
    }

    fn __richcmp__(&self, other: &PySocketAddr, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
            pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
            pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
            pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid SocketAddr format, expected 'ip:port'",
            ));
        }
        let ip_part = parts[0];
        let port_part = parts[1];
        let ip = IpAddr::from_str(ip_part)
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IP address"))?;
        let port = port_part
            .parse::<u16>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid port number"))?;
        Ok(PySocketAddr(SocketAddr::new(ip, port)))
    }

    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0.ip())
    }

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
        err_py_not_impl!()
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
}
