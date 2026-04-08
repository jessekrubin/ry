#![doc = include_str!("../README.md")]
mod body;
mod charset;
mod client;
mod client_config;
// mod cookie;
mod errors;
mod fetch;
mod form_data;
mod proxy;
mod pyo3_json_bytes;
mod request;
mod response;
mod response_head;
mod response_stream;
mod rustls_provider;
mod tls;
mod tls_version;
mod types;
mod user_agent;
#[cfg(feature = "experimental-async")]
pub use client::RyClient;
pub use client::{RyBlockingClient, RyHttpClient};
pub use client_config::ClientConfig;
pub use errors::RyReqwestError;
pub use proxy::PyProxy;
use pyo3::prelude::*;
#[cfg(feature = "experimental-async")]
pub use response::RyAsyncResponse;
pub use response::{RyBlockingResponse, RyResponse};
#[cfg(feature = "experimental-async")]
pub use response_stream::RyAsyncResponseStream;
pub use response_stream::{RyBlockingResponseStream, RyResponseStream};
pub use tls::{PyCertificate, PyCertificateRevocationList, PyIdentity};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // setup tls provider
    rustls_provider::rustls_provider_install_default();

    // m.add_class::<PyCookie>()?;
    m.add_class::<PyCertificate>()?;
    m.add_class::<PyCertificateRevocationList>()?;
    m.add_class::<PyIdentity>()?;
    m.add_class::<PyProxy>()?;
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
