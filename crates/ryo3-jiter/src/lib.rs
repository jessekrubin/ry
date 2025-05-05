#![doc = include_str!("../README.md")]
//! Wrapper for jiter based on `jiter-python`
//!
//! Provides jitter wrapper that uses `PyBackedStr` and `PyBackedBytes` and
//! allows for parsing json from bytes or str (which jiter-python does not as
//! of [2024-05-29])
use std::path::PathBuf;

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
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        parse_builder
            .python_parse(py, bytes)
            .map_err(|e| map_json_error(bytes, &e))
    } else if let Ok(custom) = data.downcast::<ryo3_bytes::PyBytes>() {
        let pybytes = custom.get();
        let json_bytes = pybytes.as_ref();
        parse_builder
            .python_parse(py, json_bytes)
            .map_err(|e| map_json_error(json_bytes, &e))
    } else if let Ok(pybytes) = data.extract::<ryo3_bytes::PyBytes>() {
        let json_bytes = pybytes.as_ref();
        parse_builder
            .python_parse(py, json_bytes)
            .map_err(|e| map_json_error(json_bytes, &e))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes, bytearray, or pyo3-bytes object",
        ))
    }
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
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    // let data = extract_bytes_ref(data)?;
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        parse_builder
            .python_parse(py, bytes)
            .map_err(|e| map_json_error(bytes, &e))
    } else if let Ok(custom) = data.downcast::<ryo3_bytes::PyBytes>() {
        let pybytes = custom.get();
        let json_bytes = pybytes.as_ref();
        parse_builder
            .python_parse(py, json_bytes)
            .map_err(|e| map_json_error(json_bytes, &e))
    } else if let Ok(pybytes) = data.extract::<ryo3_bytes::PyBytes>() {
        let json_bytes = pybytes.as_ref();
        parse_builder
            .python_parse(py, json_bytes)
            .map_err(|e| map_json_error(json_bytes, &e))
    } else if let Ok(s) = data.extract::<&str>() {
        let json_bytes = s.as_bytes();
        parse_builder
            .python_parse(py, json_bytes)
            .map_err(|e| map_json_error(json_bytes, &e))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes-like, bytearray, pyo3-bytes object or str",
        ))
    }
}

// #[pyfunction(
//     signature = (
//         data,
//         /,
//         *,
//         allow_inf_nan = false,
//         cache_mode = StringCacheMode::All,
//         partial_mode = PartialMode::Off,
//         catch_duplicate_keys = false,
//         float_mode = FloatMode::Float
//     )
// )]
// pub fn parse_jsonl<'py>(
//     py: Python<'py>,
//     data: &'py Bound<'py, PyAny>,
//     allow_inf_nan: bool,
//     cache_mode: StringCacheMode,
//     partial_mode: PartialMode,
//     catch_duplicate_keys: bool,
//     float_mode: FloatMode,
// ) -> PyResult<Bound<'py, PyAny>> {
//     let json_bytes: &'py [u8] = extract_bytes_ref_str(data)?;
//     let  parsed_lines= json_bytes
//         .split(|b| *b == b'\n')
//         .filter(|line| !line.is_empty()).map(|line| {
//             let parse_builder = PythonParse {
//                 allow_inf_nan,
//                 cache_mode,
//                 partial_mode,
//                 catch_duplicate_keys,
//                 float_mode,
//             };
//             parse_builder
//                 .python_parse(py, line)
//                 .map_err(|e| map_json_error(line, &e))
//         })
//         .collect::<Result<Vec<_>, _>>()?;

//     let pylist = PyList::new(py, parsed_lines)?;
//     // parse each line
//     let a = pylist.into_bound_py_any(py);
//     a
// }
#[pyfunction(
    signature = (
        p,
        /,
        *,
        allow_inf_nan = false,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float
    )
)]
pub fn read_json(
    py: Python<'_>,
    p: PathBuf,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'_, PyAny>> {
    let fbytes = std::fs::read(p)?;
    let parse_builder = PythonParse {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    parse_builder
        .python_parse(py, &fbytes)
        .map_err(|e| map_json_error(&fbytes, &e))
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
    m.add_function(wrap_pyfunction!(read_json, m)?)?;
    Ok(())
}
