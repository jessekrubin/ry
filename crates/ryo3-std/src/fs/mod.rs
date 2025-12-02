mod file_read_stream;
mod file_type;
use crate::fs::file_read_stream::PyFileReadStream;
pub use file_type::PyFileType;
use pyo3::exceptions::{
    PyIOError, PyIsADirectoryError, PyNotADirectoryError, PyRuntimeError, PyUnicodeDecodeError,
    PyValueError,
};
use pyo3::types::{PyBytes, PyDict};
use pyo3::{IntoPyObjectExt, intern, prelude::*};
use ryo3_core::types::PathLike;
use ryo3_macro_rules::py_type_err;
use std::convert::Into;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

#[pyclass(name = "Metadata", frozen, immutable_type)]
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
        if let Ok(created) = self.created() {
            metadata_dict.set_item(intern!(py, "created"), created)?;
        }
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

#[pyclass(name = "Permissions", frozen, immutable_type)]
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

#[pyclass(name = "DirEntry", frozen, immutable_type)]
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
#[pyo3(signature = (path, chunk_size = 65536, *, offset = 0, buffered = true))]
pub fn read_stream(
    path: PathBuf,
    chunk_size: usize,
    offset: u64,
    buffered: bool,
) -> PyResult<PyFileReadStream> {
    PyFileReadStream::py_new(path, chunk_size, offset, buffered)
}

