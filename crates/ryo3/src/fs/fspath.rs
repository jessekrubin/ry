use std::path::{Path, PathBuf};

use pyo3::prelude::*;
use pyo3::types::{PyModule, PyType};
use pyo3::{pyclass, pymethods, FromPyObject, PyObject, PyResult, Python};

use crate::fs::fileio;
use crate::fs::fileio::{read_bytes, read_text};

#[pyclass(name = "FsPath")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFsPath {
    pth: PathBuf,
}

#[cfg(target_os = "windows")]
fn path2str<P: AsRef<Path>>(p: P) -> String {
    // remove the `\\?\` prefix if it exists
    let p = p.as_ref().display().to_string();
    if let Some(p) = p.strip_prefix(r"\\?\") {
        p.to_string()
    } else {
        p
    }
}

#[cfg(not(target_os = "windows"))]
fn path2str<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[pymethods]
impl PyFsPath {
    #[new]
    #[pyo3(signature = (p=None))]
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
        let p = self.pth.canonicalize()?;
        // return the canonicalized path
        Ok(PyFsPath::from(p))
    }

    fn extension(&self) -> PyResult<Option<String>> {
        let e = self.pth.extension();
        match e {
            Some(e) => Ok(Some(
                e.to_str()
                    .expect("extension() - path contains invalid unicode characters")
                    .to_string(),
            )),
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
        let s = self.pth.to_string_lossy().to_string();
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
            Some(p) => match p.to_str() {
                Some(p) => {
                    if p.is_empty() {
                        Ok(PyFsPath::from("."))
                    } else {
                        Ok(PyFsPath::from(p))
                    }
                }
                None => Ok(PyFsPath::from(p)),
            },
            None => Ok(self.clone()),
        }
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        match self.pth.file_name() {
            Some(name) => Ok(name.to_string_lossy().to_string()),
            None => Ok(String::new()),
        }
    }

    #[getter]
    fn suffix(&self) -> PyResult<String> {
        let e = self.pth.extension();
        match e {
            Some(e) => Ok(path2str(
                e.to_str()
                    .expect("suffix() - path contains invalid unicode characters"),
            )),
            None => Ok(String::new()),
        }
    }

    #[getter]
    fn suffixes(&self) -> PyResult<Vec<String>> {
        let mut suffixes = vec![];
        let mut p = self.pth.clone();
        while let Some(e) = p.extension() {
            match e.to_str() {
                Some(e) => suffixes.push(e.to_string()),
                None => break,
            }
            p = p.with_extension("");
        }
        suffixes.reverse();
        Ok(suffixes)
    }

    #[getter]
    fn stem(&self) -> PyResult<String> {
        self.pth
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(
                    "stem() - path contains invalid unicode characters",
                )
            })
    }

    #[classmethod]
    fn home(_cls: &Bound<'_, PyType>) -> PyResult<PyFsPath> {
        let p = dirs::home_dir().expect("home() - unable to determine home directory");
        Ok(p.into())
    }

    #[classmethod]
    fn cwd(_cls: &Bound<'_, PyType>) -> PyResult<PyFsPath> {
        let p =
            std::env::current_dir().expect("cwd() - unable to determine current working directory");
        Ok(p.into())
    }

    fn read_bytes(&self, py: Python<'_>) -> PyResult<PyObject> {
        read_bytes(py, &self.string())
    }

    fn read_text(&self, py: Python<'_>) -> PyResult<String> {
        let s = read_text(py, &self.string())?;
        Ok(s)
    }

    fn write_bytes(&self, b: Vec<u8>) -> PyResult<()> {
        fileio::write_bytes(&self.string(), b)
    }

    fn write_text(&self, t: &str) -> PyResult<()> {
        fileio::write_text(&self.string(), t)
    }

    // ========================================================================
    // TODO: not implemented stuff
    // ========================================================================
    // #[pyo3(name = "match")]
    // fn match_(&self, pattern: String, case_sensitive: Option<bool>) -> PyResult<bool> {
    //     Err(pyo3::exceptions::PyNotImplementedError::new_err(
    //         "match not implemented",
    //     ))
    // }
}

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

#[derive(Debug, FromPyObject, Clone)]
pub enum PathLike {
    PathBuf(PathBuf),
    Str(String),
}

impl From<PathLike> for String {
    fn from(p: PathLike) -> Self {
        match p {
            PathLike::PathBuf(p) => p.to_string_lossy().to_string(),
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
            PathLike::PathBuf(p) => write!(f, "{}", p.to_string_lossy()),
            PathLike::Str(s) => write!(f, "{s}"),
        }
    }
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
