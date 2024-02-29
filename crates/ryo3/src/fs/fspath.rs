use std::hash::Hash;
use std::path::{Path, PathBuf};

use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, FromPyObject, PyResult};

#[pyclass(name = "FsPath")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFsPath {
    pth: PathBuf,
}

#[pymethods]
impl PyFsPath {
    #[new]
    fn new(p: Option<PathLike>) -> Self {
        match p {
            Some(p) => Self {
                pth: p.as_ref().to_path_buf(),
            },
            None => {
                // use "." as default
                Self {
                    pth: PathBuf::from("."),
                }
            }
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        let s = self.pth.to_str();
        match s {
            Some(s) => Ok(s.to_string()),
            None => Err(pyo3::exceptions::PyValueError::new_err("Invalid path")),
        }
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __eq__(&self, other: PathLike) -> bool {
        // let other = other.extract::<PyPath>().unwrap();
        self.pth == other.as_ref()
    }

    fn __ne__(&self, other: PathLike) -> bool {
        // let other = other.extract::<PyPath>().unwrap();
        self.pth != other.as_ref()
    }

    fn is_file(&self) -> bool {
        self.pth.is_file()
    }

    fn is_dir(&self) -> bool {
        self.pth.is_dir()
    }

    fn is_symlink(&self) -> bool {
        self.pth.is_symlink()
    }

    fn is_absolute(&self) -> bool {
        self.pth.is_absolute()
    }

    fn is_relative(&self) -> bool {
        self.pth.is_relative()
    }

    fn exists(&self) -> bool {
        self.pth.exists()
    }

    fn extension(&self) -> PyResult<Option<String>> {
        let e = self.pth.extension();
        match e {
            Some(e) => Ok(Some(e.to_str().unwrap().to_string())),
            None => Ok(None),
        }
    }

    fn with_extension(&self, extension: String) -> PyResult<Self> {
        let p = self.pth.with_extension(extension);
        Ok(PyFsPath::from(p))
    }

    fn with_file_name(&self, name: String) -> PyResult<Self> {
        let p = self.pth.with_file_name(name);
        Ok(PyFsPath::from(p))
    }

    fn __fspath__(&self) -> PyResult<String> {
        let s = self.pth.to_str().unwrap().to_string();
        Ok(s)
    }

    #[getter]
    fn parent(&self) -> PyResult<PyFsPath> {
        let p = self.pth.parent();
        match p {
            Some(p) => Ok(PyFsPath::from(p)),
            None => Err(pyo3::exceptions::PyValueError::new_err("No parent")),
        }
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        let s = self.pth.file_name().unwrap().to_str().unwrap().to_string();
        Ok(s)
    }

    #[getter]
    fn stem(&self) -> PyResult<String> {
        let s = self.pth.file_stem().unwrap().to_str().unwrap().to_string();
        Ok(s)
    }

    #[classmethod]
    fn home(_cls: &PyType) -> PyResult<PyFsPath> {
        let p = dirs::home_dir().unwrap();
        Ok(p.into())
    }

    #[classmethod]
    fn cwd(_cls: &PyType) -> PyResult<PyFsPath> {
        let p = std::env::current_dir().unwrap();
        Ok(p.into())
    }
}

// impl From<PathBuf> for PyFsPath {
//     fn from(p: PathBuf) -> Self {
//         PyFsPath { pth: p }
//     }
// }

impl<T> From<T> for PyFsPath
where
    T: AsRef<Path>,
{
    fn from(p: T) -> Self {
        PyFsPath {
            pth: p.as_ref().to_path_buf(),
        }
    }
}
// impl From<&Path> for PyFsPath {
//     fn from(p: &Path) -> Self {
//         PyFsPath { pth: p.to_path_buf() }
//     }
// }

#[derive(Debug, FromPyObject, Clone)]
pub enum PathLike {
    PathBuf(PathBuf),
    Str(String),
}

impl From<PathLike> for String {
    fn from(p: PathLike) -> Self {
        match p {
            PathLike::PathBuf(p) => p.to_str().unwrap().to_string(),
            PathLike::Str(s) => s,
        }
    }
}

impl AsRef<Path> for PathLike {
    fn as_ref(&self) -> &Path {
        match self {
            PathLike::PathBuf(p) => p.as_ref(),
            PathLike::Str(s) => Path::new(s),
        }
    }
}

impl From<&Path> for PathLike {
    fn from(p: &Path) -> Self {
        PathLike::PathBuf(p.to_path_buf())
    }
}

impl std::fmt::Display for PathLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathLike::PathBuf(p) => write!(f, "{}", p.to_str().unwrap()),
            PathLike::Str(s) => write!(f, "{}", s),
        }
    }
}
