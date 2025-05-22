use crate::UrlLike;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple, PyType};
use ryo3_macro_rules::py_value_error;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[pyclass(name = "URL", module = "ry.ryo3", frozen)]
pub struct PyUrl(pub url::Url);

impl PyUrl {
    fn parse_with_params(url: &str, params: &Bound<'_, PyDict>) -> PyResult<Self> {
        let params = params
            .into_iter()
            .map(|(k, v)| {
                let k_str: String = k.extract()?;
                let v_str: String = v.extract()?;
                Ok((k_str, v_str))
            })
            .collect::<PyResult<Vec<(String, String)>>>()?;

        url::Url::parse_with_params(url, params)
            .map(PyUrl)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={url})"))
            })
    }
}

#[pymethods]
impl PyUrl {
    #[new]
    #[pyo3(signature = (url, *, params = None))]
    fn py_new(url: UrlLike, params: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        if let Some(params) = params {
            Self::parse_with_params(url.0.as_str(), params)
        } else {
            Ok(Self::from(url.0))
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, vec![self.0.to_string()])
    }

    #[classmethod]
    #[pyo3(signature = (url, *, params = None))]
    fn parse(
        _cls: &Bound<'_, PyType>,
        url: &str,
        params: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        if let Some(params) = params {
            Self::parse_with_params(url, params)
        } else {
            url::Url::parse(url).map(PyUrl).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={url})"))
            })
        }
    }

    #[classmethod]
    #[pyo3(name = "parse_with_params")]
    fn py_parse_with_params<'py>(
        _cls: &Bound<'py, PyType>,
        url: &str,
        params: &Bound<'py, PyDict>,
    ) -> PyResult<Self> {
        Self::parse_with_params(url, params)
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }

    fn __repr__(&self) -> String {
        format!("URL(\'{}\')", self.0.as_str())
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __fspath__(&self) -> PyResult<PathBuf> {
        if let Ok(path) = self.0.to_file_path() {
            Ok(path)
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "URL::__fspath__: invalid path",
            ))
        }
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
            if let Some(last_part) = parts.last() {
                if last_part.ends_with('/') && !relative_path.ends_with('/') {
                    relative_path.push('/');
                }
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
            .map(PyUrl::from)
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
        self.0.join(other).map(PyUrl::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (relative={other})"))
        })
    }

    fn __rtruediv__(&self, other: &str) -> PyResult<Self> {
        self.0.join(other).map(PyUrl::from).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (relative={other})"))
        })
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
        if let Ok(other) = other.downcast::<PyUrl>() {
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

    // TODO: figure out if we are going to return a host obj
    #[getter]
    fn host(&self) -> Option<&str> {
        self.0.host_str()
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

    #[classmethod]
    fn from_directory_path(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        url::Url::from_directory_path(path)
            .map(PyUrl::from)
            .map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "invalid path (path={path})"
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
        ip_host: Option<IpAddr>,
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
        Ok(PyUrl(url))
    }

    #[pyo3(signature = (fragment = None))]
    fn replace_fragment(&self, fragment: Option<&str>) -> Self {
        let mut url = self.0.clone();
        url.set_fragment(fragment);
        PyUrl(url)
    }

    #[pyo3(signature = (host = None))]
    fn replace_host(&self, host: Option<&str>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_host(host)
            .map_err(|e| py_value_error!("{e} (host={host:?})"))?;
        Ok(PyUrl(url))
    }

    fn replace_ip_host(&self, ip_host: IpAddr) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_ip_host(ip_host)
            .map_err(|()| py_value_error!("Err setting ip_host (ip_host={ip_host})"))?;
        Ok(PyUrl(url))
    }

    #[pyo3(signature = (password = None))]
    fn replace_password(&self, password: Option<&str>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_password(password)
            .map_err(|()| py_value_error!("Err setting password (password={password:?})"))?;
        Ok(PyUrl(url))
    }

    fn replace_path(&self, path: &str) -> Self {
        let mut url = self.0.clone();
        url.set_path(path);
        Self::from(url)
    }

    #[pyo3(signature = (port = None))]
    fn replace_port(&self, port: Option<u16>) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_port(port)
            .map_err(|()| py_value_error!("Err setting port (port={port:?})"))?;
        Ok(Self::from(url))
    }

    #[pyo3(signature = (query = None))]
    fn replace_query(&self, query: Option<&str>) -> Self {
        let mut url = self.0.clone();
        url.set_query(query);
        Self::from(url)
    }

    fn replace_scheme(&self, scheme: &str) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_scheme(scheme)
            .map_err(|()| py_value_error!("Err setting scheme (scheme={scheme})"))?;
        Ok(Self::from(url))
    }

    fn replace_username(&self, username: &str) -> PyResult<Self> {
        let mut url = self.0.clone();
        url.set_username(username)
            .map_err(|()| py_value_error!("Err setting username (username={username:?})"))?;
        Ok(Self::from(url))
    }

    #[expect(clippy::unused_self)]
    fn socket_addrs(&self) -> PyResult<()> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
            "Url::socket_addrs not implemented",
        ))
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
