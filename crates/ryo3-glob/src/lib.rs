#![doc = include_str!("../README.md")]
mod pattern;

use crate::pattern::PyPattern;
use parking_lot::Mutex;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::IntoPyObjectExt;
use std::sync::Arc;

#[pyclass(name = "GlobPaths", module = "ry.ryo3", frozen)]
pub struct PyGlobPaths {
    inner: Arc<Mutex<::glob::Paths>>,
    strict: bool,
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
                    let pyany = path.into_bound_py_any(py)?;
                    return Ok(Some(pyany));
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
                        let py_any = path.into_bound_py_any(py)?;
                        results.push(py_any);
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
                    let el = path.into_bound_py_any(py)?;
                    Ok(el)
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
                let el = path.into_bound_py_any(py)?;
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
                    let el = path.into_bound_py_any(py)?;
                    Ok(el)
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
        strict=true
    )
)]
pub fn py_glob(
    pattern: &str,
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
    strict: bool,
) -> PyResult<PyGlobPaths> {
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
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPattern>()?;
    m.add_function(wrap_pyfunction!(py_glob, m)?)?;
    Ok(())
}
