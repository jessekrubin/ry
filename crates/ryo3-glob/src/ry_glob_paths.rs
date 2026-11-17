use std::ffi::OsString;
use std::path::PathBuf;

use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ryo3_core::macros::py_value_error;
use ryo3_core::sync::RyMutex;

#[derive(Clone, Copy)]
pub(crate) enum GlobDType {
    FsPath,
    PathBuf,
    OsString,
}

struct GlobPathsVec {
    dtype: GlobDType,
    paths: Vec<PathBuf>,
}

impl From<(GlobDType, Vec<PathBuf>)> for GlobPathsVec {
    fn from(value: (GlobDType, Vec<PathBuf>)) -> Self {
        Self {
            dtype: value.0,
            paths: value.1,
        }
    }
}

impl<'py> IntoPyObject<'py> for GlobPathsVec {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.dtype {
            GlobDType::FsPath => self
                .paths
                .into_iter()
                .map(ryo3_fspath::RyFsPath::from)
                .collect::<Vec<ryo3_fspath::RyFsPath>>()
                .into_pyobject(py),
            GlobDType::PathBuf => self.paths.into_pyobject(py),
            GlobDType::OsString => self
                .paths
                .into_iter()
                .map(OsString::from)
                .collect::<Vec<OsString>>()
                .into_pyobject(py),
        }
    }
}

struct GlobItem(GlobDType, PathBuf);

impl<'py> IntoPyObject<'py> for GlobItem {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            GlobDType::FsPath => ryo3_fspath::RyFsPath::from(self.1).into_bound_py_any(py),
            GlobDType::PathBuf => self.1.into_bound_py_any(py),
            GlobDType::OsString => OsString::from(self.1).into_bound_py_any(py),
        }
    }
}

#[pyclass(name = "GlobPaths", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyGlobPaths {
    pub(crate) inner: RyMutex<::glob::Paths, false>,
    pub(crate) strict: bool,
    pub(crate) dtype: GlobDType,
}

impl RyGlobPaths {
    /// Pull exactly one item -- fix `clippy::significant-drop-in-scrutinee`
    #[inline]
    fn next_path(&self) -> Option<Result<PathBuf, glob::GlobError>> {
        self.inner.py_lock().next()
    }

    fn py_next(&self) -> Result<Option<GlobItem>, ::glob::GlobError> {
        loop {
            match self.next_path() {
                Some(Ok(path)) => {
                    return Ok(Some(GlobItem(self.dtype, path)));
                }
                Some(Err(e)) => {
                    if self.strict {
                        return Err(e);
                    }
                }
                None => return Ok(None),
            }
        }
    }

    fn py_collect(&self) -> Result<Vec<PathBuf>, ::glob::GlobError> {
        if self.strict {
            let mut results = Vec::new();
            for path in self.inner.py_lock().by_ref() {
                match path {
                    Ok(path) => {
                        results.push(path);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok(results)
        } else {
            Ok(self.inner.py_lock().by_ref().flatten().collect::<Vec<_>>())
        }
    }

    fn py_take(&self, n: usize) -> Result<Vec<PathBuf>, ::glob::GlobError> {
        if self.strict {
            let mut results = Vec::new();
            for path_result in self.inner.py_lock().by_ref().take(n) {
                match path_result {
                    Ok(path) => {
                        results.push(path);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Ok(results)
        } else {
            Ok(self
                .inner
                .py_lock()
                .by_ref()
                .flatten()
                .take(n)
                .collect::<Vec<_>>())
        }
    }
}

#[pymethods]
impl RyGlobPaths {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> PyResult<Option<GlobItem>> {
        self.py_next().map_err(|e| py_value_error!("{e}"))
    }

    fn collect(&self, py: Python<'_>) -> PyResult<GlobPathsVec> {
        let paths: Vec<PathBuf> = py
            .detach(|| self.py_collect())
            .map_err(|e| py_value_error!("{e}"))?;
        Ok(GlobPathsVec::from((self.dtype, paths)))
    }

    /// Take `n` items from the iterator or 1 if `n` is not specified.
    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<GlobPathsVec> {
        let paths: Vec<PathBuf> = py
            .detach(|| self.py_take(n))
            .map_err(|e| py_value_error!("{e}"))?;
        Ok(GlobPathsVec::from((self.dtype, paths)))
    }
}
