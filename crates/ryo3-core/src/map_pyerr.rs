use std::fmt;

use pyo3::PyErr;

pub fn map_py_value_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyValueError::new_err(e.to_string())
}

pub fn map_py_overflow_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyOverflowError::new_err(e.to_string())
}

pub fn map_py_runtime_err<E>(e: E) -> PyErr
where
    E: fmt::Display,
{
    ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
}
