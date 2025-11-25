#![doc = include_str!("../README.md")]
mod cert;
mod client;
mod cookie;
mod errors;
mod fetch;
mod form_data;
mod pyo3_json_bytes;
mod response_head;
mod response_parking_lot;
mod response_stream;
mod tls_version;
mod user_agent;

pub use client::{RyBlockingClient, RyHttpClient};
pub use cookie::PyCookie;
pub use errors::RyReqwestError;
use pyo3::prelude::*;
pub use response_parking_lot::{RyBlockingResponse, RyResponse};
pub use response_stream::RyResponseStream;

use crate::cert::PyCertificate;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCookie>()?;
    m.add_class::<PyCertificate>()?;
    m.add_class::<RyHttpClient>()?;
    m.add_class::<RyBlockingClient>()?;
    m.add_class::<RyResponse>()?;
    m.add_class::<RyReqwestError>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    Ok(())
}
