// #![expect(clippy::trivially_copy_pass_by_ref)]
use crate::net::{PySocketAddrV4, PySocketAddrV6, ipaddr_props::IpAddrProps};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "Ipv4Addr", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpv4Addr(pub Ipv4Addr);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "Ipv6Addr", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpv6Addr(pub Ipv6Addr);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "IpAddr", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpAddr(pub IpAddr);

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl PyIpv4Addr {
    #[new]
    #[pyo3(
        signature = (a, b=None, c=None, d=None),
    )]
    fn py_new(a: &Bound<'_, PyAny>, b: Option<u8>, c: Option<u8>, d: Option<u8>) -> PyResult<Self> {
        extract_ipv4(a, b, c, d).map(Self)
    }

    #[must_use]
    pub(crate) fn __repr__(&self) -> String {
        format!("Ipv4Addr('{}')", self.0)
    }

    #[must_use]
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }

    fn __lt__(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn BROADCAST() -> Self {
        Self(Ipv4Addr::BROADCAST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST() -> Self {
        Self(Ipv4Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED() -> Self {
        Self(Ipv4Addr::UNSPECIFIED)
    }

    #[classattr]
    fn version() -> u8 {
        4
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn is_benchmarking(&self) -> bool {
        <Self as IpAddrProps>::is_benchmarking(self)
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        <Self as IpAddrProps>::is_broadcast(self)
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        <Self as IpAddrProps>::is_documentation(self)
    }

    #[getter]
    fn is_global(&self) -> PyResult<bool> {
        <Self as IpAddrProps>::is_global(self)
    }

    #[getter]
    fn is_link_local(&self) -> bool {
        <Self as IpAddrProps>::is_link_local(self)
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        <Self as IpAddrProps>::is_loopback(self)
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        <Self as IpAddrProps>::is_multicast(self)
    }

    #[getter]
    fn is_private(&self) -> bool {
        <Self as IpAddrProps>::is_private(self)
    }

    #[getter]
    fn is_reserved(&self) -> bool {
        <Self as IpAddrProps>::is_reserved(self)
    }

    #[getter]
    fn is_shared(&self) -> bool {
        <Self as IpAddrProps>::is_shared(self)
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        <Self as IpAddrProps>::is_unspecified(self)
    }

    #[getter]
    fn is_unicast(&self) -> bool {
        <Self as IpAddrProps>::is_unicast(self)
    }

    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================

    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Ipv4Addr {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> Ipv4Addr {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v4(&self, port: u16) -> PySocketAddrV4 {
        PySocketAddrV4::from(SocketAddrV4::new(self.0, port))
    }

    #[pyo3(signature = (port, flowinfo = 0, scope_id = 0))]
    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v6(&self, port: u16, flowinfo: u32, scope_id: u32) -> PySocketAddrV6 {
        // IPv4 addresses can be converted to IPv6-mapped addresses
        let ipv6_mapped = self.0.to_ipv6_mapped();
        PySocketAddrV6::from(SocketAddrV6::new(ipv6_mapped, port, flowinfo, scope_id))
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        s.parse::<std::net::Ipv4Addr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv4 address"))
            .map(Self)
    }

    #[staticmethod]
    fn from_bits(s: u32) -> Self {
        Self(std::net::Ipv4Addr::from(s))
    }

    #[staticmethod]
    fn from_octets(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(std::net::Ipv4Addr::new(a, b, c, d))
    }
}

#[pymethods]
impl PyIpv6Addr {
    #[new]
    fn py_new(a: &Bound<'_, PyAny>) -> PyResult<Self> {
        extract_ipv6_from_single_ob(a).map(Self)
    }

    #[must_use]
    pub(crate) fn __repr__(&self) -> String {
        format!("Ipv6Addr('{}')", self.0)
    }

    #[must_use]
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================
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

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST() -> Self {
        Self(Ipv6Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED() -> Self {
        Self(Ipv6Addr::UNSPECIFIED)
    }

    #[classattr]
    fn version() -> u8 {
        6
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn is_documentation(&self) -> bool {
        <Self as IpAddrProps>::is_documentation(self)
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        <Self as IpAddrProps>::is_loopback(self)
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        <Self as IpAddrProps>::is_multicast(self)
    }

    #[getter]
    pub(crate) fn is_ipv4_mapped(&self) -> bool {
        <Self as IpAddrProps>::is_ipv4_mapped(self)
    }

    #[getter]
    fn is_unicast(&self) -> bool {
        <Self as IpAddrProps>::is_unicast(self)
    }

    #[getter]
    fn is_unicast_global(&self) -> bool {
        <Self as IpAddrProps>::is_unicast_global(self)
    }

    #[getter]
    fn is_unicast_link_local(&self) -> bool {
        <Self as IpAddrProps>::is_unicast_link_local(self)
    }

    #[getter]
    fn is_unique_local(&self) -> bool {
        <Self as IpAddrProps>::is_unique_local(self)
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        <Self as IpAddrProps>::is_unspecified(self)
    }

    // ----------------------------------------------------------------------
    // unstable properties
    // ----------------------------------------------------------------------

    #[getter]
    fn is_benchmarking(&self) -> bool {
        <Self as IpAddrProps>::is_benchmarking(self)
    }

    #[getter]
    fn is_global(&self) -> PyResult<bool> {
        <Self as IpAddrProps>::is_global(self)
    }

    #[getter]
    fn is_reserved(&self) -> bool {
        <Self as IpAddrProps>::is_reserved(self)
    }

    #[getter]
    fn is_shared(&self) -> bool {
        <Self as IpAddrProps>::is_shared(self)
    }

    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================

    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> Ipv6Addr {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> Ipv6Addr {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v4(&self, port: u16) -> PyResult<PySocketAddrV4> {
        if let Some(addr) = self.0.to_ipv4() {
            Ok(PySocketAddrV4::from(SocketAddrV4::new(addr, port)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Cannot convert IPv6 address to IPv4; address is not IPv4-mapped",
            ))
        }
    }

    #[pyo3(signature = (port, flowinfo = 0, scope_id = 0))]
    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v6(&self, port: u16, flowinfo: u32, scope_id: u32) -> PySocketAddrV6 {
        PySocketAddrV6::from(SocketAddrV6::new(self.0, port, flowinfo, scope_id))
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        s.parse::<std::net::Ipv6Addr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv6 address"))
            .map(Self)
    }

    #[staticmethod]
    fn from_bits(s: u128) -> Self {
        Self(std::net::Ipv6Addr::from(s))
    }
}

#[pymethods]
impl PyIpAddr {
    #[new]
    fn py_new(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(ipv4) = extract_ipv4_from_single_ob(ob) {
            return Ok(Self(IpAddr::V4(ipv4)));
        }
        if let Ok(ipv6) = extract_ipv6_from_single_ob(ob) {
            return Ok(Self(IpAddr::V6(ipv6)));
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid IP address",
        ))
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("IpAddr('{}')", self.0)
    }

    #[must_use]
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================
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

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn BROADCAST() -> Self {
        Self::from(Ipv4Addr::BROADCAST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST_V4() -> Self {
        Self::from(Ipv4Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED_V4() -> Self {
        Self::from(Ipv4Addr::UNSPECIFIED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST_V6() -> Self {
        Self::from(Ipv6Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED_V6() -> Self {
        Self::from(Ipv6Addr::UNSPECIFIED)
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn version(&self) -> u8 {
        match self.0 {
            IpAddr::V4(_) => 4,
            IpAddr::V6(_) => 6,
        }
    }

    // ---------------------------------------------------------------------
    // RUST PROPERTIES
    // ---------------------------------------------------------------------
    #[getter]
    fn is_ipv4(&self) -> bool {
        <Self as IpAddrProps>::is_ipv4(self)
    }

    #[getter]
    fn is_ipv6(&self) -> bool {
        <Self as IpAddrProps>::is_ipv6(self)
    }

    #[getter]
    fn is_benchmarking(&self) -> bool {
        <Self as IpAddrProps>::is_benchmarking(self)
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        <Self as IpAddrProps>::is_broadcast(self)
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        <Self as IpAddrProps>::is_documentation(self)
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        <Self as IpAddrProps>::is_multicast(self)
    }

    #[getter]
    fn is_private(&self) -> bool {
        <Self as IpAddrProps>::is_private(self)
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        <Self as IpAddrProps>::is_unspecified(self)
    }

    #[getter]
    fn is_global(&self) -> PyResult<bool> {
        <Self as IpAddrProps>::is_global(self)
    }

    #[getter]
    fn is_ipv4_mapped(&self) -> bool {
        <Self as IpAddrProps>::is_ipv4_mapped(self)
    }

    #[getter]
    fn is_link_local(&self) -> bool {
        <Self as IpAddrProps>::is_link_local(self)
    }

    #[getter]
    fn is_reserved(&self) -> bool {
        <Self as IpAddrProps>::is_reserved(self)
    }

    #[getter]
    fn is_shared(&self) -> bool {
        <Self as IpAddrProps>::is_shared(self)
    }

    #[getter]
    fn is_unicast(&self) -> bool {
        <Self as IpAddrProps>::is_unicast(self)
    }

    #[getter]
    fn is_unicast_global(&self) -> bool {
        <Self as IpAddrProps>::is_unicast_global(self)
    }

    #[getter]
    fn is_unicast_link_local(&self) -> bool {
        <Self as IpAddrProps>::is_unicast_link_local(self)
    }

    #[getter]
    fn is_unique_local(&self) -> bool {
        <Self as IpAddrProps>::is_unique_local(self)
    }

    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================
    #[expect(clippy::wrong_self_convention)]
    fn to_py(&self) -> IpAddr {
        self.0
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> IpAddr {
        self.0
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        s.parse::<std::net::IpAddr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IP address"))
            .map(Self)
    }

    // ========================================================================
    // METHODS
    // ========================================================================
    #[expect(clippy::wrong_self_convention)]
    fn to_canonical(&self) -> Self {
        Self::from(self.0.to_canonical())
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipv4(&self) -> PyResult<PyIpv4Addr> {
        match self.0 {
            IpAddr::V4(addr) => Ok(PyIpv4Addr::from(addr)),
            IpAddr::V6(addr) => {
                if let Some(addr) = addr.to_ipv4() {
                    Ok(PyIpv4Addr::from(addr))
                } else {
                    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Cannot convert IPv6 address to IPv4; address is not IPv4-mapped",
                    ))
                }
            }
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipv6(&self) -> PyIpv6Addr {
        match self.0 {
            IpAddr::V4(addr) => PyIpv6Addr::from(addr.to_ipv6_mapped()),
            IpAddr::V6(addr) => PyIpv6Addr::from(addr),
        }
    }
}

// ========================================================================
// IpAddrLike
// ========================================================================
#[derive(FromPyObject, Clone, Debug)]
pub(crate) enum IpAddrLike {
    Ryv4(PyIpv4Addr),
    Ryv6(PyIpv6Addr),
    Ry(PyIpAddr),
    Str(String),
    Py(IpAddr),
}

impl IpAddrLike {
    pub(crate) fn get_ipv4(&self) -> PyResult<Ipv4Addr> {
        match self {
            Self::Ryv4(addr) => Ok(addr.0),
            Self::Ry(addr) => match addr.0 {
                IpAddr::V4(addr) => Ok(addr),
                IpAddr::V6(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                    "Expected an IPv4 address",
                )),
            },
            Self::Py(addr) => match addr {
                IpAddr::V4(addr) => Ok(*addr),
                IpAddr::V6(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                    "Expected an IPv4 address",
                )),
            },
            Self::Str(s) => s.parse().map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err("Expected a valid IPv4 address string")
            }),
            Self::Ryv6(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected an IPv4 address",
            )),
        }
    }

    pub(crate) fn get_ipv6(&self) -> PyResult<Ipv6Addr> {
        match self {
            Self::Ryv6(addr) => Ok(addr.0),
            Self::Ry(addr) => match addr.0 {
                IpAddr::V6(addr) => Ok(addr),
                IpAddr::V4(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                    "Expected an IPv6 address",
                )),
            },
            Self::Py(addr) => match addr {
                IpAddr::V6(addr) => Ok(*addr),
                IpAddr::V4(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                    "Expected an IPv6 address",
                )),
            },
            Self::Str(s) => s.parse().map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err("Expected a valid IPv6 address string")
            }),
            Self::Ryv4(_) => Err(pyo3::exceptions::PyTypeError::new_err(
                "Expected an IPv6 address",
            )),
        }
    }

    pub(crate) fn get_ip(&self) -> PyResult<IpAddr> {
        match self {
            Self::Ryv4(addr) => Ok(IpAddr::V4(addr.0)),
            Self::Ryv6(addr) => Ok(IpAddr::V6(addr.0)),
            Self::Ry(addr) => Ok(addr.0),
            Self::Py(addr) => Ok(*addr),
            Self::Str(s) => s.parse().map_err(|_| {
                pyo3::exceptions::PyTypeError::new_err("Expected a valid IP address string")
            }),
        }
    }
}

// ============================================================================
// UTILS
// ============================================================================

static IPV4_ADDR_ERROR: &str =
    "Invalid IPv4 address, should be a [u8; 4], u32, str, bytes (len=4), or ipaddress.IPv4Address";

fn extract_ipv4_from_single_ob(ob: &Bound<'_, PyAny>) -> PyResult<Ipv4Addr> {
    // 32 bit fitting int
    if let Ok(addr) = ob.extract::<u32>() {
        return Ok(std::net::Ipv4Addr::from(addr));
    }

    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr.parse().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid IPv4 address: {e}"))
        });
    }

    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 4]>() {
        return Ok(std::net::Ipv4Addr::from(addr));
    }

    if let Ok(IpAddr::V4(addr)) = ob.extract::<IpAddr>() {
        return Ok(addr);
    }
    // error
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        IPV4_ADDR_ERROR,
    ))
}

fn extract_ipv4(
    a: &Bound<'_, PyAny>,
    b: Option<u8>,
    c: Option<u8>,
    d: Option<u8>,
) -> PyResult<Ipv4Addr> {
    // if bcd are not None then extract a as u8 or error...
    match (b, c, d) {
        (Some(b), Some(c), Some(d)) => {
            if let Ok(addr) = a.extract::<u8>() {
                return Ok(Ipv4Addr::new(addr, b, c, d));
            }
        }
        (None, None, None) => {
            if let Ok(addr) = extract_ipv4_from_single_ob(a) {
                return Ok(addr);
            }
        }
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                IPV4_ADDR_ERROR,
            ));
        }
    }
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        IPV4_ADDR_ERROR,
    ))
}

static IPV6_ADDR_ERROR: &str =
    "Invalid IPv4 address, should be a [u8; 16], u128, str, bytes or ipaddress.IPv6Address";

fn extract_ipv6_from_single_ob(ob: &Bound<'_, PyAny>) -> PyResult<Ipv6Addr> {
    // 32 bit fitting int
    if let Ok(addr) = ob.extract::<u128>() {
        return Ok(std::net::Ipv6Addr::from(addr));
    }

    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr.parse().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid IPv6 address: {e}"))
        });
    }

    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 16]>() {
        return Ok(std::net::Ipv6Addr::from(addr));
    }

    if let Ok(IpAddr::V6(addr)) = ob.extract::<IpAddr>() {
        return Ok(addr);
    }

    // error
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        IPV6_ADDR_ERROR,
    ))
}
