#![doc = include_str!("../README.md")]
mod async_client;
mod blocking;
mod errors;
mod fetch;
mod pyo3_bytes;

use crate::async_client::RyAsyncClient;
use crate::blocking::RyClient;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyClient>()?;
    m.add_class::<RyAsyncClient>()?;
    Ok(())
}
