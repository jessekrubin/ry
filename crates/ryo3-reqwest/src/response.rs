use bytes::{Bytes, BytesMut};
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_bytes::RyBytes;
use ryo3_cookie::PyCookie;
use ryo3_core::sync::RyMutex;
use ryo3_http::{PyHeaders, PyHttpStatus, PyHttpVersion, status_code_pystring};
use ryo3_macro_rules::pytodo;
use ryo3_std::net::PySocketAddr;
use ryo3_tokio_rt::{RyCoroutine, done_coro, failed_coro, get_tokio_runtime, tokio_coro};
use ryo3_url::PyUrl;

use crate::charset::PyEncoding;
use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use crate::response_head::RyResponseHead;
use crate::response_stream::RyBlockingResponseStream;
use crate::{RyResponseStream, pyerr_response_already_consumed};

#[pyclass(name = "Response", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyResponse {
    /// The response body, consumed only once
    body: RyMutex<ResponseBody>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

#[pyclass(name = "BlockingResponse", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyBlockingResponse {
    /// The response body, consumed only once
    body: RyMutex<ResponseBody>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

#[derive(Debug)]
enum ResponseBody {
    Stream(reqwest::Body),
    Buffered(bytes::Bytes),
    Consumed,
}

enum OwnedBody {
    Stream(reqwest::Body),
    Buffered(bytes::Bytes),
}

/// bodies w/ a known length up to this get read along w/ the head
const PREFETCH_MAX_LEN: u64 = 64 * 1024;

/// chunks read so far ~ one chunk (the usual) is kept as-is, more get copied
/// into one buf sized off the content-length
#[derive(Default)]
enum Prefetched {
    #[default]
    Empty,
    One(Bytes),
    Many(BytesMut),
}

impl Prefetched {
    #[inline]
    fn push(&mut self, data: Bytes, content_length: u64) {
        match self {
            Self::Empty => *self = Self::One(data),
            Self::One(first) => {
                let total = first.len() + data.len();
                let cap = usize::try_from(content_length).map_or(total, |len| len.max(total));
                let mut buf = BytesMut::with_capacity(cap);
                buf.extend_from_slice(first);
                buf.extend_from_slice(&data);
                *self = Self::Many(buf);
            }
            Self::Many(buf) => buf.extend_from_slice(&data),
        }
    }

    #[inline]
    fn into_bytes(self) -> Option<Bytes> {
        match self {
            Self::Empty => None,
            Self::One(bytes) => Some(bytes),
            Self::Many(buf) => Some(buf.freeze()),
        }
    }
}

async fn prefetch_body(mut body: reqwest::Body, content_length: u64) -> ResponseBody {
    use futures_util::FutureExt;
    use futures_util::stream::{self, StreamExt};
    use http_body_util::{BodyDataStream, BodyExt};

    let mut chunks = Prefetched::default();
    // yield once now so frames that are about to land can catch up ~ no net wait
    let mut yielded = false;
    loop {
        match body.frame().now_or_never() {
            Some(Some(Ok(frame))) => {
                if let Ok(data) = frame.into_data() {
                    chunks.push(data, content_length);
                }
            }
            // whole body is here
            Some(None) => return ResponseBody::Buffered(chunks.into_bytes().unwrap_or_default()),
            // the err still gets raised when the body is read
            Some(Some(Err(e))) => {
                let frames = chunks.into_bytes().map(Ok).into_iter().chain([Err(e)]);
                return ResponseBody::Stream(reqwest::Body::wrap_stream(stream::iter(frames)));
            }
            None if !yielded => {
                yielded = true;
                tokio::task::yield_now().await;
            }
            // slow body ~ stream it w/ whatever already arrived right away
            None => {
                let Some(prefix) = chunks.into_bytes() else {
                    return ResponseBody::Stream(body);
                };
                return ResponseBody::Stream(reqwest::Body::wrap_stream(
                    stream::once(async { Ok(prefix) }).chain(BodyDataStream::new(body)),
                ));
            }
        }
    }
}

