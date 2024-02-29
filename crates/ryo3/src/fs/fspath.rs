use std::hash::Hash;
use std::path::{Path, PathBuf};

use pyo3::types::PyType;
use pyo3::{pyclass, pymethods, FromPyObject, PyResult};
use tracing::{debug, info};

#[pyclass(name = "FsPath")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFsPath {
    pth: PathBuf,
}

#[cfg(target_os = "windows")]
fn path2str<P: AsRef<Path>>(p: P) -> String {
    // remove the `\\?\` prefix if it exists
    let p = p.as_ref().display().to_string();
    if p.starts_with(r"\\?\") {
        p[4..].to_string()
    } else {
        p
    }
}

#[cfg(not(target_os = "windows"))]
fn path2str<P: AsRef<Path>>(p: P) -> String {
    p.to_string_lossy().to_string()
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

    fn string(&self) -> String {
        path2str(&self.pth)
    }

    fn __str__(&self) -> PyResult<String> {
        let s = self.string();
        Ok(s)
    }

    fn __repr__(&self) -> PyResult<String> {
        let s = path2str(&self.pth);
        Ok(s)
    }

    fn equiv(&self, other: PathLike) -> bool {
        let other = other.as_ref();
        self.pth == other
    }

    fn __eq__(&self, other: PathLike) -> bool {
        // start by comparing as paths
        if self.pth == other.as_ref() {
            return true;
        }
        // if that fails, compare as strings
        self.string() == path2str(other.as_ref())
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

    fn absolute(&self) -> PyResult<Self> {
        let p = self.pth.canonicalize().unwrap();
        // return the canonicalized path
        Ok(PyFsPath::from(p))
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

    fn clone(&self) -> Self {
        Self {
            pth: self.pth.clone(),
        }
    }

    #[getter]
    fn parent(&self) -> PyResult<PyFsPath> {
        let p = self.pth.parent();
        match p {
            Some(p) => {
                if p.to_str().unwrap() == "" {
                    Ok(PyFsPath::from("."))
                } else {
                    Ok(PyFsPath::from(p))
                }
            }
            None => Ok(self.clone()),
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
