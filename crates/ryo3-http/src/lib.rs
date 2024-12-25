#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
mod headers;
mod http_types;
mod py_conversions;

pub use http_types::{HttpHeaderName, HttpHeaderValue, HttpMethod};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<headers::PyHeaders>()?;
    Ok(())
}
