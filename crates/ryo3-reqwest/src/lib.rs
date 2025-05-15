#![doc = include_str!("../README.md")]

#[cfg(feature = "blocking")]
pub mod blocking;
mod client;
mod default_client;
mod errors;
mod fetch;
mod form_data;
mod pyo3_json_bytes;
mod query_like;
mod response_data;
mod response_parking_lot;
mod response_stream;
mod response_tokio_mutex;
mod user_agent;

pub use client::RyHttpClient;
pub use errors::RyReqwestError;
use pyo3::prelude::*;
// pub use response_tokio_mutex::RyResponse;
pub use response_parking_lot::RyResponse;
pub use response_stream::RyResponseStream;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RyHttpClient>()?;
    m.add_class::<RyReqwestError>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    Ok(())
}
