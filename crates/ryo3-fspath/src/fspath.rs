//! `FsPath` struct python module
use std::ffi::{OsStr, OsString};
use std::fmt;
#[cfg(target_os = "windows")]
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
#[cfg(target_os = "windows")]
use std::path::Component;
use std::path::{Path, PathBuf};

use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyFileExistsError, PyFileNotFoundError, PyNotADirectoryError, PyValueError,
};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::{BoundObject, intern};
use ryo3_bytes::{ReadableBuffer, RyBytes};
use ryo3_core::macros::{any_repr, py_type_err};
use ryo3_core::sync::RyMutex;
use ryo3_core::types::{PathLike, PyUtf8Bytes};
use ryo3_macro_rules::pytodo;

// separator
const MAIN_SEPARATOR: char = std::path::MAIN_SEPARATOR;

#[pyclass(name = "FsPath", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PyFsPath(PathBuf);

impl PyFsPath {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        Self(to_native_pathbuf(p))
    }

    #[must_use]
    #[inline]
    pub fn path(&self) -> &Path {
        &self.0
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

// PathBuf preserves forward slashes `/` in windows but afaict python's
// `pathlib.Path` normalizes them. fucking windows.
#[cfg(target_os = "windows")]
fn to_native_pathbuf<P: AsRef<Path>>(p: P) -> PathBuf {
    let s = p.as_ref().to_string_lossy();
    if s.contains('/') {
        PathBuf::from(s.replace('/', "\\"))
    } else {
        p.as_ref().to_path_buf()
    }
}

#[cfg(not(target_os = "windows"))]
#[inline]
fn to_native_pathbuf<P: AsRef<Path>>(p: P) -> PathBuf {
    p.as_ref().to_path_buf()
}

#[pymethods]
impl PyFsPath {
    #[new]
    #[pyo3(signature = (*args))]
    fn py_new(args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        if args.is_empty() {
            return Ok(Self::from("."));
        }
        let mut path = PathBuf::new();
        for arg in args.iter() {
            let segment: PathBuf = arg.extract()?;
            path = path.join(segment);
        }
        Ok(Self::from(path))
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
        let path = self.path();
        let posix_path = path.posix_display();
        format!("FsPath(\'{posix_path}\')")
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

    #[expect(clippy::needless_pass_by_value, reason = "python arg extract")]
    fn equiv(&self, other: FsPathLike) -> bool {
        let other = other.as_ref();
        self.path() == other || self.py_to_string() == path2str(other)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self == other,
            CompareOp::Ne => self != other,
            CompareOp::Lt => self < other,
            CompareOp::Le => self <= other,
            CompareOp::Gt => self > other,
            CompareOp::Ge => self >= other,
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
        Clone::clone(self)
    }

    fn __truediv__(&self, other: FsPathLike) -> Self {
        Self::from(self.path().join(other))
    }

    #[expect(clippy::needless_pass_by_value, reason = "python arg extract")]
    fn __rtruediv__(&self, other: FsPathLike) -> Self {
        let p = other.as_ref().join(self.path());
        Self::from(p)
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let s = path2str(self.path());
        let b = s.as_bytes();
        PyBytes::new(py, b)
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        let py = value.py();
        if let Ok(val) = value.cast_exact::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(p) = value.extract::<PathBuf>() {
            Self::from(p).into_pyobject(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("FsPath conversion error: {valtype}")
        }
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
    fn drive(&self) -> Option<OsString> {
        let drive = self.path().components().next();
        match drive {
            Some(Component::Prefix(pref)) => Some(pref.as_os_str().to_os_string()),
            _ => None,
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
                let anchor_disp = anchor.as_os_str().display();
                // ensure that the anchor ends with a separator
                format!("{anchor_disp}{MAIN_SEPARATOR}")
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
    fn parents(&self) -> PyFsPathAncestors {
        PyFsPathAncestors::parents(self.path())
    }

    #[getter]
    fn parts<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        self.components(py)
    }

    #[getter]
    fn name(&self) -> OsString {
        match self.path().file_name() {
            Some(name) => name.to_os_string(),
            None => OsString::new(),
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
                if e.as_encoded_bytes().starts_with(b".") {
                    e.to_string_lossy().to_string()
                } else {
                    format!(".{}", e.display()) // python pathlib.path does this
                }
            }
            None => String::new(),
        }
    }

    fn with_suffix(&self, suffix: &str) -> Self {
        // auto strip leading dot
        let suffix = if suffix.starts_with('.') {
            suffix.trim_start_matches('.')
        } else {
            suffix
        };
        self.path().with_extension(suffix).into()
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
    fn stem(&self) -> PyResult<&OsStr> {
        self.path().file_stem().ok_or_else(|| {
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
        self.path().as_posix_str()
    }

    #[pyo3(signature = (*args))]
    fn joinpath(&self, args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let mut path = self.path().to_path_buf();
        for arg in args.iter() {
            let segment: PathBuf = arg.extract()?;
            path = path.join(segment);
        }
        Ok(path.into())
    }

    fn read(&self, py: Python<'_>) -> PyResult<RyBytes> {
        let fbytes = py.detach(|| std::fs::read(self.path()))?;
        Ok(fbytes.into())
    }

    fn read_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let fbytes = py
            .detach(|| std::fs::read(self.path()))
            .map(|b| PyBytes::new(py, &b))?;
        Ok(fbytes)
    }

    fn read_text(&self, py: Python<'_>) -> PyResult<PyUtf8Bytes> {
        let fbytes = py.detach(|| std::fs::read(self.path()))?;
        Ok(fbytes.into())
    }

    #[expect(clippy::needless_pass_by_value, reason = "python arg extract")]
    fn write(&self, py: Python<'_>, data: ReadableBuffer) -> PyResult<usize> {
        let b = data.as_slice();
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

    fn write_bytes(&self, py: Python<'_>, data: ReadableBuffer) -> PyResult<usize> {
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
    // Path.touch(mode=0o666, exist_ok=True)
    // Create a file at this given path. If mode is given, it is combined with the
    // process's umask value to determine the file mode and access flags. If the
    // file already exists, the function succeeds when exist_ok is true (and its
    // modification time is updated to the current time), otherwise FileExistsError
    // is raised.
    //
    // See also The open(), write_text() and write_bytes() methods are often used to
    // create files.
    #[pyo3(signature = (mode = None, exist_ok = true))]
    fn touch(&self, py: Python<'_>, mode: Option<u32>, exist_ok: bool) -> PyResult<bool> {
        if mode.is_some() {
            pytodo!("touch - mode parameter not implemented yet")
        }
        let path = self.path();
        let exists = path.exists();
        if exists {
            if exist_ok {
                Ok(false)
            } else {
                Err(PyFileExistsError::new_err(format!(
                    "{}",
                    self.path().display()
                )))
            }
        } else {
            py.detach(|| {
                std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(false)
                    .open(path)
            })
            .map_err(|e| {
                let fspath_display = self.path().display();
                PyFileNotFoundError::new_err(format!(
                    "No such file or directory: {fspath_display} ~ {e}"
                ))
            })?;
            Ok(true)
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
        if !self.exists()? {
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
        if !self.exists()? {
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

    fn rename(&self, py: Python<'_>, new_path: FsPathLike) -> PyResult<Self> {
        if new_path.as_ref() == self.path() {
            return Ok(self.clone());
        }
        let path_ref = new_path.as_ref();
        if path_ref.exists() {
            return Err(PyFileExistsError::new_err(format!(
                "rename - parent: {} - destination already exists",
                self.py_to_string()
            )));
        }
        py.detach(|| std::fs::rename(self.path(), path_ref))
            .map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("rename - parent: {fspath} - {e}"))
            })?;
        Ok(Self::from(new_path))
    }

    fn replace(&self, py: Python<'_>, new_path: FsPathLike) -> PyResult<Self> {
        if new_path.as_ref() == self.path() {
            return Ok(self.clone());
        }
        let path_ref = new_path.as_ref();
        if path_ref.exists() {
            // nuke file/dir
            if path_ref.is_dir() {
                py.detach(|| std::fs::remove_dir_all(path_ref))?;
            } else {
                py.detach(|| std::fs::remove_file(path_ref))?;
            }
        }
        py.detach(|| std::fs::rename(self.path(), path_ref))
            .map_err(|e| {
                let fspath = self.py_to_string();
                PyNotADirectoryError::new_err(format!("replace - parent: {fspath} - {e}"))
            })?;
        Ok(Self::from(new_path))
    }

    #[expect(clippy::unused_self, reason = "not implemented")]
    fn as_uri(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "as_uri not implemented",
        ))
    }

    fn iterdir(&self, py: Python<'_>) -> PyResult<PyFsPathReadDir> {
        self.read_dir(py)
    }

    #[pyo3(signature = (*args, **kwargs))]
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

    #[expect(clippy::unused_self, reason = "not implemented")]
    #[expect(unused_variables, reason = "not implemented")]
    #[expect(clippy::needless_pass_by_value, reason = "not implemented")]
    fn relative_to(&self, other: FsPathLike) -> PyResult<Self> {
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
        PyTuple::new(
            py,
            self.path()
                .components()
                .map(std::path::Component::as_os_str)
                .collect::<Vec<_>>(),
        )
    }

    fn display(&self) -> String {
        self.path().display().to_string()
    }

    fn ends_with(&self, child: FsPathLike) -> bool {
        self.path().ends_with(child)
    }

    fn exists(&self) -> PyResult<bool> {
        self.path()
            .try_exists()
            .map_err(|e| PyFileNotFoundError::new_err(format!("try_exists: {e}")))
    }

    fn extension(&self) -> Option<OsString> {
        self.path().extension().map(OsStr::to_os_string)
    }

    fn file_name(&self) -> Option<OsString> {
        self.path().file_name().map(OsStr::to_os_string)
    }

    fn file_prefix(&self) -> Option<OsString> {
        self.path().file_stem().map(OsStr::to_os_string)
    }

    fn file_stem(&self) -> Option<OsString> {
        self.path().file_stem().map(OsStr::to_os_string)
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

    fn join(&self, p: FsPathLike) -> Self {
        Self::from(self.path().join(p))
    }

    fn metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
        self.path()
            .metadata()
            .map(ryo3_std::fs::PyMetadata::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("FsPath.metadata: {e}")))
    }

    fn read_dir(&self, py: Python<'_>) -> PyResult<PyFsPathReadDir> {
        py.detach(|| std::fs::read_dir(self.path()))
            .map(PyFsPathReadDir::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("FsPath.read_dir: {e}")))
    }

    fn read_link(&self) -> PyResult<Self> {
        self.path()
            .read_link()
            .map(Self::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("FsPath.read_link: {e}")))
    }

    fn starts_with(&self, base: FsPathLike) -> bool {
        self.path().starts_with(base)
    }

    fn strip_prefix(&self, base: FsPathLike) -> PyResult<Self> {
        self.path()
            .strip_prefix(base)
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("FsPath.strip_prefix: {e}")))
    }

    fn symlink_metadata(&self) -> PyResult<ryo3_std::fs::PyMetadata> {
        self.path()
            .symlink_metadata()
            .map(ryo3_std::fs::PyMetadata::from)
            .map_err(|e| PyFileNotFoundError::new_err(format!("FsPath.symlink_metadata: {e}")))
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

    // ------------------------------------------------------------------------
    // `pydantic` feature
    // ------------------------------------------------------------------------

    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::map_py_value_err;
        Self::from_any(value).map_err(map_py_value_err)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_json_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticJsonSchemaCls;
        Self::get_pydantic_json_schema(cls, source, handler)
    }
    // -------------------------------------------------------------------------
    // `same-file` feature
    // ------------------------------------------------------------------------

    #[cfg(feature = "same-file")]
    fn samefile(&self, py: Python<'_>, other: PathBuf) -> PyResult<bool> {
        py.detach(|| same_file::is_same_file(self.path(), other))
            .map_err(|e| PyFileNotFoundError::new_err(format!("FsPath.samefile: {e}")))
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
    #[pyo3(signature = (cmd, path = None))]
    fn which(py: Python<'_>, cmd: &str, path: Option<&str>) -> PyResult<Option<Self>> {
        ryo3_which::which(py, cmd, path).map(|opt| opt.map(Self::from))
    }

    #[cfg(not(feature = "which"))]
    #[staticmethod]
    #[pyo3(signature = (_cmd, _path = None))]
    fn which(_cmd: &str, _path: Option<&str>) -> PyResult<Option<Self>> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`which` feature not enabled",
        ))
    }

    #[cfg(feature = "which")]
    #[staticmethod]
    #[pyo3(signature = (cmd, path = None))]
    fn which_all(py: Python<'_>, cmd: &str, path: Option<&str>) -> PyResult<Vec<Self>> {
        ryo3_which::which_all(py, cmd, path)
            .map(|opt| opt.into_iter().map(Self::from).collect::<Vec<_>>())
    }

    #[cfg(not(feature = "which"))]
    #[staticmethod]
    #[pyo3(signature = (_cmd, _path = None))]
    fn which_all(_cmd: &str, _path: Option<&str>) -> PyResult<Vec<Self>> {
        Err(ryo3_core::FeatureNotEnabledError::new_err(
            "`which` feature not enabled",
        ))
    }

    #[cfg(feature = "which-regex")]
    #[staticmethod]
    #[pyo3(signature = (regex, path = None))]
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
    #[pyo3(signature = (_regex, _path = None))]
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
        Self::new(p)
    }
}

