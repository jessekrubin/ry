// #![expect(clippy::trivially_copy_pass_by_ref)]
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use pyo3::BoundObject;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_core::{PyAsciiString, PyFromStr, PyParse, py_value_error};
use ryo3_macro_rules::{any_repr, py_type_err, py_type_error};

use crate::net::ipaddr_props::IpAddrProps;
use crate::net::{PySocketAddrV4, PySocketAddrV6};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "Ipv4Addr", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpv4Addr(pub Ipv4Addr);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "Ipv6Addr", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpv6Addr(pub Ipv6Addr);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "IpAddr", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PyIpAddr(pub IpAddr);

#[expect(clippy::trivially_copy_pass_by_ref)]
#[pymethods]
impl PyIpv4Addr {
    #[new]
    #[pyo3(signature = (*args))]
    fn py_new(args: Ipv4Args) -> PyResult<Self> {
        Ok(Self(args.0))
    }

    #[must_use]
    pub(crate) fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    #[must_use]
    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> PyAsciiString {
        self.__str__()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.py_to_string();
        PyTuple::new(py, &[str])
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
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
    fn to_socketaddrv4(&self, port: u16) -> PySocketAddrV4 {
        PySocketAddrV4::from(SocketAddrV4::new(self.0, port))
    }

    #[pyo3(signature = (port, flowinfo = 0, scope_id = 0))]
    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddrv6(&self, port: u16, flowinfo: u32, scope_id: u32) -> PySocketAddrV6 {
        // IPv4 addresses can be converted to IPv6-mapped addresses
        let ipv6_mapped = self.0.to_ipv6_mapped();
        PySocketAddrV6::from(SocketAddrV6::new(ipv6_mapped, port, flowinfo, scope_id))
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
    }

    #[staticmethod]
    fn from_bits(bits: u32) -> Self {
        Self(std::net::Ipv4Addr::from(bits))
    }

    #[staticmethod]
    fn from_octets(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self(std::net::Ipv4Addr::new(a, b, c, d))
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(ip) = value.extract::<Ipv4Like>() {
            Self::from(ip.0).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Ipv4Addr conversion error: {valtype}")
        }
    }

    // ------------------------------------------------------------------------
    // pydantic
    // ------------------------------------------------------------------------
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::map_py_value_err;
        Self::from_any(value).map_err(map_py_value_err)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_json_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticJsonSchemaCls;
        Self::get_pydantic_json_schema(cls, source, handler)
    }
}

#[pymethods]
impl PyIpv6Addr {
    #[new]
    #[pyo3(signature = (*args))]
    fn py_new(args: Ipv6Args) -> Self {
        Self(args.0)
    }

    #[must_use]
    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    #[must_use]
    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> PyAsciiString {
        self.__str__()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.py_to_string();
        PyTuple::new(py, &[str])
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
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
    fn is_ipv4(&self) -> bool {
        <Self as IpAddrProps>::is_ipv4(self)
    }

    #[getter]
    fn is_ipv6(&self) -> bool {
        <Self as IpAddrProps>::is_ipv6(self)
    }

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
    fn to_socketaddrv4(&self, port: u16) -> PyResult<PySocketAddrV4> {
        if let Some(addr) = self.0.to_ipv4() {
            Ok(PySocketAddrV4::from(SocketAddrV4::new(addr, port)))
        } else {
            py_type_err!("Cannot convert IPv6 address to IPv4; address is not IPv4-mapped")
        }
    }

    #[pyo3(signature = (port, flowinfo = 0, scope_id = 0))]
    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddrv6(&self, port: u16, flowinfo: u32, scope_id: u32) -> PySocketAddrV6 {
        PySocketAddrV6::from(SocketAddrV6::new(self.0, port, flowinfo, scope_id))
    }

    // ========================================================================
    // CLASSMETHODS
    // ========================================================================
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
    }

    #[staticmethod]
    fn from_bits(bits: u128) -> Self {
        Self(std::net::Ipv6Addr::from(bits))
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(ip) = value.extract::<Ipv6Like>() {
            Self::from(ip.0).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Ipv4Addr conversion error: {valtype}")
        }
    }

    // ------------------------------------------------------------------------
    // pydantic
    // ------------------------------------------------------------------------
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::map_py_value_err;
        Self::from_any(value).map_err(map_py_value_err)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_json_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticJsonSchemaCls;
        Self::get_pydantic_json_schema(cls, source, handler)
    }
}

