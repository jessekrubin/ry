use ::jiter::map_json_error;
use ::jiter::python_parse;
use ::jiter::StringCacheMode;
use pyo3::prelude::*;
use pyo3::pybacked::{PyBackedBytes, PyBackedStr};

#[derive(FromPyObject)]
pub enum BytesOrString {
    Str(PyBackedStr),
    Bytes(PyBackedBytes),
}

#[pyfunction(signature = (data, *, allow_inf_nan = true, cache_strings = true, allow_partial = false))]
pub fn parse_json_bytes<'py>(
    py: Python<'py>,
    data: &[u8],
    allow_inf_nan: bool,
    cache_strings: bool,
    allow_partial: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let json_bytes = data;
    let cache_mode = if cache_strings {
        StringCacheMode::All
    } else {
        StringCacheMode::None
    };
    python_parse(py, json_bytes, allow_inf_nan, cache_mode, allow_partial)
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
    let cache_mode = if cache_strings {
        StringCacheMode::All
    } else {
        StringCacheMode::None
    };
    python_parse(py, json_bytes, allow_inf_nan, cache_mode, false)
        .map_err(|e| map_json_error(json_bytes, &e))
        .map(|v| v.into_py(py))
}

#[pyfunction(signature = (data, *, allow_inf_nan = true, cache_strings = true, allow_partial = false))]
pub fn parse_json(
    py: Python<'_>,
    data: BytesOrString,
    allow_inf_nan: bool,
    cache_strings: bool,
    allow_partial: bool,
) -> PyResult<Bound<'_, PyAny>> {
    let cache_mode = if cache_strings {
        StringCacheMode::All
    } else {
        StringCacheMode::None
    };

    // Directly call python_parse within the match arms
    match data {
        BytesOrString::Str(s) => {
            let json_bytes: &[u8] = s.as_ref();
            python_parse(py, json_bytes, allow_inf_nan, cache_mode, allow_partial)
                .map_err(|e| map_json_error(json_bytes, &e))
        }
        BytesOrString::Bytes(b) => {
            let json_bytes: &[u8] = b.as_ref();
            python_parse(py, json_bytes, allow_inf_nan, cache_mode, allow_partial)
                .map_err(|e| map_json_error(json_bytes, &e))
        }
    }
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    Ok(())
}
