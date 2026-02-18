use crate::charset::PyEncodingName;
use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use crate::response_head::RyResponseHead;
#[cfg(feature = "experimental-async")]
use crate::response_stream::RyAsyncResponseStream;
use crate::response_stream::RyBlockingResponseStream;
use crate::{PyCookie, RyResponseStream, pyerr_response_already_consumed};
use cookie::Cookie;
use parking_lot::Mutex;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use reqwest::header::{CONTENT_ENCODING, SET_COOKIE};
use ryo3_bytes::PyBytes as RyBytes;
use ryo3_http::{PyHeaders, PyHttpStatus, PyHttpVersion, status_code_pystring};
#[cfg(feature = "experimental-async")]
use ryo3_macro_rules::py_runtime_error;
use ryo3_macro_rules::pytodo;
use ryo3_std::net::PySocketAddr;
use ryo3_url::PyUrl;
use std::sync::Arc;

#[pyclass(name = "Response", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyResponse {
    /// The actual response which will be consumed when read
    res: Arc<Mutex<Option<reqwest::Response>>>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

#[cfg(feature = "experimental-async")]
#[pyclass(name = "AsyncResponse", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyAsyncResponse {
    /// The actual response which will be consumed when read
    res: Arc<Mutex<Option<reqwest::Response>>>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

#[pyclass(name = "BlockingResponse", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug)]
pub struct RyBlockingResponse {
    /// The actual response which will be consumed when read
    res: Arc<Mutex<Option<reqwest::Response>>>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

impl RyResponse {
    /// Create a new response from a reqwest response
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
        Self {
            head: RyResponseHead::from(&res),
            res: Arc::new(Mutex::new(Some(res))),
        }
    }

    fn take_response(&self) -> PyResult<reqwest::Response> {
        let mut opt = self.res.lock();
        opt.take().ok_or_else(|| pyerr_response_already_consumed!())
    }
}

#[cfg(feature = "experimental-async")]
impl RyAsyncResponse {
    /// Create a new response from a reqwest response
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
        Self {
            head: RyResponseHead::from(&res),
            res: Arc::new(Mutex::new(Some(res))),
        }
    }

    fn take_response(&self) -> PyResult<reqwest::Response> {
        let mut opt = self.res.lock();
        opt.take().ok_or_else(|| pyerr_response_already_consumed!())
    }
}

impl RyBlockingResponse {
    /// Create a new response from a reqwest response
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
        Self {
            head: RyResponseHead::from(&res),
            res: Arc::new(Mutex::new(Some(res))),
        }
    }

    fn take_response(&self) -> PyResult<reqwest::Response> {
        let mut opt = self.res.lock();
        opt.take().ok_or_else(|| pyerr_response_already_consumed!())
    }
}

impl From<reqwest::Response> for RyResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

#[cfg(feature = "experimental-async")]
impl From<reqwest::Response> for RyAsyncResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

