//! TODO: implement this wrapper!

mod py_regex;

pub use crate::py_regex::PyRegex;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRegex>()?;
    Ok(())
}
