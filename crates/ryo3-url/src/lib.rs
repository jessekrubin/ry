//! ryo3 url wrapper library for python
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyTuple};
use pyo3::types::{PyModule, PyType};
use pyo3::{pyclass, Bound, PyResult};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[pyclass(name = "Url", module = "ryo3")]
pub struct PyUrl(pub(crate) url::Url);

#[pymethods]
impl PyUrl {
    #[new]
    fn new(url: &str) -> PyResult<Self> {
        url::Url::parse(url).map(PyUrl).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={url})"))
        })
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, url: &str) -> PyResult<Self> {
        url::Url::parse(url).map(PyUrl).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (url={url})"))
        })
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }

    fn __repr__(&self) -> String {
        format!("Url(\'{}\')", self.0.as_str())
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

    fn __richcmp__(&self, other: &PyUrl, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
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

    fn has_host(&self) -> bool {
        self.0.has_host()
    }

    fn is_special(&self) -> bool {
        self.0.is_special()
    }

    #[allow(clippy::unused_self)]
    fn options(&self) -> PyResult<()> {
        Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(""))
    }

    #[allow(clippy::unused_self)]
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

    #[allow(clippy::unused_self)]
    fn set_fragment(&mut self, _fragment: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_fragment".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_host(&mut self, _host: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_host".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_ip_host(&mut self, _host: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_ip_host".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_password(&mut self, _password: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_password".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_path(&mut self, _path: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_path".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_port(&mut self, _port: u16) -> PyResult<()> {
        py_err_not_implemented("Url::set_port".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_query(&mut self, _query: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_query".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_scheme(&mut self, _scheme: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_scheme".to_string())
    }

    #[allow(clippy::unused_self)]
    fn set_username(&mut self, _username: &str) -> PyResult<()> {
        py_err_not_implemented("Url::set_username".to_string())
    }

    #[allow(clippy::unused_self)]
    fn socket_addrs(&self) -> PyResult<()> {
        py_err_not_implemented("Url::socket_addrs".to_string())
    }
}

#[inline]
fn py_err_not_implemented(s: String) -> PyResult<()> {
    Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(s))
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
