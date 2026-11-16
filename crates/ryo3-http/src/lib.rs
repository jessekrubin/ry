#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyMapping;
mod http_types;
mod py_conversions;
mod py_headers_like;
mod ry_headers;
mod ry_http_status;

#[cfg(feature = "serde")]
mod serde;

pub use http_types::{
    PyHttpHeaderMap, PyHttpHeaderName, PyHttpHeaderNameRef, PyHttpHeaderValue, PyHttpMethod,
    PyHttpVersion,
};
pub use py_headers_like::PyHeadersLike;
pub use ry_headers::RyHeaders;
pub use ry_http_status::{RyHttpStatus, status_code_pystring};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyHeaders>()?;
    PyMapping::register::<RyHeaders>(m.py())?;
    m.add_class::<RyHttpStatus>()?;
    Ok(())
}
