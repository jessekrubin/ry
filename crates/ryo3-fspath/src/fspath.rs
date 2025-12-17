//! `FsPath` struct python module
use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyFileExistsError, PyFileNotFoundError, PyNotADirectoryError, PyUnicodeDecodeError,
    PyValueError,
};

use pyo3::types::{PyBytes, PyTuple};
use pyo3::{IntoPyObjectExt, intern, prelude::*};
use ryo3_bytes::extract_bytes_ref;
use ryo3_core::RyMutex;
use ryo3_core::types::PathLike;
use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

// separator
const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

type ArcPathBuf = std::sync::Arc<PathBuf>;

#[pyclass(name = "FsPath", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyFsPath {
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

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let os_str = self.path().as_os_str();
        PyTuple::new(py, vec![os_str])
    }

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> String {
        self.__str__()
    }

    fn __str__(&self) -> String {
        path2str(self.path())
    }

    fn __repr__(&self) -> String {
        let posix_str = self.as_posix();
        format!("FsPath(\'{posix_str}\')")
    }

    fn equiv(&self, other: PathLike) -> bool {
        let other = other.as_ref();
        self.path() == other
    }

    fn to_pathlib(&self) -> &Path {
        self.path()
    }

    fn to_py(&self) -> &Path {
        self.path()
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.path().hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: PathLike, op: CompareOp) -> bool {
        let o = other.as_ref();
        let p = self.path();
        match op {
            CompareOp::Eq => {
                // start by comparing as paths
                if p == other.as_ref() {
                    return true;
                }
                // if that fails, compare as strings
                self.py_to_string() == path2str(other.as_ref())
            }
            CompareOp::Ne => {
                // start by comparing as paths
                if p != other.as_ref() {
                    return true;
                }
                // if that fails, compare as strings
                self.py_to_string() != path2str(other.as_ref())
            }
            CompareOp::Lt => p < o,
            CompareOp::Le => p <= o,
            CompareOp::Gt => p > o,
            CompareOp::Ge => p >= o,
        }
    }

    fn absolute(&self, py: Python<'_>) -> PyResult<Self> {
        let p = py.detach(|| self.path().canonicalize())?;
        Ok(Self::from(p))
    }

    fn resolve(&self, py: Python<'_>) -> PyResult<Self> {
        let p = py.detach(|| self.path().canonicalize())?;
        Ok(Self::from(p))
    }

    fn __fspath__(&self) -> &OsStr {
        self.path().as_os_str()
    }

    fn clone(&self) -> Self {
        Self {
            pth: self.pth.clone(),
        }
    }

    fn __truediv__(&self, other: PathLike) -> Self {
        Self::from(self.path().join(other.as_ref()))
    }

    fn __rtruediv__(&self, other: PathLike) -> Self {
        let p = other.as_ref().join(self.path());
        Self::from(p)
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let s = path2str(self.path());
        let b = s.as_bytes();
        PyBytes::new(py, b)
    }

    #[getter]
    fn root(&self) -> Option<Self> {
        self.path()
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

    #[getter]
    fn parents<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parent = self.path().parent();
        if let Some(par) = parent {
            let parents: Vec<Self> = par.ancestors().map(Self::from).collect();
            PyTuple::new(py, parents)
        } else {
            // no parents
            PyTuple::new(py, Vec::<Self>::new())
        }
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
        Self::from(p)
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
                Some(e) => {
                    // push with leading dot to match python pathlib
                    suffixes.push(format!(".{e}"));
                }
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

    #[staticmethod]
    fn home() -> Option<Self> {
        ryo3_dirs::home().map(Self::from)
    }

    #[staticmethod]
    fn cwd() -> PyResult<Self> {
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

    // TODO: allow *args for joinpath
    fn joinpath(&self, other: PathLike) -> Self {
        let p = self.path().join(other.as_ref());
        Self::from(p)
    }

    fn read(&self, py: Python<'_>) -> PyResult<ryo3_bytes::PyBytes> {
        let fbytes = py.detach(|| std::fs::read(self.path()))?;
        Ok(fbytes.into())
    }

    fn read_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let fbytes = py
            .detach(|| std::fs::read(self.path()))
            .map(|b| PyBytes::new(py, &b))?;
        Ok(fbytes)
    }

    fn read_text<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let fbytes = py.detach(|| std::fs::read(self.path()))?;
        match std::str::from_utf8(&fbytes) {
            Ok(s) => s.into_bound_py_any(py),
            Err(e) => {
                let decode_err = PyUnicodeDecodeError::new_utf8(py, &fbytes, e)?;
                Err(decode_err.into())
            }
        }
    }

    fn write(&self, py: Python<'_>, data: &Bound<'_, PyAny>) -> PyResult<usize> {
        let b = extract_bytes_ref(data)?;
        let write_res = py.detach(|| std::fs::write(self.path(), b));
        match write_res {
            Ok(()) => Ok(b.len()),
            Err(e) => {
                let fspath = self.py_to_string();
                Err(PyNotADirectoryError::new_err(format!(
                    "write_bytes - parent: {fspath} - {e}"
                )))
            }
        }
    }

    fn write_bytes(&self, py: Python<'_>, data: &Bound<'_, PyAny>) -> PyResult<usize> {
        self.write(py, data)
    }

    fn write_text(&self, py: Python<'_>, data: &str) -> PyResult<()> {
        let write_result = py.detach(|| std::fs::write(self.path(), data));
        match write_result {
            Ok(()) => Ok(()),
            Err(e) => {
                let fspath = self.py_to_string();
                Err(PyNotADirectoryError::new_err(format!(
                    "write_bytes - parent: {fspath} - {e}"
                )))
            }
        }
    }

    #[pyo3(signature = (mode =  0o777, parents = false, exist_ok = false))]
    #[allow(unused_variables)]
    fn mkdir(&self, py: Python<'_>, mode: u32, parents: bool, exist_ok: bool) -> PyResult<()> {
        let path = self.path();

        let exists = path.exists();
        if !exist_ok && exists {
            return Err(PyFileExistsError::new_err(format!(
                "mkdir - parent: {} - directory already exists",
                self.py_to_string()
            )));
        }
        if parents {
            py.detach(|| std::fs::create_dir_all(path)).map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("mkdir - parent: {fspath} - {e}"))
            })?;
        } else {
            py.detach(|| std::fs::create_dir(path)).map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("mkdir - parent: {fspath} - {e}"))
            })?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = py
                .detach(|| std::fs::metadata(path))
                .map_err(|e| {
                    PyFileNotFoundError::new_err(format!(
                        "mkdir - parent: {} - {e}",
                        self.py_to_string()
                    ))
                })?
                .permissions();
            perms.set_mode(mode);
            py.detach(|| std::fs::set_permissions(path, perms))
                .map_err(|e| {
                    let fspath = self.py_to_string();
                    PyNotADirectoryError::new_err(format!("mkdir - parent: {fspath} - {e}"))
                })?;
        }
        Ok(())
    }

    #[pyo3(signature = (recursive = false))]
    fn rmdir(&self, py: Python<'_>, recursive: bool) -> PyResult<()> {
        if !self.exists(py)? {
            return Err(PyFileNotFoundError::new_err(format!(
                "rmdir - parent: {} - directory does not exist",
                self.py_to_string()
            )));
        }
        if !self.is_dir() {
            return Err(PyNotADirectoryError::new_err(format!(
                "rmdir - parent: {} - not a directory",
                self.py_to_string()
            )));
        }
        if recursive {
            py.detach(|| std::fs::remove_dir_all(self.path()))
                .map_err(|e| {
                    let fspath = self.py_to_string();
                    PyNotADirectoryError::new_err(format!("rmdir - parent: {fspath} - {e}"))
                })?;
        } else {
            py.detach(|| std::fs::remove_dir(self.path()))
                .map_err(|e| {
                    let fspath = self.py_to_string();
                    PyNotADirectoryError::new_err(format!("rmdir - parent: {fspath} - {e}"))
                })?;
        }
        Ok(())
    }

    #[pyo3(signature = (missing_ok = false, recursive = false))]
    fn unlink(&self, py: Python<'_>, missing_ok: bool, recursive: bool) -> PyResult<()> {
        if !self.exists(py)? {
            if missing_ok {
                return Ok(());
            }
            return Err(PyFileNotFoundError::new_err(format!(
                "unlink - parent: {} - file does not exist",
                self.py_to_string()
            )));
        }
        if self.is_dir() {
            self.rmdir(py, recursive)?;
        } else {
            py.detach(|| std::fs::remove_file(self.path()))
                .map_err(|e| {
                    let fspath = self.py_to_string();
                    PyNotADirectoryError::new_err(format!("unlink - parent: {fspath} - {e}"))
                })?;
        }
        Ok(())
    }

    fn rename(&self, py: Python<'_>, new_path: PathLike) -> PyResult<Self> {
        if new_path.as_ref() == self.path() {
            return Ok(self.clone());
        }
        let new_path = new_path.as_ref();
        let new_path_exists = py.detach(|| new_path.exists());
        if new_path_exists {
            return Err(PyFileExistsError::new_err(format!(
                "rename - parent: {} - destination already exists",
                self.py_to_string()
            )));
        }
        py.detach(|| std::fs::rename(self.path(), new_path))
            .map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("rename - parent: {fspath} - {e}"))
            })?;
        Ok(Self::from(new_path))
    }

    fn replace(&self, py: Python<'_>, new_path: PathLike) -> PyResult<Self> {
        if new_path.as_ref() == self.path() {
            return Ok(self.clone());
        }
        let new_path = new_path.as_ref();
        if new_path.exists() {
            // nuke file/dir
            if new_path.is_dir() {
                py.detach(|| std::fs::remove_dir_all(new_path))?;
            } else {
                py.detach(|| std::fs::remove_file(new_path))?;
            }
        }
        py.detach(|| std::fs::rename(self.path(), new_path))
            .map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("replace - parent: {fspath} - {e}"))
            })?;
        Ok(Self::from(new_path))
    }

    fn as_uri(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "as_uri not implemented",
        ))
    }

    fn iterdir(&self, py: Python<'_>) -> PyResult<PyFsPathReadDir> {
        self.read_dir(py)
    }

    #[pyo3(signature = (
        *args,
        **kwargs
    ))]
    fn open<'py>(
        &self,
        py: Python<'py>,
        args: &Bound<'py, PyTuple>,
        kwargs: Option<&Bound<'py, pyo3::types::PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // forward to python's built-in open function
        let pypathlib_ob = crate::pathlib::path2pathlib(py, self.path())?;
        pypathlib_ob.call_method(intern!(py, "open"), args, kwargs)
    }

    #[expect(unused_variables)]
    fn relative_to(&self, other: PathLike) -> PyResult<Self> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "relative_to not implemented",
        ))
    }

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
    //  - [x] Path.join
    //  - [x] Path.metadata
    //  - [x] Path.read_dir
    //  - [x] Path.read_link
    //  - [x] Path.starts_with
    //  - [x] Path.strip_prefix
    //  - [x] Path.symlink_metadata
    //  - [ ] Path.with_added_extension - unstable
    //  - [x] Path.with_extension
    //  - [x] Path.with_file_name
    // __PYTHON_IMPL__ (implemented to adhere to pathlib.Path)
    //  - [x] Path.parent
    // __PATH_NOT_PYTHONABLE__
    //  - Path.to_str
    //  - Path.to_string_lossy
    //  - Path.to_path_buf
    //  - Path.as_mut_os_str
    //  - Path.as_os_str
    fn ancestors(&self) -> PyFsPathAncestors {
        PyFsPathAncestors::new(self.path())
    }

    fn canonicalize(&self, py: Python<'_>) -> PyResult<Self> {
        let p = py.detach(|| self.path().canonicalize())?;
        Ok(Self::from(p))
    }

    fn components<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let parts = self
            .path()
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<String>>();
        PyTuple::new(py, parts)
    }

    fn display(&self) -> String {
        self.path().display().to_string()
    }

    fn ends_with(&self, child: PathLike) -> bool {
        self.path().ends_with(child.as_ref())
    }

    fn exists(&self, py: Python<'_>) -> PyResult<bool> {
        py.detach(|| self.path().try_exists())
            .map_err(|e| PyFileNotFoundError::new_err(format!("try_exists: {e}")))
    }

    fn extension(&self) -> Option<String> {
        self.path()
            .extension()
            .map(|e| e.to_string_lossy().to_string())
    }

    fn file_name(&self) -> Option<String> {
        self.path()
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn file_prefix(&self) -> Option<String> {
        self.path()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn file_stem(&self) -> Option<String> {
        self.path()
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
    }

    fn has_root(&self) -> bool {
        self.path().has_root()
    }

    fn is_absolute(&self) -> bool {
        self.path().is_absolute()
    }

    fn is_dir(&self) -> bool {
        self.path().is_dir()
    }

    fn is_file(&self) -> bool {
        self.path().is_file()
    }

    fn is_relative(&self) -> bool {
        self.path().is_relative()
    }

    fn is_symlink(&self) -> bool {
        self.path().is_symlink()
    }

    fn join(&self, p: PathLike) -> Self {
        Self::from(self.path().join(p))
    }

    fn metadata(&self, py: Python<'_>) -> PyResult<ryo3_std::fs::PyMetadata> {
        py.detach(|| self.path().metadata())
            .map(ryo3_std::fs::PyMetadata::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("metadata: {e}")))
    }

    fn read_dir(&self, py: Python<'_>) -> PyResult<PyFsPathReadDir> {
        py.detach(|| std::fs::read_dir(self.path()))
            .map(PyFsPathReadDir::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("read_dir: {e}")))
    }

    fn read_link(&self, py: Python<'_>) -> PyResult<Self> {
        py.detach(|| self.path().read_link())
            .map(Self::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("read_link: {e}")))
    }

    fn starts_with(&self, base: PathLike) -> bool {
        self.path().starts_with(base.as_ref())
    }

    fn strip_prefix(&self, base: PathLike) -> PyResult<Self> {
        self.path()
            .strip_prefix(base.as_ref())
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("strip_prefix: {e}")))
    }

    fn symlink_metadata(&self, py: Python<'_>) -> PyResult<ryo3_std::fs::PyMetadata> {
        py.detach(|| self.path().symlink_metadata())
            .map(ryo3_std::fs::PyMetadata::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("metadata: {e}")))
    }

    fn with_extension(&self, extension: String) -> Self {
        Self::from(self.path().with_extension(extension))
    }

    fn with_file_name(&self, name: String) -> Self {
        Self::from(self.path().with_file_name(name))
    }

    // ========================================================================
    // FEATURES
    // ========================================================================

    // -------------------------------------------------------------------------
    // `same-file` feature
    // ------------------------------------------------------------------------

    #[cfg(feature = "same-file")]
    fn samefile(&self, py: Python<'_>, other: PathBuf) -> PyResult<bool> {
        py.detach(|| same_file::is_same_file(self.path(), &other))
            .map_err(|e| PyFileNotFoundError::new_err(format!("samefile: {e}")))
    }

    #[cfg(not(feature = "same-file"))]
    fn samefile(&self, _other: PathBuf) -> PyResult<bool> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`same-file` feature not enabled",
        ))
    }

    // -------------------------------------------------------------------------
    // `which` feature
    // -------------------------------------------------------------------------
    #[cfg(feature = "which")]
    #[staticmethod]
    #[pyo3(signature = (cmd, path=None))]
    fn which(py: Python<'_>, cmd: &str, path: Option<&str>) -> PyResult<Option<Self>> {
        ryo3_which::which(py, cmd, path).map(|opt| opt.map(Self::from))
    }

    #[cfg(not(feature = "which"))]
    #[staticmethod]
    #[pyo3(signature = (_cmd, _path=None))]
    fn which(_cmd: &str, _path: Option<&str>) -> PyResult<Option<Self>> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`which` feature not enabled",
        ))
    }

    #[cfg(feature = "which")]
    #[staticmethod]
    #[pyo3(signature = (cmd, path=None))]
    fn which_all(py: Python<'_>, cmd: &str, path: Option<&str>) -> PyResult<Vec<Self>> {
        ryo3_which::which_all(py, cmd, path)
            .map(|opt| opt.into_iter().map(Self::from).collect::<Vec<_>>())
    }

    #[cfg(not(feature = "which"))]
    #[staticmethod]
    #[pyo3(signature = (_cmd, _path=None))]
    fn which_all(_cmd: &str, _path: Option<&str>) -> PyResult<Vec<Self>> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`which` feature not enabled",
        ))
    }

    #[cfg(feature = "which-regex")]
    #[staticmethod]
    #[pyo3(signature = (regex, path=None))]
    fn which_re(
        py: Python<'_>,
        regex: &Bound<'_, PyAny>,
        path: Option<&str>,
    ) -> PyResult<Vec<Self>> {
        ryo3_which::which_re(py, regex, path)
            .map(|opt| opt.into_iter().map(Self::from).collect::<Vec<_>>())
    }

    #[cfg(not(feature = "which-regex"))]
    #[staticmethod]
    #[pyo3(signature = (_regex, _path=None))]
    fn which_re(_regex: &Bound<'_, PyAny>, _path: Option<&str>) -> PyResult<Vec<Self>> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`which` feature not enabled",
        ))
    }
}

