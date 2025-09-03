//! Pathlib utilities
//!
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyType;
use pyo3::{PyAny, PyResult};
use std::path::Path;

/// Deprecated
/// builtin conversion added to pyo3 in version 0.24.0
pub(crate) fn path2pathlib<T: AsRef<Path>>(py: Python<'_>, path: T) -> PyResult<Bound<'_, PyAny>> {
    static PATHLIB: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    PATHLIB
        .import(py, "pathlib", "Path")?
        .call1((path.as_ref().as_os_str(),))
}
