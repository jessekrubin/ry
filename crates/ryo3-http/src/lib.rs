#![doc = include_str!("../README.md")]
use pyo3::{prelude::*, types::PyMapping};
mod headers;
mod headers_like;
mod http_types;
mod py_conversions;
mod status_code;

#[cfg(feature = "serde")]
mod http_serde;

pub use headers::PyHeaders;
pub use headers_like::PyHeadersLike;
pub use http_types::{
    HttpHeaderMap, HttpHeaderName, HttpHeaderNameRef, HttpHeaderValue, HttpMethod, HttpStatusCode,
    HttpVersion,
};
pub use status_code::{PyHttpStatus, status_code_pystring};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyHeaders>()?;
    // PyHeaders::type_object_raw(py)
    // let pyheaders_type = m.py().::<PyHeaders>();
    PyMapping::register::<PyHeaders>(m.py())?;
    m.add_class::<PyHttpStatus>()?;
    Ok(())
}
