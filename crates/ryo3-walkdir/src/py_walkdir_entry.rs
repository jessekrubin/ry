//! python wrapper for `walkdir::DirEntry`
use std::ffi::OsStr;
use std::io;

use pyo3::prelude::*;
use ryo3_core::PyIoResultExt;
use ryo3_core::macros::py_type_err;

#[pyclass(name = "WalkDirEntry", frozen, immutable_type, eq, ord)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry(walkdir::DirEntry);

// compares by path
impl PartialEq for PyWalkDirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.path() == other.0.path()
    }
}

impl PartialOrd for PyWalkDirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.path().partial_cmp(other.0.path())
    }
}

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

    #[getter]
    fn path_is_symlink(&self) -> bool {
        self.0.path_is_symlink()
    }

    fn metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
        // TODO: investigate if there is a nicer way to do this chain
        // which is aimed at providing the pathbuf to the usr facing pyerr
        self.0
            .metadata()
            .map_err(|e| {
                let msg = e.to_string();
                e.into_io_error().unwrap_or_else(|| io::Error::other(msg))
            })
            .map_py_io_err_with_path(self.0.path())
            .map(Into::into)
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
        self.metadata().map(|m| m.inner().len())
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
