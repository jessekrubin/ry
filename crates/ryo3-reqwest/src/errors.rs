use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use std::fmt;

// pub(crate) fn map_reqwest_err(e: &Error) -> PyErr {
//     PyValueError::new_err(format!("{e}"))
// }
pub(crate) fn map_reqwest_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    PyErr::new::<PyValueError, _>(format!("{e}"))
}
