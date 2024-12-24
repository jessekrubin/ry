use crate::http_types::{HttpHeaderName, HttpHeaderValue};
use crate::py_conversions::{header_name_to_pystring, header_value_to_pystring};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_macros::err_py_not_impl_yet;
use std::collections::HashMap;

#[pyclass(name = "Headers", module = "ry.ryo3.http")]
#[derive(Clone, Debug)]
pub struct PyHeaders(pub http::header::HeaderMap);

#[pymethods]
impl PyHeaders {
    #[new]
    #[pyo3(signature = (d = None))]
    fn py_new<'py>(py: Python<'py>, d: Option<HashMap<String, String>>) -> PyResult<Self> {
        let mut headers = http::header::HeaderMap::new();
        if let Some(d) = d {
            for (k, v) in d {
                let header_name =
                    http::header::HeaderName::from_bytes(k.as_bytes()).map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                            "header-name-error: {e} (k={k}, v={v})"
                        ))
                    })?;
                println!("header_name: {:?}", header_name);
                let header_value = http::header::HeaderValue::from_str(&v).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "header-value-error: {e} (k={k}, v={v})"
                    ))
                })?;
                println!("header_value: {:?}", header_value);
                headers.insert(header_name, header_value);
            }
        }
        Ok(Self(headers))
    }

    /// Return struct Debug-string
    pub fn __dbg__(&self) {
        println!("{:?}", self);
    }

    pub fn __str__(&self) -> String {
        format!("Headers({:?})", self.0)
    }

    pub fn __repr__(&self) -> String {
        format!("Headers({:?})", self.0)
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __eq__(&self, other: &PyHeaders) -> bool {
        self.0 == other.0
    }

    pub fn __ne__(&self, other: &PyHeaders) -> bool {
        self.0 != other.0
    }

    pub fn __contains__(&self, key: &str) -> PyResult<bool> {
        self.contains_key(key)
    }

    pub fn __getitem__(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.get(key).map(|h| HttpHeaderValue::from(h))
    }

    pub fn __setitem__(&mut self, key: &str, value: &str) -> PyResult<()> {
        self.insert(key, value)
    }

    pub fn __delitem__(&mut self, key: HttpHeaderName) -> PyResult<()> {
        self.remove(key)?;
        Ok(())
    }

    pub fn __iter__(&self) -> PyResult<Vec<String>> {
        let keys = self.0.keys().map(|h| h.as_str().to_string()).collect();
        Ok(keys)
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

    pub fn append(&mut self, key: &str, value: &str) -> PyResult<bool> {
        let header_name = http::header::HeaderName::from_bytes(key.as_bytes()).unwrap();
        let header_value = http::header::HeaderValue::from_str(value).unwrap();
        let res = self.0.try_append(header_name, header_value).map_err(|e| {
            PyRuntimeError::new_err(format!(
                "header-append-error: {e} (key={key}, value={value})"
            ))
        })?;
        Ok(res)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn contains_key(&self, key: &str) -> PyResult<bool> {
        let header_name = http::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "header-name-error: {e} (key={key})"
            ))
        })?;
        Ok(self.0.contains_key(&header_name))
    }
    pub fn get(&self, key: &str) -> Option<HttpHeaderValue> {
        self.0.get(key).map(|h| HttpHeaderValue::from(h))
    }

    pub fn get_all(&self, key: &str) -> PyResult<Vec<String>> {
        let hname = http::header::HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "header-name-error: {e} (key={key})"
            ))
        })?;
        // iterate and collect but filter out errors...
        let mut hvalues = vec![];
        for v in self.0.get_all(&hname) {
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

    pub fn insert(&mut self, key: &str, value: &str) -> PyResult<()> {
        eprintln!("insert: {:?} {:?}", key, value);
        err_py_not_impl_yet!()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyString>>> {
        let k = self
            .0
            .keys()
            .map(|h| header_name_to_pystring(py, h))
            .collect();
        Ok(k)
    }

    pub fn keys_len(&self) -> usize {
        self.0.keys_len()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn remove(&mut self, key: HttpHeaderName) -> PyResult<Option<HttpHeaderValue>> {
        Ok(self.0.remove(key.0).map(|h| HttpHeaderValue::from(h)))
    }

    pub fn pop(&mut self, key: HttpHeaderName) -> PyResult<Option<HttpHeaderValue>> {
        self.remove(key)
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyString>>> {
        let mut vals = vec![];
        for v in self.0.values() {
            let pystr = header_value_to_pystring(py, v).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("header-value-error: {e}"))
            })?;
            vals.push(pystr);
        }
        Ok(vals)
        // let vals = self
        //     .0
        //     .values()
        //     .map(|header_value| header_value_to_pystring(py, header_value)).filter_map(
        //
        //
        // Ok(vals)
    }
}

#[pyclass(name = "HeadersValues", module = "ry.ryo3.http")]
pub struct PyHeadersValues {
    headers: PyHeaders,
}
