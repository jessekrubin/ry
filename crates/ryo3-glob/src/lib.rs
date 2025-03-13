#![doc = include_str!("../README.md")]
mod pattern;

use crate::pattern::PyPattern;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::IntoPyObjectExt;

#[pyclass]
pub struct PyGlobPaths {
    inner: ::glob::Paths,
}

#[pymethods]
impl PyGlobPaths {
    /// __iter__ just returns self
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// __next__ just pulls one item from the underlying iterator
    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<Bound<PyAny>>> {
        //     while loop to get next path from inner that isn't an error
        //     return path or if empty return None
        let py = slf.py();
        while let Some(path) = slf.inner.next() {
            match path {
                Ok(path) => {
                    let pyany = path.into_bound_py_any(py)?;
                    return Ok(Some(pyany));
                }
                Err(e) => {
                    // log error
                    continue;
                }
            }
        }
        Ok(None)
    }

    fn collect<'py>(&mut self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>> {
        self.inner
            .by_ref()
            .flatten()
            .map(|path| {
                let el = path.into_bound_py_any(py)?;
                Ok(el)
            })
            .collect::<PyResult<Vec<_>>>()
    }

    fn take<'py>(&mut self, py: Python<'py>, n: usize) -> PyResult<Vec<Bound<'py, PyAny>>> {
        self.inner
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

#[pyfunction]
#[pyo3(
    name = "glob",
    signature = (
        pattern,
        /,
        *,
        case_sensitive=true,
        require_literal_separator=false,
        require_literal_leading_dot=false
    )
)]
pub fn py_glob(
    pattern: &str,
    case_sensitive: bool,
    require_literal_separator: bool,
    require_literal_leading_dot: bool,
) -> PyResult<PyGlobPaths> {
    ::glob::glob_with(
        pattern,
        ::glob::MatchOptions {
            case_sensitive,
            require_literal_separator,
            require_literal_leading_dot,
        },
    )
    .map(|paths| PyGlobPaths { inner: paths })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPattern>()?;
    m.add_function(wrap_pyfunction!(py_glob, m)?)?;
    Ok(())
}
