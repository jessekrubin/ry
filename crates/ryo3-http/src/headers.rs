use crate::http_types::{HttpHeaderName, HttpHeaderValue};
use crate::py_conversions::{header_name_to_pystring, header_value_to_pystring};
use crate::PyHeadersLike;
use http::header::HeaderMap;
use parking_lot::Mutex;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyString, PyTuple};
use std::sync::Arc;

#[pyclass(name = "Headers", module = "ry", frozen)]
#[derive(Clone, Debug)]
pub struct PyHeaders(pub Arc<Mutex<HeaderMap>>);

impl PyHeaders {
    fn extract_kwargs(kwargs: &Bound<'_, PyDict>) -> PyResult<HeaderMap> {
        let mut hm = HeaderMap::new();
        for (key, value) in kwargs.iter() {
            let key = key
                .extract::<HttpHeaderName>()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
            let value = value
                .extract::<HttpHeaderValue>()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
            hm.insert(key.0, value.0);
        }
        Ok(hm)
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

#[pymethods]
impl PyHeaders {
    #[new]
    #[pyo3(signature = (d = None, **kwargs))]
    fn py_new(
        _py: Python<'_>,
        d: Option<PyHeadersLike>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        match (d, kwargs) {
            (Some(d), Some(kwargs)) => {
                let mut headers_map = HeaderMap::try_from(d)?;
                let kw_headers = PyHeaders::extract_kwargs(kwargs)?;
                headers_map.extend(kw_headers);
                Ok(PyHeaders::from(headers_map))
            }
            (Some(d), None) => {
                let headers_map = HeaderMap::try_from(d)?;
                Ok(PyHeaders::from(headers_map))
            }
            (None, Some(kwargs)) => {
                let kw_headers = PyHeaders::extract_kwargs(kwargs)?;
                Ok(PyHeaders::from(kw_headers))
            }
            (None, None) => Ok(PyHeaders::from(HeaderMap::new())),
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let dict = self.asdict(py)?;
        PyTuple::new(py, vec![dict])
    }

    /// Return struct Debug-string
    #[must_use]
    pub fn __dbg__(&self) -> String {
        format!("{self:?}")
    }

    #[must_use]
    pub fn __str__(&self) -> String {
        format!("Headers({:?})", self.0)
    }

    #[must_use]
    pub fn __repr__(&self) -> String {
        format!("Headers({:?})", self.0)
    }

    #[must_use]
    pub fn __len__(&self) -> usize {
        self.0.lock().len()
    }

    #[must_use]
    pub fn __eq__(&self, other: &PyHeaders) -> bool {
        *(self.0.lock()) == *(other.0.lock())
    }

    #[must_use]
    pub fn __ne__(&self, other: &PyHeaders) -> bool {
        *(self.0.lock()) != *(other.0.lock())
    }

    pub fn __contains__(&self, key: &str) -> PyResult<bool> {
        self.contains_key(key)
    }

    pub fn __getitem__(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.lock().get(key).map(HttpHeaderValue::from)
    }

    pub fn __setitem__(&self, key: HttpHeaderName, value: HttpHeaderValue) -> PyResult<()> {
        self.insert(key, value)
    }

    pub fn __delitem__(&self, key: HttpHeaderName) {
        self.remove(key);
    }

    #[must_use]
    pub fn __iter__(&self) -> Vec<String> {
        self.0
            .lock()
            .keys()
            .map(|h| h.as_str().to_string())
            .collect()
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

    pub fn append(&self, key: HttpHeaderName, value: HttpHeaderValue) -> PyResult<bool> {
        self.0
            .lock()
            .try_append(key.0, value.0)
            .map_err(|e| PyRuntimeError::new_err(format!("header-append-error: {e}")))
    }

    pub fn clear(&self) {
        self.0.lock().clear();
    }

    pub fn contains_key(&self, key: &str) -> PyResult<bool> {
        let header_name = http::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "header-name-error: {e} (key={key})"
            ))
        })?;
        Ok(self.0.lock().contains_key(&header_name))
    }

    pub fn get(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.lock().get(key).map(HttpHeaderValue::from)
    }

    pub fn get_all(&self, key: &str) -> PyResult<Vec<String>> {
        let hname = http::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "header-name-error: {e} (key={key})"
            ))
        })?;
        // iterate and collect but filter out errors...
        let mut hvalues = vec![];
        for v in self.0.lock().get_all(&hname) {
            match v.to_str() {
                Ok(s) => hvalues.push(s.to_string()),
                Err(e) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "header-value-error: {e} (key={key})"
                    )));
                }
            }
        }
        Ok(hvalues)
    }

    pub fn insert(&self, key: HttpHeaderName, value: HttpHeaderValue) -> PyResult<()> {
        self.0
            .lock()
            .try_insert(key.0, value.0)
            .map_err(|e| PyRuntimeError::new_err(format!("header-insert-error: {e}")))?;
        Ok(())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.lock().is_empty()
    }

    #[must_use]
    pub fn __bool__(&self) -> bool {
        !self.0.lock().is_empty()
    }

    #[must_use]
    pub fn keys<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyString>> {
        self.0
            .lock()
            .keys()
            .map(|h| header_name_to_pystring(py, h))
            .collect()
    }

    #[must_use]
    pub fn keys_len(&self) -> usize {
        self.0.lock().keys_len()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.lock().len()
    }

    pub fn remove(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.0.lock().remove(key.0).map(HttpHeaderValue::from)
    }

    pub fn pop(&self, key: HttpHeaderName) -> Option<HttpHeaderValue> {
        self.remove(key)
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyString>>> {
        let mut vals = vec![];
        for v in self.0.lock().values() {
            let pystr = header_value_to_pystring(py, v).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("header-value-error: {e}"))
            })?;
            vals.push(pystr);
        }
        Ok(vals)
    }

    #[pyo3(signature = (other, append = false))]
    pub fn update(&self, other: PyHeadersLike, append: bool) -> PyResult<()> {
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

    pub fn __or__(&self, other: PyHeadersLike) -> PyResult<PyHeaders> {
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
        Ok(PyHeaders::from(headers))
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let d = PyDict::new(py);

        let inner = self.0.lock();
        for key in inner.keys() {
            let key_str = key.as_str();
            let values: Vec<_> = inner.get_all(key).iter().collect();

            if values.len() == 1 {
                let v = values[0];
                if let Ok(vstr) = v.to_str() {
                    d.set_item(key_str, vstr)?;
                } else {
                    let pybytes = PyBytes::new(py, v.as_bytes());
                    d.set_item(key_str, pybytes)?;
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
                d.set_item(key_str, py_list)?;
            }
        }

        Ok(d)
    }

    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.asdict(py)
    }
}