fn split_response(res: reqwest::Response) -> (RyResponseHead, reqwest::Body) {
    let url = res.url().clone();
    let content_length = res.content_length();
    let remote_addr = res.remote_addr();
    let res: http::Response<reqwest::Body> = res.into();
    let (parts, body) = res.into_parts();
    let head = RyResponseHead::from_parts(
        parts.status,
        parts.headers,
        url,
        content_length,
        parts.version,
        remote_addr,
    );
    (head, body)
}

/// head + (maybe prefetched) body ~ small known-length bodies that are already
/// here get read w/ the head
async fn split_prefetched(res: reqwest::Response) -> (RyResponseHead, ResponseBody) {
    let prefetch_len = res
        .content_length()
        .filter(|&length| length <= PREFETCH_MAX_LEN);
    let (head, body) = split_response(res);
    let body = match prefetch_len {
        Some(length) => prefetch_body(body, length).await,
        None => ResponseBody::Stream(body),
    };
    (head, body)
}

fn take_owned(body: &RyMutex<ResponseBody>) -> PyResult<OwnedBody> {
    match std::mem::replace(&mut *body.py_lock()?, ResponseBody::Consumed) {
        ResponseBody::Stream(body) => Ok(OwnedBody::Stream(body)),
        ResponseBody::Buffered(bytes) => Ok(OwnedBody::Buffered(bytes)),
        ResponseBody::Consumed => Err(pyerr_response_already_consumed!()),
    }
}

impl RyResponse {
    pub(crate) async fn from_response(res: reqwest::Response) -> Self {
        let (head, body) = split_prefetched(res).await;
        Self {
            head,
            body: RyMutex::new(body),
        }
    }

    fn take_owned(&self) -> PyResult<OwnedBody> {
        take_owned(&self.body)
    }

    fn stream_from(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        Ok(match self.take_owned()? {
            OwnedBody::Stream(body) => {
                RyResponseStream::from_body(self.head.status, body, min_read_size)
            }
            OwnedBody::Buffered(bytes) => RyResponseStream::from_bytes(self.head.status, bytes),
        })
    }

    fn text_coro<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        encoding: PyEncoding,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let encoding_name = self.head.encoding(encoding);
        match self.take_owned() {
            Ok(OwnedBody::Buffered(bytes)) => {
                done_coro(py, qualname, BodyText(bytes, encoding_name))
            }
            Ok(OwnedBody::Stream(body)) => tokio_coro(py, qualname, async move {
                read_body_bytes(body)
                    .await
                    .map(|bytes| BodyText(bytes, encoding_name))
                    .map_err(map_reqwest_err)
            }),
            Err(e) => failed_coro(py, qualname, e),
        }
    }
}

impl RyBlockingResponse {
    pub(crate) async fn from_response(res: reqwest::Response) -> Self {
        let (head, body) = split_prefetched(res).await;
        Self {
            head,
            body: RyMutex::new(body),
        }
    }

    fn take_owned(&self) -> PyResult<OwnedBody> {
        take_owned(&self.body)
    }

    /// whole body ~ only lets go of the gil + blocks if it isnt buffered
    fn body_bytes(&self, py: Python<'_>) -> PyResult<Bytes> {
        match self.take_owned()? {
            OwnedBody::Buffered(bytes) => Ok(bytes),
            OwnedBody::Stream(body) => py
                .detach(|| get_tokio_runtime().block_on(read_body_bytes(body)))
                .map_err(map_reqwest_err),
        }
    }
}

