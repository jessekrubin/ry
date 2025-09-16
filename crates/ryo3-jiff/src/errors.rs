use pyo3::PyErr;

use std::fmt;

pub(crate) fn map_py_value_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))
}

pub(crate) fn map_py_overflow_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    PyErr::new::<pyo3::exceptions::PyOverflowError, _>(format!("{e}"))
}

pub(crate) fn map_py_runtime_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{e}"))
}
