//! TODO: implement this wrapper!

mod py_regex;
mod dev_sandbox;
mod py_captures;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<py_regex::PyRegex>()?;
    Ok(())
}
// //! TODO: implement this wrapper!
// use pyo3::prelude::*;
// use pyo3::types::PyModule;
// use pyo3::PyResult;
//
// #[pyfunction]
// fn compile(_pattern: &str) -> PyResult<PyRegex> {
//     Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
//         "Not implemented",
//     ))
// }
//
// pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<PyRegex>()?;
//     m.add_function(wrap_pyfunction!(compile, m)?)?;
//     Ok(())
// }
