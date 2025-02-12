#![doc = include_str!("../README.md")]

mod url_like;
pub use url_like::{extract_url, UrlLike};

use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyDict, PyTuple};
use pyo3::types::{PyModule, PyType};
use pyo3::{pyclass, Bound, PyResult};
use ryo3_macros::py_value_error;
use std::hash::{Hash, Hasher};
use std::net::IpAddr;
use std::path::PathBuf;
use url;

#[derive(Debug, Clone)]
#[pyclass(name = "URL", module = "ryo3")]
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

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, url: &str) -> PyResult<Self> {
        url::Url::parse(url).map(PyUrl).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={url})"))
        })
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
        self.0.as_str().hash(&mut hasher);
        hasher.finish()
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

    #[expect(clippy::unused_self)]
    fn options(&self) -> PyResult<()> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(""))
    }

    #[expect(clippy::unused_self)]
    fn origin(&self) -> PyResult<()> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(""))
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

    #[pyo3(signature = (fragment = None))]
    fn _set_fragment(&mut self, fragment: Option<&str>) {
        self.0.set_fragment(fragment);
    }

    #[pyo3(signature = (host = None))]
    fn _set_host(&mut self, host: Option<&str>) -> PyResult<()> {
        self.0
            .set_host(host)
            .map_err(|e| py_value_error!("{e} (host={host:?})"))
    }

    fn _set_ip_host(&mut self, ip_host: IpAddr) -> PyResult<()> {
        self.0
            .set_ip_host(ip_host)
            .map_err(|()| py_value_error!("Err setting ip_host (ip_host={ip_host})"))
    }

    #[pyo3(signature = (password = None))]
    fn _set_password(&mut self, password: Option<&str>) -> PyResult<()> {
        self.0.set_password(password).map_err(|()| {
            let pw_str = password.map_or_else(|| "<None>".to_string(), ToString::to_string);
            py_value_error!("Err setting password (password={pw_str})")
        })
    }

    fn _set_path(&mut self, path: &str) {
        self.0.set_path(path);
    }

    #[pyo3(signature = (port = None))]
    fn _set_port(&mut self, port: Option<u16>) -> PyResult<()> {
        self.0
            .set_port(port)
            .map_err(|()| py_value_error!("Err setting port (port={port:?})"))
    }

    #[pyo3(signature = (username = None))]
    fn _set_query(&mut self, username: Option<&str>) {
        self.0.set_query(username);
    }

    fn _set_scheme(&mut self, scheme: &str) -> PyResult<()> {
        self.0
            .set_scheme(scheme)
            .map_err(|()| py_value_error!("Err setting scheme (scheme={scheme})"))
    }

    fn _set_username(&mut self, username: &str) -> PyResult<()> {
        self.0
            .set_username(username)
            .map_err(|()| py_value_error!("Err setting username (username={username:?})"))
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

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUrl>()?;
    Ok(())
}
