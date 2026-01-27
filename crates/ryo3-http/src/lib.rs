#![doc = include_str!("../README.md")]
use pyo3::{prelude::*, types::PyMapping};
mod headers;
mod headers_like;
mod http_types;
mod py_conversions;
mod py_http_status;

#[cfg(feature = "serde")]
mod http_serde;

pub use headers::PyHeaders;
pub use headers_like::PyHeadersLike;
pub use http_types::{
    PyHttpHeaderMap, PyHttpHeaderName, PyHttpHeaderNameRef, PyHttpHeaderValue, PyHttpMethod,
    PyHttpVersion,
};
pub use py_http_status::{PyHttpStatus, status_code_pystring};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyHeaders>()?;
    PyMapping::register::<PyHeaders>(m.py())?;
    m.add_class::<PyHttpStatus>()?;
    Ok(())
}
