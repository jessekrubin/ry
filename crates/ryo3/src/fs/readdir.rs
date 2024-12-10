use crate::fs::fspath::PyFsPath;
use pyo3::{pyclass, pymethods, PyRef, PyRefMut};

#[pyclass(name = "ReadDir", module = "ryo3")]
pub struct PyReadDir {
    iter: std::fs::ReadDir,
}

#[pymethods]
impl PyReadDir {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyFsPath> {
        match slf.iter.next() {
            Some(Ok(entry)) => Some(PyFsPath::from(entry.path())),
            _ => None,
        }
    }
}

impl From<std::fs::ReadDir> for PyReadDir {
    fn from(iter: std::fs::ReadDir) -> Self {
        Self { iter }
    }
}
