use std::path::{Path, PathBuf};

use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;

use crate::PyCastExactOpt;

mod pathlib {

    use pyo3::prelude::*;
    use pyo3::sync::PyOnceLock;
    use pyo3::types::PyType;

    /// Returns the pointer to the concrete `pathlib.Path`
    ///
    /// if "its a unix system": `pathlib.PosixPath`
    /// if (stupid) windows: `pathlib.WindowsPath`
    fn concrete_pathlib_path_type_ptr(py: Python<'_>) -> PyResult<usize> {
        static CONCRETE_PATHLIB_PATH_TYPE_PTR: PyOnceLock<usize> = PyOnceLock::new();
        CONCRETE_PATHLIB_PATH_TYPE_PTR
            .get_or_try_init(py, || {
                static CONCRETE_PATHLIB_PATH_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
                let path_type = CONCRETE_PATHLIB_PATH_TYPE.import(
                    py,
                    "pathlib",
                    if cfg!(windows) {
                        "WindowsPath"
                    } else {
                        "PosixPath"
                    },
                )?;
                Ok(path_type.as_type_ptr() as usize)
            })
            .copied()
    }

    #[inline]
    pub(super) fn is_exact_pathlib_path(obj: Borrowed<'_, '_, PyAny>) -> PyResult<bool> {
        Ok(obj.get_type_ptr() as usize == concrete_pathlib_path_type_ptr(obj.py())?)
    }
}

#[derive(Debug)]
pub enum PathLike {
    PathBuf(PathBuf),
    PyStr(PyBackedStr),
    Str(String),
}

impl<'a, 'py> FromPyObject<'a, 'py> for PathLike {
    type Error = pyo3::PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check string exactly
        if let Some(pystr) = obj.cast_exact_opt::<pyo3::types::PyString>() {
            return pystr.extract::<PyBackedStr>().map(Self::PyStr);
        }
        // check pathlib.Path exactly
        if pathlib::is_exact_pathlib_path(obj)? {
            let p: PathBuf = obj.extract()?;
            return Ok(Self::PathBuf(p));
        }
        // attempt to extract string...
        if let Ok(s) = obj.extract::<PyBackedStr>() {
            return Ok(Self::PyStr(s));
        }
        // finally fall back to the `__fspath__` protocol
        let p: PathBuf = obj.extract()?;
        Ok(Self::PathBuf(p))
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
