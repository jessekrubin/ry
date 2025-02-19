use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyType;
use pyo3::{PyAny, PyResult};
use std::path::Path;

pub(crate) fn path2pathlib<T: AsRef<Path>>(py: Python<'_>, path: T) -> PyResult<Bound<'_, PyAny>> {
    static PATHLIB: GILOnceCell<Py<PyType>> = GILOnceCell::new();

    PATHLIB
        .import(py, "pathlib", "Path")?
        .call1((path.as_ref().to_string_lossy().to_string(),))
}
