use crate::http_types::{HttpHeaderName, HttpHeaderValue, HttpHeaderValueRef};
use crate::py_conversions::{header_name_to_pystring, header_value_to_pystring};
use crate::{HttpHeaderMap, PyHeadersLike};
use http::header::HeaderMap;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyString, PyTuple};
use ryo3_core::{RyRwLock, py_runtime_error};
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

#[pyclass(name = "Headers", frozen, immutable_type, mapping, from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyHeaders(pub Arc<RyRwLock<HeaderMap, false>>);

impl Deref for PyHeaders {
    type Target = Arc<RyRwLock<HeaderMap, false>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PyHeaders {
    #[inline]
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, HeaderMap> {
        self.0.py_read()
    }

    #[inline]
    fn write(&self) -> RwLockWriteGuard<'_, HeaderMap> {
        self.0.py_write()
    }

    fn py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let inner = self.read();
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
        let inner = self.read();
        if f.alternate() {
            write!(f, "{inner:?}")
        } else {
            write!(f, "Headers({inner:?})")
        }
    }
}

impl PartialEq for PyHeaders {
    fn eq(&self, other: &Self) -> bool {
        *(self.read()) == *(other.read())
    }
}

impl Eq for PyHeaders {}

impl From<HeaderMap> for PyHeaders {
    fn from(hm: HeaderMap) -> Self {
        Self(Arc::new(RyRwLock::new(hm)))
    }
}

impl From<HttpHeaderMap> for PyHeaders {
    fn from(hm: HttpHeaderMap) -> Self {
        Self::from(HeaderMap::from(hm))
    }
}

impl From<Arc<RyRwLock<HeaderMap, false>>> for PyHeaders {
    fn from(hm: Arc<RyRwLock<HeaderMap, false>>) -> Self {
        Self(hm)
    }
}

#[pymethods]
impl PyHeaders {
    #[new]
    #[pyo3(signature = (d = None, **kwargs))]
    fn py_new(d: Option<PyHeadersLike>, kwargs: Option<PyHeadersLike>) -> Self {
        match (d, kwargs) {
            (Some(d), Some(kwargs)) => {
                let mut headers_map = HeaderMap::from(d);
                headers_map.extend(HeaderMap::from(kwargs));
                Self::from(headers_map)
            }
            (Some(d), None) => {
                let headers_map = HeaderMap::from(d);
                Self::from(headers_map)
            }
            (None, Some(kwargs)) => {
                let kw_headers = HeaderMap::from(kwargs);
                Self::from(kw_headers)
            }
            (None, None) => Self::from(HeaderMap::new()),
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
        self.read().len()
    }

    #[must_use]
    fn __eq__(&self, other: &Self) -> bool {
        *(self.read()) == *(other.read())
    }

    #[must_use]
    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    #[must_use]
    fn __contains__(&self, key: &str) -> bool {
        self.contains_key(key)
    }

    fn __getitem__(&self, key: &str) -> Option<HttpHeaderValue> {
        self.read().get(key).map(HttpHeaderValue::from)
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
        self.write()
            .try_append(key.0, value.0)
            .map_err(|e| py_runtime_error!("header-append-error: {e}"))
    }

    #[getter]
    fn is_flat(&self) -> bool {
        let inner = self.read();
        inner.len() == inner.keys_len()
    }

    fn clear(&self) {
        self.write().clear();
    }

    #[must_use]
    fn contains_key(&self, key: &str) -> bool {
        self.read().contains_key(key)
    }

    fn get(&self, key: &str) -> Option<HttpHeaderValue> {
        self.read().get(key).map(HttpHeaderValue::from)
    }

    fn get_all<'py>(&'py self, py: Python<'py>, key: &str) -> PyResult<Bound<'py, PyAny>> {
        // iterate and collect but filter out errors...
        self.read()
            .get_all(key)
            .iter()
            .map(HttpHeaderValueRef::from)
            .collect::<Vec<_>>()
            .into_pyobject(py)
    }

    fn insert(
        &self,
        key: HttpHeaderName,
        value: HttpHeaderValue,
    ) -> PyResult<Option<HttpHeaderValue>> {
        self.write()
            .try_insert(key.0, value.0)
            .map_err(|e| py_runtime_error!("header-insert-error: {e}"))
            .map(|v| v.map(HttpHeaderValue::from))
    }

    #[must_use]
    fn is_empty(&self) -> bool {
        self.read().is_empty()
    }

    #[must_use]
    fn __bool__(&self) -> bool {
        !self.read().is_empty()
    }

    #[must_use]
    fn keys<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyAny>> {
        self.read()
            .keys()
            .flat_map(|h| header_name_to_pystring(py, h))
            .collect()
    }

    #[must_use]
    fn keys_len(&self) -> usize {
        self.read().keys_len()
    }

    #[must_use]
    fn len(&self) -> usize {
        self.read().len()
    }

    fn remove(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.write().remove(key.0).map(HttpHeaderValue::from)
    }

    fn pop(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.remove(key)
    }

    fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyString>>> {
        let mut vals = vec![];
        for v in self.read().values() {
            let pystr = header_value_to_pystring(py, v)?;
            vals.push(pystr);
        }
        Ok(vals)
    }

    #[pyo3(signature = (headers, *, append = false))]
    fn update(&self, headers: PyHeadersLike, append: bool) {
        match headers {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.read();
                let mut inner = self.write();
                if append {
                    for (k, v) in other_inner.iter() {
                        inner.append(k, v.into());
                    }
                } else {
                    for (k, v) in other_inner.iter() {
                        inner.insert(k, v.into());
                    }
                }
            }
            PyHeadersLike::Map(other) => {
                let hm = HeaderMap::from(other);
                if append {
                    let mut inner = self.write();
                    for (k, v) in hm {
                        if let Some(k) = k {
                            inner.append(k, v);
                        }
                    }
                } else {
                    let mut inner = self.write();
                    for (k, v) in hm {
                        if let Some(k) = k {
                            inner.insert(k, v);
                        }
                    }
                }
            }
        }
    }

