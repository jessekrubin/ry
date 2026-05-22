use http::HeaderMap;
use pyo3::prelude::*;
use ryo3_core::py_type_err;

use crate::{PyHeaders, PyHttpHeaderMap, PyHttpHeaderValue};
#[derive(Debug, FromPyObject)]
pub(crate) enum HeaderValueOrValues {
    One(PyHttpHeaderValue),
    Many(Vec<PyHttpHeaderValue>),
}

#[derive(Debug, Clone)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(PyHttpHeaderMap),
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
