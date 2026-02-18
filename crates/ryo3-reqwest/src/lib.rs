#![doc = include_str!("../README.md")]
mod body;
mod charset;
mod client;
mod client_config;
mod cookie;
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
pub use client::RyBlockingClient;
pub use client::RyHttpClient;
pub use client_config::ClientConfig;
pub use cookie::PyCookie;
pub use errors::RyReqwestError;
pub use proxy::PyProxy;
use pyo3::prelude::*;
#[cfg(feature = "experimental-async")]
pub use response::RyAsyncResponse;
pub use response::RyBlockingResponse;
pub use response::RyResponse;
#[cfg(feature = "experimental-async")]
pub use response_stream::RyAsyncResponseStream;
pub use response_stream::RyBlockingResponseStream;
pub use response_stream::RyResponseStream;
pub use tls::PyCertificate;
pub use tls::PyCertificateRevocationList;
pub use tls::PyIdentity;

#[cfg(feature = "experimental-async")]
pub use client::RyClient;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // setup tls provider
    rustls_provider::rustls_provider_install_default();

    m.add_class::<PyCookie>()?;
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
