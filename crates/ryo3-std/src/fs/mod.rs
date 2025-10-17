mod file_read_stream;
mod file_type;
use crate::fs::file_read_stream::{DEFAULT_CHUNK_SIZE, PyFileReadStream};
pub use file_type::PyFileType;
use pyo3::exceptions::{
    PyIOError, PyIsADirectoryError, PyNotADirectoryError, PyRuntimeError, PyUnicodeDecodeError,
    PyValueError,
};
use pyo3::types::{PyBytes, PyDict};
use pyo3::{intern, prelude::*};
use ryo3_bytes::extract_bytes_ref_str;
use ryo3_core::types::PathLike;
use std::convert::Into;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

#[pyclass(name = "Metadata", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone)]
pub struct PyMetadata(pub std::fs::Metadata);

impl From<std::fs::Metadata> for PyMetadata {
    fn from(m: std::fs::Metadata) -> Self {
        Self(m)
    }
}

impl PyMetadata {
    #[must_use]
    pub fn new(m: std::fs::Metadata) -> Self {
        Self(m)
    }
}

#[pymethods]
impl PyMetadata {
    fn __repr__(&self) -> String {
        format!(
            "Metadata<is_dir={}, is_file={}, is_symlink={}, len={}, readonly={}>",
            self.0.is_dir(),
            self.0.is_file(),
            self.0.file_type().is_symlink(),
            self.0.len(),
            self.0.permissions().readonly(),
        )
    }

    fn to_py<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let metadata_dict = PyDict::new(py);
        metadata_dict.set_item(intern!(py, "is_dir"), self.is_dir())?;
        metadata_dict.set_item(intern!(py, "is_file"), self.is_file())?;
        metadata_dict.set_item(intern!(py, "is_symlink"), self.is_symlink())?;
        metadata_dict.set_item(intern!(py, "len"), self.len())?;
        metadata_dict.set_item(intern!(py, "readonly"), self.readonly())?;
        metadata_dict.set_item(intern!(py, "file_type"), self.file_type().to_py(py))?;
        metadata_dict.set_item(intern!(py, "accessed"), self.accessed()?)?;
        metadata_dict.set_item(intern!(py, "created"), self.created()?)?;
        metadata_dict.set_item(intern!(py, "modified"), self.modified()?)?;
        Ok(metadata_dict)
    }

    #[getter]
    fn accessed(&self) -> PyResult<SystemTime> {
        let accessed = self.0.accessed()?;
        Ok(accessed)
    }

    #[getter]
    fn created(&self) -> PyResult<SystemTime> {
        let created = self.0.created()?;
        Ok(created)
    }

    #[getter]
    #[must_use]
    fn file_type(&self) -> PyFileType {
        PyFileType::new(self.0.file_type())
    }

    #[getter]
    #[must_use]
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[getter]
    #[must_use]
    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[getter]
    #[must_use]
    fn is_symlink(&self) -> bool {
        self.0.file_type().is_symlink()
    }

    #[getter]
    #[must_use]
    fn len(&self) -> u64 {
        self.0.len()
    }

    #[getter]
    #[must_use]
    fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    #[getter]
    fn modified(&self) -> PyResult<SystemTime> {
        let modified = self.0.modified()?;
        Ok(modified)
    }

    #[getter]
    #[must_use]
    fn readonly(&self) -> bool {
        self.0.permissions().readonly()
    }

    #[getter]
    fn permissions(&self) -> PyPermissions {
        let permissions = self.0.permissions();
        PyPermissions::from(permissions)
    }
}

#[pyclass(name = "Permissions", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PyPermissions(pub std::fs::Permissions);

impl From<std::fs::Permissions> for PyPermissions {
    fn from(p: std::fs::Permissions) -> Self {
        Self(p)
    }
}

impl PyPermissions {
    #[must_use]
    pub fn new(p: std::fs::Permissions) -> Self {
        Self(p)
    }
}

#[pymethods]
impl PyPermissions {
    #[getter]
    #[must_use]
    fn readonly(&self) -> bool {
        self.0.readonly()
    }

