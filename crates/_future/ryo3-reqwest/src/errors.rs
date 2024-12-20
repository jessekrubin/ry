use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use reqwest::Error;

pub fn map_reqwest_err(e: Error) -> PyErr {
    PyValueError::new_err(format!("{e}"))
}
