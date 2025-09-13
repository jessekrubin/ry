use crate::PyHeadersLike;
use crate::http_types::{HttpHeaderName, HttpHeaderValue};
use crate::py_conversions::{header_name_to_pystring, header_value_to_pystring};
use http::header::HeaderMap;
use parking_lot::lock_api::MutexGuard;
use parking_lot::{Mutex, RawMutex};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyString, PyTuple};
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;

#[pyclass(name = "Headers", frozen, mapping)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyHeaders(pub Arc<Mutex<HeaderMap>>);

impl Deref for PyHeaders {
    type Target = Arc<Mutex<HeaderMap>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PyHeaders {
    fn extract_kwargs(kwargs: &Bound<'_, PyDict>) -> PyResult<HeaderMap> {
        let mut hm = HeaderMap::new();
        for (key, value) in kwargs.iter() {
            let key = key
                .extract::<HttpHeaderName>()
                .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
            let value = value
                .extract::<HttpHeaderValue>()
                .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
            hm.insert(key.0, value.0);
        }
        Ok(hm)
    }

    fn inner(&self) -> MutexGuard<'_, RawMutex, HeaderMap> {
        self.0.lock()
    }

    fn py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let inner = self.inner();
        if inner.is_empty() {
            Ok(PyDict::new(py))
        } else if inner.len() == inner.keys_len() {
            // don't have to worry about duplicates bc of keys_len == len
            let d = PyDict::new(py);
            for (k, v) in inner.iter() {
                let key_pystr = header_name_to_pystring(py, k)?;
                let value_pystr = header_value_to_pystring(py, v)?;
                d.set_item(key_pystr, value_pystr)?;
            }
            Ok(d)
        } else {
            // need to handle duplicates
            let d = PyDict::new(py);
            for key in inner.keys() {
                let key_pystr = header_name_to_pystring(py, key)?;
                let values: Vec<_> = inner.get_all(key).iter().collect();
                if values.len() == 1 {
                    let v = values[0];
                    if let Ok(vstr) = v.to_str() {
                        d.set_item(key_pystr, vstr)?;
                    } else {
                        let pybytes = PyBytes::new(py, v.as_bytes());
                        d.set_item(key_pystr, pybytes)?;
                    }
                } else {
                    let py_list = PyList::empty(py);
                    for v in values {
                        if let Ok(vstr) = v.to_str() {
                            py_list.append(vstr)?;
                        } else {
                            let pybytes = PyBytes::new(py, v.as_bytes());
                            py_list.append(pybytes)?;
                        }
                    }
                    d.set_item(key_pystr, py_list)?;
                }
            }
            Ok(d)
        }
    }
}

impl Display for PyHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0.lock();
        write!(f, "Headers({inner:?})")
    }
}

impl PartialEq for PyHeaders {
    fn eq(&self, other: &Self) -> bool {
        *(self.0.lock()) == *(other.0.lock())
    }
}