#[pyclass(name = "FsPathReadDir", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyFsPathReadDir(RyMutex<std::fs::ReadDir, false>);

#[pymethods]
impl PyFsPathReadDir {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> Option<PyFsPath> {
        let value = py.detach(|| self.0.py_lock().next());
        match value {
            Some(Ok(entry)) => Some(PyFsPath::from(entry.path())),
            _ => None,
        }
    }

    fn collect(&self, py: Python<'_>) -> Vec<PyFsPath> {
        py.detach(|| {
            let mut paths = vec![];
            for entry in self.0.py_lock().by_ref() {
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
            self.0
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
        Self(iter.into())
    }
}

#[pyclass(name = "FsPathAncestors", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
struct PyFsPathAncestors {
    path: PathBuf,
    current: RyMutex<Option<PathBuf>, false>,
}

impl PyFsPathAncestors {
    fn empty() -> Self {
        Self {
            path: PathBuf::new(),
            current: RyMutex::new(None),
        }
    }

    fn parents(p: &Path) -> Self {
        if let Some(p) = p.parent() {
            Self::new(p)
        } else {
            Self::empty()
        }
    }

    fn new<P: AsRef<Path>>(p: P) -> Self {
        let buf = to_native_pathbuf(p);
        Self {
            current: RyMutex::new(Some(buf.clone())),
            path: buf,
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

        let cur = current.take()?;
        *current = match cur.parent() {
            None => None,
            Some(p) if p.as_os_str().is_empty() => None,
            Some(p) => Some(p.to_path_buf()),
        };
        let out = PyFsPath::new(&cur);

        Some(out)
    }

    fn __len__(&self) -> usize {
        self.path.ancestors().count()
    }

    fn collect(&self) -> Vec<PyFsPath> {
        let mut current = self.current.py_lock();

        let mut paths = vec![];
        while let Some(cur) = current.take() {
            *current = match cur.parent() {
                None => None,
                Some(p) if p.as_os_str().is_empty() => None,
                Some(p) => Some(p.to_path_buf()),
            };
            paths.push(PyFsPath::new(&cur));
        }
        paths
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> Vec<PyFsPath> {
        let mut current = self.current.py_lock();
        let mut paths = vec![];
        for _ in 0..n {
            let Some(cur) = current.take() else { break };
            *current = match cur.parent() {
                None => None,
                Some(p) if p.as_os_str().is_empty() => None,
                Some(p) => Some(p.to_path_buf()),
            };
            paths.push(PyFsPath::new(&cur));
        }
        paths
    }
}

impl fmt::Display for PyFsPathAncestors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FsPathAncestors<{}>", self.path.posix_display())
    }
}

struct PosixPathDisplay<'a>(&'a Path);

impl fmt::Display for PosixPathDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(target_os = "windows")]
        {
            let mut write_sep = false;

