#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyMapping;
mod headers_like;
mod http_types;
mod py_conversions;
mod ry_headers;
mod ry_http_status;

#[cfg(feature = "serde")]
mod serde;

pub use headers_like::PyHeadersLike;
pub use http_types::{
    PyHttpHeaderMap, PyHttpHeaderName, PyHttpHeaderNameRef, PyHttpHeaderValue, PyHttpMethod,
    PyHttpVersion,
};
pub use ry_headers::RyHeaders;
pub use ry_http_status::{RyHttpStatus, status_code_pystring};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyHeaders>()?;
    PyMapping::register::<RyHeaders>(m.py())?;
    m.add_class::<RyHttpStatus>()?;
    Ok(())
}
