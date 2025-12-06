use crate::UrlLike;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use ryo3_macro_rules::{py_type_err, py_value_error};
use std::ffi::OsString;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[pyclass(name = "URL", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyUrl(pub(crate) url::Url);

impl PyUrl {
    #[must_use]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }
}


#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PyUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        url::Url::deserialize(deserializer).map(PyUrl)
    }
}

#[pymethods]
impl PyUrl {
    #[new]
    #[pyo3(signature = (url, *, params = None))]
    fn py_new(url: UrlLike, params: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        if let Some(params) = params {
            url.py_with_params(params).map(Self::from)
        } else {
            Ok(Self::from(url.0))
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, vec![self.0.to_string()])
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        use ryo3_core::PyFromStr;
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyParse;
        Self::py_parse(s)
    }

    #[staticmethod]
    #[pyo3(name = "parse_with_params", signature = (url, params))]
    fn py_parse_with_params(url: UrlLike, params: &Bound<'_, PyDict>) -> PyResult<Self> {
        url.py_with_params(params).map(Self::from)
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __fspath__(&self) -> PyResult<OsString> {
        let fpath = self.to_filepath()?;
        Ok(fpath.into_os_string())
    }

    fn __len__(&self) -> usize {
        self.0.as_str().len()
    }

    #[pyo3(signature = (*parts))]
    fn join(&self, parts: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let parts = parts.extract::<Vec<String>>()?;

        if parts.is_empty() {
            Ok(self.clone())
        } else {
            let mut relative_path = parts.join("/");
            if let Some(last_part) = parts.last()
                && last_part.ends_with('/')
                && !relative_path.ends_with('/')
            {
                relative_path.push('/');
            }
            // jesus what was I doing here............. that I have this thing
            // chained from a block...
            {
                let mut base = self.0.clone();
                if !base.path().ends_with('/') {
                    base.set_path(&format!("{}/", base.path()));
                }
                base
            }
            .join(&relative_path)
            .map(Self::from)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "{e} (relative={relative_path})"
                ))
            })
        }
    }

    fn make_relative(&self, other: &Self) -> Option<String> {
        self.0.make_relative(&other.0)
    }

    fn __truediv__(&self, other: &str) -> PyResult<Self> {
        self.0.join(other).map(Self::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (relative={other})"))
        })
    }

    fn __rtruediv__(&self, other: &str) -> PyResult<Self> {
        self.0.join(other).map(Self::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (relative={other})"))
        })
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        if let Ok(other) = other.cast::<Self>() {
            let other = other.borrow();
            match op {
                CompareOp::Eq => Ok(self.0 == other.0),
                CompareOp::Ne => Ok(self.0 != other.0),
                CompareOp::Lt => Ok(self.0 < other.0),
                CompareOp::Le => Ok(self.0 <= other.0),
                CompareOp::Gt => Ok(self.0 > other.0),
                CompareOp::Ge => Ok(self.0 >= other.0),
            }
        } else if let Ok(other) = other.extract::<&str>() {
            match op {
                CompareOp::Eq => Ok(self.0.as_str() == other),
                CompareOp::Ne => Ok(self.0.as_str() != other),
                CompareOp::Lt => Ok(self.0.as_str() < other),
                CompareOp::Le => Ok(self.0.as_str() <= other),
                CompareOp::Gt => Ok(self.0.as_str() > other),
                CompareOp::Ge => Ok(self.0.as_str() >= other),
            }
        } else {
            match op {
                CompareOp::Eq => Ok(false),
                CompareOp::Ne => Ok(true),
                _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "unsupported operand type(s) for comparison",
                )),
            }
        }
    }

    #[getter]
    fn scheme(&self) -> &str {
        self.0.scheme()
    }

    #[getter]
    fn domain(&self) -> Option<&str> {
        self.0.domain()
    }

    #[cfg(not(feature = "ryo3-std"))]
    #[getter]
    fn host(&self) -> Option<&str> {
        self.0.host_str()
    }

    #[cfg(feature = "ryo3-std")]
    #[getter]
    fn host<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        use pyo3::IntoPyObjectExt;
        if let Some(host) = self.0.host() {
            match host {
                url::Host::Domain(d) => d.into_bound_py_any(py),
                url::Host::Ipv4(ipv4) => {
                    ryo3_std::net::PyIpv4Addr::from(ipv4).into_bound_py_any(py)
                }
                url::Host::Ipv6(ipv6) => {
                    ryo3_std::net::PyIpv6Addr::from(ipv6).into_bound_py_any(py)
                }
            }
        } else {
            let n = py.None();
            n.into_bound_py_any(py)
        }
    }

    #[getter]
    fn host_str(&self) -> Option<&str> {
        self.0.host_str()
    }

    #[getter]
    fn port(&self) -> Option<u16> {
        self.0.port()
    }

    #[getter]
    fn path(&self) -> &str {
        self.0.path()
    }

    #[getter]
    fn path_segments<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        if let Some(segs) = self.0.path_segments() {
            let segs = segs.collect::<Vec<_>>();
            PyTuple::new(py, segs)
        } else {
            Ok(PyTuple::empty(py))
        }
    }

    #[getter]
    fn query(&self) -> Option<&str> {
        self.0.query()
    }

    #[getter]
    fn query_string(&self) -> &str {
        self.0.query().unwrap_or("")
    }

    #[getter]
    fn query_pairs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let query_pairs = self
            .0
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>();
        PyTuple::new(py, query_pairs)
    }

    #[getter]
    fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    #[getter]
    fn user(&self) -> &str {
        self.0.username()
    }

    #[getter]
    fn username(&self) -> &str {
        self.0.username()
    }

    #[getter]
    fn password(&self) -> Option<&str> {
        self.0.password()
    }

    #[getter]
    fn port_or_known_default(&self) -> Option<u16> {
        self.0.port_or_known_default()
    }

    #[getter]
    fn authority(&self) -> &str {
        self.0.authority()
    }

    fn has_authority(&self) -> bool {
        self.0.has_authority()
    }

    #[getter]
    fn netloc(&self) -> &str {
        // not provided by python
        self.0.authority()
    }

    fn has_host(&self) -> bool {
        self.0.has_host()
    }

    fn is_special(&self) -> bool {
        self.0.is_special()
    }

    #[getter]
    fn origin(&self) -> String {
        self.0.origin().ascii_serialization()
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_directory_path(path: PathBuf) -> PyResult<Self> {
        url::Url::from_directory_path(&path)
            .map(Self::from)
            .map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "invalid path (path={})",
                    path.display()
                ))
            })
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_filepath(path: PathBuf) -> PyResult<Self> {
        url::Url::from_file_path(&path)
            .map(Self::from)
            .map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "invalid path (path={})",
                    path.display()
                ))
            })
    }

    fn to_filepath(&self) -> PyResult<PathBuf> {
        self.0.to_file_path().map_err(|_e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Url::to_filepath: {}",
                self.__str__()
            ))
        })
    }

    // TODO: figure out if this is problematic... it could be a problem w/ how some of the
    //       underlying set methods take `Option` values...
    #[expect(clippy::too_many_arguments)]
    #[pyo3(
        signature = (
            *,
            fragment = None,
            host = None,
            ip_host = None,
            password = None,
            path = None,
            port = None,
            query = None,
            scheme = None,
            username = None
        )
    )]
    fn replace(
        &self,
        fragment: Option<&str>,
        host: Option<&str>,
        ip_host: Option<&Bound<'_, PyAny>>,
        password: Option<&str>,
        path: Option<&str>,
        port: Option<u16>,
        query: Option<&str>,
        scheme: Option<&str>,
        username: Option<&str>,
    ) -> PyResult<Self> {
        let mut url = self.0.clone();
        if let Some(fragment) = fragment {
            url.set_fragment(fragment.into());
        }
        if let Some(host) = host {
            url.set_host(host.into())
                .map_err(|e| py_value_error!("{e} (host={host:?})"))?;
        }
        if let Some(ip_host) = ip_host {
            let ip_host = extract_ip_host(ip_host)?;
            url.set_ip_host(ip_host)
                .map_err(|()| py_value_error!("Err setting ip_host (ip_host={ip_host})"))?;
        }
        if let Some(password) = password {
            url.set_password(password.into())
                .map_err(|()| py_value_error!("Err setting password (password={password:?})"))?;
        }
        if let Some(path) = path {
            url.set_path(path);
        }
        if let Some(port) = port {
            url.set_port(port.into())
                .map_err(|()| py_value_error!("Err setting port (port={port:?})"))?;
        }
        if let Some(query) = query {
            url.set_query(query.into());
        }
        if let Some(scheme) = scheme {
            url.set_scheme(scheme)
                .map_err(|()| py_value_error!("Err setting scheme (scheme={scheme})"))?;
        }
        if let Some(username) = username {
            url.set_username(username)
                .map_err(|()| py_value_error!("Err setting username (username={username:?})"))?;
        }
        Ok(Self(url))
    }

    #[pyo3(signature = (fragment = None))]
    fn with_fragment(&self, fragment: Option<&str>) -> Self {
        let mut url = self.0.clone();
        url.set_fragment(fragment);
        Self(url)
    }

    #[pyo3(signature = (host = None))]
    fn with_host(&self, host: Option<&str>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_host(host)
            .map_err(|e| py_value_error!("{e} (host={host:?})"))?;
        Ok(Self(url))
    }

    fn with_ip_host(&self, address: &Bound<'_, PyAny>) -> PyResult<Self> {
        let address = extract_ip_host(address)?;
        let mut url = self.0.clone();
        url.set_ip_host(address)
            .map_err(|()| py_value_error!("Err setting ip_host (address={address})"))?;
        Ok(Self(url))
    }

    #[pyo3(signature = (password = None))]
    fn with_password(&self, password: Option<&str>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_password(password)
            .map_err(|()| py_value_error!("Err setting password (password={password:?})"))?;
        Ok(Self(url))
    }

    fn with_path(&self, path: &str) -> Self {
        let mut url = self.0.clone();
        url.set_path(path);
        Self::from(url)
    }

    #[pyo3(signature = (port = None))]
    fn with_port(&self, port: Option<u16>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_port(port)
            .map_err(|()| py_value_error!("Err setting port (port={port:?})"))?;
        Ok(Self::from(url))
    }

    #[pyo3(signature = (query = None))]
    fn with_query(&self, query: Option<&str>) -> Self {
        let mut url = self.0.clone();
        url.set_query(query);
        Self::from(url)
    }

    fn with_scheme(&self, scheme: &str) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_scheme(scheme)
            .map_err(|()| py_value_error!("Err setting scheme (scheme={scheme})"))?;
        Ok(Self::from(url))
    }

    fn with_username(&self, username: &str) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_username(username)
            .map_err(|()| py_value_error!("Err setting username (username={username:?})"))?;
        Ok(Self::from(url))
    }

    #[cfg(feature = "ryo3-std")]
    #[pyo3(signature = (default_port_number = None))]
    fn socket_addrs(
        &self,
        default_port_number: Option<u16>,
    ) -> PyResult<Vec<ryo3_std::net::PySocketAddr>> {
        let sockets = self.0.socket_addrs(|| default_port_number)?;
        let socks = sockets
            .into_iter()
            .map(ryo3_std::net::PySocketAddr::from)
            .collect();
        Ok(socks)
    }

    #[cfg(not(feature = "ryo3-std"))]
    #[pyo3(signature = (default_port_number = None))]
    fn socket_addrs(&self, default_port_number: Option<u16>) -> PyResult<Vec<String>> {
        let sockets = self.0.socket_addrs(|| default_port_number)?;
        let socks = sockets.into_iter().map(|sock| sock.to_string()).collect();
        Ok(socks)
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3::IntoPyObjectExt;
        use ryo3_macro_rules::{py_value_err, py_value_error};
        if let Ok(url) = value.cast_exact::<Self>() {
            url.into_bound_py_any(value.py())
        } else if let Ok(s) = value.extract::<&str>() {
            if s.is_empty() {
                // match pydantic's AnyUrl err msg
                return py_value_err!("URL validation error: input is empty");
            }
            let url = url::Url::parse(s)
                .map_err(|e| py_value_error!("URL validation error: {e} (url={s})"))?;
            let py_url = Self::from(url);
            py_url.into_bound_py_any(value.py())
        } else if let Ok(b) = value.extract::<&[u8]>() {
            if b.is_empty() {
                // match pydantic's AnyUrl err msg
                return py_value_err!("URL validation error: input is empty");
            }
            // to str
            let str = std::str::from_utf8(b).map_err(|e| {
                py_value_error!("URL validation error: invalid UTF-8 sequence: {e} (url={b:?})")
            })?;
            let url = url::Url::parse(str)
                .map_err(|e| py_value_error!("URL validation error: {e} (url={b:?})"))?;
            let py_url = Self::from(url);
            py_url.into_bound_py_any(value.py())
        } else {
            // TODO: figure out how to match pydantic's ability to do value-type-errors?
            ryo3_macro_rules::py_value_err!("Expected str or bytes or URL object",)
        }
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

    // ========================================================================
    // DEPRECATED METHODS
    // ========================================================================
    #[pyo3(
        signature = (fragment = None),
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_fragment(&self, fragment: Option<&str>) -> Self {
        self.with_fragment(fragment)
    }

    #[pyo3(
        signature = (host = None),
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_host(&self, host: Option<&str>) -> PyResult<Self> {
        self.with_host(host)
    }

    #[pyo3(
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_ip_host(&self, address: &Bound<'_, PyAny>) -> PyResult<Self> {
        self.with_ip_host(address)
    }

    #[pyo3(
        signature = (password = None),
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_password(&self, password: Option<&str>) -> PyResult<Self> {
        self.with_password(password)
    }

    #[pyo3(
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_path(&self, path: &str) -> Self {
        self.with_path(path)
    }

    #[pyo3(
        signature = (port = None),
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_port(&self, port: Option<u16>) -> PyResult<Self> {
        self.with_port(port)
    }

    #[pyo3(
        signature = (query = None),
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_query(&self, query: Option<&str>) -> Self {
        self.with_query(query)
    }

    #[pyo3(
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_scheme(&self, scheme: &str) -> PyResult<Self> {
        self.with_scheme(scheme)
    }

    #[pyo3(
        warn(
            message = "`replace_*` methods are deprecated, use `with_*` methods instead",
            category = pyo3::exceptions::PyDeprecationWarning
        )
    )]
    fn replace_username(&self, username: &str) -> PyResult<Self> {
        self.with_username(username)
    }
}

