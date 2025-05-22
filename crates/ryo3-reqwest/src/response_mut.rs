use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::types::PyString;
use pyo3::{pyclass, pymethods, Bound, PyAny, PyRef, PyResult, Python};
use reqwest::header::{HeaderMap, CONTENT_ENCODING};
use reqwest::StatusCode;
use ryo3_http::{status_code_pystring, HttpVersion, PyHeaders, PyHttpStatus};
use ryo3_macro_rules::err_py_not_impl;
use ryo3_url::PyUrl;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
#[pyclass(name = "Response", module = "ry.ryo3.reqwest")]
pub struct RyResponse {
    /// The actual response which will be consumed when read
    res: Option<reqwest::Response>,

    // ========================================================================
    /// das status code
    status_code: StatusCode,
    /// das headers
    headers: HeaderMap,
    /// das url
    url: Option<reqwest::Url>,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    content_length: Option<u64>,

    /// version of http spec
    version: reqwest::Version,
}

impl From<reqwest::Response> for RyResponse {
    fn from(res: reqwest::Response) -> Self {
        Self {
            status_code: res.status(),
            headers: res.headers().clone(),
            // cookies: res.cookies().clone(),
            url: Some(res.url().clone()),
            content_length: res.content_length(),
            version: res.version(),
            res: Some(res),
        }
    }
}

#[pymethods]
impl RyResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        err_py_not_impl!("Response::new")
    }

    fn __repr__(&self) -> String {
        format!("Response<{}>", self.status_code)
    }

    #[getter]
    fn status(&self) -> u16 {
        self.status_code.as_u16()
    }

    #[getter]
    fn status_text<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.status_code.as_u16())
    }

    #[getter]
    fn status_code(&self) -> PyHttpStatus {
        PyHttpStatus(self.status_code)
    }

    /// Returns true if the response was redirected
    #[getter]
    fn redirected(&self) -> bool {
        self.status_code.is_redirection()
    }

    #[getter]
    fn version(&self) -> HttpVersion {
        HttpVersion(self.version)
    }

    #[getter]
    fn http_version(&self) -> HttpVersion {
        HttpVersion(self.version)
    }

    #[getter]
    fn url(&self) -> Option<PyUrl> {
        self.url.as_ref().map(|url| PyUrl(url.clone()))
    }

    #[getter]
    fn headers(&self) -> PyHeaders {
        let c = self.headers.clone();
        PyHeaders::from(c)
    }

    /// Return the content length of the response, if it is known or `None`.
    #[getter]
    fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Return true if the status code is a success code (200-299)
    #[getter]
    fn ok(&self) -> bool {
        self.status_code.is_success()
    }

    /// __bool__ dunder method returns true if `ok` is true
    fn __bool__(&self) -> bool {
        self.status_code.is_success()
    }

    /// Return true if the body has been consumed
    ///
    /// named after jawascript fetch
    #[getter]
    fn body_used(&self) -> bool {
        self.res.is_none()
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let bytes_result = response.bytes().await;
            bytes_result
                .map(ryo3_bytes::PyBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return the response body as text/string (consumes the response)
    fn text<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response.text().await.map_err(map_reqwest_err)
        })
    }

    /// Return the response body as json (consumes the response)
    fn json<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(Pyo3JsonBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return a response consuming async iterator over the response body
    fn bytes_stream(&mut self) -> PyResult<RyResponseStream> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;

        // HOLY SHIT THIS TOOK A LOT OF TRIAL AND ERROR
        let stream = response.bytes_stream();
        let stream = Box::pin(stream);
        Ok(RyResponseStream {
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    #[getter]
    fn content_encoding(&self) -> Option<String> {
        self.headers.get(CONTENT_ENCODING).map(|en| {
            let s = en.to_str().expect("Invalid content encoding");
            s.to_string()
        })
    }
}

// This whole response iterator was a difficult thing to figure out.
//
// NOTE: I (jesse) am pretty proud of this. I was struggling to get the
//       async-iterator thingy to work bc rust + async is quite hard, but
//       after lots and lots and lots of trial and error this works!
//
// clippy says this is too long and complicated to just sit in the struct def
type AsyncResponseStreamInner =
    Arc<Mutex<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>>;
#[pyclass(name = "ResponseStream", module = "ry.ryo3.reqwest")]
pub struct RyResponseStream {
    stream: AsyncResponseStreamInner,
}

#[pymethods]
impl RyResponseStream {
    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            match guard.as_mut().next().await {
                Some(Ok(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-fin")),
            }
        })
    }
}
