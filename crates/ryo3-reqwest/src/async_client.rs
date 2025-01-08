use crate::errors::map_reqwest_err;

use crate::pyo3_bytes::Pyo3JsonBytes;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use ryo3_bytes::Pyo3Bytes;
use ryo3_http::PyHeadersLike;
use ryo3_url::PyUrl;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

#[pyclass]
#[pyo3(name = "AsyncClient", module = "ry.ryo3.reqwest")]
#[derive(Debug, Clone)]
pub struct RyAsyncClient(pub reqwest::Client);
#[pyclass]
#[pyo3(name = "Response", module = "ry.ryo3.reqwest")]
#[derive(Debug)]
pub struct RyResponse {
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

impl From<reqwest::Response> for RyResponse {
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
impl RyResponse {
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

    /// Return the content length of the response, if it is known or `None`.
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
        format!("Response<{}>", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response<{}>", self.status_code)
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
    #[pyo3(
        signature = (
            headers = None,
            timeout = 30,
            gzip = true,
            brotli = true,
            deflate = true
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        timeout: Option<u64>,
        gzip: Option<bool>,
        brotli: Option<bool>,
        deflate: Option<bool>,
    ) -> PyResult<Self> {
        let mut client_builder = reqwest::Client::builder();
        if let Some(headers) = headers {
            client_builder = client_builder.default_headers(HeaderMap::from(headers));
        }
        client_builder = client_builder
            .brotli(brotli.unwrap_or(true))
            .gzip(gzip.unwrap_or(true))
            .deflate(deflate.unwrap_or(true))
            .timeout(std::time::Duration::from_secs(timeout.unwrap_or(30)));
        let client = client_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("client-build: {e}")))?;
        Ok(Self(client))
    }

    #[pyo3(
      signature = (url, *, headers = None),
    )]
    fn get<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut req = self.0.get(url);
        // fing-fang-foom make de headers...
        if let Some(headers) = headers {
            req = req.headers(HeaderMap::from(headers));
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn post<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let req = self.0.post(url).body(body.to_vec());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn put<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let req = self.0.put(url).body(body.to_vec());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn patch<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        body: &[u8],
    ) -> PyResult<Bound<'py, PyAny>> {
        let req = self.0.patch(url).body(body.to_vec());
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn delete<'py>(&'py mut self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
        let req = self.0.delete(url);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    fn head<'py>(&'py mut self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
        let req = self.0.head(url);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
        signature = (
            url,
            *,
            method = None,
            body = None,
            headers = None
        )
    )]
    pub fn fetch<'py>(
        &'py mut self,
        py: Python<'py>,
        url: &str,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<&[u8]>,
        headers: Option<Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let method = method.unwrap_or(ryo3_http::HttpMethod(reqwest::Method::GET));
        let mut req = self.0.request(method.0, url);
        if let Some(body) = body {
            req = req.body(body.to_vec());
        }
        if let Some(headers) = headers {
            let mut default_headers = reqwest::header::HeaderMap::new();
            for (k, v) in headers {
                let k = k.to_string();
                let v = v.to_string();
                let header_name = reqwest::header::HeaderName::from_bytes(k.as_bytes())
                    .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
                let header_value = reqwest::header::HeaderValue::from_str(&v)
                    .map_err(|e| PyValueError::new_err(format!("header-value-error: {e}")))?;
                default_headers.insert(header_name, header_value);
            }
            req = req.headers(default_headers);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }
}
