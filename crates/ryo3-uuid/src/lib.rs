#![doc = include_str!("../README.md")]
mod py_uuid;
pub use py_uuid::{getnode, uuid1, uuid2, uuid3, uuid4, uuid5, uuid6, uuid7, uuid8, PyUuid};
use pyo3::prelude::PyModule;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUuid>()?;
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid2, m)?)?;
    m.add_function(wrap_pyfunction!(uuid3, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid5, m)?)?;
    m.add_function(wrap_pyfunction!(uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(uuid8, m)?)?;
    m.add_function(wrap_pyfunction!(getnode, m)?)?;

    Ok(())
}