    fn __or__(&self, other: PyHeadersLike) -> Self {
        let mut headers = self.read().clone();
        match other {
            PyHeadersLike::Headers(other) => {
                let other_inner = other.read();
                for (k, v) in other_inner.iter() {
                    headers.insert(k, v.clone());
                }
            }
            PyHeadersLike::Map(other) => {
                let h = HeaderMap::from(other);
                for (k, v) in h {
                    if let Some(k) = k {
                        headers.insert(k, v);
                    }
                }
            }
        }
        Self::from(headers)
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
        use ryo3_core::py_value_error;
        let inner = self.read();
        if fmt {
            serde_json::to_string_pretty(&crate::http_serde::HttpHeaderMapRef(&inner))
                .map_err(|e| py_value_error!("{e}"))
        } else {
            serde_json::to_string(&crate::http_serde::HttpHeaderMapRef(&inner))
                .map_err(|e| py_value_error!("{e}"))
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
    fn from_json(data: &str) -> PyResult<Self> {
        use ryo3_core::py_value_error;

        serde_json::from_str::<crate::HttpHeaderMap>(data)
            .map(|e| Self::from(e.0))
            .map_err(|e| py_value_error!("{e}"))
    }

    #[cfg(not(feature = "json"))]
    #[staticmethod]
    fn from_json(_json: &str) -> PyResult<Self> {
        Err(::ryo3_core::FeatureNotEnabledError::new_err(
            "ryo3-http: `json` feature not enabled",
        ))
    }
}

impl std::hash::Hash for PyHeaders {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner = self.read();
        HeaderMapRef(&inner).hash(state);
    }
}

struct HeaderMapRef<'a>(&'a HeaderMap);
impl std::hash::Hash for HeaderMapRef<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // sorted keys
        let mut keys: Vec<_> = self.0.keys().collect();
        keys.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
        for key in keys {
            key.hash(state);
            let values: Vec<_> = self.0.get_all(key).iter().collect();
            for value in values {
                value.as_bytes().hash(state);
            }
        }
    }
}
