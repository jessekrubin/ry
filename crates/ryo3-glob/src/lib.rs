#![doc = include_str!("../README.md")]
mod ry_glob_paths;
mod ry_glob_pattern;

use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyType;
use ryo3_core::macros::{py_value_err, py_value_error};
use ryo3_core::sync::RyMutex;

use crate::ry_glob_paths::GlobDType;
pub use crate::ry_glob_paths::RyGlobPaths;
pub use crate::ry_glob_pattern::RyGlobPattern;

/// Return iterator of paths that match the given pattern
///
/// Pattern syntax (as taken from the `glob` crate):
///
/// A compiled Unix shell style pattern.
///
/// - `?` matches any single character.
///
/// - `*` matches any (possibly empty) sequence of characters.
///
/// - `**` matches the current directory and arbitrary subdirectories. To match
///   files in arbitrary subdirectories, use `**/*`.
///
///   This sequence **must** form a single path component, so both
///   `**a` and `b**` are invalid and will result in an error.  A
///   sequence of more than two consecutive `*` characters is also
///   invalid.
///
/// - `[...]` matches any character inside the brackets.  Character sequences
///   can also specify ranges of characters, as ordered by Unicode, so e.g.
///   `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed
///   bracket is invalid.
///
/// - `[!...]` is the negation of `[...]`, i.e. it matches any characters
///   **not** in the brackets.
///
/// - The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets
///   (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it
///   is interpreted as being part of, rather then ending, the character set, so
///   `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`
///   character can be specified inside a character sequence pattern by placing
///   it at the start or the end, e.g. `[abc-]`.
#[expect(clippy::fn_params_excessive_bools)]
#[pyfunction]
#[pyo3(
    name = "glob",
    signature = (
        pattern,
        *,
        case_sensitive = true,
        require_literal_separator = false,
        require_literal_leading_dot = false,
        strict = true,
        dtype = None,
    )
)]
pub fn py_glob(
    pattern: &str,
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
    strict: bool,
    dtype: Option<Bound<'_, PyType>>,
) -> PyResult<RyGlobPaths> {
    let dtype = extract_dtype(dtype)?;
    ::glob::glob_with(
        pattern,
        ::glob::MatchOptions {
            case_sensitive,
            require_literal_separator,
            require_literal_leading_dot,
        },
    )
    .map(|paths| RyGlobPaths {
        inner: RyMutex::new(paths),
        strict,
        dtype,
    })
    .map_err(|e| py_value_error!("{e}"))
}

fn pathlib_path_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static PATHLIB_PATH_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    PATHLIB_PATH_TYPE.import(py, "pathlib", "Path")
}

fn str_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static STR_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    STR_TYPE.import(py, "builtins", "str")
}

fn ry_fspath_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static FSPATH_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    FSPATH_TYPE.import(py, "ry.ryo3", "FsPath")
}

fn extract_dtype(dtype: Option<Bound<'_, PyType>>) -> PyResult<GlobDType> {
    if let Some(dtype) = dtype {
        let py = dtype.py();
        if dtype.is(str_type(py)?) {
            Ok(GlobDType::OsString)
        } else if dtype.is(pathlib_path_type(py)?) {
            Ok(GlobDType::PathBuf)
        } else if dtype.is(ry_fspath_type(py)?) {
            Ok(GlobDType::FsPath)
        } else {
            let repr = dtype.repr()?.to_string();
            py_value_err!(
                "Invalid dtype: {repr} (only `str`, `pathlib.Path` or `ry.ryo3.FsPath` are supported)"
            )
        }
    } else {
        // default to PathBuf when no dtype is provided
        Ok(GlobDType::PathBuf)
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyGlobPattern>()?;
    m.add_function(wrap_pyfunction!(py_glob, m)?)?;
    Ok(())
}
