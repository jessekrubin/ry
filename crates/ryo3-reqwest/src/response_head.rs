// use parking_lot::RwLock;
use reqwest::StatusCode;
use ryo3_core::RyRwLock;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct RyResponseHead {
    /// das status code
    pub(crate) status: StatusCode,
    /// das headers
    pub(crate) headers: Arc<RyRwLock<reqwest::header::HeaderMap, false>>,
    /// das url
    pub(crate) url: reqwest::Url,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    pub(crate) content_length: Option<u64>,
    /// version of http spec
    pub(crate) version: reqwest::Version,
    /// Remote address
    pub(crate) remote_addr: Option<SocketAddr>,
}

impl RyResponseHead {
    /// Create a new response from a reqwest response
    pub(crate) fn new(res: &reqwest::Response) -> Self {
        Self {
            status: res.status(),
            headers: Arc::new(RyRwLock::new(res.headers().clone())),
            url: res.url().clone(),
            content_length: res.content_length(),
            version: res.version(),
            remote_addr: res.remote_addr(),
        }
    }
}

impl From<&reqwest::Response> for RyResponseHead {
    fn from(res: &reqwest::Response) -> Self {
        Self::new(res)
    }
}