impl<T> From<T> for PyFsPath
where
    T: AsRef<Path>,
{
    fn from(p: T) -> Self {
        Self {
            pth: ArcPathBuf::new(p.as_ref().to_path_buf()),
        }
    }
}

#[pyclass(name = "FsPathReadDir", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyFsPathReadDir {
    iter: RyMutex<std::fs::ReadDir, false>,
}

#[pymethods]
impl PyFsPathReadDir {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> Option<PyFsPath> {
        let value = py.detach(|| self.iter.py_lock().next());
        match value {
            Some(Ok(entry)) => Some(PyFsPath::from(entry.path())),
            _ => None,
        }
    }

    fn collect(&self, py: Python<'_>) -> Vec<PyFsPath> {
        py.detach(|| {
            let mut paths = vec![];
            for entry in self.iter.py_lock().by_ref() {
                match entry {
                    Ok(entry) => paths.push(PyFsPath::from(entry.path())),
                    Err(_e) => break, // TODO: handle error
                }
            }
            paths
        })
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<PyFsPath> {
        py.detach(|| {
            self.iter
                .py_lock()
                .by_ref()
                .take(n)
                .filter_map(|entry| match entry {
                    Ok(entry) => Some(PyFsPath::from(entry.path())),
                    Err(_) => None,
                })
                .collect()
        })
    }
}

impl From<std::fs::ReadDir> for PyFsPathReadDir {
    fn from(iter: std::fs::ReadDir) -> Self {
        Self { iter: iter.into() }
    }
}

#[pyclass(name = "FsPathAncestors", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyFsPathAncestors {
    path: ArcPathBuf,
    current: RyMutex<Option<ArcPathBuf>, false>,
}

impl PyFsPathAncestors {
    fn new<P: AsRef<Path>>(p: P) -> Self {
        Self {
            path: ArcPathBuf::new(p.as_ref().to_path_buf()),
            current: RyMutex::new(Some(ArcPathBuf::new(p.as_ref().to_path_buf()))),
        }
    }
}

#[pymethods]
impl PyFsPathAncestors {
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<PyFsPath> {
        let mut current = self.current.py_lock();
        let taken = current.take().map(|p| PyFsPath::from(p.as_ref()));
        if let Some(ref p) = taken {
            let next = p.path().parent().map(|p| ArcPathBuf::new(p.to_path_buf()));
            *current = next;
        }
        taken
    }

    fn collect(&self) -> Vec<PyFsPath> {
        let mut paths = vec![];
        while let Some(path) = self.__next__() {
            paths.push(path);
        }
        paths
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> Vec<PyFsPath> {
        let mut paths = vec![];
        for _ in 0..n {
            if let Some(path) = self.__next__() {
                paths.push(path);
            } else {
                break;
            }
        }
        paths
    }
}

impl std::fmt::Display for PyFsPathAncestors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FsPathAncestors<{}>", self.path.display())
    }
}
