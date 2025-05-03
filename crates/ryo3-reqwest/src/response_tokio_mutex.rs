use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use crate::{pyerr_response_already_consumed, RyResponseStream};
use futures_util::StreamExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use reqwest::header::{HeaderMap, CONTENT_ENCODING};
use reqwest::StatusCode;
use ryo3_http::{status_code_pystring, HttpVersion, PyHeaders, PyHttpStatus};
use ryo3_macros::err_py_not_impl;
use ryo3_url::PyUrl;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
#[pyo3(name = "Response", module = "ry.ryo3.reqwest", frozen)]
#[derive(Debug)]
pub struct RyResponse {
    /// The actual response which will be consumed when read
    res: Arc<Mutex<Option<reqwest::Response>>>,

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
            // version: res.version(),
            url: Some(res.url().clone()),
            content_length: res.content_length(),
            version: res.version(),
            res: Arc::new(Mutex::new(Some(res))),
        }
    }
}

#[pymethods]
impl RyResponse {
    #[new]
    pub fn py_new() -> PyResult<Self> {
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
    #[pyo3(name = "url")]
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
        return true;
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res_arc = self.res.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut opt = res_arc.lock().await;
            let resp = opt
                .take()
                .ok_or(pyerr_response_already_consumed!())?;
                // .ok_or_else(|| PyErr::new::<PyValueError, _>("response already consumed"))?;
                // .ok_or_else(|| PyErr::new::<PyValueError, _>("response already consumed"))?;
            resp.bytes()
                .await
                .map(ryo3_bytes::PyBytes::from)
                .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
        })
    }

    /// Return the response body as text/string (consumes the response)
    fn text<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res_arc = self.res.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut opt = res_arc.lock().await;
            let resp = opt
                .take()
                .ok_or(pyerr_response_already_consumed!())?;
            resp.text().await.map_err(map_reqwest_err)
        })
    }

    /// Return the response body as json (consumes the response)
    fn json<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res_arc = self.res.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut opt = res_arc.lock().await;
            let resp = opt
                .take()
                .ok_or(pyerr_response_already_consumed!())?;
            resp.bytes()
                .await
                .map(Pyo3JsonBytes::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return a response consuming async iterator over the response body
    fn bytes_stream(&self) -> PyResult<RyResponseStream> {
        let res = self
            .res
            .clone()
            .blocking_lock()
            .take()
            .ok_or_else(|| PyErr::new::<PyValueError, _>("response already consumed"))?;

        let stream = Box::pin(res.bytes_stream());
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
