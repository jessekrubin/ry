#![doc = include_str!("../README.md")]
mod charset;
mod client;
mod client_config;
mod constants;
mod errors;
mod fetch;
mod form_data;
mod proxy;
mod pyo3_json_bytes;
mod request;
mod request_body;
mod response;
mod response_head;
mod response_stream;
mod rustls_provider;
mod sse;
mod tls;
mod tls_version;
mod types;
pub use client::{RyBlockingClient, RyClient};
pub use client_config::ClientConfig;
pub use errors::RyReqwestError;
pub use proxy::RyProxy;
use pyo3::prelude::*;
pub use response::{RyBlockingResponse, RyResponse};
pub use response_stream::{RyBlockingResponseStream, RyResponseStream};
pub use sse::{RyBlockingEventStream, RyEventStream, RyServerSentEvent};
pub use tls::{RyCertificate, RyCertificateRevocationList, RyIdentity};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // setup tls provider
    rustls_provider::rustls_provider_install_default();

    m.add_class::<RyCertificate>()?;
    m.add_class::<RyCertificateRevocationList>()?;
    m.add_class::<RyIdentity>()?;
    m.add_class::<RyProxy>()?;
    m.add_class::<RyClient>()?;
    m.add_class::<RyBlockingClient>()?;
    m.add_class::<RyResponse>()?;
    m.add_class::<RyReqwestError>()?;
    m.add_function(wrap_pyfunction!(fetch::fetch, m)?)?;
    m.add_function(wrap_pyfunction!(fetch::fetch_sync, m)?)?;
    Ok(())
}
