//! python wrapper for `walkdir::DirEntry`
use std::ffi::OsStr;

use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use ryo3_core::macros::{py_permission_error, py_type_err};

#[pyclass(name = "WalkDirEntry", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry(walkdir::DirEntry);

#[pymethods]
impl PyWalkDirEntry {
    #[new]
    fn py_new() -> PyResult<Self> {
        py_type_err!("WalkDirEntry cannot be instantiated directly")
    }

    fn __fspath__(&self) -> &OsStr {
        self.0.path().as_os_str()
    }

    #[getter]
    fn path(&self) -> &OsStr {
        self.0.path().as_os_str()
    }

    #[getter]
    fn file_name(&self) -> &OsStr {
        self.0.file_name()
    }

    #[getter]
    fn name(&self) -> &OsStr {
        self.0.file_name()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.0.depth()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> &OsStr {
        self.0.path().as_os_str()
    }

    fn __str__(&self) -> &OsStr {
        self.py_to_string()
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.path().hash(&mut hasher);
        hasher.finish()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0.path() == other.0.path(),
            CompareOp::Ne => self.0.path() != other.0.path(),
            CompareOp::Lt => self.0.path() < other.0.path(),
            CompareOp::Le => self.0.path() <= other.0.path(),
            CompareOp::Gt => self.0.path() > other.0.path(),
            CompareOp::Ge => self.0.path() >= other.0.path(),
        }
    }

    #[getter]
    fn path_is_symlink(&self) -> bool {
        self.0.path_is_symlink()
    }

    fn metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
        self.0
            .metadata()
            .map(Into::into)
            .map_err(|e| py_permission_error!("{e}"))
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
        self.0
            .metadata()
            .map(|m| m.len())
            .map_err(|e| py_permission_error!("{e}"))
    }
}

impl From<walkdir::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir::DirEntry) -> Self {
        Self(de)
    }
}

impl std::fmt::Display for PyWalkDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WalkDirEntry<'{}'>", self.0.path().display())
    }
}
