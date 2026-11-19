use std::net::SocketAddr;
use std::sync::Arc;

use reqwest::StatusCode;
use reqwest::header::CONTENT_TYPE;
use ryo3_cookie::RyCookie;
use ryo3_core::sync::RyRwLock;
use ryo3_url::RyUrl;

use crate::charset::PyEncoding;
use crate::tls::RyCertificate;
use crate::tls_version::PyTlsVersion;

/// undefined content length sentinel value
///
/// `u64::MAX` would be a ridiculous body size
const UNDEFINED_CONTENT_LENGTH: u64 = u64::MAX;

#[derive(Debug, Clone)]
pub(crate) struct RyResponseHead {
    /// das status code
    pub(crate) status: StatusCode,
    /// das headers
    pub(crate) headers: Arc<RyRwLock<http::HeaderMap, false>>,
    /// das url
    pub(crate) url: Box<reqwest::Url>,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    ///
    /// NOTE: stored as `u64` ~ `u64::MAX` == undefined content length
    pub(crate) content_length: u64,
    /// version of http spec
    pub(crate) version: reqwest::Version,
    /// Remote address
    pub(crate) remote_addr: Option<SocketAddr>,
    /// tls info ~ only there if client was built w/ `tls_info=True` + https
    pub(crate) tls_info: Option<Box<RyTlsInfo>>,
}

#[derive(Debug, Clone)]
pub(crate) struct RyTlsInfo {
    /// DER encoded leaf cert of the peer
    pub(crate) peer_certificate: Option<bytes::Bytes>,
    pub(crate) version: Option<PyTlsVersion>,
}

impl From<&reqwest::tls::TlsInfo> for RyTlsInfo {
    fn from(info: &reqwest::tls::TlsInfo) -> Self {
        Self {
            peer_certificate: info.peer_certificate().map(bytes::Bytes::copy_from_slice),
            version: info.version().and_then(PyTlsVersion::from_reqwest),
        }
    }
}

impl RyResponseHead {
    #[inline]
    pub(crate) fn py_content_length(&self) -> Option<u64> {
        (self.content_length != UNDEFINED_CONTENT_LENGTH).then_some(self.content_length)
    }

    pub(crate) fn py_peer_certificate(&self) -> pyo3::PyResult<Option<RyCertificate>> {
        self.tls_info
            .as_ref()
            .and_then(|i| i.peer_certificate.as_deref())
            .map(RyCertificate::from_der)
            .transpose()
    }

    pub(crate) fn ry_url(&self) -> RyUrl {
        RyUrl::new((*self.url).clone())
    }

    /// 4xx/5xx -> `reqwest::Error` (only `reqwest::Response` can make one)
    pub(crate) fn error_for_status(&self) -> Result<(), reqwest::Error> {
        if !self.status.is_client_error() && !self.status.is_server_error() {
            return Ok(());
        }
        let mut res = http::Response::new(reqwest::Body::default());
        *res.status_mut() = self.status;
        reqwest::Response::from(res)
            .error_for_status()
            .map(drop)
            .map_err(|e| e.with_url((*self.url).clone()))
    }

    pub(crate) fn from_parts(
        status: StatusCode,
        headers: http::HeaderMap,
        url: reqwest::Url,
        content_length: Option<u64>,
        version: reqwest::Version,
        remote_addr: Option<SocketAddr>,
        tls_info: Option<RyTlsInfo>,
    ) -> Self {
        Self {
            status,
            headers: Arc::new(RyRwLock::new(headers)),
            url: Box::new(url),
            content_length: content_length.unwrap_or(UNDEFINED_CONTENT_LENGTH),
            version,
            remote_addr,
            tls_info: tls_info.map(Box::new),
        }
    }

    /// text encoding from the `content-type` charset, else `default`
    pub(crate) fn encoding(&self, default: PyEncoding) -> PyEncoding {
        let content_type = self
            .headers
            .py_read()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok());
        let encoding_name = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or(default.as_static_str());
        encoding_rs::Encoding::for_label(encoding_name.as_bytes())
            .map_or(default, PyEncoding::from_encoding)
    }

    /// `content-coding = token` (RFC 9110-8.4.1) so valid values are ascii
    ///
    /// Non-ASCII values are decoded as latin-1 so it round-trips instead of
    /// failing (which matches the impl of python's stdlib and other http
    /// libraries)
    ///
    /// REF: <https://www.rfc-editor.org/rfc/rfc9110#section-8.4.1>
    pub(crate) fn content_encoding(&self) -> Option<String> {
        let headers = self.headers.py_read();
        let hv = headers.get(reqwest::header::CONTENT_ENCODING)?;
        let s = match hv.to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => hv.as_bytes().iter().map(|&b| char::from(b)).collect(),
        };
        Some(s)
    }

    pub(crate) fn py_set_cookies(&self) -> Option<Vec<RyCookie>> {
        let headers = self.headers.py_read();
        if headers.is_empty() {
            return None;
        }
        let py_cookies: Vec<RyCookie> = headers // nom nom nom nom nom
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .map(ToOwned::to_owned)
            .filter_map(|s| RyCookie::parse_cookie(s).ok())
            .collect();
        if py_cookies.is_empty() {
            None
        } else {
            Some(py_cookies)
        }
    }
}
