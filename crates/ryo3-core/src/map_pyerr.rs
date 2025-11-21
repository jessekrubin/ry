use pyo3::PyErr;

use std::fmt;

pub fn map_py_value_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))
}
