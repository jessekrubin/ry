use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use crate::response_data::RyResponseHead;
use crate::{pyerr_response_already_consumed, RyResponseStream};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use reqwest::header::CONTENT_ENCODING;
use ryo3_http::{status_code_pystring, HttpVersion, PyHeaders, PyHttpStatus};
use ryo3_macro_rules::err_py_not_impl;
use ryo3_url::PyUrl;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass(name = "Response", module = "ry.ryo3.reqwest", frozen)]
#[derive(Debug)]
pub struct RyResponse {
    /// The actual response which will be consumed when read
    res: Arc<Mutex<Option<reqwest::Response>>>,

    /// Response "head" data (status, headers, url, http-version, etc.)
    head: RyResponseHead,
}

impl RyResponse {
    #[must_use]
    pub fn new(res: reqwest::Response) -> Self {
        Self {
            head: RyResponseHead::from(&res),
            res: Arc::new(Mutex::new(Some(res))),
        }
    }
}

impl From<reqwest::Response> for RyResponse {
    fn from(res: reqwest::Response) -> Self {
        Self::new(res)
    }
}

#[pymethods]
impl RyResponse {
    #[new]
    fn py_new() -> PyResult<Self> {
        err_py_not_impl!("Response::new")
    }

    fn __repr__(&self) -> String {
        format!("Response<{}>", self.head.status_code)
    }

    #[getter]
    fn status(&self) -> u16 {
        self.head.status_code.as_u16()
    }

    #[getter]
    fn status_text<'py>(&self, py: Python<'py>) -> Option<&Bound<'py, PyString>> {
        status_code_pystring(py, self.head.status_code.as_u16())
    }

    #[getter]
    fn status_code(&self) -> PyHttpStatus {
        PyHttpStatus(self.head.status_code)
    }

    /// Returns true if the response was redirected
    #[getter]
    fn redirected(&self) -> bool {
        self.head.status_code.is_redirection()
    }

    #[getter]
    fn version(&self) -> HttpVersion {
        HttpVersion(self.head.version)
    }

    #[getter]
    fn http_version(&self) -> HttpVersion {
        HttpVersion(self.head.version)
    }

    #[getter]
    #[pyo3(name = "url")]
    fn url(&self) -> Option<PyUrl> {
        self.head.url.as_ref().map(|url| PyUrl(url.clone()))
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

    /// Return true if the status code is a success code (200-299)
    #[getter]
    fn ok(&self) -> bool {
        self.head.status_code.is_success()
    }

    /// __bool__ dunder method returns true if `ok` is true
    fn __bool__(&self) -> bool {
        self.head.status_code.is_success()
    }

    /// Return true if the body has been consumed
    ///
    /// named after jawascript fetch
    #[getter]
    fn body_used(&self) -> bool {
        self.res.blocking_lock().is_none()
    }

    #[getter]
    fn content_encoding(&self) -> Option<String> {
        self.head.headers.get(CONTENT_ENCODING).map(|en| {
            let s = en.to_str().expect("Invalid content encoding");
            s.to_string()
        })
    }

    /// Return the response body as bytes (consumes the response)
    fn bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res_arc = self.res.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut opt = res_arc.lock().await;
            let resp = opt.take().ok_or(pyerr_response_already_consumed!())?;
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
            let resp = opt.take().ok_or(pyerr_response_already_consumed!())?;
            resp.text().await.map_err(map_reqwest_err)
        })
    }

    /// Return the response body as json (consumes the response)
    fn json<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let res_arc = self.res.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut opt = res_arc.lock().await;
            let resp = opt.take().ok_or(pyerr_response_already_consumed!())?;
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
}
