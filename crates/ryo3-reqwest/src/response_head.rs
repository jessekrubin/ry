use std::net::SocketAddr;
use std::sync::Arc;

use reqwest::StatusCode;
use ryo3_cookie::PyCookie;
use ryo3_core::RyRwLock;

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

    pub(crate) fn py_set_cookies(&self) -> Option<Vec<PyCookie>> {
        let headers = self.headers.py_read();
        if headers.is_empty() {
            return None;
        }
        let py_cookies: Vec<PyCookie> = headers // nom nom nom nom nom
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .map(ToOwned::to_owned)
            .filter_map(|s| PyCookie::parse_cookie(s).ok())
            .collect();
        if py_cookies.is_empty() {
            None
        } else {
            Some(py_cookies)
        }
    }
}

impl From<&reqwest::Response> for RyResponseHead {
    fn from(res: &reqwest::Response) -> Self {
        Self::new(res)
    }
}
