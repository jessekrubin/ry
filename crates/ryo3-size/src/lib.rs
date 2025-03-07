#![doc = include_str!("../README.md")]

mod fns;
mod py_size;
mod size_formatter;
mod types;

pub use fns::*;
use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<size_formatter::PySizeFormatter>()?;
    m.add_class::<py_size::PySize>()?;
    m.add_function(wrap_pyfunction!(parse_size, m)?)?;
    m.add_function(wrap_pyfunction!(fmt_size, m)?)?;
    Ok(())
}
