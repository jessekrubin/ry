use pyo3::prelude::*;
use pyo3::types::PyString;
#[cfg(feature = "experimental-async")]
use pyo3::{coroutine::CancelHandle, exceptions::asyncio::CancelledError};
use reqwest::header::{CONTENT_ENCODING, CONTENT_TYPE};
use ryo3_bytes::RyBytes;
use ryo3_cookie::PyCookie;
use ryo3_core::sync::RyMutex;
use ryo3_http::{PyHeaders, PyHttpStatus, PyHttpVersion, status_code_pystring};
use ryo3_macro_rules::pytodo;
use ryo3_std::net::PySocketAddr;
#[cfg(not(feature = "experimental-async"))]
use ryo3_tokio_rt::future_into_py;
use ryo3_tokio_rt::get_tokio_runtime;
#[cfg(feature = "experimental-async")]
use ryo3_tokio_rt::on_tokio_py;
use ryo3_url::PyUrl;

use crate::charset::PyEncodingName;
use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use crate::response_head::RyResponseHead;
use crate::response_stream::RyBlockingResponseStream;
use crate::{RyResponseStream, pyerr_response_already_consumed};

#[pyclass(name = "Response", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyResponse {
    /// The response body, consumed only once
    body: RyMutex<ResponseBody>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

#[pyclass(name = "BlockingResponse", frozen, immutable_type, skip_from_py_object)]
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
    Consumed,
}

impl RyResponse {
    /// Create a new response from a reqwest response
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
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
        Self {
            head,
            body: RyMutex::new(ResponseBody::Stream(body)),
        }
    }

    fn take_body(&self) -> PyResult<reqwest::Body> {
        let mut state = self.body.py_lock()?;
        match std::mem::replace(&mut *state, ResponseBody::Consumed) {
            ResponseBody::Stream(body) => Ok(body),
            ResponseBody::Consumed => Err(pyerr_response_already_consumed!()),
        }
    }

    fn response_encoding(&self, default: PyEncodingName) -> &'static str {
        let default_encoding = default.as_static_str();
        let content_type = self
            .head
            .headers
            .py_read()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok());
        let encoding_name = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or(default_encoding);
        encoding_rs::Encoding::for_label(encoding_name.as_bytes())
            .map_or(default_encoding, encoding_rs::Encoding::name)
    }
}

impl RyBlockingResponse {
    /// Create a new response from a reqwest response
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
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
        Self {
            head,
            body: RyMutex::new(ResponseBody::Stream(body)),
        }
    }

    fn take_body(&self) -> PyResult<reqwest::Body> {
        let mut state = self.body.py_lock()?;
        match std::mem::replace(&mut *state, ResponseBody::Consumed) {
            ResponseBody::Stream(body) => Ok(body),
            ResponseBody::Consumed => Err(pyerr_response_already_consumed!()),
        }
    }

    fn response_encoding(&self, default: PyEncodingName) -> &'static str {
        let default_encoding = default.as_static_str();
        let content_type = self
            .head
            .headers
            .py_read()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok());
        let encoding_name = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or(default_encoding);
        encoding_rs::Encoding::for_label(encoding_name.as_bytes())
            .map_or(default_encoding, encoding_rs::Encoding::name)
    }
}

impl From<reqwest::Response> for RyResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

impl From<reqwest::Response> for RyBlockingResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

