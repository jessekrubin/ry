//! python wrapper for `walkdir::DirEntry`
use pyo3::prelude::*;

#[pyclass(name = "WalkDirEntry", module = "ryo3", frozen)]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry(walkdir::DirEntry);

#[pymethods]
impl PyWalkDirEntry {
    fn __fspath__(&self) -> String {
        self.0.path().to_path_buf().to_string_lossy().to_string()
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

    fn __str__(&self) -> PyResult<String> {
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
        let s = self.__str__().unwrap_or_else(|_| String::from("???"));
        format!("WalkDirEntry({s:?})")
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
        PyWalkDirEntry(de)
    }
}