impl FromStr for PyUrl {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::parse(s).map(PyUrl)
    }
}

impl AsRef<url::Url> for PyUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}

impl From<url::Url> for PyUrl {
    fn from(url: url::Url) -> Self {
        Self(url)
    }
}

impl std::fmt::Display for PyUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "URL(\'{}\')", self.0.as_str())
    }
}

#[cfg(feature = "pydantic")]
impl ryo3_pydantic::GetPydanticCoreSchemaCls for PyUrl {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::interns;

        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let url_schema = core_schema.call_method(pyo3::intern!(py, "url_schema"), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &url_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = pyo3::types::PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}

#[cfg(feature = "ryo3-std")]
fn extract_ip_host(address: &Bound<'_, PyAny>) -> PyResult<IpAddr> {
    use ryo3_std::net::{PyIpAddr, PyIpv4Addr, PyIpv6Addr};
    if let Ok(pyipv4) = address.cast_exact::<PyIpv4Addr>() {
        Ok(pyipv4.get().0.into())
    } else if let Ok(pyipv6) = address.cast_exact::<PyIpv6Addr>() {
        Ok(pyipv6.get().0.into())
    } else if let Ok(pyipaddr) = address.cast_exact::<PyIpAddr>() {
        Ok(pyipaddr.get().0)
    } else if let Ok(ip) = address.extract::<std::net::IpAddr>() {
        Ok(ip)
    } else {
        py_type_err!(
            "Expected Ipv4Addr, Ipv6Addr, IpAddr, ipaddress.IPv4Address, ipaddress.IPv6Address",
        )
    }
}

#[cfg(not(feature = "ryo3-std"))]
fn extract_ip_host<'py>(address: &Bound<'py, PyAny>) -> PyResult<IpAddr> {
    if let Ok(ip) = address.extract::<std::net::IpAddr>() {
        Ok(ip)
    } else {
        py_type_err!("Expected ipaddress.IPv4Address or ipaddress.IPv6Address",)
    }
}
