//! `FsPath` struct python module

// TODO: remove in future? if possible?
use crate::pathlib::path2pathlib;
use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyFileNotFoundError, PyNotADirectoryError, PyUnicodeDecodeError, PyValueError,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple, PyType};
use ryo3_bytes::extract_bytes_ref;
use ryo3_core::types::PathLike;
use ryo3_macros::err_py_not_impl;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

// separator
const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

type ArcPathBuf = std::sync::Arc<PathBuf>;

#[pyclass(name = "FsPath", module = "ryo3", frozen)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFsPath {
    // pth: PathBuf,
    pth: ArcPathBuf,
}

impl PyFsPath {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        Self {
            pth: ArcPathBuf::new(p.as_ref().to_path_buf()),
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.pth.as_ref()
    }
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

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unused_self)]
#[pymethods]
impl PyFsPath {
    #[new]
    #[pyo3(signature = (p=None))]
    fn py_new(p: Option<PathBuf>) -> Self {
        match p {
            Some(p) => Self::from(p),
            None => Self::from("."),
        }
    }

    fn string(&self) -> String {
        path2str(self.path())
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        let posix_str = self.as_posix();
        format!("FsPath(\'{posix_str}\')")
    }

    fn equiv(&self, other: PathLike) -> bool {
        let other = other.as_ref();
        self.path() == other
    }

