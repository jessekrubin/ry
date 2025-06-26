#![doc = include_str!("../README.md")]

mod minify;
pub mod orjson;
mod serialize;

pub use serialize::{dumps, stringify};

use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn py_submod_register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serialize::stringify, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::dumps, m)?)?;
    m.add_function(wrap_pyfunction!(minify::minify, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::parse, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::loads, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::cache_usage, m)?)?;
    Ok(())
}
#[pymodule(gil_used = false, name = "JSON", submodule)]
pub fn json_py_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    py_submod_register(m)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // root level registration
    m.add_function(wrap_pyfunction!(orjson::orjson_default, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::stringify, m)?)?;
    // m.add_wrapped(wrap_pymodule!(orjson::oj))?;
    Ok(())
}
