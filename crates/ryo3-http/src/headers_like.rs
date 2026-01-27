use crate::{PyHeaders, PyHttpHeaderMap};
use http::HeaderMap;
use pyo3::prelude::*;
use ryo3_core::{py_type_err, py_value_error};
use std::collections::HashMap;
#[derive(Debug, FromPyObject)]
pub(crate) enum StringOrStrings {
    String(pyo3::pybacked::PyBackedStr),
    Strings(Vec<pyo3::pybacked::PyBackedStr>),
}

#[derive(Debug, Clone)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(PyHttpHeaderMap),
}

// TODO: move this to conversions module
impl<'py> FromPyObject<'_, 'py> for PyHttpHeaderMap {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(d) = ob.cast_exact::<pyo3::types::PyDict>() {
            let extracted = d.extract::<HashMap<String, StringOrStrings>>()?;
            let mut hm: http::HeaderMap = HeaderMap::with_capacity(extracted.len());
            for (k, v) in extracted {
                match v {
                    StringOrStrings::String(s) => {
                        let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                            .map_err(|e| py_value_error!("header-name-error: {e}"))?;
                        let header_value = http::header::HeaderValue::from_bytes(s.as_bytes())
                            .map_err(|e| py_value_error!("header-value-error: {e}"))?;
                        hm.insert(header_name, header_value);
                    }
                    StringOrStrings::Strings(v) => {
                        let header_name = http::header::HeaderName::from_bytes(k.as_bytes())
                            .map_err(|e| py_value_error!("header-name-error: {e}"))?;
                        for s in v {
                            let header_value = http::header::HeaderValue::from_bytes(s.as_bytes())
                                .map_err(|e| py_value_error!("header-value-error: {e}"))?;
                            hm.append(&header_name, header_value);
                        }
                    }
                }
            }
            Ok(Self::from(hm))
        } else {
            py_type_err!("Expected a dict for HttpHeaderMap")
        }
    }
}

impl From<PyHeadersLike> for HeaderMap {
    fn from(h: PyHeadersLike) -> Self {
        match h {
            PyHeadersLike::Headers(h) => h.read().clone(),
            PyHeadersLike::Map(d) => d.into(),
        }
    }
}

impl From<PyHeadersLike> for PyHeaders {
    fn from(h: PyHeadersLike) -> Self {
        match h {
            PyHeadersLike::Headers(h) => h,
            PyHeadersLike::Map(d) => Self::from(d),
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for PyHeadersLike {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(h) = obj.cast_exact::<PyHeaders>() {
            Ok(Self::Headers(h.get().clone()))
        } else if let Ok(d) = obj.extract::<PyHttpHeaderMap>() {
            Ok(Self::Map(d))
        } else {
            py_type_err!("Expected Headers or dict[str, str | list[str]]")
        }
    }
}