            for component in self.0.components() {
                match component {
                    Component::Prefix(prefix) => {
                        write_posix_os_str(prefix.as_os_str(), f)?;
                        write_sep = true;
                    }
                    Component::RootDir => {
                        if write_sep {
                            f.write_char('/')?;
                        } else {
                            write_posix_os_str(component.as_os_str(), f)?;
                        }
                        write_sep = false;
                    }
                    Component::CurDir | Component::ParentDir | Component::Normal(_) => {
                        if write_sep {
                            f.write_char('/')?;
                        }
                        write_posix_os_str(component.as_os_str(), f)?;
                        write_sep = true;
                    }
                }
            }
            Ok(())
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.0.display().fmt(f)
        }
    }
}

#[cfg(target_os = "windows")]
fn write_posix_os_str(s: &OsStr, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for ch in s.to_string_lossy().chars() {
        if ch == '\\' {
            f.write_char('/')?;
        } else {
            f.write_char(ch)?;
        }
    }
    Ok(())
}

trait AsPosixPath {
    fn posix_display(&self) -> PosixPathDisplay<'_>;

    fn as_posix_str(&self) -> String {
        self.posix_display().to_string()
    }
}

impl<T> AsPosixPath for T
where
    T: AsRef<Path>,
{
    fn posix_display(&self) -> PosixPathDisplay<'_> {
        PosixPathDisplay(self.as_ref())
    }
}

enum FsPathLike<'a, 'py> {
    FsPath(Borrowed<'a, 'py, PyFsPath>),
    Path(PathLike),
}

impl<'a, 'py> FromPyObject<'a, 'py> for FsPathLike<'a, 'py> {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(val) = obj.cast_exact::<PyFsPath>() {
            Ok(FsPathLike::FsPath(val))
        } else {
            obj.extract::<PathLike>().map(FsPathLike::Path)
        }
    }
}

impl AsRef<Path> for FsPathLike<'_, '_> {
    fn as_ref(&self) -> &Path {
        match self {
            FsPathLike::FsPath(p) => p.get().path(),
            FsPathLike::Path(p) => p.as_ref(),
        }
    }
}