#[pymethods]
impl RyResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        pytodo!("Response::new");
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[getter]
    fn status(&self) -> u16 {
        self.head.status.as_u16()
    }

    #[getter]
    fn status_text<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.head.status.as_u16())
    }

    #[getter]
    fn status_code(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        PyHttpStatus::from_status_code_cached(py, self.head.status)
    }

    /// Returns true if the response was redirected
    #[getter]
    fn redirected(&self) -> bool {
        self.head.status.is_redirection()
    }

    #[getter]
    fn version(&self) -> PyHttpVersion {
        PyHttpVersion::from(self.head.version)
    }

    #[getter]
    fn http_version(&self) -> PyHttpVersion {
        PyHttpVersion::from(self.head.version)
    }

    #[getter]
    #[pyo3(name = "url")]
    fn url(&self) -> PyUrl {
        self.head.py_url()
    }

    #[getter]
    fn headers(&self) -> PyHeaders {
        let c = self.head.headers.clone();
        PyHeaders::from(c)
    }

    /// Return the content length of the response, if it is known or `None`.
    #[getter]
    fn content_length(&self) -> Option<u64> {
        self.head.content_length
    }

    #[getter]
    fn remote_addr(&self) -> Option<PySocketAddr> {
        self.head.remote_addr.map(PySocketAddr::from)
    }

    /// Return true if the status code is a success code (200-299)
    #[getter]
    fn ok(&self) -> bool {
        self.head.status.is_success()
    }

    /// __bool__ dunder method returns true if `ok` is true
    fn __bool__(&self) -> bool {
        self.head.status.is_success()
    }

    /// Return true if the body has been consumed
    ///
    /// named after jawascript fetch
    #[getter]
    fn body_used(&self) -> PyResult<bool> {
        self.body
            .py_lock()
            .map(|state| matches!(*state, ResponseBody::Consumed))
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, RyCoroutine>> {
        const QUALNAME: &str = "Response.bytes";
        match self.take_owned() {
            Ok(OwnedBody::Buffered(bytes)) => done_coro(py, QUALNAME, RyBytes::from(bytes)),
            Ok(OwnedBody::Stream(body)) => tokio_coro(py, QUALNAME, async move {
                read_body_bytes(body)
                    .await
                    .map(RyBytes::from)
                    .map_err(map_reqwest_err)
            }),
            Err(e) => failed_coro(py, QUALNAME, e),
        }
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding = PyEncoding::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text<'py>(
        &self,
        py: Python<'py>,
        encoding: PyEncoding,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.text_coro(py, "Response.text", encoding)
    }

    /// Return the response body as text with encoding (consumes the response)
    fn text_with_charset<'py>(
        &self,
        py: Python<'py>,
        encoding: PyEncoding,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.text_coro(py, "Response.text_with_charset", encoding)
    }

    /// Return the response body as json (consumes the response)
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = jiter::StringCacheMode::All,
            partial_mode = jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        ),
        text_signature = "(self, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
    )]
    fn json<'py>(
        &self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        const QUALNAME: &str = "Response.json";
        let options = ryo3_jiter::JiterParseOptions::new()
            .with_allow_inf_nan(allow_inf_nan)
            .with_cache_mode(cache_mode)
            .with_partial_mode(partial_mode)
            .with_catch_duplicate_keys(catch_duplicate_keys);
        match self.take_owned() {
            // a parse err gets stashed + raised on await
            Ok(OwnedBody::Buffered(bytes)) => {
                done_coro(py, QUALNAME, Pyo3JsonBytes::from((bytes, options)))
            }
            Ok(OwnedBody::Stream(body)) => tokio_coro(py, QUALNAME, async move {
                read_body_bytes(body)
                    .await
                    .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
                    .map_err(map_reqwest_err)
            }),
            Err(e) => failed_coro(py, QUALNAME, e),
        }
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        self.stream_from(min_read_size)
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        self.stream_from(min_read_size)
    }

    /// Return the `content-encoding` header value of the response or `None`
    #[getter]
    fn content_encoding(&self) -> Option<String> {
        self.head.content_encoding()
    }

    /// Return the cookies set in the response headers
    #[getter]
    fn set_cookies(&self) -> Option<Vec<PyCookie>> {
        self.head.py_set_cookies()
    }

    /// Alias for `set_cookies` property
    #[getter]
    fn cookies(&self) -> Option<Vec<PyCookie>> {
        self.set_cookies()
    }
}

impl std::fmt::Display for RyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Response [{status}; {url}]>",
            status = self.head.status.as_u16(),
            url = self.head.url,
        )
    }
}

