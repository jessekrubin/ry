#![doc = include_str!("../README.md")]
mod py_uuid;
pub use py_uuid::{uuid4, PyUuid};
use pyo3::prelude::PyModule;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUuid>()?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    Ok(())
}
