#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
mod headers;
mod headers_like;
mod http_types;
mod py_conversions;
mod status_code;

#[cfg(feature = "json")]
mod http_serde;

pub use headers::PyHeaders;
pub use headers_like::PyHeadersLike;
pub use http_types::{
    HttpHeaderMap, HttpHeaderName, HttpHeaderNameRef, HttpHeaderValue, HttpMethod, HttpStatusCode,
    HttpVersion,
};
pub use status_code::{status_code_pystring, PyHttpStatus};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyHeaders>()?;
    m.add_class::<PyHttpStatus>()?;
    Ok(())
}
