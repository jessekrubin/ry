#![doc = include_str!("../README.md")]
//! Wrapper for jiter based on `jiter-python`
//!
//! Provides jitter wrapper that uses `PyBackedStr` and `PyBackedBytes` and
//! allows for parsing json from bytes or str (which jiter-python does not as
//! of [2024-05-29])
use std::path::PathBuf;
mod py_json_decode_error;
pub use ::jiter::{FloatMode, PartialMode, PythonParse, StringCacheMode};
pub use py_json_decode_error::RyJSONDecodeError;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyList;
mod parse_options;
pub use parse_options::JiterParseOptions;

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

    pub fn parse<'py>(
        self,
        py: Python<'py>,
        data: &[u8],
    ) -> Result<Bound<'py, PyAny>, RyJSONDecodeError> {
        self.parser()
            .python_parse(py, data)
            .map_err(|e| RyJSONDecodeError::new(data, e))
    }

    // TODO: this could be an iterable that has options or generics to control
    //       return type and avoid stupid python token eg [data, ...] vs [ data | err , ...]
    pub fn parse_lines<'py>(self, py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
        let parser = self.parser();
        // parse each line and collect into a Vec
        let mut parsed_lines = Vec::new();
        let mut line_start = 0;
        for line in data.split(|b| *b == b'\n') {
            let offset = line_start;
            line_start += line.len() + 1;
            if line.is_empty() {
                continue;
            }
            let parsed = parser.python_parse(py, line).map_err(|mut e| {
                // error index relative to the whole document
                e.index += offset;
                RyJSONDecodeError::new(data, e)
            })?;
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
    ),
    text_signature = "(data, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
)]
pub fn parse_json<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions::new()
        .with_allow_inf_nan(allow_inf_nan)
        .with_cache_mode(cache_mode)
        .with_partial_mode(partial_mode)
        .with_catch_duplicate_keys(catch_duplicate_keys);
    // TODO: use fast_str_read at somepoint?
    if let Ok(py_str) = data.cast_exact::<pyo3::types::PyString>() {
        let json_bytes = py_str.to_str()?.as_bytes();
        Ok(options.parse(py, json_bytes)?)
    } else if let Ok(bytes) = data.extract::<ryo3_bytes::ReadableBuffer>() {
        let json_bytes = bytes.as_slice();
        Ok(options.parse(py, json_bytes)?)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes, bytearray, str, or buffer",
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
    ),
    text_signature = "(data, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
)]
pub fn parse_jsonl<'py>(
    py: Python<'py>,
    data: &Bound<'py, PyAny>,
    allow_inf_nan: bool,
    cache_mode: StringCacheMode,
    partial_mode: PartialMode,
    catch_duplicate_keys: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let options = JiterParseOptions::new()
        .with_allow_inf_nan(allow_inf_nan)
        .with_cache_mode(cache_mode)
        .with_partial_mode(partial_mode)
        .with_catch_duplicate_keys(catch_duplicate_keys);
    if let Ok(py_str) = data.cast_exact::<pyo3::types::PyString>() {
        let json_bytes = py_str.to_str()?.as_bytes();
        options.parse_lines(py, json_bytes)
    } else if let Ok(bytes) = data.extract::<ryo3_bytes::ReadableBuffer>() {
        let json_bytes = bytes.as_slice();
        options.parse_lines(py, json_bytes)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes, bytearray, str, or buffer",
        ))
    }
}

// creates a function with the given name for use in root module ('parse_json`)
macro_rules! py_parse_fn {
    ($name:ident) => {
        #[pyfunction(
            signature = (data, /, *, allow_inf_nan = false, cache_mode = StringCacheMode::All, partial_mode = PartialMode::Off, catch_duplicate_keys = false),
            text_signature = "(data, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
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
    ),
    text_signature = "(p, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False, lines=False)"
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
    let options = JiterParseOptions::new()
        .with_allow_inf_nan(allow_inf_nan)
        .with_cache_mode(cache_mode)
        .with_partial_mode(partial_mode)
        .with_catch_duplicate_keys(catch_duplicate_keys);
    if lines {
        options.parse_lines(py, &fbytes)
    } else {
        let parsed = options.parser().python_parse(py, &fbytes);
        // move the file bytes into the error (no copy)
        Ok(parsed.map_err(|e| RyJSONDecodeError::from((e, ryo3_bytes::Bytes::from(fbytes))))?)
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
    m.add_class::<RyJSONDecodeError>()?;
    Ok(())
}