#[cfg(not(feature = "experimental-async"))]
#[pymethods]
impl RyResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        pytodo!("Response::new")
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
        PyUrl::from(self.head.url.clone())
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
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let body = self.take_body()?;
        future_into_py(py, async move {
            read_body_bytes(body)
                .await
                .map(RyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding = PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.text_with_charset(py, encoding)
    }

    /// Return the response body as text with encoding (consumes the response)
    fn text_with_charset<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body = self.take_body()?;
        let encoding_name = self.response_encoding(encoding);
        future_into_py(py, async move { read_body_text(body, encoding_name).await })
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
        &'py self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let body = self.take_body()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };
        future_into_py(py, async move {
            read_body_bytes(body)
                .await
                .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
                .map_err(map_reqwest_err)
        })
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        let body = self.take_body()?;
        Ok(RyResponseStream::from_body(
            self.head.status,
            body,
            min_read_size,
        ))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        self.bytes_stream(min_read_size)
    }

    #[getter]
    fn content_encoding(&self) -> Option<String> {
        (*self.head.headers.py_read())
            .get(CONTENT_ENCODING)
            .map(|en| en.to_str().expect("wildly unlikely").to_string())
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

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        pytodo!("Response::new")
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
        PyUrl::from(self.head.url.clone())
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
    async fn bytes(&self, #[pyo3(cancel_handle)] cancel: CancelHandle) -> PyResult<RyBytes> {
        let body = self.take_body()?;
        on_tokio_py(async move {
            read_body_bytes_with_cancel(body, cancel)
                .await
                .map(RyBytes::from)
        })
        .await
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding = PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    async fn text(
        &self,
        encoding: PyEncodingName,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<String> {
        let body = self.take_body()?;
        let encoding_name = self.response_encoding(encoding);
        on_tokio_py(async move {
            let full = read_body_bytes_with_cancel(body, cancel).await?;
            decode_body_text(full, encoding_name)
        })
        .await
    }

    /// Return the response body as text with encoding (consumes the response)
    async fn text_with_charset(
        &self,
        encoding: PyEncodingName,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<String> {
        let body = self.take_body()?;
        let encoding_name = self.response_encoding(encoding);
        on_tokio_py(async move {
            let full = read_body_bytes_with_cancel(body, cancel).await?;
            decode_body_text(full, encoding_name)
        })
        .await
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
    async fn json(
        &self,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<Pyo3JsonBytes> {
        let body = self.take_body()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };
        on_tokio_py(async move {
            read_body_bytes_with_cancel(body, cancel)
                .await
                .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
        })
        .await
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        let body = self.take_body()?;
        Ok(RyResponseStream::from_body(
            self.head.status,
            body,
            min_read_size,
        ))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        self.bytes_stream(min_read_size)
    }

    /// Return the `content-encoding` header value of the response or `None`
    #[getter]
    fn content_encoding(&self) -> Option<String> {
        (*self.head.headers.py_read())
            .get(CONTENT_ENCODING)
            .map(|en| {
                let s = en.to_str().expect("Invalid content encoding");
                s.to_string()
            })
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
        pytodo!("BlockingResponse::new")
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
        PyUrl::from(self.head.url.clone())
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
        let body = self.take_body()?;

        py.detach(|| {
            get_tokio_runtime()
                .block_on(read_body_bytes(body))
                .map(RyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding = PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text<'py>(&'py self, py: Python<'py>, encoding: PyEncodingName) -> PyResult<String> {
        let body = self.take_body()?;
        let encoding_name = self.response_encoding(encoding);
        py.detach(|| get_tokio_runtime().block_on(read_body_text(body, encoding_name)))
    }

    /// Return the response body as text with encoding (consumes the response)
    fn text_with_charset<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<String> {
        let body = self.take_body()?;
        let encoding_name = self.response_encoding(encoding);
        py.detach(|| get_tokio_runtime().block_on(read_body_text(body, encoding_name)))
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
        &'py self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Pyo3JsonBytes> {
        let body = self.take_body()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };

        py.detach(|| {
            get_tokio_runtime().block_on(async {
                read_body_bytes(body)
                    .await
                    .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
                    .map_err(map_reqwest_err)
            })
        })
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyBlockingResponseStream> {
        let body = self.take_body()?;
        Ok(RyBlockingResponseStream::from_body(
            self.head.status,
            body,
            min_read_size,
        ))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size = 0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyBlockingResponseStream> {
        self.bytes_stream(min_read_size)
    }

    #[getter]
    fn content_encoding(&self) -> Option<String> {
        (*self.head.headers.py_read())
            .get(CONTENT_ENCODING)
            .map(|en| {
                let s = en.to_str().expect("Invalid content encoding");
                s.to_string()
            })
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

#[cfg(feature = "experimental-async")]
#[inline]
async fn read_body_bytes_with_cancel(
    body: reqwest::Body,
    mut cancel: CancelHandle,
) -> PyResult<bytes::Bytes> {
    tokio::select! {
        res = read_body_bytes(body) => res.map_err(map_reqwest_err),
        _ = cancel.cancelled() => Err(CancelledError::new_err("Response read was cancelled")),
    }
}

#[inline]
async fn read_body_text(body: reqwest::Body, encoding_name: &str) -> PyResult<String> {
    let full = read_body_bytes(body).await.map_err(map_reqwest_err)?;
    decode_body_text(full, encoding_name)
}

#[inline]
fn decode_body_text(full: bytes::Bytes, encoding_name: &str) -> PyResult<String> {
    let encoding =
        encoding_rs::Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    let (text, _, _) = encoding.decode(&full);
    Ok(text.into_owned())
}
