#![allow(clippy::needless_pass_by_value)]

use std::path::{Path, PathBuf};

use crate::fs::fileio;
use crate::fs::iterdir::PyIterdirGen;
use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyFileNotFoundError, PyNotADirectoryError, PyUnicodeDecodeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule, PyTuple, PyType};
use pyo3::{pyclass, pymethods, PyObject, PyResult, Python};
use ryo3_types::PathLike;

// separator
const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

#[pyclass(name = "FsPath", module = "ryo3")]
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

    // fn __eq__(&self, other: PathLike) -> bool {
    //     // start by comparing as paths
    //     if self.pth == other.as_ref() {
    //         return true;
    //     }
    //     // if that fails, compare as strings
    //     self.string() == path2str(other.as_ref())
    // }

    fn __richcmp__(&self, other: PathLike, op: CompareOp) -> PyResult<bool> {
        let o = other.as_ref();
        match op {
            CompareOp::Eq => {
                // start by comparing as paths
                if self.pth == other.as_ref() {
                    return Ok(true);
                }
                // if that fails, compare as strings
                Ok(self.string() == path2str(other.as_ref()))
            }
            CompareOp::Ne => {
                // start by comparing as paths
                if self.pth != other.as_ref() {
                    return Ok(true);
                }
                // if that fails, compare as strings
                Ok(self.string() != path2str(other.as_ref()))
            }
            CompareOp::Lt => Ok(self.pth < o),
            CompareOp::Le => Ok(self.pth <= o),
            CompareOp::Gt => Ok(self.pth > o),
            CompareOp::Ge => Ok(self.pth >= o),
        }
    }

    // fn __ne__(&self, other: PathLike) -> bool {
    //     // let other = other.extract::<PyPath>().unwrap();
    //     self.pth != other.as_ref()
    // }

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

    fn __truediv__(&self, other: PathLike) -> PyResult<Self> {
        let p = self.pth.join(other.as_ref());
        Ok(PyFsPath::from(p))
    }

    fn __rtruediv__(&self, other: PathLike) -> PyResult<Self> {
        let p = other.as_ref().join(&self.pth);
        Ok(PyFsPath::from(p))
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let s = path2str(self.pth.clone());
        let b = s.as_bytes();
        PyBytes::new(py, b)
    }

    #[getter]
    fn root(&self) -> PyResult<Option<PyFsPath>> {
        if let Some(p) = self.pth.components().next() {
            Ok(Some(PyFsPath::from(p.as_os_str())))
        } else {
            Ok(None)
        }
    }

    #[cfg(target_os = "windows")]
    #[getter]
    fn drive(&self) -> PyResult<Option<String>> {
        let drive = self.pth.components().next();
        match drive {
            Some(drive_component) => {
                let drive_str = drive_component.as_os_str().to_string_lossy().to_string();
                Ok(Some(drive_str))
            }
            None => Ok(None),
        }
        // #[cfg(not(target_os = "windows"))]
        // {
        //     Ok(None)
        // }
    }

    #[cfg(not(target_os = "windows"))]
    #[getter]
    fn drive(&self) -> PyResult<Option<String>> {
        Ok(None)
    }

    #[getter]
    fn anchor(&self) -> PyResult<String> {
        let anchor = self.pth.components().next();
        match anchor {
            Some(anchor) => {
                let a = anchor.as_os_str().to_string_lossy().to_string();
                // ensure that the anchor ends with a separator
                Ok(format!("{a}{MAIN_SEPARATOR}"))
            }
            None => Ok(String::new()),
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
    fn parts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parts = self
            .pth
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        PyTuple::new(py, parts)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        match self.pth.file_name() {
            Some(name) => Ok(name.to_string_lossy().to_string()),
            None => Ok(String::new()),
        }
    }

    fn with_name(&self, name: String) -> PyResult<Self> {
        let p = self.pth.with_file_name(name);
        Ok(PyFsPath::from(p))
    }

    #[getter]
    fn suffix(&self) -> PyResult<String> {
        let e = self.pth.extension();
        match e {
            Some(e) => {
                let ext = path2str(
                    e.to_str()
                        .expect("suffix() - path contains invalid unicode characters"),
                );

                Ok(format!(".{ext}"))
            }
            None => Ok(String::new()),
        }
    }

    fn with_suffix(&self, suffix: String) -> PyResult<Self> {
        // auto strip leading dot
        let suffix = if suffix.starts_with('.') {
            suffix.trim_start_matches('.')
        } else {
            suffix.as_ref()
        };
        let p = self.pth.with_extension(suffix);
        Ok(PyFsPath::from(p))
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

    fn as_posix(&self) -> PyResult<String> {
        Ok(self.pth.to_string_lossy().to_string())
    }

    fn joinpath(&self, other: PathLike) -> PyResult<PyFsPath> {
        let p = self.pth.join(other.as_ref());
        Ok(PyFsPath::from(p))
    }

    pub fn read_vec_u8(&self) -> PyResult<Vec<u8>> {
        // let fpath = Path::new(s);
        let fbytes = std::fs::read(&self.pth);
        match fbytes {
            Ok(b) => Ok(b),
            Err(e) => {
                // TODO: figure out cleaner way of doing this
                let pathstr = self.string();
                let emsg = format!("read_vec_u8 - path: {pathstr} - {e}");
                Err(PyFileNotFoundError::new_err(emsg))
            }
        }
    }

    pub fn read_bytes(&self, py: Python<'_>) -> PyResult<PyObject> {
        let bvec = self.read_vec_u8()?;
        Ok(PyBytes::new(py, &bvec).into())
    }

    pub fn read_text(&self, py: Python<'_>) -> PyResult<String> {
        let bvec = self.read_vec_u8()?;
        let r = std::str::from_utf8(&bvec);
        match r {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let decode_err = PyUnicodeDecodeError::new_utf8(py, &bvec, e)?;
                Err(decode_err.into())
            }
        }
    }

    pub fn write_bytes(&self, b: Vec<u8>) -> PyResult<()> {
        let write_res = std::fs::write(&self.pth, b);
        match write_res {
            Ok(()) => Ok(()),
            Err(e) => {
                let fspath = self.string();
                Err(PyNotADirectoryError::new_err(format!(
                    "write_bytes - parent: {fspath} - {e}"
                )))
            }
        }
    }

    pub fn write_text(&self, t: &str) -> PyResult<()> {
        fileio::write_text(&self.string(), t)
    }

    pub fn as_uri(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "as_uri not implemented",
        ))
    }

    fn iterdir(&self) -> PyResult<PyIterdirGen> {
        let rd = std::fs::read_dir(&self.pth)
            .map(PyIterdirGen::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("iterdir: {e}")))?;
        Ok(rd)
    }

    fn relative_to(&self, _other: PathLike) -> PyResult<PyFsPath> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "relative_to not implemented",
        ))
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
//
// #[derive(Debug, FromPyObject, Clone)]
// pub enum PathLike {
//     PathBuf(PathBuf),
//     Str(String),
// }
//
// impl From<PathLike> for String {
//     fn from(p: PathLike) -> Self {
//         match p {
//             PathLike::PathBuf(p) => p.to_string_lossy().to_string(),
//             PathLike::Str(s) => s,
//         }
//     }
// }
//
// impl AsRef<Path> for PathLike {
//     fn as_ref(&self) -> &Path {
//         match self {
//             PathLike::PathBuf(p) => p.as_ref(),
//             PathLike::Str(s) => Path::new(s),
//         }
//     }
// }
//
// impl From<&Path> for PathLike {
//     fn from(p: &Path) -> Self {
//         PathLike::PathBuf(p.to_path_buf())
//     }
// }
//
// impl std::fmt::Display for PathLike {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             PathLike::PathBuf(p) => write!(f, "{}", p.to_string_lossy()),
//             PathLike::Str(s) => write!(f, "{s}"),
//         }
//     }
// }

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