impl From<reqwest::Response> for RyBlockingResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

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
    fn body_used(&self) -> bool {
        self.res.lock().is_none()
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self.take_response()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(RyBytes::from)
                .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
        })
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding=PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.text_with_charset(py, encoding)
    }

    /// Return the response body as text/string (consumes the response) with default-encoding
    fn text_with_charset<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<Bound<'py, PyAny>> {
        let response = self.take_response()?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .text_with_charset(encoding.as_ref())
                .await
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as json (consumes the response)
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = jiter::StringCacheMode::All,
            partial_mode = jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        )
    )]
    fn json<'py>(
        &'py self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let response = self.take_response()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
                .map_err(map_reqwest_err)
        })
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyResponseStream> {
        let response = self.take_response()?;
        Ok(RyResponseStream::from_response(response, min_read_size))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
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
        let headers = self.head.headers.py_read();
        let py_cookies: Vec<PyCookie> = headers // nom nom nom nom nom
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .map(ToOwned::to_owned)
            .filter_map(|s| Cookie::parse(s).ok())
            .map(PyCookie::from)
            .collect();
        if py_cookies.is_empty() {
            None
        } else {
            Some(py_cookies)
        }
    }

    /// Alias for `set_cookies` property
    #[getter]
    fn cookies(&self) -> Option<Vec<PyCookie>> {
        self.set_cookies()
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyAsyncResponse {
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
    fn body_used(&self) -> bool {
        self.res.lock().is_none()
    }

    /// Return the response body as bytes (consumes the response)
    async fn bytes(&self) -> PyResult<RyBytes> {
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let response = self.take_response()?;
        rt.spawn(async move { response.bytes().await })
            .await
            .map_err(|e| py_runtime_error!("{e}"))?
            .map(RyBytes::from)
            .map_err(map_reqwest_err)
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding=PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    async fn text(&self, encoding: PyEncodingName) -> PyResult<String> {
        let response = self.take_response()?;
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        rt.spawn(async move { response.text_with_charset(encoding.as_ref()).await })
            .await
            .map_err(|e| py_runtime_error!("{e}"))?
            .map_err(map_reqwest_err)
    }

    /// Return the response body as text/string (consumes the response) with default-encoding
    async fn text_with_charset(&self, encoding: PyEncodingName) -> PyResult<String> {
        let response = self.take_response()?;
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        rt.spawn(async move { response.text_with_charset(encoding.as_ref()).await })
            .await
            .map_err(|e| py_runtime_error!("{e}"))?
            .map_err(map_reqwest_err)
    }

    /// Return the response body as json (consumes the response)
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = jiter::StringCacheMode::All,
            partial_mode = jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        )
    )]
    async fn json(
        &self,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Pyo3JsonBytes> {
        let response = self.take_response()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };
        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let j = rt
            .spawn(async move { response.bytes().await })
            .await
            .map_err(|e| py_runtime_error!("{e}"))?;
        j.map(|bytes| Pyo3JsonBytes::from((bytes, options)))
            .map_err(map_reqwest_err)
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyAsyncResponseStream> {
        let response = self.take_response()?;
        Ok(RyAsyncResponseStream::from_response(
            response,
            min_read_size,
        ))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
    fn stream(&self, min_read_size: usize) -> PyResult<RyAsyncResponseStream> {
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
        let headers = self.head.headers.py_read();
        let py_cookies: Vec<PyCookie> = headers // nom nom nom nom nom
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .map(ToOwned::to_owned)
            .filter_map(|s| Cookie::parse(s).ok())
            .map(PyCookie::from)
            .collect();
        if py_cookies.is_empty() {
            None
        } else {
            Some(py_cookies)
        }
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

#[cfg(feature = "experimental-async")]
impl std::fmt::Display for RyAsyncResponse {
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
    fn body_used(&self) -> bool {
        self.res.lock().is_none()
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes(&self, py: Python<'_>) -> PyResult<RyBytes> {
        let response = self.take_response()?;

        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async { response.bytes().await })
                .map(RyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as text/string (consumes the response)
    #[pyo3(
        signature = (*, encoding=PyEncodingName::UTF_8),
        text_signature = "(self, *, encoding=\"utf-8\")"
    )]
    fn text<'py>(&'py self, py: Python<'py>, encoding: PyEncodingName) -> PyResult<String> {
        let response = self.take_response()?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async { response.text_with_charset(encoding.as_ref()).await })
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as text/string (consumes the response) with default-encoding
    fn text_with_charset<'py>(
        &'py self,
        py: Python<'py>,
        encoding: PyEncodingName,
    ) -> PyResult<String> {
        let response = self.take_response()?;
        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime()
                .block_on(async { response.text_with_charset(encoding.as_ref()).await })
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as json (consumes the response)
    #[pyo3(
        signature = (
            *,
            allow_inf_nan = false,
            cache_mode = jiter::StringCacheMode::All,
            partial_mode = jiter::PartialMode::Off,
            catch_duplicate_keys = false,
        )
    )]
    fn json<'py>(
        &'py self,
        py: Python<'py>,
        allow_inf_nan: bool,
        cache_mode: jiter::StringCacheMode,
        partial_mode: jiter::PartialMode,
        catch_duplicate_keys: bool,
    ) -> PyResult<Pyo3JsonBytes> {
        let response = self.take_response()?;
        let options = ryo3_jiter::JiterParseOptions {
            allow_inf_nan,
            cache_mode,
            partial_mode,
            catch_duplicate_keys,
        };

        py.detach(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(async {
                response
                    .bytes()
                    .await
                    .map(|bytes| Pyo3JsonBytes::from((bytes, options)))
                    .map_err(map_reqwest_err)
            })
        })
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
    fn bytes_stream(&self, min_read_size: usize) -> PyResult<RyBlockingResponseStream> {
        let response = self.take_response()?;
        Ok(RyBlockingResponseStream::from_response(
            response,
            min_read_size,
        ))
    }

    /// Return a response consuming async iterator over the response body
    #[pyo3(signature = (min_read_size=0, /))]
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
        let headers = self.head.headers.py_read();
        let py_cookies: Vec<PyCookie> = headers // nom nom nom nom nom
            .get_all(SET_COOKIE)
            .iter()
            .filter_map(|hv| hv.to_str().ok())
            .map(ToOwned::to_owned)
            .filter_map(|s| Cookie::parse(s).ok())
            .map(PyCookie::from)
            .collect();
        if py_cookies.is_empty() {
            None
        } else {
            Some(py_cookies)
        }
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
