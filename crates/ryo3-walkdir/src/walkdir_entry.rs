//! python wrapper for `walkdir::DirEntry`
use std::ffi::OsStr;

use pyo3::prelude::*;

#[pyclass(name = "WalkDirEntry", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry(walkdir::DirEntry);

#[pymethods]
impl PyWalkDirEntry {
    fn __fspath__(&self) -> &OsStr {
        self.0.path().as_os_str()
    }

    #[getter]
    fn path(&self) -> PyResult<String> {
        self.0
            .path()
            .to_str()
            .map(ToString::to_string)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
                    "Path contains invalid unicode characters",
                )
            })
    }

    #[getter]
    fn file_name(&self) -> String {
        self.0.file_name().to_string_lossy().to_string()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.0.depth()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> PyResult<String> {
        self.0
            .path()
            .to_str()
            .map(ToString::to_string)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(
                    "Path contains invalid unicode characters",
                )
            })
    }

    fn __repr__(&self) -> String {
        let s = self.py_to_string().unwrap_or_else(|_| String::from("???"));
        format!("WalkDirEntry<'{s}'>")
    }

    #[getter]
    fn path_is_symlink(&self) -> bool {
        self.0.path_is_symlink()
    }

    fn metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
        self.0
            .metadata()
            .map(std::convert::Into::into)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyPermissionError, _>(format!("{e}")))
    }

    #[getter]
    fn file_type(&self) -> ryo3_std::fs::PyFileType {
        self.0.file_type().into()
    }

    #[getter]
    fn is_dir(&self) -> bool {
        self.0.file_type().is_dir()
    }

    #[getter]
    fn is_file(&self) -> bool {
        self.0.file_type().is_file()
    }

    #[getter]
    fn is_symlink(&self) -> bool {
        self.0.file_type().is_symlink()
    }

    #[getter]
    fn len(&self) -> PyResult<u64> {
        let mlen = self
            .0
            .metadata()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyPermissionError, _>(format!("{e}")))?
            .len();
        Ok(mlen)
    }
}

impl From<walkdir::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir::DirEntry) -> Self {
        Self(de)
    }
}
