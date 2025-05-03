use reqwest::header::HeaderMap;
use reqwest::StatusCode;

#[derive(Debug, Clone)]
pub(crate) struct RyResponseHead {
    /// das status code
    pub status_code: StatusCode,
    /// das headers
    pub headers: HeaderMap,
    /// das url
    pub url: Option<reqwest::Url>,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    pub content_length: Option<u64>,
    /// version of http spec
    pub version: reqwest::Version,
}

impl RyResponseHead {
    /// Create a new response from a reqwest response
    pub(crate) fn new(res: &reqwest::Response) -> Self {
        Self {
            status_code: res.status(),
            headers: res.headers().clone(),
            url: Some(res.url().clone()),
            content_length: res.content_length(),
            version: res.version(),
        }
    }
}

impl From<&reqwest::Response> for RyResponseHead {
    fn from(res: &reqwest::Response) -> Self {
        Self::new(res)
    }
}
