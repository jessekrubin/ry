#![doc = include_str!("../README.md")]

mod fns;
mod ry_size;
mod ry_size_formatter;
mod types;

pub use fns::*;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ry_size_formatter::RySizeFormatter>()?;
    m.add_class::<ry_size::RySize>()?;
    m.add_function(wrap_pyfunction!(parse_size, m)?)?;
    m.add_function(wrap_pyfunction!(fmt_size, m)?)?;
    Ok(())
}
