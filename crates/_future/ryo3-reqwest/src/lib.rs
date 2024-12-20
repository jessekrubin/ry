#![doc = include_str!("../README.md")]
mod async_client;
mod blocking;
mod pyo3_bytes;

use crate::async_client::RyAsyncClient;
use crate::blocking::RyClient;
use pyo3::prelude::*;
use std::borrow::Borrow;

#[pyclass]
#[pyo3(name = "Client")]
#[derive(Debug, Clone)]
pub struct RysyncClient(reqwest::blocking::Client);
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyClient>()?;
    m.add_class::<RyAsyncClient>()?;
    // m.add_class::<RyClient>()?;
    // m.add_function(wrap_pyfunction!(self::which, m)?)?;
    Ok(())
}
