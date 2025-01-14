use crate::PyHeaders;
use http::HeaderMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, FromPyObject)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(HashMap<String, String>),
}

impl PyHeadersLike {
    pub fn map2headers(d: &HashMap<String, String>) -> PyResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (k, v) in d {
            let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
            let header_value = http::header::HeaderValue::from_str(v)
                .map_err(|e| PyValueError::new_err(format!("header-value-error: {e}")))?;
            headers.insert(header_name, header_value);
        }
        Ok(headers)
    }
}

impl TryFrom<PyHeadersLike> for HeaderMap {
    type Error = PyErr;
    fn try_from(h: PyHeadersLike) -> Result<Self, Self::Error> {
        match h {
            PyHeadersLike::Headers(h) => Ok(h.0),
            PyHeadersLike::Map(d) => {
                let mut default_headers = HeaderMap::new();
                for (k, v) in d {
                    let k = k.to_string();
                    let v = v.to_string();
                    let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                        .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
                    let header_value = http::header::HeaderValue::from_str(&v)
                        .map_err(|e| PyValueError::new_err(format!("header-value-error: {e}")))?;
                    default_headers.insert(header_name, header_value);
                }
                Ok(default_headers)
            }
        }
    }
}
