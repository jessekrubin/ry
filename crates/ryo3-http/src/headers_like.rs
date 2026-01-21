use crate::{HttpHeaderMap, PyHeaders};
use http::HeaderMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_core::py_type_err;
use std::collections::HashMap;
#[derive(Debug, FromPyObject)]
pub(crate) enum StringOrStrings {
    String(pyo3::pybacked::PyBackedStr),
    Strings(Vec<pyo3::pybacked::PyBackedStr>),
}

#[derive(Debug, Clone, FromPyObject)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(HttpHeaderMap),
}

impl<'py> FromPyObject<'_, 'py> for HttpHeaderMap {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(d) = ob.cast_exact::<pyo3::types::PyDict>() {
            let extracted = d.extract::<HashMap<String, StringOrStrings>>()?;
            let mut hm: http::HeaderMap = HeaderMap::new();
            for (k, v) in extracted {
                match v {
                    StringOrStrings::String(s) => {
                        let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                            .map_err(|e| {
                                PyValueError::new_err(format!("header-name-error: {e}"))
                            })?;
                        let header_value = http::header::HeaderValue::from_bytes(s.as_bytes())
                            .map_err(|e| {
                                PyValueError::new_err(format!("header-value-error: {e}"))
                            })?;
                        hm.insert(header_name, header_value);
                    }
                    StringOrStrings::Strings(v) => {
                        let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                            .map_err(|e| {
                                PyValueError::new_err(format!("header-name-error: {e}"))
                            })?;
                        for s in v {
                            let header_value = http::header::HeaderValue::from_bytes(s.as_bytes())
                                .map_err(|e| {
                                    PyValueError::new_err(format!("header-value-error: {e}"))
                                })?;
                            hm.append(&header_name, header_value);
                        }
                    }
                }
            }
            Ok(HttpHeaderMap::from(hm))
        } else {
            py_type_err!("Expected a dict for HttpHeaderMap")
        }
    }
}

impl PyHeadersLike {
    pub fn map2headers(d: &HashMap<String, StringOrStrings>) -> PyResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (k, v) in d {
            match v {
                StringOrStrings::String(s) => {
                    let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                        .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
                    let header_value = http::header::HeaderValue::from_str(s)
                        .map_err(|e| PyValueError::new_err(format!("header-value-error: {e}")))?;
                    headers.insert(header_name, header_value);
                }
                StringOrStrings::Strings(v) => {
                    let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                        .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
                    for s in v {
                        let header_value = http::header::HeaderValue::from_str(s).map_err(|e| {
                            PyValueError::new_err(format!("header-value-error: {e}"))
                        })?;
                        headers.append(&header_name, header_value);
                    }
                }
            }
        }
        Ok(headers)
    }
}

impl From<PyHeadersLike> for HeaderMap {
    // type Error = PyErr;
    fn from(h: PyHeadersLike) -> Self {
        match h {
            PyHeadersLike::Headers(h) => h.read().clone(),
            PyHeadersLike::Map(d) =>  d.into()
        }
    }
}

impl From<PyHeadersLike> for PyHeaders {
    fn from(h: PyHeadersLike) -> Self {
        match h {
            PyHeadersLike::Headers(h) => h,
            PyHeadersLike::Map(d) => {
                Self::from(d)
            }
        }
    }
}
