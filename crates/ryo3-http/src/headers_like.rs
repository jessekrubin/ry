use http::HeaderMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use ryo3_core::macros::py_type_err;
use ryo3_core::{FromPyOperand, PyCastExactOpt};

use crate::{PyHeaders, PyHttpHeaderMap};
#[derive(Debug, Clone)]
pub enum PyHeadersLike {
    Headers(PyHeaders),
    Map(PyHttpHeaderMap),
}

impl PyHeadersLike {
    #[must_use]
    pub fn into_header_map(self) -> HeaderMap {
        match self {
            Self::Headers(headers) => headers.into_header_map(),
            Self::Map(map) => map.into(),
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

impl FromPyOperand<'_, '_> for PyHeadersLike {
    fn from_py_operand(obj: Borrowed<'_, '_, PyAny>) -> Option<Self> {
        if let Some(h) = obj.cast_exact_opt::<PyHeaders>() {
            Some(Self::Headers(h.get().clone()))
        } else if obj.is_exact_instance_of::<PyDict>() {
            obj.extract::<PyHttpHeaderMap>().ok().map(Self::Map)
        } else {
            None
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
