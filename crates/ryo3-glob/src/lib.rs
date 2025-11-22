#![doc = include_str!("../README.md")]
mod pattern;

use crate::pattern::PyPattern;
use parking_lot::Mutex;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyModule, PyType};
use ryo3_macro_rules::py_value_err;
use ryo3_macro_rules::py_value_error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Copy)]
enum GlobDType {
    FsPath,
    PathBuf,
    OsString,
}

struct GlobPathsVec {
    dtype: GlobDType,
    paths: Vec<PathBuf>,
}

impl From<(GlobDType, Vec<PathBuf>)> for GlobPathsVec {
    fn from(value: (GlobDType, Vec<PathBuf>)) -> Self {
        Self {
            dtype: value.0,
            paths: value.1,
        }
    }
}

impl<'py> IntoPyObject<'py> for GlobPathsVec {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.dtype {
            GlobDType::FsPath => self
                .paths
                .into_iter()
                .map(ryo3_fspath::PyFsPath::from)
                .collect::<Vec<ryo3_fspath::PyFsPath>>()
                .into_pyobject(py),
            GlobDType::PathBuf => self.paths.into_pyobject(py),
            GlobDType::OsString => self
                .paths
                .into_iter()
                .map(OsString::from)
                .collect::<Vec<OsString>>()
                .into_pyobject(py),
        }
    }
}

impl GlobDType {
    fn dtype_into_bound_py_any(self, py: Python<'_>, path: PathBuf) -> PyResult<Bound<'_, PyAny>> {
        match self {
            Self::FsPath => {
                let fspath = ryo3_fspath::PyFsPath::from(path);
                let any = fspath.into_bound_py_any(py)?;
                Ok(any)
            }
            Self::PathBuf => {
                let any = path.into_bound_py_any(py)?;
                Ok(any)
            }
            Self::OsString => {
                let os_string = path.into_os_string();
                let any = os_string.into_bound_py_any(py)?;
                Ok(any)
            }
        }
    }
}

#[pyclass(name = "GlobPaths", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyGlobPaths {
    inner: Arc<Mutex<::glob::Paths>>,
    strict: bool,
    dtype: GlobDType,
}

impl PyGlobPaths {
    /// Pull exactly one item -- fix `clippy::significant-drop-in-scrutinee`
    #[inline]
    fn next_path(&self) -> Option<Result<PathBuf, glob::GlobError>> {
        self.inner.lock().next()
    }
}

#[pymethods]
impl PyGlobPaths {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        loop {
            match self.next_path() {
                Some(Ok(path)) => {
                    let pyany = self.dtype.dtype_into_bound_py_any(py, path)?;
                    return Ok(Some(pyany));
                }
                Some(Err(e)) => {
                    if self.strict {
                        return py_value_err!("{e}");
                    }
                }
                None => return Ok(None),
            }
        }
    }

    fn collect(&self, py: Python<'_>) -> PyResult<GlobPathsVec> {
        let paths: Vec<PathBuf> = py
            .detach(|| {
                if self.strict {
                    let mut results = Vec::new();
                    for path in self.inner.lock().by_ref() {
                        match path {
                            Ok(path) => {
                                results.push(path);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Ok(results)
                } else {
                    let a = self.inner.lock().by_ref().flatten().collect::<Vec<_>>();
                    Ok(a)
                }
            })
            .map_err(|e| py_value_error!("{e}"))?;
        Ok(GlobPathsVec::from((self.dtype, paths)))
    }

    /// Take `n` items from the iterator or 1 if `n` is not specified.
    #[pyo3(signature = (n=1))]
    fn take(&self, py: Python<'_>, n: usize) -> PyResult<GlobPathsVec> {
        let paths: Vec<PathBuf> = py
            .detach(|| {
                if self.strict {
                    let mut results = Vec::new();
                    for path_result in self.inner.lock().by_ref().take(n) {
                        match path_result {
                            Ok(path) => {
                                results.push(path);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Ok(results)
                } else {
                    let pathbufs = self
                        .inner
                        .lock()
                        .by_ref()
                        .flatten()
                        .take(n)
                        .collect::<Vec<_>>();
                    Ok(pathbufs)
                }
            })
            .map_err(|e| py_value_error!("{e}"))?;
        Ok(GlobPathsVec::from((self.dtype, paths)))
    }
}

/// Return iterator of paths that match the given pattern
///
/// Pattern syntax (as taken from the `glob` crate):
///
/// A compiled Unix shell style pattern.
///
/// - `?` matches any single character.
///
/// - `*` matches any (possibly empty) sequence of characters.
///
/// - `**` matches the current directory and arbitrary
///   subdirectories. To match files in arbitrary subdirectories, use
///   `**/*`.
///
///   This sequence **must** form a single path component, so both
///   `**a` and `b**` are invalid and will result in an error.  A
///   sequence of more than two consecutive `*` characters is also
///   invalid.
///
/// - `[...]` matches any character inside the brackets.  Character sequences
///   can also specify ranges of characters, as ordered by Unicode, so e.g.
///   `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed
///   bracket is invalid.
///
/// - `[!...]` is the negation of `[...]`, i.e. it matches any characters
///   **not** in the brackets.
///
/// - The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets
///   (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it
///   is interpreted as being part of, rather then ending, the character set, so
///   `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`
///   character can be specified inside a character sequence pattern by placing
///   it at the start or the end, e.g. `[abc-]`.
#[expect(clippy::fn_params_excessive_bools)]
#[pyfunction]
#[pyo3(
    name = "glob",
    signature = (
        pattern,
        *,
        case_sensitive=true,
        require_literal_separator=false,
        require_literal_leading_dot=false,
        strict=true,
        dtype=None,
    )
)]
pub fn py_glob(
    pattern: &str,
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
    strict: bool,
    dtype: Option<Bound<'_, PyType>>,
) -> PyResult<PyGlobPaths> {
    let dtype = extract_dtype(dtype)?;
    ::glob::glob_with(
        pattern,
        ::glob::MatchOptions {
            case_sensitive,
            require_literal_separator,
            require_literal_leading_dot,
        },
    )
    .map(|paths| PyGlobPaths {
        inner: Arc::new(Mutex::new(paths)),
        strict,
        dtype,
    })
    .map_err(|e| py_value_error!("{e}"))
}

fn pathlib_path_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static PATHLIB_PATH_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    PATHLIB_PATH_TYPE.import(py, "pathlib", "Path")
}

fn str_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static STR_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    STR_TYPE.import(py, "builtins", "str")
}

fn ry_fspath_type(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static FSPATH_TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    FSPATH_TYPE.import(py, "ry.ryo3", "FsPath")
}

fn extract_dtype(dtype: Option<Bound<'_, PyType>>) -> PyResult<GlobDType> {
    if let Some(dtype) = dtype {
        let py = dtype.py();
        if dtype.is(str_type(py)?) {
            Ok(GlobDType::OsString)
        } else if dtype.is(pathlib_path_type(py)?) {
            Ok(GlobDType::PathBuf)
        } else if dtype.is(ry_fspath_type(py)?) {
            Ok(GlobDType::FsPath)
        } else {
            let repr = dtype.repr()?.to_string_lossy().into_owned();
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid dtype: {repr} (only `str`, `pathlib.Path` or `ry.ryo3.FsPath` are supported)"
            )))
        }
    } else {
        // default to PathBuf when no dtype is provided
        Ok(GlobDType::PathBuf)
    }
}
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPattern>()?;
    m.add_function(wrap_pyfunction!(py_glob, m)?)?;
    Ok(())
}