#[pymethods]
impl PyIpAddr {
    #[new]
    fn py_new(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(ipv4) = ob.extract::<Ipv4Like>() {
            Ok(Self(IpAddr::V4(ipv4.0)))
        } else if let Ok(ipv6) = ob.extract::<Ipv6Like>() {
            Ok(Self(IpAddr::V6(ipv6.0)))
        } else {
            py_type_err!("Invalid IP address")
        }
    }

    #[must_use]
    fn __repr__(&self) -> PyAsciiString {
        format!("{self}").into()
    }

    #[must_use]
    fn __str__(&self) -> PyAsciiString {
        self.0.to_string().into()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> PyAsciiString {
        self.__str__()
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let str = self.py_to_string();
        PyTuple::new(py, &[str])
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
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
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
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
                    py_type_err!("Cannot convert IPv6 address to IPv4; address is not IPv4-mapped")
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

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(ip) = value.extract::<Ipv4Args>() {
            Self::from(IpAddr::V4(ip.0)).into_pyobject(py)
        } else if let Ok(ip) = value.extract::<Ipv6Args>() {
            Self::from(IpAddr::V6(ip.0)).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("Ipv4Addr conversion error: {valtype}")
        }
    }

    // ------------------------------------------------------------------------
    // pydantic
    // ------------------------------------------------------------------------

    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::map_py_value_err;
        Self::from_any(value).map_err(map_py_value_err)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_json_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticJsonSchemaCls;
        Self::get_pydantic_json_schema(cls, source, handler)
    }
}

// ========================================================================
// IpAddrLike
// ========================================================================
#[derive(Debug)]
pub(crate) enum IpAddrLike {
    Ryv4(PyIpv4Addr),
    Ryv6(PyIpv6Addr),
    Ry(PyIpAddr),
    Str(String),
    Py(IpAddr),
}

impl<'py> FromPyObject<'_, 'py> for IpAddrLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(addr) = obj.cast_exact::<PyIpv4Addr>() {
            Ok(Self::Ryv4(addr.get().0.into()))
        } else if let Ok(addr) = obj.cast_exact::<PyIpv6Addr>() {
            Ok(Self::Ryv6(addr.get().0.into()))
        } else if let Ok(addr) = obj.cast_exact::<PyIpAddr>() {
            Ok(Self::Ry(addr.get().0.into()))
        } else if let Ok(addr) = obj.extract::<IpAddr>() {
            Ok(Self::Py(addr))
        } else if let Ok(s) = obj.extract::<String>() {
            Ok(Self::Str(s))
        } else {
            let valtype = any_repr!(obj);

            py_type_err!("IpAddrLike conversion error: {valtype}")
        }
    }
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
            Self::Ryv4(_) => py_type_err!("Expected an IPv6 address"),
        }
    }

    pub(crate) fn get_ip(&self) -> PyResult<IpAddr> {
        match self {
            Self::Ryv4(addr) => Ok(IpAddr::V4(addr.0)),
            Self::Ryv6(addr) => Ok(IpAddr::V6(addr.0)),
            Self::Ry(addr) => Ok(addr.0),
            Self::Py(addr) => Ok(*addr),
            Self::Str(s) => s
                .parse()
                .map_err(|_| py_type_error!("Expected a valid IP address string")),
        }
    }
}

// ============================================================================
// DISPLAY
// ============================================================================
impl std::fmt::Display for PyIpv4Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ipv4Addr('{}')", self.0)
    }
}

impl std::fmt::Display for PyIpv6Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ipv6Addr('{}')", self.0)
    }
}

impl std::fmt::Display for PyIpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IpAddr('{}')", self.0)
    }
}

// ============================================================================
// UTILS
// ============================================================================

static IPV4_ADDR_ERROR: &str =
    "Invalid IPv4 address, should be a [u8; 4], u32, str, bytes (len=4), or ipaddress.IPv4Address";

struct Ipv4Like(Ipv4Addr);

impl<'py> FromPyObject<'_, 'py> for Ipv4Like {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(i) = obj.extract::<u32>() {
            return Ok(Self(Ipv4Addr::from(i)));
        } else if let Ok(s) = obj.extract::<&str>() {
            return s
                .parse()
                .map(Ipv4Like)
                .map_err(|e| py_value_error!("Invalid IPv4 address string: {e}"));
        } else if let Ok(bytes) = obj.extract::<[u8; 4]>() {
            return Ok(Self(Ipv4Addr::from(bytes)));
        } else if let Ok(IpAddr::V4(addr)) = obj.extract::<IpAddr>() {
            return Ok(Self(addr));
        } else {
            py_type_err!("{IPV4_ADDR_ERROR}")
        }
    }
}

