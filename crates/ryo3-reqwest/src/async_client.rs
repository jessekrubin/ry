use crate::errors::map_reqwest_err;
use crate::pyo3_bytes::Pyo3JsonBytes;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest::StatusCode;
use ryo3_bytes::Pyo3Bytes;
use ryo3_url::PyUrl;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
#[pyo3(name = "AsyncClient")]
#[derive(Debug, Clone)]
pub struct RyAsyncClient(reqwest::Client);
#[pyclass]
#[pyo3(name = "AsyncResponse")]
#[derive(Debug)]
pub struct RyAsyncResponse {
    /// The actual response which will be consumed when read
    res: Option<reqwest::Response>,

    // ========================================================================
    /// das status code
    status_code: StatusCode,
    /// das headers
    headers: reqwest::header::HeaderMap,
    /// das url
    url: reqwest::Url,
    /// das content length -- if it exists (tho it might not and/or be
    /// different if the response is compressed)
    content_length: Option<u64>,
}

impl From<reqwest::Response> for RyAsyncResponse {
    fn from(res: reqwest::Response) -> Self {
        Self {
            status_code: res.status(),
            headers: res.headers().clone(),
            // cookies: res.cookies().clone(),
            // version: res.version(),
            url: res.url().clone(),
            content_length: res.content_length(),
            // body: None,
            res: Some(res),
        }
    }
}
#[pymethods]
impl RyAsyncResponse {
    #[getter]
    fn status_code(&self) -> u16 {
        self.status_code.as_u16()
    }

    #[getter]
    #[pyo3(name = "url")]
    fn py_url(&self) -> PyUrl {
        PyUrl(self.url.clone())
    }

    #[getter]
    #[pyo3(name = "headers")]
    fn py_headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let c = self.headers.clone();
        let pydict = PyDict::new(py);
        for (name, value) in &c {
            let k = name.to_string();
            let v = value
                .to_str()
                .map(String::from)
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            pydict.set_item(k, v)?;
        }
        Ok(pydict)
    }

    #[getter]
    fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    fn bytes<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response
                .bytes()
                .await
                .map(Pyo3Bytes::from)
                .map_err(map_reqwest_err)
        })
    }
    fn text<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response.text().await.map_err(map_reqwest_err)
        })
    }

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
    fn bytes_stream(&mut self) -> PyResult<RyAsyncResponseIter> {
        let response = self
            .res
            .take()
            .ok_or(PyValueError::new_err("Response already consumed"))?;

        // HOLY SHIT THIS TOOK A LOT OF TRIAL AND ERROR
        let stream = response.bytes_stream();
        let stream = Box::pin(stream);
        Ok(RyAsyncResponseIter {
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    fn __str__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response: {}", self.status_code)
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
#[pyclass]
pub struct RyAsyncResponseIter {
    stream: AsyncResponseStreamInner,
}

#[pymethods]
impl RyAsyncResponseIter {
    fn __aiter__(this: PyRef<Self>) -> PyRef<Self> {
        this
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            match guard.as_mut().next().await {
                Some(Ok(bytes)) => Ok(Some(Pyo3Bytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-fin")),
            }
        })
    }
}

#[pymethods]
impl RyAsyncClient {
    #[new]
    fn new() -> Self {
        Self(reqwest::Client::new())
    }

    fn get<'py>(&'py mut self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.get(url).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn post<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.post(url).body(body.to_vec()).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn put<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.put(url).body(body.to_vec()).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn patch<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.patch(url).body(body.to_vec()).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn delete<'py>(&'py mut self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.delete(url).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn head<'py>(&'py mut self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
        let response_future = self.0.head(url).send();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            response_future
                .await
                .map(RyAsyncResponse::from)
                .map_err(map_reqwest_err)
        })
    }
}