#[pymethods]
impl RyBlockingResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        pytodo!("BlockingResponse::new");
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[getter]
    fn status(&self) -> u16 {
        self.head.status.as_u16()
    }

    #[getter]
    fn status_text<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.head.status.as_u16())
    }

    #[getter]
    fn status_code(&self, py: Python<'_>) -> PyResult<Py<PyHttpStatus>> {
        PyHttpStatus::from_status_code_cached(py, self.head.status)
    }

    /// Returns true if the response was redirected
    #[getter]
    fn redirected(&self) -> bool {
        self.head.status.is_redirection()
    }

    #[getter]
    fn version(&self) -> PyHttpVersion {
        PyHttpVersion::from(self.head.version)
    }

    #[getter]
    fn http_version(&self) -> PyHttpVersion {
        PyHttpVersion::from(self.head.version)
    }

    #[getter]
    #[pyo3(name = "url")]
    fn url(&self) -> PyUrl {
        self.head.py_url()
    }

    #[getter]
    fn headers(&self) -> PyHeaders {
        let c = self.head.headers.clone();
        PyHeaders::from(c)
    }

    /// Return the content length of the response, if it is known or `None`.
    #[getter]
    fn content_length(&self) -> Option<u64> {
        self.head.content_length
    }

    #[getter]
    fn remote_addr(&self) -> Option<PySocketAddr> {
        self.head.remote_addr.map(PySocketAddr::from)
    }

    /// Return true if the status code is a success code (200-299)
    #[getter]
    fn ok(&self) -> bool {
        self.head.status.is_success()
    }

    /// __bool__ dunder method returns true if `ok` is true
    fn __bool__(&self) -> bool {
        self.head.status.is_success()
    }

    /// Return true if the body has been consumed
    ///
    /// named after jawascript fetch
    #[getter]
    fn body_used(&self) -> PyResult<bool> {
        Ok(matches!(*self.body.py_lock()?, ResponseBody::Consumed))
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes(&self, py: Python<'_>) -> PyResult<RyBytes> {
        self.body_bytes(py).map(RyBytes::from)
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding = PyEncoding::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text(&self, py: Python<'_>, encoding: PyEncoding) -> PyResult<BodyText> {
        self.text_with_charset(py, encoding)
    }

    /// Return the response body as text with encoding (consumes the response)
    fn text_with_charset(&self, py: Python<'_>, encoding: PyEncoding) -> PyResult<BodyText> {
        self.body_bytes(py)
            .map(|b| BodyText(b, self.head.encoding(encoding)))
    }

    /// Return the response body as json (consumes the response)
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = jiter::StringCacheMode::All,
            partial_mode = jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        ),
        text_signature = "(self, *, allow_inf_nan=False, cache_mode=\"all\", partial_mode=False, catch_duplicate_keys=False)"
    )]
    fn json(
        &self,
        py: Python<'_>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Pyo3JsonBytes> {
        let options = ryo3_jiter::JiterParseOptions::new()
            .with_allow_inf_nan(allow_inf_nan)
            .with_cache_mode(cache_mode)
            .with_partial_mode(partial_mode)
            .with_catch_duplicate_keys(catch_duplicate_keys);
        self.body_bytes(py)
            .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyBlockingResponseStream> {
        Ok(match self.take_owned()? {
            OwnedBody::Buffered(bytes) => {
                RyBlockingResponseStream::from_bytes(self.head.status, bytes)
            }
            OwnedBody::Stream(body) => {
                RyBlockingResponseStream::from_body(self.head.status, body, min_read_size)
            }
        })
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyBlockingResponseStream> {
        self.bytes_stream(min_read_size)
    }

    #[getter]
    fn content_encoding(&self) -> Option<String> {
        self.head.content_encoding()
    }

    /// Return the cookies set in the response headers
    #[getter]
    fn set_cookies(&self) -> Option<Vec<PyCookie>> {
        self.head.py_set_cookies()
    }

    /// Alias for `set_cookies` property
    #[getter]
    fn cookies(&self) -> Option<Vec<PyCookie>> {
        self.set_cookies()
    }
}

impl std::fmt::Display for RyBlockingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<BlockingResponse [{status}; {url}]>",
            status = self.head.status.as_u16(),
            url = self.head.url,
        )
    }
}

#[inline]
async fn read_body_bytes(body: reqwest::Body) -> Result<bytes::Bytes, reqwest::Error> {
    use http_body_util::BodyExt;
    BodyExt::collect(body)
        .await
        .map(http_body_util::Collected::to_bytes)
}

/// body text decoded straight into python `str` when converted.
struct BodyText(Bytes, PyEncoding);

impl<'py> IntoPyObject<'py> for BodyText {
    type Target = PyString;
    type Output = Bound<'py, PyString>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let (text, _, _) = self.1.decode(&self.0);
        Ok(PyString::new(py, &text))
    }
}
