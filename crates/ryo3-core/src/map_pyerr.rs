use std::fmt;
use std::path::Path;

use pyo3::PyErr;

pub fn map_py_value_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyValueError::new_err(e.to_string())
}

pub fn map_py_overflow_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyOverflowError::new_err(e.to_string())
}

pub fn map_py_runtime_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
}

/// map `std::io::Error` to Python `OSError` with a filepath str
///
/// adapted from pyo3 src
#[must_use]
pub fn map_py_io_err_with_path(e: std::io::Error, path: &Path) -> PyErr {
    let Some(code) = e.raw_os_error() else {
        return PyErr::from(e);
    };
    let msg = e.to_string();
    let strerror = msg
        .strip_suffix(&format!(" (os error {code})"))
        .unwrap_or(&msg);
    // win32 + cpython trim trailing . (parity?)
    #[cfg(windows)]
    let strerror = strerror.trim_end_matches(|c: char| c == '.' || c.is_whitespace());
    let strerror = strerror.to_owned();
    let filename = path.as_os_str().to_owned();
    #[cfg(windows)]
    let args = (0, strerror, filename, code);
    #[cfg(not(windows))]
    let args = (code, strerror, filename);
    pyo3::exceptions::PyOSError::new_err(args)
}