    fn __repr__(&self) -> &str {
        if self.readonly() {
            "Permissions<readonly=True>"
        } else {
            "Permissions<readonly=False>"
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

#[pyclass(name = "DirEntry", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyDirEntry(std::fs::DirEntry);

impl From<std::fs::DirEntry> for PyDirEntry {
    fn from(de: std::fs::DirEntry) -> Self {
        Self(de)
    }
}

#[pymethods]
impl PyDirEntry {
    fn __repr__(&self) -> String {
        let path = self.0.path();
        let pathstr = path.to_string_lossy();
        let s = format!("DirEntry('{pathstr}')");
        s
    }

    #[must_use]
    fn __fspath__(&self) -> OsString {
        self.0.path().into_os_string()
    }

    #[getter]
    fn path(&self) -> PathBuf {
        self.0.path()
    }

    #[getter]
    fn file_type(&self) -> PyResult<PyFileType> {
        let file_type = self.0.file_type()?;
        Ok(PyFileType::new(file_type))
    }

    #[getter]
    fn metadata(&self) -> PyResult<PyMetadata> {
        let metadata = self.0.metadata()?;
        Ok(PyMetadata::new(metadata))
    }

    #[getter]
    fn basename(&self) -> PyResult<OsString> {
        let path = self.0.path();
        let name = path.file_name().ok_or_else(|| {
            PyValueError::new_err(format!(
                "basename - path: {} - no file name",
                path.to_string_lossy()
            ))
        })?;
        Ok(name.to_os_string())
    }
}

// ============================================================================
// FUNCTIONS
// ============================================================================

#[pyfunction]
#[pyo3(signature = (pth, *, chunk_size = DEFAULT_CHUNK_SIZE, offset = 0, buffered = true))]
pub fn read_stream(
    pth: PathBuf,
    chunk_size: usize,
    offset: u64,
    buffered: bool,
) -> PyResult<PyFileReadStream> {
    PyFileReadStream::py_new(pth, chunk_size, offset, buffered)
}

#[pyfunction]
pub fn read(pth: PathLike) -> PyResult<ryo3_bytes::PyBytes> {
    let fbytes = std::fs::read(pth)?;
    Ok(fbytes.into())
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: PathLike) -> PyResult<Py<PyAny>> {
    let fbytes = std::fs::read(s)?;
    Ok(PyBytes::new(py, &fbytes).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: PathLike) -> PyResult<String> {
    let fbytes = std::fs::read(s)?;
    match std::str::from_utf8(&fbytes).map(ToString::to_string) {
        Ok(s) => Ok(s),
        Err(e) => {
            let decode_err = PyUnicodeDecodeError::new_utf8(py, &fbytes, e)?;
            Err(decode_err.into())
        }
    }
}

fn write_impl<P: AsRef<Path>, C: AsRef<[u8]>>(fspath: P, b: C) -> PyResult<usize> {
    let write_res = std::fs::write(fspath.as_ref(), b.as_ref());
    match write_res {
        Ok(()) => Ok(b.as_ref().len()),
        Err(e) => {
            let fspath_str = fspath.as_ref().to_string_lossy();
            Err(PyNotADirectoryError::new_err(format!(
                "write_bytes - parent: {fspath_str} - {e}"
            )))
        }
    }
}

#[pyfunction]
pub fn read_link(pth: PathBuf) -> PyResult<PathBuf> {
    let p = std::fs::read_link(pth)?;
    Ok(p)
}

#[pyfunction]
pub fn read_to_string(pth: PathBuf) -> PyResult<String> {
    std::fs::read_to_string(pth).map_err(|e| PyIOError::new_err(format!("read_to_string - {e}")))
}

#[pyfunction]
pub fn write(fspath: PathBuf, b: &Bound<'_, PyAny>) -> PyResult<usize> {
    let bref = extract_bytes_ref_str(b)?;
    write_impl(fspath, bref)
}

#[pyfunction]
pub fn write_bytes(fspath: PathBuf, b: &Bound<'_, PyAny>) -> PyResult<usize> {
    let bref = extract_bytes_ref_str(b)?;
    write_impl(fspath, bref)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_text(fspath: PathBuf, string: &str) -> PyResult<usize> {
    let str_bytes = string.as_bytes();
    match std::fs::write(&fspath, str_bytes) {
        Ok(()) => Ok(str_bytes.len()),
        Err(e) => {
            let fspath_str = fspath.to_string_lossy();
            Err(PyNotADirectoryError::new_err(format!(
                "write_bytes - parent: {fspath_str} - {e}"
            )))
        }
    }
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn rename(from: PathBuf, to: PathBuf) -> PyResult<()> {
    std::fs::rename(&from, &to)?;
    Ok(())
}

#[pyfunction]
pub fn metadata(pth: PathLike) -> PyResult<PyMetadata> {
    std::fs::metadata(pth)
        .map(PyMetadata::from)
        .map_err(Into::into)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn copy(from: PathBuf, to: PathBuf) -> PyResult<u64> {
    let copy_res = std::fs::copy(&from, &to)?;
    Ok(copy_res)
}

#[pyfunction]
pub fn remove_file(pth: PathLike) -> PyResult<()> {
    std::fs::remove_file(pth)?;
    Ok(())
}

#[pyfunction]
pub fn remove_dir(pth: PathLike) -> PyResult<()> {
    std::fs::remove_dir(pth)?;
    Ok(())
}

#[pyfunction]
pub fn remove_dir_all(pth: PathLike) -> PyResult<()> {
    std::fs::remove_dir_all(pth)?;
    Ok(())
}

#[pyfunction]
pub fn create_dir(pth: PathLike) -> PyResult<()> {
    std::fs::create_dir(pth)?;
    Ok(())
}

#[pyfunction]
pub fn create_dir_all(pth: PathLike) -> PyResult<()> {
    std::fs::create_dir_all(pth)?;
    Ok(())
}

#[pyfunction]
pub fn canonicalize(pth: PathLike) -> PyResult<()> {
    std::fs::canonicalize(pth)?;
    Ok(())
}

#[pyfunction]
pub fn exists(pth: PathBuf) -> PyResult<bool> {
    std::fs::exists(pth).map_err(|e| PyIOError::new_err(format!("exists - {e}")))
}

#[pyfunction]
pub fn hard_link(src: PathBuf, dst: PathBuf) -> PyResult<()> {
    std::fs::hard_link(src, dst)?;
    Ok(())
}

#[pyfunction]
pub fn set_permissions(pth: PathBuf, perm: &PyPermissions) -> PyResult<()> {
    std::fs::set_permissions(pth, perm.0.clone())
        .map_err(|e| PyIOError::new_err(format!("set_permissions - {e}")))
}

#[pyfunction]
pub fn soft_link(src: PathBuf, dst: PathBuf) -> PyResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;
        unix_fs::symlink(src, dst).map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs as windows_fs;
        let metadata =
            std::fs::metadata(&src).map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))?;
        if metadata.is_dir() {
            windows_fs::symlink_dir(src, dst)
                .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
        } else {
            windows_fs::symlink_file(src, dst)
                .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        pytodo!("soft_link is not implemented on this platform");
    }
}

#[pyfunction]
pub fn symlink_metadata(pth: PathBuf) -> PyResult<PyMetadata> {
    let m = std::fs::symlink_metadata(pth).map(PyMetadata::from)?;
    Ok(m)
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn read_dir(pth: PathLike) -> PyResult<PyReadDir> {
    let pth = pth.as_ref();
    let read_dir_res = std::fs::read_dir(pth);
    match read_dir_res {
        Ok(iter) => Ok(PyReadDir {
            path: pth.to_path_buf(),
            iter: Mutex::new(iter),
        }),
        Err(e) => {
            if pth.is_dir() {
                let pth_str = pth.to_string_lossy();
                Err(PyIsADirectoryError::new_err(format!(
                    "read_dir - parent: {pth_str} - {e}"
                )))
            } else {
                Err(e.into())
            }
        }
    }
}

#[pyclass(name = "ReadDir", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyReadDir {
    path: PathBuf,
    iter: Mutex<std::fs::ReadDir>,
}

#[pymethods]
impl PyReadDir {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> PyResult<Option<PyDirEntry>> {
        if let Ok(mut iter) = self.iter.lock() {
            match iter.next() {
                Some(Ok(entry)) => Ok(Some(PyDirEntry::from(entry))),
                _ => Ok(None),
            }
        } else {
            Err(PyRuntimeError::new_err("PyReadDir lock poisoned"))
        }
    }

    fn collect(&self) -> PyResult<Vec<PyDirEntry>> {
        let mut paths = vec![];
        let mut iter = self
            .iter
            .lock()
            .map_err(|_| PyRuntimeError::new_err("PyReadDir lock poisoned"))?;
        for entry in iter.by_ref() {
            match entry {
                Ok(entry) => paths.push(PyDirEntry::from(entry)),
                Err(_) => break,
            }
        }
        Ok(paths)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> PyResult<Vec<PyDirEntry>> {
        let mut paths = vec![];
        let mut iter = self
            .iter
            .lock()
            .map_err(|_| PyRuntimeError::new_err("PyReadDir lock poisoned"))?;
        for entry in iter.by_ref().take(n) {
            match entry {
                Ok(entry) => paths.push(PyDirEntry::from(entry)),
                Err(_) => break,
            }
        }
        Ok(paths)
    }
}

impl std::fmt::Debug for PyReadDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.to_string_lossy();
        write!(f, "ReadDir(\"{path}\")")
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFileReadStream>()?;
    m.add_class::<PyFileType>()?;
    m.add_class::<PyDirEntry>()?;
    m.add_class::<PyReadDir>()?;
    m.add_class::<PyMetadata>()?;
    m.add_function(wrap_pyfunction!(canonicalize, m)?)?;
    m.add_function(wrap_pyfunction!(copy, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir, m)?)?;
    m.add_function(wrap_pyfunction!(create_dir_all, m)?)?;
    m.add_function(wrap_pyfunction!(exists, m)?)?;
    m.add_function(wrap_pyfunction!(hard_link, m)?)?;
    m.add_function(wrap_pyfunction!(metadata, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(read_dir, m)?)?;
    m.add_function(wrap_pyfunction!(read_link, m)?)?;
    m.add_function(wrap_pyfunction!(read_stream, m)?)?;
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(read_to_string, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir, m)?)?;
    m.add_function(wrap_pyfunction!(remove_dir_all, m)?)?;
    m.add_function(wrap_pyfunction!(remove_file, m)?)?;
    m.add_function(wrap_pyfunction!(rename, m)?)?;
    m.add_function(wrap_pyfunction!(set_permissions, m)?)?;
    m.add_function(wrap_pyfunction!(soft_link, m)?)?;
    m.add_function(wrap_pyfunction!(symlink_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(write, m)?)?;
    m.add_function(wrap_pyfunction!(write_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(write_text, m)?)?;
    Ok(())
}
