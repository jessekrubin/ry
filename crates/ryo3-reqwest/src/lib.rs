#![doc = include_str!("../README.md")]
mod body;
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
mod rustls_provider;
mod tls_version;
mod user_agent;

pub use cert::PyCertificate;
pub use client::RyBlockingClient;
pub use client::RyHttpClient;
pub use cookie::PyCookie;
pub use errors::RyReqwestError;
use pyo3::prelude::*;
pub use response_parking_lot::RyBlockingResponse;
pub use response_parking_lot::RyResponse;
pub use response_stream::RyBlockingResponseStream;
pub use response_stream::RyResponseStream;

#[cfg(feature = "experimental-async")]
pub use client::RyClient;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // setup tls provider
    rustls_provider::rustls_provider_install_default();

    m.add_class::<PyCookie>()?;
    m.add_class::<PyCertificate>()?;
    m.add_class::<RyHttpClient>()?;
    #[cfg(feature = "experimental-async")]
    m.add_class::<RyClient>()?;
    m.add_class::<RyBlockingClient>()?;
    m.add_class::<RyResponse>()?;
    m.add_class::<RyReqwestError>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    m.add_function(wrap_pyfunction!(fetch::fetch_sync, m)?)?;
    Ok(())
}
