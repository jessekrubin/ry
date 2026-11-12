#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
mod ry_url;
mod url_like;
pub use ry_url::RyUrl;
pub use url_like::UrlLike;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyUrl>()?;
    Ok(())
}
