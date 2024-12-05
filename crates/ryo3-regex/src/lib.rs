//! TODO: implement this wrapper!
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub fn pymod_add(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
// //! TODO: implement this wrapper!
// use pyo3::prelude::*;
// use pyo3::types::PyModule;
// use pyo3::PyResult;
//
// #[pyclass(name = "Regex", frozen, module = "ryo3")]
// #[derive(Clone, Debug)]
// struct PyRegex(regex::Regex);
//
// #[pymethods]
// impl PyRegex {
//     #[new]
//     fn new(_pattern: &str) -> PyResult<Self> {
//         Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
//             "Not implemented",
//         ))
//     }
// }
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