impl From<HeaderMap> for PyHeaders {
    fn from(hm: HeaderMap) -> Self {
        Self(Arc::new(Mutex::new(hm)))
    }
}
impl From<Arc<Mutex<HeaderMap>>> for PyHeaders {
    fn from(hm: Arc<Mutex<HeaderMap>>) -> Self {
        Self(hm)
    }
}
#[pymethods]
impl PyHeaders {
    #[new]
    #[pyo3(signature = (d = None, **kwargs))]
    fn py_new(d: Option<PyHeadersLike>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        match (d, kwargs) {
            (Some(d), Some(kwargs)) => {
                let mut headers_map = HeaderMap::try_from(d)?;
                let kw_headers = Self::extract_kwargs(kwargs)?;
                headers_map.extend(kw_headers);
                Ok(Self::from(headers_map))
            }
            (Some(d), None) => {
                let headers_map = HeaderMap::try_from(d)?;
                Ok(Self::from(headers_map))
            }
            (None, Some(kwargs)) => {
                let kw_headers = Self::extract_kwargs(kwargs)?;
                Ok(Self::from(kw_headers))
            }
            (None, None) => Ok(Self::from(HeaderMap::new())),
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let dict = self.py_dict(py)?;
        PyTuple::new(py, vec![dict])
    }

    /// Return struct Debug-string
    #[must_use]
    fn __dbg__(&self) -> String {
        format!("{self:?}")
    }

    #[must_use]
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[must_use]
    fn __len__(&self) -> usize {
        self.0.lock().len()
    }

    #[must_use]
    fn __eq__(&self, other: &Self) -> bool {
        *(self.0.lock()) == *(other.0.lock())
    }

    #[must_use]
    fn __ne__(&self, other: &Self) -> bool {
        *(self.0.lock()) != *(other.0.lock())
    }

    #[must_use]
    fn __contains__(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn __getitem__(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.lock().get(key).map(HttpHeaderValue::from)
    }

    fn __setitem__(&self, key: HttpHeaderName, value: HttpHeaderValue) -> PyResult<()> {
        self.insert(key, value)?;
        Ok(())
    }

    fn __delitem__(&self, key: HttpHeaderName) {
        self.remove(key);
    }

    #[must_use]
    fn __iter__<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyAny>> {
        self.keys(py)
    }

    // ========================================================================
    // Methods of `HeaderMap`:
    // ========================================================================
    // - `append`: impl via `try_append`
    // - `capacity`:
    // - `clear`:
    // - `contains_key`:
    // - `entry`:
    // - `get`:
    // - `get_all`:
    // - `insert`:
    // - `is_empty`:
    // - `iter`:
    // - `keys`:
    // - `keys_len`:
    // - `len`:
    // - `remove`:
    // - `try_append`: `append`
    // - `try_entry`: `entry`
    // - `try_insert`: `insert`
    // - `values`:
    // - TBD
    //     - `drain`
    //     - `get_mut`
    //     - `iter_mut`
    //     - `reserve`
    //     - `try_with_capacity`
    //     - `values_mut`
    //     - `with_capacity`

    fn append(&self, key: HttpHeaderName, value: HttpHeaderValue) -> PyResult<bool> {
        self.0
            .lock()
            .try_append(key.0, value.0)
            .map_err(|e| PyRuntimeError::new_err(format!("header-append-error: {e}")))
    }

    #[getter]
    fn is_flat(&self) -> bool {
        let inner = self.inner();
        inner.len() == inner.keys_len()
    }

    fn clear(&self) {
        self.0.lock().clear();
    }

    #[must_use]
    fn contains_key(&self, key: &str) -> bool {
        self.0.lock().contains_key(key)
    }

    fn get(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.lock().get(key).map(HttpHeaderValue::from)
    }

    fn get_all(&self, key: &str) -> PyResult<Vec<String>> {
        // iterate and collect but filter out errors...
        let mut hvalues = vec![];
        for v in self.0.lock().get_all(key) {
            match v.to_str() {
                Ok(s) => hvalues.push(s.to_string()),
                Err(e) => {
                    let emsg = format!("header-value-error: {e} (key={key})");
                    return Err(PyErr::new::<PyValueError, _>(emsg));
                }
            }
        }
        Ok(hvalues)
    }

    fn insert(
        &self,
        key: HttpHeaderName,
        value: HttpHeaderValue,
    ) -> PyResult<Option<HttpHeaderValue>> {
        self.0
            .lock()
            .try_insert(key.0, value.0)
            .map_err(|e| PyRuntimeError::new_err(format!("header-insert-error: {e}")))
            .map(|v| v.map(HttpHeaderValue::from))
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        self.0.lock().is_empty()
    }

    #[must_use]
    fn __bool__(&self) -> bool {
        !self.0.lock().is_empty()
    }

    #[must_use]
    fn keys<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyAny>> {
        self.0
            .lock()
            .keys()
            .flat_map(|h| header_name_to_pystring(py, h))
            .collect()
    }

    #[must_use]
    fn keys_len(&self) -> usize {
        self.0.lock().keys_len()
    }

    #[must_use]
    fn len(&self) -> usize {
        self.0.lock().len()
    }

    fn remove(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.0.lock().remove(key.0).map(HttpHeaderValue::from)
    }

    fn pop(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.remove(key)
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyString>>> {
        let mut vals = vec![];
        for v in self.0.lock().values() {
            let pystr = header_value_to_pystring(py, v)
                .map_err(|e| PyErr::new::<PyValueError, _>(format!("header-value-error: {e}")))?;
            vals.push(pystr);
        }
        Ok(vals)
    }

    #[pyo3(signature = (other, append = false))]
    fn update(&self, other: PyHeadersLike, append: bool) -> PyResult<()> {
        match other {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.0.lock();
                let mut inner = self.0.lock();
                if append {
                    for (k, v) in other_inner.iter() {
                        inner.append(k, v.clone());
                        // if let Some(k) = k {
                        //     inner.append(k, v);
                        // }
                    }
                } else {
                    for (k, v) in other_inner.iter() {
                        inner.insert(k, v.clone());
                    }
                }
            }
            PyHeadersLike::Map(other) => {
                let hm = PyHeadersLike::map2headers(&other)?;

                if append {
                    for (k, v) in hm {
                        if let Some(k) = k {
                            self.0.lock().append(k, v);
                        }
                    }
                } else {
                    for (k, v) in hm {
                        if let Some(k) = k {
                            self.0.lock().insert(k, v);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn __or__(&self, other: PyHeadersLike) -> PyResult<Self> {
        let mut headers = self.0.clone().lock().clone();
        match other {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.0.lock();
                for (k, v) in other_inner.iter() {
                    headers.insert(k, v.clone());
                }
            }
            PyHeadersLike::Map(other) => {
                let h = PyHeadersLike::map2headers(&other)?;
                for (k, v) in h {
                    if let Some(k) = k {
                        headers.insert(k, v);
                    }
                }
            }
        }
        Ok(Self::from(headers))
    }

    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.py_dict(py)
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.to_py(py)
    }

    #[cfg(feature = "json")]
    #[pyo3(signature = (*, fmt=false))]
    fn stringify(&self, fmt: bool) -> PyResult<String> {
        {
            let inner = self.0.lock();
            if fmt {
                let a = serde_json::to_string_pretty(&crate::http_serde::HttpHeaderMapRef(&inner))
                    .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
                Ok(a)
            } else {
                let a = serde_json::to_string(&crate::http_serde::HttpHeaderMapRef(&inner))
                    .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))?;
                Ok(a)
            }
        }
    }

    #[cfg(not(feature = "json"))]
    #[expect(clippy::unused_self)]
    #[expect(unused_variables)]
    #[pyo3(signature = (*args, **kwargs))]
    fn stringify(
        &self,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<String> {
        Err(::ryo3_core::FeatureNotEnabledError::new_err(
            "ryo3-http: `json` feature not enabled",
        ))
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn from_json(json: &str) -> PyResult<Self> {
        serde_json::from_str::<crate::HttpHeaderMap>(json)
            .map(|e| Self::from(e.0))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{e}")))
    }

    #[cfg(not(feature = "json"))]
    #[staticmethod]
    fn from_json(_json: &str) -> PyResult<Self> {
        Err(::ryo3_core::FeatureNotEnabledError::new_err(
            "ryo3-http: `json` feature not enabled",
        ))
    }
}
