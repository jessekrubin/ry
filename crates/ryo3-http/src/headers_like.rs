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

impl From<PyHeadersLike> for HeaderMap {
    fn from(h: PyHeadersLike) -> Self {
        match h {
            PyHeadersLike::Headers(h) => h.0,
            PyHeadersLike::Map(d) => {
                let mut default_headers = HeaderMap::new();
                for (k, v) in d {
                    let k = k.to_string();
                    let v = v.to_string();
                    let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                        .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))
                        .unwrap();
                    let header_value = http::header::HeaderValue::from_str(&v)
                        .map_err(|e| PyValueError::new_err(format!("header-value-error: {e}")))
                        .unwrap();
                    default_headers.insert(header_name, header_value);
                }
                default_headers
            }
        }
    }
}
