#![doc = include_str!("../README.md")]

mod py_url;
mod url_like;

pub use py_url::PyUrl;
pub use url_like::{UrlLike, extract_url};

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult};
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUrl>()?;
    Ok(())
}