#[pyfunction]
pub fn read(py: Python<'_>, path: PathLike) -> PyResult<ryo3_bytes::PyBytes> {
    let fbytes = py.detach(|| std::fs::read(path))?;
    Ok(fbytes.into())
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, path: PathLike) -> PyResult<Py<PyAny>> {
    let fbytes = py.detach(|| std::fs::read(path))?;
    Ok(PyBytes::new(py, &fbytes).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, path: PathLike) -> PyResult<Bound<'_, PyAny>> {
    let fbytes = py.detach(|| std::fs::read(path))?;
    match std::str::from_utf8(&fbytes) {
        Ok(s) => s.into_bound_py_any(py),
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
pub fn read_link(py: Python<'_>, path: PathBuf) -> PyResult<PathBuf> {
    let p = py.detach(|| std::fs::read_link(path))?;
    Ok(p)
}

#[pyfunction]
pub fn read_to_string(py: Python<'_>, path: PathBuf) -> PyResult<String> {
    py.detach(|| std::fs::read_to_string(path))
        .map_err(|e| PyIOError::new_err(format!("read_to_string - {e}")))
}

#[pyfunction]
pub fn read_str(py: Python<'_>, path: PathBuf) -> PyResult<String> {
    read_to_string(py, path)
}

#[pyfunction]
pub fn write(py: Python<'_>, path: PathBuf, data: &Bound<'_, PyAny>) -> PyResult<usize> {
    if let Ok(pystr) = data.cast_exact::<pyo3::types::PyString>() {
        let s = pystr.extract::<&str>()?;
        let bytes = s.as_bytes();
        py.detach(|| write_impl(path, bytes))
    } else if let Ok(b) = data.extract::<ryo3_bytes::PyBytes>() {
        py.detach(|| write_impl(path, b))
    } else {
        py_type_err!("write - expected str, bytes, bytes-like or buffer-protocol object")
    }
}

#[pyfunction]
pub fn write_bytes(py: Python<'_>, path: PathBuf, buf: &Bound<'_, PyAny>) -> PyResult<usize> {
    let bref = buf.extract::<ryo3_bytes::PyBytes>()?;
    py.detach(|| write_impl(path, bref))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_text(py: Python<'_>, path: PathBuf, s: &str) -> PyResult<usize> {
    let str_bytes = s.as_bytes();
    let r = py.detach(|| std::fs::write(&path, str_bytes).map(|()| str_bytes.len()))?;
    Ok(r)
}

#[pyfunction]
pub fn write_str(py: Python<'_>, path: PathBuf, string: &str) -> PyResult<usize> {
    write_text(py, path, string)
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn rename(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<()> {
    py.detach(|| std::fs::rename(&from, &to))?;
    Ok(())
}

#[pyfunction]
pub fn metadata(py: Python<'_>, path: PathLike) -> PyResult<PyMetadata> {
    let metadata = py.detach(|| std::fs::metadata(path))?;
    Ok(PyMetadata::from(metadata))
}

#[expect(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn copy(py: Python<'_>, from: PathBuf, to: PathBuf) -> PyResult<u64> {
    let copy_res = py.detach(|| std::fs::copy(&from, &to))?;
    Ok(copy_res)
}

#[pyfunction]
pub fn remove_file(py: Python<'_>, path: PathLike) -> PyResult<()> {
    py.detach(|| std::fs::remove_file(path))?;
    Ok(())
}

#[pyfunction]
pub fn remove_dir(py: Python<'_>, path: PathLike) -> PyResult<()> {
    py.detach(|| std::fs::remove_dir(path))?;
    Ok(())
}

#[pyfunction]
pub fn remove_dir_all(py: Python<'_>, path: PathLike) -> PyResult<()> {
    py.detach(|| std::fs::remove_dir_all(path))?;
    Ok(())
}

#[pyfunction]
pub fn create_dir(py: Python<'_>, path: PathLike) -> PyResult<()> {
    py.detach(|| std::fs::create_dir(path))?;
    Ok(())
}

#[pyfunction]
pub fn create_dir_all(py: Python<'_>, path: PathLike) -> PyResult<()> {
    py.detach(|| std::fs::create_dir_all(path))?;
    Ok(())
}

#[pyfunction]
pub fn canonicalize(path: PathLike) -> PyResult<()> {
    std::fs::canonicalize(path)?;
    Ok(())
}

#[pyfunction]
pub fn exists(py: Python<'_>, path: PathBuf) -> PyResult<bool> {
    py.detach(|| std::fs::exists(path))
        .map_err(|e| PyIOError::new_err(format!("exists - {e}")))
}

#[pyfunction]
pub fn hard_link(py: Python<'_>, original: PathBuf, link: PathBuf) -> PyResult<()> {
    py.detach(|| std::fs::hard_link(original, link))?;
    Ok(())
}

#[pyfunction]
pub fn set_permissions(py: Python<'_>, path: PathBuf, perm: &PyPermissions) -> PyResult<()> {
    py.detach(|| std::fs::set_permissions(path, perm.0.clone()))
        .map_err(|e| PyIOError::new_err(format!("set_permissions - {e}")))
}

#[pyfunction]
pub fn soft_link(py: Python<'_>, original: PathBuf, link: PathBuf) -> PyResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;
        py.detach(|| unix_fs::symlink(original, link))
            .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs as windows_fs;
        let metadata = py
            .detach(|| std::fs::metadata(&src))
            .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))?;
        if metadata.is_dir() {
            py.detach(|| windows_fs::symlink_dir(original, link))
                .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
        } else {
            py.detach(|| windows_fs::symlink_file(original, link))
                .map_err(|e| PyIOError::new_err(format!("soft_link - {e}")))
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        pytodo!("soft_link is not implemented on this platform");
    }
}

#[pyfunction]
pub fn symlink_metadata(py: Python<'_>, path: PathBuf) -> PyResult<PyMetadata> {
    let m = py
        .detach(|| std::fs::symlink_metadata(path))
        .map(PyMetadata::from)?;
    Ok(m)
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn read_dir(path: PathLike) -> PyResult<PyReadDir> {
    let path_ref = path.as_ref();
    let read_dir_res = std::fs::read_dir(path_ref);
    match read_dir_res {
        Ok(iter) => Ok(PyReadDir {
            path: path_ref.to_path_buf(),
            iter: Mutex::new(iter),
        }),
        Err(e) => {
            if path_ref.is_dir() {
                let pth_str = path_ref.to_string_lossy();
                Err(PyIsADirectoryError::new_err(format!(
                    "read_dir - parent: {pth_str} - {e}"
                )))
            } else {
                Err(e.into())
            }
        }
    }
}

#[pyclass(name = "ReadDir", frozen, immutable_type)]
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
    m.add_function(wrap_pyfunction!(read_str, m)?)?;
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
