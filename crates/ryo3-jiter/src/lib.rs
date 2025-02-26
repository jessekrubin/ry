#![doc = include_str!("../README.md")]
//! Wrapper for jiter based on `jiter-python`
//!
//! Provides jitter wrapper that uses `PyBackedStr` and `PyBackedBytes` and
//! allows for parsing json from bytes or str (which jiter-python does not as
//! of [2024-05-29])
use ::jiter::{
    cache_clear, cache_usage, map_json_error, PartialMode, PythonParse, StringCacheMode,
};
use jiter::FloatMode;
use pyo3::prelude::*;
use pyo3::pybacked::{PyBackedBytes, PyBackedStr};
use ryo3_bytes::{extract_bytes_ref, extract_bytes_ref_str};

#[derive(FromPyObject)]
pub enum BytesOrString {
    Str(PyBackedStr),
    Bytes(PyBackedBytes),
}

#[pyfunction(
    signature = (
        data,
        /,
        *,
        allow_inf_nan = false,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float
    )
)]
pub fn parse_json_bytes<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    let data = extract_bytes_ref(data)?;
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    parse_builder
        .python_parse(py, data)
        .map_err(|e| map_json_error(data, &e))
}

#[pyfunction(
    signature = (
        data,
        /,
        *,
        allow_inf_nan = false,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float
    )
)]
pub fn parse_json<'py>(
    py: Python<'py>,
    data: &'py Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    let json_bytes: &'py [u8] = extract_bytes_ref_str(data)?;

    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    parse_builder
        .python_parse(py, json_bytes)
        .map_err(|e| map_json_error(json_bytes, &e))
}

#[pyfunction]
pub fn json_cache_clear() {
    cache_clear();
}

#[pyfunction]
#[must_use]
pub fn json_cache_usage() -> usize {
    cache_usage()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_usage, m)?)?;
    Ok(())
}
