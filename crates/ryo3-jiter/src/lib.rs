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
        allow_inf_nan = true,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float
    )
)]
pub fn parse_json_bytes<'py>(
    py: Python<'py>,
    data: &[u8],
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
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
        allow_inf_nan = true,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float,
    )
)]
pub fn parse_json_str<'py>(
    py: Python<'py>,
    data: &str,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    let json_bytes: &[u8] = data.as_ref();
    parse_builder
        .python_parse(py, json_bytes)
        .map_err(|e| map_json_error(json_bytes, &e))
}

#[allow(clippy::fn_params_excessive_bools)]
#[pyfunction(
    signature = (
        data,
        /,
        *,
        allow_inf_nan = true,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float
    )
)]
pub fn parse_json(
    py: Python<'_>,
    data: BytesOrString,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'_, PyAny>> {
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    match data {
        BytesOrString::Str(s) => {
            let json_bytes: &[u8] = s.as_ref();
            parse_builder
                .python_parse(py, json_bytes)
                .map_err(|e| map_json_error(json_bytes, &e))
        }
        BytesOrString::Bytes(b) => {
            let json_bytes: &[u8] = b.as_ref();

            parse_builder
                .python_parse(py, json_bytes)
                .map_err(|e| map_json_error(json_bytes, &e))
        }
    }
}

#[pyfunction]
pub fn jiter_cache_clear() {
    cache_clear();
}

#[pyfunction]
#[must_use]
pub fn jiter_cache_usage() -> usize {
    cache_usage()
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_str, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(jiter_cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(jiter_cache_usage, m)?)?;
    Ok(())
}