    fn to_pathlib<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        path2pathlib(py, self.path())
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.pth.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: PathLike, op: CompareOp) -> bool {
        let o = other.as_ref();
        let p = self.pth.as_path();
        match op {
            CompareOp::Eq => {
                // start by comparing as paths
                if p == other.as_ref() {
                    return true;
                }
                // if that fails, compare as strings
                self.string() == path2str(other.as_ref())
            }
            CompareOp::Ne => {
                // start by comparing as paths
                if p != other.as_ref() {
                    return true;
                }
                // if that fails, compare as strings
                self.string() != path2str(other.as_ref())
            }
            CompareOp::Lt => p < o,
            CompareOp::Le => p <= o,
            CompareOp::Gt => p > o,
            CompareOp::Ge => p >= o,
        }
    }

    fn absolute(&self) -> PyResult<Self> {
        let p = self.pth.canonicalize()?;
        Ok(PyFsPath::from(p))
    }

    fn resolve(&self) -> PyResult<Self> {
        let p = self.pth.canonicalize()?;
        Ok(PyFsPath::from(p))
    }

    fn __fspath__(&self) -> String {
        self.pth.to_string_lossy().to_string()
    }

    fn clone(&self) -> Self {
        Self {
            pth: self.pth.clone(),
        }
    }

    fn __truediv__(&self, other: PathLike) -> Self {
        Self::from(self.pth.join(other.as_ref()))
    }

    fn __rtruediv__(&self, other: PathLike) -> Self {
        let p = other.as_ref().join(self.path());
        PyFsPath::from(p)
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let s = path2str(self.path());
        let b = s.as_bytes();
        PyBytes::new(py, b)
    }

    #[getter]
    fn root(&self) -> Option<Self> {
        self.pth
            .components()
            .next()
            .map(|p| Self::from(p.as_os_str()))
    }

    #[cfg(target_os = "windows")]
    #[getter]
    fn drive(&self) -> Option<String> {
        let drive = self.path().components().next();
        match drive {
            Some(drive_component) => {
                let drive_str = drive_component.as_os_str().to_string_lossy().to_string();
                Some(drive_str)
            }
            None => None,
        }
    }

    #[cfg(not(target_os = "windows"))]
    #[getter]
    fn drive(&self) -> Option<String> {
        None
    }

    #[getter]
    fn anchor(&self) -> String {
        let anchor = self.path().components().next();
        match anchor {
            Some(anchor) => {
                let a = anchor.as_os_str().to_string_lossy().to_string();
                // ensure that the anchor ends with a separator
                format!("{a}{MAIN_SEPARATOR}")
            }
            None => String::new(),
        }
    }
    #[getter]
    fn parent(&self) -> Self {
        let p = self.path().parent();
        match p {
            Some(p) => match p.to_str() {
                Some(p) => {
                    if p.is_empty() {
                        Self::from(".")
                    } else {
                        Self::from(p)
                    }
                }
                None => Self::from(p),
            },
            None => self.clone(),
        }
    }

    // TODO - implement ad iterator not tuple
    #[getter]
    fn parents<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parents: Vec<Self> = self.path().ancestors().map(PyFsPath::from).collect();
        PyTuple::new(py, parents)
    }

    #[getter]
    fn parts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parts = self
            .path()
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        PyTuple::new(py, parts)
    }

    #[getter]
    fn name(&self) -> String {
        match self.path().file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => String::new(),
        }
    }

    fn with_name(&self, name: String) -> Self {
        let p = self.path().with_file_name(name);
        PyFsPath::from(p)
    }

    #[getter]
    fn suffix(&self) -> String {
        let e = self.path().extension();
        match e {
            Some(e) => {
                let e = e.to_string_lossy().to_string();
                if e.starts_with('.') {
                    e
                } else {
                    format!(".{e}") // python pathlib.path does this
                }
            }
            None => String::new(),
        }
    }

    fn with_suffix(&self, suffix: String) -> Self {
        // auto strip leading dot
        let suffix = if suffix.starts_with('.') {
            suffix.trim_start_matches('.')
        } else {
            suffix.as_ref()
        };
        let p = self.path().with_extension(suffix);
        Self::from(p)
    }

    #[getter]
    fn suffixes(&self) -> Vec<String> {
        let mut suffixes = vec![];
        let mut p = self.path().to_path_buf();
        while let Some(e) = p.extension() {
            match e.to_str() {
                Some(e) => suffixes.push(e.to_string()),
                None => break,
            }
            p = p.with_extension("");
        }
        suffixes.reverse();
        suffixes
    }

    #[getter]
    fn stem(&self) -> PyResult<String> {
        self.path()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .ok_or_else(|| {
                PyValueError::new_err("stem() - path contains invalid unicode characters")
            })
    }

    #[classmethod]
    fn home(_cls: &Bound<'_, PyType>) -> Option<Self> {
        ryo3_dirs::home().map(Self::from)
    }

    #[classmethod]
    fn cwd(_cls: &Bound<'_, PyType>) -> PyResult<Self> {
        std::env::current_dir()
            .map(Self::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("cwd: {e}")))
    }

    fn as_posix(&self) -> String {
        #[cfg(target_os = "windows")]
        {
            let p = self.path().to_string_lossy().to_string();
            p.replace('\\', "/")
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.path().to_string_lossy().to_string()
        }
    }

    fn joinpath(&self, other: PathLike) -> Self {
        let p = self.path().join(other.as_ref());
        Self::from(p)
    }

    pub fn read(&self) -> PyResult<ryo3_bytes::PyBytes> {
        let fbytes = std::fs::read(self.path())?;
        Ok(fbytes.into())
    }

    pub fn read_bytes(&self, py: Python<'_>) -> PyResult<PyObject> {
        let fbytes = std::fs::read(self.path());
        match fbytes {
            Ok(b) => Ok(PyBytes::new(py, &b).into()),
            Err(e) => {
                // TODO: figure out cleaner way of doing this
                let pathstr = self.string();
                let emsg = format!("read_vec_u8 - path: {pathstr} - {e}");
                Err(PyFileNotFoundError::new_err(emsg))
            }
        }
    }

    pub fn read_text(&self, py: Python<'_>) -> PyResult<String> {
        let bvec = std::fs::read(self.path()).map_err(|e| {
            let pathstr = self.string();
            PyFileNotFoundError::new_err(format!("read_text - path: {pathstr} - {e}"))
        })?;
        let r = std::str::from_utf8(&bvec);
        match r {
            Ok(s) => Ok(s.to_string()),
            Err(e) => {
                let decode_err = PyUnicodeDecodeError::new_utf8(py, &bvec, e)?;
                Err(decode_err.into())
            }
        }
    }

    pub fn write(&self, b: &Bound<'_, PyAny>) -> PyResult<usize> {
        let b = extract_bytes_ref(b)?;
        let write_res = std::fs::write(self.path(), b);
        match write_res {
            Ok(()) => Ok(b.len()),
            Err(e) => {
                let fspath = self.string();
                Err(PyNotADirectoryError::new_err(format!(
                    "write_bytes - parent: {fspath} - {e}"
                )))
            }
        }
    }

    pub fn write_bytes(&self, b: &Bound<'_, PyAny>) -> PyResult<usize> {
        self.write(b)
    }

    pub fn write_text(&self, t: &str) -> PyResult<()> {
        let write_result = std::fs::write(self.path(), t);
        match write_result {
            Ok(()) => Ok(()),
            Err(e) => {
                let fspath = self.string();
                Err(PyNotADirectoryError::new_err(format!(
                    "write_bytes - parent: {fspath} - {e}"
                )))
            }
        }
    }

    pub fn as_uri(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "as_uri not implemented",
        ))
    }

    fn iterdir(&self) -> PyResult<PyFsPathReadDir> {
        self.read_dir()
    }

    fn relative_to(&self, _other: PathLike) -> PyResult<PyFsPath> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "relative_to not implemented",
        ))
    }

    fn samefile(&self, other: PathBuf) -> PyResult<bool> {
        #[cfg(feature = "same-file")]
        {
            Ok(same_file::is_same_file(self.path(), &other)?)
        }
        #[cfg(not(feature = "same-file"))]
        {
            Err(ryo3_core::FeatureNotEnabledError::new_err(
                "`same-file` feature not enabled",
            ))
        }
    }

    // ========================================================================
    // Methods from ::std::path::PathBuf
    // ========================================================================
    //  - PathBuf.add_extension
    //  - PathBuf.pop
    //  - PathBuf.push
    //  - PathBuf.set_extension
    //  - PathBuf.set_file_name
    // REMOVED FOR `frozen`
    // fn _push(mut slf: PyRefMut<'_, Self>, path: PathLike) -> PyRefMut<'_, PyFsPath> {
    //     slf.pth.push(path);
    //     slf
    // }
    //
    // fn _pop(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, PyFsPath> {
    //     slf.pth.pop();
    //     slf
    // }
    //
    // fn _set_extension(mut slf: PyRefMut<'_, Self>, ext: String) -> PyRefMut<'_, PyFsPath> {
    //     slf.pth.set_extension(ext);
    //     slf
    // }
    //
    // fn _set_file_name(mut slf: PyRefMut<'_, Self>, name: String) -> PyRefMut<'_, PyFsPath> {
    //     slf.pth.set_file_name(name);
    //     slf
    // }

    // ========================================================================
    // Methods from ::std::path::Path (Deref<Target=PathBuf>)
    // ========================================================================
    //  - [x] Path.ancestors
    //  - [x] Path.canonicalize
    //  - [x] Path.components
    //  - [x] Path.display
    //  - [x] Path.ends_with
    //  - [x] Path.exists
    //  - [x] Path.extension
    //  - [x] Path.file_name
    //  - [x] Path.file_prefix
    //  - [x] Path.file_stem
    //  - [x] Path.has_root
    //  - [x] Path.is_absolute
    //  - [x] Path.is_dir
    //  - [x] Path.is_file
    //  - [x] Path.is_relative
    //  - [x] Path.is_symlink
    //  - [ ] Path.iter
    //  - [ ] Path.join
    //  - [ ] Path.metadata
    //  - [ ] Path.read_dir
    //  - [ ] Path.read_link
    //  - [ ] Path.starts_with
    //  - [ ] Path.strip_prefix
    //  - [ ] Path.symlink_metadata
    //  - [ ] Path.try_exists
    //  - [ ] Path.with_added_extension
    //  - [ ] Path.with_extension
    //  - [ ] Path.with_file_name
    // __PYTHON_IMPL__ (implemented to adhere to pathlib.Path)
    //  - [x] Path.parent
    // __PATH_NOT_PYTHONABLE__
    //  - Path.to_str
    //  - Path.to_string_lossy
    //  - Path.to_path_buf
    //  - Path.as_mut_os_str
    //  - Path.as_os_str
    fn ancestors<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parents: Vec<Self> = self.pth.ancestors().map(PyFsPath::from).collect();
        PyTuple::new(py, parents)
    }

    fn canonicalize(&self) -> PyResult<Self> {
        let p = self.pth.canonicalize()?;
        Ok(PyFsPath::from(p))
    }

    fn components<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parts = self
            .pth
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        PyTuple::new(py, parts)
    }

    fn display(&self) -> String {
        self.pth.display().to_string()
    }

    fn ends_with(&self, path: PathLike) -> bool {
        self.pth.ends_with(path.as_ref())
    }

    fn exists(&self) -> PyResult<bool> {
        self.pth
            .try_exists()
            .map_err(|e| PyFileNotFoundError::new_err(format!("try_exists: {e}")))
    }

    fn extension(&self) -> Option<String> {
        self.pth
            .extension()
            .map(|e| e.to_string_lossy().to_string())
    }

    fn file_name(&self) -> Option<String> {
        self.pth
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn file_prefix(&self) -> Option<String> {
        self.pth
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn file_stem(&self) -> Option<String> {
        self.pth
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn has_root(&self) -> bool {
        self.pth.has_root()
    }

    fn is_absolute(&self) -> bool {
        self.pth.is_absolute()
    }
    fn is_dir(&self) -> bool {
        self.pth.is_dir()
    }
    fn is_file(&self) -> bool {
        self.pth.is_file()
    }
    fn is_relative(&self) -> bool {
        self.pth.is_relative()
    }
    fn is_symlink(&self) -> bool {
        self.pth.is_symlink()
    }

    fn join(&self, p: PathLike) -> PyFsPath {
        Self::from(self.pth.join(p))
    }

    fn metadata(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn read_dir(&self) -> PyResult<PyFsPathReadDir> {
        let rd = std::fs::read_dir(self.path())
            .map(PyFsPathReadDir::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("iterdir: {e}")))?;
        Ok(rd)
    }
    fn read_link(&self) -> PyResult<Self> {
        self.pth
            .read_link()
            .map(Self::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("read_link: {e}")))
    }
    fn starts_with(&self, p: PathLike) -> bool {
        self.pth.starts_with(p.as_ref())
    }
    fn strip_prefix(&self, p: PathLike) -> PyResult<PyFsPath> {
        self.pth
            .strip_prefix(p.as_ref())
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("strip_prefix: {e}")))
    }
    fn symlink_metadata(&self) -> PyResult<()> {
        err_py_not_impl!()
    }

    fn with_extension(&self, extension: String) -> Self {
        Self::from(self.pth.with_extension(extension))
    }

    fn with_file_name(&self, name: String) -> Self {
        Self::from(self.pth.with_file_name(name))
    }
}

impl<T> From<T> for PyFsPath
where
    T: AsRef<Path>,
{
    fn from(p: T) -> Self {
        PyFsPath {
            pth: ArcPathBuf::new(p.as_ref().to_path_buf()),
        }
    }
}

#[pyclass(name = "FsPathReadDir", module = "ryo3")]
pub struct PyFsPathReadDir {
    iter: std::fs::ReadDir,
}

#[pymethods]
impl PyFsPathReadDir {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyFsPath> {
        match slf.iter.next() {
            Some(Ok(entry)) => Some(PyFsPath::from(entry.path())),
            _ => None,
        }
    }
}

impl From<std::fs::ReadDir> for PyFsPathReadDir {
    fn from(iter: std::fs::ReadDir) -> Self {
        Self { iter }
    }
}
