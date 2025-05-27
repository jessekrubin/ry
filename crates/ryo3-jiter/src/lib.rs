#![doc = include_str!("../README.md")]
//! Wrapper for jiter based on `jiter-python`
//!
//! Provides jitter wrapper that uses `PyBackedStr` and `PyBackedBytes` and
//! allows for parsing json from bytes or str (which jiter-python does not as
//! of [2024-05-29])
use std::path::PathBuf;

use ::jiter::{
    cache_clear, cache_usage, map_json_error, FloatMode, PartialMode, PythonParse, StringCacheMode,
};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::IntoPyObjectExt;

#[derive(Debug, Clone, Copy)]
pub struct JiterParseOptions {
    pub allow_inf_nan: bool,
    pub cache_mode: StringCacheMode,
    pub partial_mode: PartialMode,
    pub catch_duplicate_keys: bool,
    pub float_mode: FloatMode,
}

impl Default for JiterParseOptions {
    fn default() -> Self {
        JiterParseOptions {
            allow_inf_nan: false,
            cache_mode: StringCacheMode::All,
            partial_mode: PartialMode::Off,
            catch_duplicate_keys: false,
            float_mode: FloatMode::Float,
        }
    }
}

impl JiterParseOptions {
    fn parser(self) -> PythonParse {
        PythonParse {
            allow_inf_nan: self.allow_inf_nan,
            cache_mode: self.cache_mode,
            partial_mode: self.partial_mode,
            catch_duplicate_keys: self.catch_duplicate_keys,
            float_mode: self.float_mode,
        }
    }

    fn parse<'py>(self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        self.parser()
            .python_parse(py, data)
            .map_err(|e| map_json_error(data, &e))
    }

    fn parse_lines<'py>(self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let lines_iter = data.split(|b| *b == b'\n').filter(|line| !line.is_empty());
        let parsed_lines = lines_iter
            .map(|line| self.parse(py, line))
            .collect::<Result<Vec<_>, _>>()?;
        let pylist = PyList::new(py, parsed_lines)?;
        pylist.into_bound_py_any(py)
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
pub fn parse_json_bytes<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        options.parse(py, bytes)
    } else if let Ok(custom) = data.downcast::<ryo3_bytes::PyBytes>() {
        let pybytes = custom.get();
        let json_bytes = pybytes.as_ref();
        options.parse(py, json_bytes)
    } else if let Ok(pybytes) = data.extract::<ryo3_bytes::PyBytes>() {
        let json_bytes = pybytes.as_ref();
        options.parse(py, json_bytes)
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
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        options.parse(py, bytes)
    } else if let Ok(s) = data.extract::<&str>() {
        let json_bytes = s.as_bytes();
        options.parse(py, json_bytes)
    } else if let Ok(custom) = data.downcast::<ryo3_bytes::PyBytes>() {
        let pybytes = custom.get();
        let json_bytes = pybytes.as_slice();
        options.parse(py, json_bytes)
    } else if let Ok(pybytes) = data.extract::<ryo3_bytes::PyBytes>() {
        let json_bytes = pybytes.as_slice();
        options.parse(py, json_bytes)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes-like, bytearray, pyo3-bytes object or str",
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
pub fn parse_jsonl<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        options.parse_lines(py, bytes)
    } else if let Ok(custom) = data.downcast::<ryo3_bytes::PyBytes>() {
        let pybytes = custom.get();
        let json_bytes = pybytes.as_ref();
        options.parse_lines(py, json_bytes)
    } else if let Ok(pybytes) = data.extract::<ryo3_bytes::PyBytes>() {
        let json_bytes = pybytes.as_ref();
        options.parse_lines(py, json_bytes)
    } else if let Ok(s) = data.extract::<&str>() {
        let json_bytes = s.as_bytes();
        options.parse_lines(py, json_bytes)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes-like, bytearray, pyo3-bytes object or str",
        ))
    }
}

#[pyfunction(
    signature = (
        p,
        /,
        *,
        allow_inf_nan = false,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        float_mode = FloatMode::Float,
        lines = false
    )
)]
#[expect(clippy::too_many_arguments)]
pub fn read_json(
    py: Python<'_>,
    p: PathBuf,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    float_mode: FloatMode,
    lines: bool,
) -> PyResult<Bound<'_, PyAny>> {
    let fbytes = std::fs::read(p)?;
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
        float_mode,
    };
    if lines {
        options.parse_lines(py, &fbytes)
    } else {
        options.parse(py, &fbytes)
    }
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
    m.add_function(wrap_pyfunction!(parse_jsonl, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_usage, m)?)?;
    m.add_function(wrap_pyfunction!(read_json, m)?)?;
    Ok(())
}