struct Ipv4Args(Ipv4Addr);

impl<'py> FromPyObject<'_, 'py> for Ipv4Args {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(tup) = obj.cast_exact::<PyTuple>() {
            match tup.len() {
                4 => {
                    let (a, b, c, d) = tup
                        .extract::<(u8, u8, u8, u8)>()
                        .map_err(|_| py_type_error!("Expected four octets for IPv4 address"))?;
                    Ok(Self(Ipv4Addr::new(a, b, c, d)))
                }
                1 => {
                    let a = tup.get_item(0)?;
                    let innerip = a.extract::<Ipv4Like>()?;
                    Ok(Self(innerip.0))
                }
                _ => {
                    py_type_err!(
                        "expected either a single argument or four octets for IPv4 address"
                    )
                }
            }
        } else if let Ok(ipv4) = obj.extract::<Ipv4Like>() {
            Ok(Self(ipv4.0))
        } else {
            py_type_err!("{IPV4_ADDR_ERROR}")
        }
    }
}

struct Ipv6Like(Ipv6Addr);

impl<'py> FromPyObject<'_, 'py> for Ipv6Like {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(i) = obj.extract::<u128>() {
            return Ok(Self(Ipv6Addr::from(i)));
        } else if let Ok(s) = obj.extract::<&str>() {
            return s
                .parse()
                .map(Ipv6Like)
                .map_err(|e| py_value_error!("Invalid IPv6 address string: {e}"));
        } else if let Ok(bytes) = obj.extract::<[u8; 16]>() {
            return Ok(Self(Ipv6Addr::from(bytes)));
        } else if let Ok(IpAddr::V6(addr)) = obj.extract::<IpAddr>() {
            return Ok(Self(addr));
        } else {
            py_type_err!("{IPV6_ADDR_ERROR}")
        }
    }
}

struct Ipv6Args(Ipv6Addr);

impl<'py> FromPyObject<'_, 'py> for Ipv6Args {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(tup) = obj.cast_exact::<PyTuple>() {
            match tup.len() {
                8 => {
                    let (a, b, c, d, e, f, g, h) = tup
                        .extract::<(u16, u16, u16, u16, u16, u16, u16, u16)>()
                        .map_err(|_| {
                            py_type_error!("Expected eight u16 segments for IPv6 address")
                        })?;
                    Ok(Self(Ipv6Addr::new(a, b, c, d, e, f, g, h)))
                }
                16 => {
                    let octets = tup
                        .extract::<[u8; 16]>()
                        .map_err(|_| py_type_error!("Expected 16 octets for IPv6 address"))?;
                    Ok(Self(Ipv6Addr::from(octets)))
                }
                1 => {
                    let a = tup.get_item(0)?;
                    let innerip = a.extract::<Ipv6Like>()?;
                    Ok(Self(innerip.0))
                }
                _ => {
                    py_type_err!("expected 1, 8, or 16 arguments for IPv6 address")
                }
            }
        } else if let Ok(iplike) = obj.extract::<Ipv6Like>() {
            Ok(Self(iplike.0))
        } else {
            py_type_err!("{IPV6_ADDR_ERROR}")
        }
    }
}

static IPV6_ADDR_ERROR: &str =
    "Invalid IPv4 address, should be a [u8; 16], u128, str, bytes or ipaddress.IPv6Address";

#[cfg(feature = "pydantic")]
mod pydantic {
    use pyo3::prelude::*;
    use pyo3::types::{PyAny, PyDict, PyTuple, PyType};
    use ryo3_pydantic::{GetPydanticCoreSchemaCls, GetPydanticJsonSchemaCls, interns};

    use super::{PyIpAddr, PyIpv4Addr, PyIpv6Addr};

