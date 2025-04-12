use pyo3::prelude::*;
use pyo3::types::PyType;
use ryo3_macros::err_py_not_impl;
use std::net::{Ipv4Addr, Ipv6Addr};

#[pyclass(name = "Ipv4Addr", module = "ry.ryo3")]
pub struct PyIpv4Addr(pub std::net::Ipv4Addr);

#[pyclass(name = "Ipv6Addr", module = "ry.ryo3")]
pub struct PyIpv6Addr(pub std::net::Ipv6Addr);

// ===========================================================================
// standard FROM impls
// ===========================================================================

impl From<std::net::Ipv6Addr> for PyIpv6Addr {
    fn from(addr: std::net::Ipv6Addr) -> Self {
        PyIpv6Addr(addr)
    }
}

impl From<PyIpv6Addr> for std::net::Ipv6Addr {
    fn from(addr: PyIpv6Addr) -> Self {
        addr.0
    }
}
impl From<std::net::Ipv4Addr> for PyIpv4Addr {
    fn from(addr: std::net::Ipv4Addr) -> Self {
        PyIpv4Addr(addr)
    }
}

impl From<PyIpv4Addr> for std::net::Ipv4Addr {
    fn from(addr: PyIpv4Addr) -> Self {
        addr.0
    }
}

// impl<'py> FromPyObject<'py> for JiffRoundMode {
//     fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//         if let Ok(str_mode) = ob.extract::<&str>() {
//             match str_mode.to_ascii_lowercase().replace('_', "-").as_str() {
//                 "ceil" => Ok(Self(RoundMode::Ceil)),
//                 "floor" => Ok(Self(RoundMode::Floor)),
//                 "expand" => Ok(Self(RoundMode::Expand)),
//                 "trunc" => Ok(Self(RoundMode::Trunc)),
//                 "half-ceil" => Ok(Self(RoundMode::HalfCeil)),
//                 "half-floor" => Ok(Self(RoundMode::HalfFloor)),
//                 "half-expand" => Ok(Self(RoundMode::HalfExpand)),
//                 "half-trunc" => Ok(Self(RoundMode::HalfTrunc)),
//                 "half-even" => Ok(Self(RoundMode::HalfEven)),
//                 _ => Err(PyValueError::new_err(JIFF_ROUND_MODE_ERROR)),
//             }
//         } else {
//             Err(PyValueError::new_err(JIFF_ROUND_MODE_ERROR))
//         }
//     }
// }

// #[derive(FromPyObject)]
// pub enum Ipv4NewArgs{
//     /// u8 parts
//     Parts(u8, u8, u8, u8),
//     /// string
//     Str(String),
//     /// bytes
//     Bytes(Vec<u8>),
//     ///

// }

static IPV4_ADDR_ERROR: &str = "Invalid IPv4 address, should be a u8, u32, str or bytes";

fn extract_ipv4_from_single_ob(ob: &Bound<'_, PyAny>) -> PyResult<Ipv4Addr> {
    // 32 bit fitting int
    if let Ok(addr) = ob.extract::<u32>() {
        return Ok(Ipv4Addr::from(addr));
    }
    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr
            .parse()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv4 address"));
    }
    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 4]>() {
        return Ok(Ipv4Addr::from(addr));
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

static IPV6_ADDR_ERROR: &str = "Invalid IPv4 address, should be a [u8;16], u128, str or bytes";

fn extract_ipv6_from_single_ob(ob: &Bound<'_, PyAny>) -> PyResult<Ipv6Addr> {
    // 32 bit fitting int
    if let Ok(addr) = ob.extract::<u128>() {
        return Ok(Ipv6Addr::from(addr));
    }
    // if is string then parse
    if let Ok(addr) = ob.extract::<&str>() {
        return addr
            .parse()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid IPv4 address"));
    }
    // if is bytes then parse
    if let Ok(addr) = ob.extract::<[u8; 16]>() {
        return Ok(Ipv6Addr::from(addr));
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
