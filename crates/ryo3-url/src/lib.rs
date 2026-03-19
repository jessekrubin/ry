#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
mod py_url;
mod url_like;
pub use py_url::PyUrl;
pub use url_like::UrlLike;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUrl>()?;
    Ok(())
}
