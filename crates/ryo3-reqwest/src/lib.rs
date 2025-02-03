#![doc = include_str!("../README.md")]
pub mod blocking;
mod client;
mod default_client;
mod errors;
mod fetch;
mod pyo3_json_bytes;
pub use crate::client::RyHttpClient;
use crate::errors::RyReqwestError;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyHttpClient>()?;
    m.add_class::<RyReqwestError>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    Ok(())
}
