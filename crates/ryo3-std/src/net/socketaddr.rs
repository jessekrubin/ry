use crate::net::PyIpv6Addr;
use crate::net::ipaddr::{IpAddrLike, PyIpAddr, PyIpv4Addr};
use crate::net::ipaddr_props::IpAddrProps;
use pyo3::{BoundObject, prelude::*};
use ryo3_core::{PyFromStr, PyParse};
use ryo3_macro_rules::{any_repr, py_type_err, py_value_err};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddrV4", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PySocketAddrV4(pub(crate) SocketAddrV4);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddrV6", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PySocketAddrV6(pub(crate) SocketAddrV6);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[pyclass(name = "SocketAddr", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
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
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipv4(&self) -> PyIpv4Addr {
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
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
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

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr(&self) -> PySocketAddr {
        PySocketAddr(SocketAddr::V4(self.0))
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(sockaddr) = value.cast::<PySocketAddr>() {
            match sockaddr.get().0 {
                SocketAddr::V4(sa) => Self::from(sa).into_pyobject(py),
                SocketAddr::V6(_) => {
                    py_type_err!(
                        "SocketAddrV4 conversion error: expected SocketAddrV4, got SocketAddrV6"
                    )
                }
            }
        } else {
            let valtype = any_repr!(value);
            py_type_err!("SocketAddrV4 conversion error: {valtype}")
        }
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================

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
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_pyipaddress(&self) -> Ipv6Addr {
        *self.0.ip()
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_ipv6(&self) -> PyIpv6Addr {
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
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
    }

    #[classattr]
    fn version() -> u8 {
        6
    }

    // ========================================================================
    // IpAddr forwarded
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

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr(&self) -> PySocketAddr {
        PySocketAddr(SocketAddr::V6(self.0))
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(socketaddr) = value.cast::<PySocketAddr>() {
            match socketaddr.get().0 {
                SocketAddr::V6(sa6) => Self::from(sa6).into_pyobject(py),
                SocketAddr::V4(_) => {
                    py_type_err!(
                        "SocketAddrV6 conversion error: expected SocketAddrV6, got SocketAddrV4"
                    )
                }
            }
        } else {
            let valtype = any_repr!(value);
            py_type_err!("SocketAddrV6 conversion error: {valtype}")
        }
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================

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
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        Self::py_parse(s)
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

    #[getter]
    fn is_ipv4(&self) -> bool {
        <Self as IpAddrProps>::is_ipv4(self)
    }

    #[getter]
    fn is_ipv6(&self) -> bool {
        <Self as IpAddrProps>::is_ipv6(self)
    }

    // #[expect(clippy::unused_self)]
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
    fn is_unspecified(&self) -> bool {
        <Self as IpAddrProps>::is_unspecified(self)
    }

    #[getter]
    // #[expect(clippy::unused_self)]
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
    // #[expect(clippy::unused_self)]
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

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v4(&self) -> PyResult<PySocketAddrV4> {
        match self.0 {
            SocketAddr::V4(sa4) => Ok(PySocketAddrV4(sa4)),
            SocketAddr::V6(_) => py_value_err!("Cannot convert SocketAddr (v6) to SocketAddrV4"),
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_socketaddr_v6(&self) -> PyResult<PySocketAddrV6> {
        match self.0 {
            SocketAddr::V6(sa6) => Ok(PySocketAddrV6(sa6)),
            SocketAddr::V4(_) => py_value_err!("Cannot convert SocketAddr (v4) to SocketAddrV6"),
        }
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            let s = pystr.extract::<&str>()?;
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(sockaddrv4) = value.cast::<PySocketAddrV4>() {
            let sockaddr = Self::from(SocketAddr::V4(sockaddrv4.get().0));
            sockaddr.into_pyobject(py)
        } else if let Ok(sockaddrv6) = value.cast::<PySocketAddrV6>() {
            let sockaddr = Self::from(SocketAddr::V6(sockaddrv6.get().0));
            sockaddr.into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("SocketAddr conversion error: {valtype}")
        }
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================

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
}
