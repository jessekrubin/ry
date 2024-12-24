#![doc = include_str!("../README.md")]
mod async_client;
pub mod blocking;
mod default_client;
mod errors;
mod fetch;
mod pyo3_bytes;

pub use crate::async_client::RyAsyncClient;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyAsyncClient>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    Ok(())
}
