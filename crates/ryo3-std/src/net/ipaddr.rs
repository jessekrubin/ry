use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::types::PyType;
use ryo3_macro_rules::err_py_not_impl;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[pyclass(name = "Ipv4Addr", module = "ry.ryo3", frozen)]
pub struct PyIpv4Addr(pub std::net::Ipv4Addr);

#[pyclass(name = "Ipv6Addr", module = "ry.ryo3", frozen)]
pub struct PyIpv6Addr(pub std::net::Ipv6Addr);

#[pyclass(name = "IpAddr", module = "ry.ryo3", frozen)]
pub struct PyIpAddr(pub std::net::IpAddr);

static IPV4_ADDR_ERROR: &str =
    "Invalid IPv4 address, should be a [u8; 4], u32, str, bytes (len=4), or ipaddress.IPv4Address";

fn extract_ipv4_from_single_ob(ob: &Bound<'_, PyAny>) -> PyResult<Ipv4Addr> {
    // 32 bit fitting int
    if let Ok(addr) = ob.extract::<u32>() {
        return Ok(Ipv4Addr::from(addr));
    }

    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr.parse().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid IPv4 address: {e}"))
        });
    }

    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 4]>() {
        return Ok(Ipv4Addr::from(addr));
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
        return Ok(Ipv6Addr::from(addr));
    }
    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr.parse().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid IPv6 address: {e}"))
        });
    }
    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 16]>() {
        return Ok(Ipv6Addr::from(addr));
    }

    if let Ok(IpAddr::V6(addr)) = ob.extract::<IpAddr>() {
        return Ok(addr);
    }
    // error
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        IPV6_ADDR_ERROR,
    ))
}

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
    pub fn __repr__(&self) -> String {
        format!("Ipv4Addr('{}')", self.0)
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================

    fn __eq__(&self, other: &PyIpv4Addr) -> bool {
        self.0 == other.0
    }
    fn __ne__(&self, other: &PyIpv4Addr) -> bool {
        self.0 != other.0
    }
    fn __lt__(&self, other: &PyIpv4Addr) -> bool {
        self.0 < other.0
    }
    fn __le__(&self, other: &PyIpv4Addr) -> bool {
        self.0 <= other.0
    }
    fn __gt__(&self, other: &PyIpv4Addr) -> bool {
        self.0 > other.0
    }
    fn __ge__(&self, other: &PyIpv4Addr) -> bool {
        self.0 >= other.0
    }

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn BROADCAST() -> Self {
        Self(std::net::Ipv4Addr::BROADCAST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST() -> Self {
        Self(std::net::Ipv4Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED() -> Self {
        Self(std::net::Ipv4Addr::UNSPECIFIED)
    }

    #[classattr]
    fn version() -> u8 {
        4
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_benchmarking(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        self.0.is_broadcast()
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        self.0.is_documentation()
    }

    #[expect(clippy::unused_self)]
    #[getter]
    fn is_global(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    fn is_link_local(&self) -> bool {
        self.0.is_link_local()
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.is_multicast()
    }

    #[getter]
    fn is_private(&self) -> bool {
        self.0.is_private()
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
        self.0.is_unspecified()
    }
    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================

    fn to_py(&self) -> std::net::Ipv4Addr {
        self.0
    }

    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<Ipv4Addr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv4 address"))
            .map(Self)
    }

    #[classmethod]
    fn from_bits(_cls: &Bound<'_, PyType>, s: u32) -> Self {
        Self(Ipv4Addr::from(s))
    }

    #[classmethod]
    fn from_octets(_cls: &Bound<'_, PyType>, a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(Ipv4Addr::new(a, b, c, d))
    }
}

#[pymethods]
impl PyIpv6Addr {
    #[new]
    fn py_new(a: &Bound<'_, PyAny>) -> PyResult<Self> {
        extract_ipv6_from_single_ob(a).map(Self)
    }

    #[must_use]
    pub fn __repr__(&self) -> String {
        format!("Ipv6Addr('{}')", self.0)
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================

    fn __eq__(&self, other: &PyIpv6Addr) -> bool {
        self.0 == other.0
    }
    fn __ne__(&self, other: &PyIpv6Addr) -> bool {
        self.0 != other.0
    }
    fn __lt__(&self, other: &PyIpv6Addr) -> bool {
        self.0 < other.0
    }
    fn __le__(&self, other: &PyIpv6Addr) -> bool {
        self.0 <= other.0
    }
    fn __gt__(&self, other: &PyIpv6Addr) -> bool {
        self.0 > other.0
    }
    fn __ge__(&self, other: &PyIpv6Addr) -> bool {
        self.0 >= other.0
    }

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST() -> Self {
        Self(std::net::Ipv6Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED() -> Self {
        Self(std::net::Ipv6Addr::UNSPECIFIED)
    }

    #[classattr]
    fn version() -> u8 {
        6
    }

    // ========================================================================
    // PROPERTIES
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
        self.0.is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.is_multicast()
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
        self.0.is_unicast_link_local()
    }

    #[getter]
    fn is_unique_local(&self) -> bool {
        self.0.is_unique_local()
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        self.0.is_unspecified()
    }
    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================

    fn to_py(&self) -> std::net::Ipv6Addr {
        self.0
    }

    fn to_ipaddr(&self) -> PyIpAddr {
        PyIpAddr::from(self.0)
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<Ipv6Addr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv6 address"))
            .map(Self)
    }

    #[classmethod]
    fn from_bits(_cls: &Bound<'_, PyType>, s: u128) -> Self {
        Self(Ipv6Addr::from(s))
    }
}

#[pymethods]
impl PyIpAddr {
    #[new]
    fn py_new(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(ipv4) = extract_ipv4_from_single_ob(ob) {
            return Ok(Self(std::net::IpAddr::V4(ipv4)));
        }
        if let Ok(ipv6) = extract_ipv6_from_single_ob(ob) {
            return Ok(Self(std::net::IpAddr::V6(ipv6)));
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid IP address",
        ))
    }

    #[must_use]
    pub fn __repr__(&self) -> String {
        format!("IpAddr('{}')", self.0)
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.0.to_string();
        PyTuple::new(py, &[str])
    }

    // ========================================================================
    // CMP
    // ========================================================================

    fn __eq__(&self, other: &PyIpAddr) -> bool {
        self.0 == other.0
    }
    fn __ne__(&self, other: &PyIpAddr) -> bool {
        self.0 != other.0
    }
    fn __lt__(&self, other: &PyIpAddr) -> bool {
        self.0 < other.0
    }
    fn __le__(&self, other: &PyIpAddr) -> bool {
        self.0 <= other.0
    }
    fn __gt__(&self, other: &PyIpAddr) -> bool {
        self.0 > other.0
    }
    fn __ge__(&self, other: &PyIpAddr) -> bool {
        self.0 >= other.0
    }

    // ========================================================================
    // CONSTANTS
    // ========================================================================
    #[expect(non_snake_case)]
    #[classattr]
    fn BROADCAST() -> Self {
        Self::from(std::net::Ipv4Addr::BROADCAST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST_V4() -> Self {
        Self::from(std::net::Ipv4Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED_V4() -> Self {
        Self::from(std::net::Ipv4Addr::UNSPECIFIED)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn LOCALHOST_V6() -> Self {
        Self::from(std::net::Ipv6Addr::LOCALHOST)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UNSPECIFIED_V6() -> Self {
        Self::from(std::net::Ipv6Addr::UNSPECIFIED)
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn version(&self) -> u8 {
        match self.0 {
            std::net::IpAddr::V4(_) => 4,
            std::net::IpAddr::V6(_) => 6,
        }
    }

    // ---------------------------------------------------------------------
    // RUST PROPERTIES
    // ---------------------------------------------------------------------
    #[expect(clippy::unused_self)]
    #[getter]
    fn is_benchmarking(&self) -> PyResult<bool> {
        err_py_not_impl!()
    }

    #[getter]
    fn is_ipv4(&self) -> bool {
        self.0.is_ipv4()
    }

    #[getter]
    fn is_ipv6(&self) -> bool {
        self.0.is_ipv6()
    }

    #[getter]
    fn is_broadcast(&self) -> bool {
        match self.0 {
            std::net::IpAddr::V4(addr) => addr.is_broadcast(),
            std::net::IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_documentation(&self) -> bool {
        match self.0 {
            std::net::IpAddr::V4(addr) => addr.is_documentation(),
            std::net::IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_loopback(&self) -> bool {
        self.0.is_loopback()
    }

    #[getter]
    fn is_multicast(&self) -> bool {
        self.0.is_multicast()
    }

    #[getter]
    fn is_private(&self) -> bool {
        match self.0 {
            std::net::IpAddr::V4(addr) => addr.is_private(),
            std::net::IpAddr::V6(_) => false,
        }
    }

    #[getter]
    fn is_unspecified(&self) -> bool {
        self.0.is_unspecified()
    }

    // ========================================================================
    // PY-CONVERSIONS
    // ========================================================================
    fn to_py(&self) -> std::net::IpAddr {
        self.0
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        s.parse::<IpAddr>()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IP address"))
            .map(Self)
    }

    // ========================================================================
    // METHODS
    // ========================================================================
    fn to_canonical(&self) -> Self {
        Self::from(self.0.to_canonical())
    }

    fn to_ipv4(&self) -> PyResult<PyIpv4Addr> {
        match self.0 {
            std::net::IpAddr::V4(addr) => Ok(PyIpv4Addr::from(addr)),
            std::net::IpAddr::V6(addr) => {
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

    fn to_ipv6(&self) -> PyIpv6Addr {
        match self.0 {
            std::net::IpAddr::V4(addr) => PyIpv6Addr::from(addr.to_ipv6_mapped()),
            std::net::IpAddr::V6(addr) => PyIpv6Addr::from(addr),
        }
    }
}

// ===========================================================================
// FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ===========================================================================
impl From<Ipv4Addr> for PyIpv4Addr {
    fn from(addr: Ipv4Addr) -> Self {
        PyIpv4Addr(addr)
    }
}

impl From<PyIpv4Addr> for Ipv4Addr {
    fn from(addr: PyIpv4Addr) -> Self {
        addr.0
    }
}

impl From<Ipv6Addr> for PyIpv6Addr {
    fn from(addr: Ipv6Addr) -> Self {
        PyIpv6Addr(addr)
    }
}

impl From<PyIpv6Addr> for Ipv6Addr {
    fn from(addr: PyIpv6Addr) -> Self {
        addr.0
    }
}

impl From<IpAddr> for PyIpAddr {
    fn from(addr: IpAddr) -> Self {
        PyIpAddr(addr)
    }
}

impl From<Ipv4Addr> for PyIpAddr {
    fn from(addr: Ipv4Addr) -> Self {
        PyIpAddr(IpAddr::V4(addr))
    }
}

impl From<Ipv6Addr> for PyIpAddr {
    fn from(addr: Ipv6Addr) -> Self {
        PyIpAddr(IpAddr::V6(addr))
    }
}

impl From<PyIpAddr> for IpAddr {
    fn from(addr: PyIpAddr) -> Self {
        addr.0
    }
}