    // =======================================================================
    // Ipv4Addr
    // ======================================================================
    impl GetPydanticCoreSchemaCls for PyIpv4Addr {
        fn get_pydantic_core_schema<'py>(
            cls: &Bound<'py, PyType>,
            source: &Bound<'py, PyAny>,
            _handler: &Bound<'py, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            let py = source.py();
            let core_schema = ryo3_pydantic::core_schema(py)?;
            let str_schema = core_schema.call_method(interns::str_schema(py), (), None)?;
            let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
            let args = PyTuple::new(py, vec![&validation_fn, &str_schema])?;
            let string_serialization_schema =
                core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
            let serialization_kwargs = PyDict::new(py);
            serialization_kwargs
                .set_item(interns::serialization(py), &string_serialization_schema)?;
            core_schema.call_method(
                interns::no_info_wrap_validator_function(py),
                args,
                Some(&serialization_kwargs),
            )
        }
    }

    impl GetPydanticJsonSchemaCls for PyIpv4Addr {
        fn get_pydantic_json_schema<'py>(
            _cls: &pyo3::Bound<'py, pyo3::types::PyType>,
            core_schema: &pyo3::Bound<'py, pyo3::PyAny>,
            handler: &pyo3::Bound<'py, pyo3::PyAny>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let py = handler.py();
            let json_schema = handler.call1((core_schema,))?;
            json_schema.set_item(interns::format(py), interns::ipv4(py))?;
            Ok(json_schema)
        }
    }

    // =======================================================================
    // Ipv6Addr
    // ======================================================================
    impl GetPydanticCoreSchemaCls for PyIpv6Addr {
        fn get_pydantic_core_schema<'py>(
            cls: &pyo3::Bound<'py, pyo3::types::PyType>,
            source: &pyo3::Bound<'py, pyo3::PyAny>,
            _handler: &pyo3::Bound<'py, pyo3::PyAny>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let py = source.py();
            let core_schema = ryo3_pydantic::core_schema(py)?;
            let str_schema = core_schema.call_method(interns::str_schema(py), (), None)?;
            let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
            let args = PyTuple::new(py, vec![&validation_fn, &str_schema])?;
            let string_serialization_schema =
                core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
            let serialization_kwargs = PyDict::new(py);
            serialization_kwargs
                .set_item(interns::serialization(py), &string_serialization_schema)?;
            core_schema.call_method(
                interns::no_info_wrap_validator_function(py),
                args,
                Some(&serialization_kwargs),
            )
        }
    }

    impl GetPydanticJsonSchemaCls for PyIpv6Addr {
        fn get_pydantic_json_schema<'py>(
            _cls: &pyo3::Bound<'py, pyo3::types::PyType>,
            core_schema: &pyo3::Bound<'py, pyo3::PyAny>,
            handler: &pyo3::Bound<'py, pyo3::PyAny>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let py = handler.py();
            let json_schema = handler.call1((core_schema,))?;
            // just set format to ipv6
            json_schema.set_item(interns::format(py), interns::ipv6(py))?;
            Ok(json_schema)
        }
    }

    // =======================================================================
    // IpAddr
    // ======================================================================
    impl GetPydanticCoreSchemaCls for PyIpAddr {
        fn get_pydantic_core_schema<'py>(
            cls: &pyo3::Bound<'py, pyo3::types::PyType>,
            source: &pyo3::Bound<'py, pyo3::PyAny>,
            _handler: &pyo3::Bound<'py, pyo3::PyAny>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let py = source.py();
            let core_schema = ryo3_pydantic::core_schema(py)?;
            let ip_schema = core_schema.call_method(interns::str_schema(py), (), None)?;
            let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
            let args = PyTuple::new(py, vec![&validation_fn, &ip_schema])?;
            let string_serialization_schema =
                core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
            let serialization_kwargs = PyDict::new(py);
            serialization_kwargs
                .set_item(interns::serialization(py), &string_serialization_schema)?;
            core_schema.call_method(
                interns::no_info_wrap_validator_function(py),
                args,
                Some(&serialization_kwargs),
            )
        }
    }

    impl GetPydanticJsonSchemaCls for PyIpAddr {
        fn get_pydantic_json_schema<'py>(
            _cls: &pyo3::Bound<'py, pyo3::types::PyType>,
            core_schema: &pyo3::Bound<'py, pyo3::PyAny>,
            handler: &pyo3::Bound<'py, pyo3::PyAny>,
        ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let py = handler.py();
            let json_schema = handler.call1((core_schema,))?;
            json_schema.del_item(interns::r#type(py)).ok();

            // Build the anyOf array bc I think that is the sanest way to represent it...
            let ipv4 = PyDict::new(py);
            ipv4.set_item(interns::r#type(py), interns::string(py))?;
            ipv4.set_item(interns::format(py), interns::ipv4(py))?;

            let ipv6 = PyDict::new(py);
            ipv6.set_item(interns::r#type(py), interns::string(py))?;
            ipv6.set_item(interns::format(py), interns::ipv6(py))?;
            let anyof = pyo3::types::PyList::new(py, &[ipv4, ipv6])?;
            json_schema.set_item(interns::any_of(py), anyof)?;
            Ok(json_schema)
        }
    }
}
