#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{wrap_pyfunction, PyResult};

use ::jiter::{map_json_error, PythonParse};
use bytes::Bytes;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::{Bound, PyResult};
use reqwest::StatusCode;
use std::borrow::Borrow;
#[pyclass]
#[pyo3(name = "AsyncClient")]
#[derive(Debug, Clone)]
pub struct RyAsyncClient(reqwest::Client);

#[pyclass]
#[pyo3(name = "Client")]
#[derive(Debug, Clone)]
pub struct RysyncClient(reqwest::blocking::Client);
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyClient>()?;
    // m.add_class::<RyClient>()?;
    // m.add_function(wrap_pyfunction!(self::which, m)?)?;
    Ok(())
}
