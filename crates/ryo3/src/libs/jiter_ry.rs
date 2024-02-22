use ::jiter::map_json_error;
use ::jiter::python_parse;
use pyo3::prelude::*;
use std::fmt::Debug;

use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
pub enum BytesOrString<'a> {
    Str(&'a str),
    Bytes(&'a [u8]),
}

#[pyfunction(signature = (data, *, allow_inf_nan = true, cache_strings = true))]
pub fn parse_json_bytes(
    py: Python,
    data: &[u8],
    allow_inf_nan: bool,
    cache_strings: bool,
) -> PyResult<PyObject> {
    let json_bytes = data;
    python_parse(py, json_bytes, allow_inf_nan, cache_strings)
        .map_err(|e| map_json_error(json_bytes, &e))
}

#[pyfunction(signature = (data, *, allow_inf_nan = true, cache_strings = true))]
pub fn parse_json_str(
    py: Python,
    data: &str,
    allow_inf_nan: bool,
    cache_strings: bool,
) -> PyResult<PyObject> {
    let json_bytes = data.as_bytes();
    python_parse(py, json_bytes, allow_inf_nan, cache_strings)
        .map_err(|e| map_json_error(json_bytes, &e))
}

#[pyfunction(signature = (data, *, allow_inf_nan = true, cache_strings = true))]
pub fn parse_json(
    py: Python,
    data: BytesOrString,
    allow_inf_nan: bool,
    cache_strings: bool,
) -> PyResult<PyObject> {
    let json_bytes = match data {
        BytesOrString::Str(s) => s.as_bytes(),
        BytesOrString::Bytes(b) => b,
    };
    python_parse(py, json_bytes, allow_inf_nan, cache_strings)
        .map_err(|e| map_json_error(json_bytes, &e))
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    Ok(())
}
