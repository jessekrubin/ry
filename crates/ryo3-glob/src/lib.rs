#![doc = include_str!("../README.md")]
mod pattern;

use crate::pattern::PyPattern;
use parking_lot::Mutex;
use pyo3::types::{PyModule, PyString, PyType, PyTypeMethods};
use pyo3::IntoPyObjectExt;
use pyo3::{prelude::*, PyTypeInfo};
use std::path::PathBuf;
use std::sync::Arc;

enum GlobDType {
    FsPath,
    PathBuf,
    OsString,
}

impl GlobDType {
    fn into_bound_py_any<'py>(
        &self,
        py: Python<'py>,
        path: PathBuf,
    ) -> PyResult<Bound<'py, PyAny>> {
        match self {
            GlobDType::FsPath => {
                let fspath = ryo3_fspath::PyFsPath::from(path);
                let any = fspath.into_bound_py_any(py)?;
                Ok(any)
            }
            GlobDType::PathBuf => {
                let any = path.into_bound_py_any(py)?;
                Ok(any)
            }
            GlobDType::OsString => {
                let os_string = path.into_os_string();
                // let py_str = PyString::new(py, os_string.to_str().unwrap_or_default());
                let any = os_string.into_bound_py_any(py)?;
                Ok(any)
            }
        }
    }
}

#[pyclass(name = "GlobPaths", module = "ry.ryo3", frozen)]
pub struct PyGlobPaths {
    inner: Arc<Mutex<::glob::Paths>>,
    strict: bool,
    dtype: GlobDType,
}

#[pymethods]
impl PyGlobPaths {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        while let Some(path) = self.inner.lock().next() {
            match path {
                Ok(path) => {
                    let pyany = self.dtype.into_bound_py_any(py, path)?;
                    return Ok(Some(pyany));

                    //  path.into_bound_py_any(py)?;
                    // return Ok(Some(pyany));
                }
                Err(e) => {
                    if self.strict {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                            "{e}"
                        )));
                    }
                }
            }
        }
        Ok(None)
    }

    fn collect<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>> {
        if self.strict {
            let mut results = Vec::new();
            for path in self.inner.lock().by_ref() {
                match path {
                    Ok(path) => {
                        let any = self.dtype.into_bound_py_any(py, path)?;
                        results.push(any);
                    }
                    Err(e) => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                            "{e}"
                        )));
                    }
                }
            }
            Ok(results)
        } else {
            self.inner
                .lock()
                .by_ref()
                .flatten()
                .map(|path| {
                    let py_any = self.dtype.into_bound_py_any(py, path)?;
                    Ok(py_any)
                })
                .collect::<PyResult<Vec<_>>>()
        }
    }

    fn take<'py>(&self, py: Python<'py>, n: usize) -> PyResult<Vec<Bound<'py, PyAny>>> {
        if self.strict {
            let mut results = Vec::new();

            for path_result in self.inner.lock().by_ref().take(n) {
                let path = path_result
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
                let el = self.dtype.into_bound_py_any(py, path)?;
                results.push(el);
            }

            Ok(results)
        } else {
            self.inner
                .lock()
                .by_ref()
                .take(n)
                .flatten()
                .map(|path| {
                    let py_any = self.dtype.into_bound_py_any(py, path)?;
                    Ok(py_any)
                })
                .collect::<PyResult<Vec<_>>>()
        }
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
///   subdirectories. To match files in arbitrary subdiretories, use
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
        dtype=None,
        strict=true
    )
)]
pub fn py_glob<'py>(
    py: Python<'py>,
    pattern: &str,
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
    dtype: Option<Bound<'_, PyType>>,
    strict: bool,
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
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
}

fn extract_dtype<'py>(dtype: Option<Bound<'py, PyType>>) -> PyResult<GlobDType> {
    match dtype {
        Some(dtype) => {
            let fully_qualified_name_pystr = dtype.fully_qualified_name()?;
            let fully_qualified_name = fully_qualified_name_pystr.to_string();
            if fully_qualified_name == "ry.ryo3.FsPath" {
                return Ok(GlobDType::FsPath);
            } else if fully_qualified_name == "pathlib.Path" {
                return Ok(GlobDType::PathBuf);
            } else if fully_qualified_name == "str" {
                return Ok(GlobDType::OsString);
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid dtype: {fully_qualified_name} not supported"
                )));
            }
        }
        None => Ok(GlobDType::OsString),
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPattern>()?;
    m.add_function(wrap_pyfunction!(py_glob, m)?)?;
    Ok(())
}
