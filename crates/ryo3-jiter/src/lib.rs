#![doc = include_str!("../README.md")]
//! Wrapper for jiter based on `jiter-python`
//!
//! Provides jitter wrapper that uses `PyBackedStr` and `PyBackedBytes` and
//! allows for parsing json from bytes or str (which jiter-python does not as
//! of [2024-05-29])
use ::jiter::{FloatMode, PartialMode, PythonParse, StringCacheMode, map_json_error};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct JiterParseOptions {
    pub allow_inf_nan: bool,
    pub cache_mode: StringCacheMode,
    pub partial_mode: PartialMode,
    pub catch_duplicate_keys: bool,
}

impl Default for JiterParseOptions {
    fn default() -> Self {
        Self {
            allow_inf_nan: false,
            cache_mode: StringCacheMode::All,
            partial_mode: PartialMode::Off,
            catch_duplicate_keys: false,
        }
    }
}

impl From<&JiterParseOptions> for PythonParse {
    fn from(options: &JiterParseOptions) -> Self {
        Self {
            allow_inf_nan: options.allow_inf_nan,
            cache_mode: options.cache_mode,
            partial_mode: options.partial_mode,
            catch_duplicate_keys: options.catch_duplicate_keys,
            float_mode: FloatMode::Float,
        }
    }
}

impl JiterParseOptions {
    #[must_use]
    pub fn parser(self) -> PythonParse {
        PythonParse::from(&self)
    }

    fn parse<'py>(self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        self.parser()
            .python_parse(py, data)
            .map_err(|e| map_json_error(data, &e))
    }

    fn parse_lines<'py>(self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let lines_iter = data.split(|b| *b == b'\n').filter(|line| !line.is_empty());
        let parser = self.parser();
        // parse each line and collect into a Vec
        let mut parsed_lines = Vec::new();
        for line in lines_iter.clone() {
            let parsed = parser
                .python_parse(py, line)
                .map_err(|e| map_json_error(line, &e))?;
            parsed_lines.push(parsed);
        }
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
    )
)]
pub fn parse_json<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        options.parse(py, bytes)
    } else if let Ok(s) = data.extract::<&str>() {
        let json_bytes = s.as_bytes();
        options.parse(py, json_bytes)
    } else if let Ok(custom) = data.cast::<ryo3_bytes::PyBytes>() {
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
    )
)]
pub fn parse_jsonl<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
    };
    if let Ok(bytes) = data.extract::<&[u8]>() {
        options.parse_lines(py, bytes)
    } else if let Ok(custom) = data.cast::<ryo3_bytes::PyBytes>() {
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

// creates a function with the given name for use in root module ('parse_json`)
macro_rules! py_parse_fn {
    ($name:ident) => {
        #[pyfunction(
            signature = (data, /, *, allow_inf_nan = false, cache_mode = StringCacheMode::All, partial_mode = PartialMode::Off, catch_duplicate_keys = false)
        )]
        pub fn $name<'py>(
            py: Python<'py>,
            data: &Bound<'py, PyAny>,
            allow_inf_nan: bool,
            cache_mode: StringCacheMode,
            partial_mode: PartialMode,
            catch_duplicate_keys: bool,
        ) -> PyResult<Bound<'py, PyAny>> {
            parse_json(
                py,
                data,
                allow_inf_nan,
                cache_mode,
                partial_mode,
                catch_duplicate_keys,
            )
        }
    };
}
py_parse_fn!(parse);
py_parse_fn!(loads);

#[pyfunction(
    signature = (
        p,
        /,
        *,
        allow_inf_nan = false,
        cache_mode = StringCacheMode::All,
        partial_mode = PartialMode::Off,
        catch_duplicate_keys = false,
        lines = false
    )
)]
pub fn read_json(
    py: Python<'_>,
    p: PathBuf,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
    lines: bool,
) -> PyResult<Bound<'_, PyAny>> {
    let fbytes = std::fs::read(p)?;
    let options = JiterParseOptions {
        allow_inf_nan,
        cache_mode,
        partial_mode,
        catch_duplicate_keys,
    };
    if lines {
        options.parse_lines(py, &fbytes)
    } else {
        options.parse(py, &fbytes)
    }
}

macro_rules! py_cache_clear_fn {
    ($name:ident) => {
        #[pyfunction]
        pub fn $name() {
            ::jiter::cache_clear();
        }
    };
}

py_cache_clear_fn!(json_cache_clear);
py_cache_clear_fn!(cache_clear);

macro_rules! py_cache_usage_fn {
    ($name:ident) => {
        #[pyfunction]
        #[must_use]
        pub fn $name() -> usize {
            ::jiter::cache_usage()
        }
    };
}
py_cache_usage_fn!(json_cache_usage);
py_cache_usage_fn!(cache_usage);

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_jsonl, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_clear, m)?)?;
    m.add_function(wrap_pyfunction!(json_cache_usage, m)?)?;
    m.add_function(wrap_pyfunction!(read_json, m)?)?;
    Ok(())
}
