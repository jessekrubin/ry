#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
mod headers;
mod headers_like;
mod http_types;
mod py_conversions;
mod status_code;

pub use headers::PyHeaders;
pub use headers_like::PyHeadersLike;
pub use http_types::{HttpHeaderName, HttpHeaderValue, HttpMethod, HttpStatusCode};
pub use status_code::PyHttpStatus;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyHeaders>()?;
    m.add_class::<PyHttpStatus>()?;
    Ok(())
}
