use pyo3::{FromPyObject, IntoPyObject, pybacked::PyBackedStr};
use pyo3::{IntoPyObjectExt, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum PathLike {
    PathBuf(PathBuf),
    PyStr(PyBackedStr),
    Str(String),
}

impl<'a, 'py> FromPyObject<'a, 'py> for PathLike {
    type Error = pyo3::PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<PyBackedStr>() {
            Ok(Self::PyStr(s))
        } else {
            let p: PathBuf = obj.extract()?;
            Ok(Self::PathBuf(p))
        }
    }
}

impl<'py> IntoPyObject<'py> for PathLike {
    type Target = pyo3::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::PathBuf(p) => p.into_bound_py_any(py),
            Self::PyStr(s) => s.into_bound_py_any(py),
            Self::Str(s) => s.into_bound_py_any(py),
        }
    }
}

impl From<PathLike> for String {
    #[inline]
    fn from(p: PathLike) -> Self {
        match p {
            PathLike::PathBuf(p) => p.to_string_lossy().to_string(),
            PathLike::PyStr(s) => s.to_string(),
            PathLike::Str(s) => s,
        }
    }
}

impl AsRef<Path> for PathLike {
    #[inline]
    fn as_ref(&self) -> &Path {
        match self {
            Self::PathBuf(p) => p.as_ref(),
            Self::PyStr(s) => Path::new(&**s),
            Self::Str(s) => Path::new(s),
        }
    }
}

impl From<&Path> for PathLike {
    #[inline]
    fn from(p: &Path) -> Self {
        Self::PathBuf(p.to_path_buf())
    }
}

impl std::fmt::Display for PathLike {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathBuf(p) => write!(f, "{}", p.to_string_lossy()),
            Self::PyStr(s) => write!(f, "{s}"),
            Self::Str(s) => write!(f, "{s}"),
        }
    }
}
