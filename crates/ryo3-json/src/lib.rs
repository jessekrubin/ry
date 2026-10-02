#![doc = include_str!("../README.md")]

pub mod orjson;
mod ser;
mod ser_opts;
mod serialize;
mod serialize_v2;
mod transcode;

use pyo3::prelude::*;
pub use serialize::{dumps, stringify, to_vec};

pub fn py_submod_register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serialize::stringify, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_v2::stringify_v2, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::dumps, m)?)?;
    m.add_function(wrap_pyfunction!(transcode::minify, m)?)?;
    m.add_function(wrap_pyfunction!(transcode::fmt, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::parse, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::loads, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(ryo3_jiter::cache_usage, m)?)?;
    m.add_class::<ryo3_jiter::RyJSONDecodeError>()?;
    Ok(())
}

#[pymodule(gil_used = false, name = "JSON", submodule)]
pub fn json_py_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    py_submod_register(m)?;
    Ok(())
}

// pub fn pymod_add_experimental(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(serialize_v2::stringify_v2, m)?)?;
//     Ok(())
// }

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(orjson::orjson_default, m)?)?;
    m.add_function(wrap_pyfunction!(serialize::stringify, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_v2::stringify_v2, m)?)?;
    Ok(())
}
