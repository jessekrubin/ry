use crate::PyHeaders;
use http::HeaderMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_core::types::StringOrStrings;
use std::collections::HashMap;

#[derive(Debug, Clone, FromPyObject)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(HashMap<String, StringOrStrings>),
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

impl TryFrom<PyHeadersLike> for HeaderMap {
    type Error = PyErr;
    fn try_from(h: PyHeadersLike) -> Result<Self, Self::Error> {
        match h {
            PyHeadersLike::Headers(h) => Ok(h.0.lock().clone()),
            PyHeadersLike::Map(d) => PyHeadersLike::map2headers(&d)
                .map_err(|e| PyValueError::new_err(format!("header-map-error: {e}"))),
        }
    }
}

impl TryFrom<PyHeadersLike> for PyHeaders {
    type Error = PyErr;
    fn try_from(h: PyHeadersLike) -> Result<Self, Self::Error> {
        match h {
            PyHeadersLike::Headers(h) => Ok(h),
            PyHeadersLike::Map(d) => {
                let headers = PyHeadersLike::map2headers(&d)?;
                Ok(Self::from(headers))
            }
        }
    }
}
