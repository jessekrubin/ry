use crate::errors::map_reqwest_err;
use crate::pyo3_json_bytes::Pyo3JsonBytes;
use bytes::Bytes;
use futures_core::Stream;
use futures_util::StreamExt;
use pyo3::exceptions::{PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{intern, IntoPyObjectExt};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use ryo3_http::{PyHeaders, PyHeadersLike, PyHttpStatus};
use ryo3_macros::err_py_not_impl;
use ryo3_url::{extract_url, PyUrl};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

#[pyclass]
#[pyo3(name = "HttpClient", module = "ry.ryo3.reqwest", frozen)]
#[derive(Debug, Clone)]
pub struct RyHttpClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

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
            res: Some(res),
        }
    }
}

#[pymethods]
impl RyResponse {
    #[new]
    pub fn py_new() -> PyResult<Self> {
        err_py_not_impl!("Response::new")
    }

    fn __str__(&self) -> String {
        format!("Response<{}>", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response<{}>", self.status_code)
    }

    #[getter]
    fn status(&self) -> u16 {
        self.status_code.as_u16()
    }

    #[getter]
    fn status_text(&self) -> String {
        self.status_code
            .canonical_reason()
            .unwrap_or_default()
            .to_string()
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
    fn version_str(&self) -> String {
        format!("{:?}", self.version)
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
                Some(Ok(bytes)) => Ok(Some(ryo3_bytes::PyBytes::from(bytes))),
                Some(Err(e)) => Err(map_reqwest_err(e)),
                // I totally forgot that this was a thing and that I couldn't just return None
                None => Err(PyStopAsyncIteration::new_err("response-stream-fin")),
            }
        })
    }
}

fn parse_user_agent(user_agent: Option<String>) -> PyResult<HeaderValue> {
    let ua_str = user_agent.unwrap_or_else(|| {
        format!(
            "ry/{} - OSS (github.com/jessekrubin/ry)",
            env!("CARGO_PKG_VERSION")
        )
    });
    ua_str
        .parse()
        .map_err(|e| PyValueError::new_err(format!("{e}")))
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientConfig {
    headers: Option<PyHeaders>,
    user_agent: Option<ryo3_http::HttpHeaderValue>,
    timeout: Option<ryo3_std::PyDuration>,
    read_timeout: Option<ryo3_std::PyDuration>,
    connect_timeout: Option<ryo3_std::PyDuration>,
    gzip: bool,
    brotli: bool,
    deflate: bool,
}

impl<'py> IntoPyObject<'py> for &ClientConfig {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "headers"), self.headers.clone())?;
        dict.set_item(intern!(py, "user_agent"), self.user_agent.clone())?;
        dict.set_item(intern!(py, "timeout"), self.timeout.clone())?;
        dict.set_item(intern!(py, "read_timeout"), self.read_timeout.clone())?;
        dict.set_item(intern!(py, "connect_timeout"), self.connect_timeout.clone())?;
        dict.set_item(intern!(py, "gzip"), self.gzip)?;
        dict.set_item(intern!(py, "brotli"), self.brotli)?;
        dict.set_item(intern!(py, "deflate"), self.deflate)?;
        Ok(dict)
    }
}

impl ClientConfig {
    fn apply(&self, client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let mut client_builder = client_builder;
        if let Some(user_agent) = &self.user_agent {
            client_builder = client_builder.user_agent(user_agent.clone());
        }
        if let Some(headers) = &self.headers {
            client_builder = client_builder.default_headers(headers.0.clone());
        }
        if let Some(timeout) = &self.timeout {
            client_builder = client_builder.timeout(timeout.0);
        }
        if let Some(read_timeout) = &self.read_timeout {
            client_builder = client_builder.read_timeout(read_timeout.0);
        }
        if let Some(connect_timeout) = &self.connect_timeout {
            client_builder = client_builder.connect_timeout(connect_timeout.0);
        }
        client_builder = client_builder
            .connection_verbose(false)
            .brotli(self.brotli)
            .gzip(self.gzip)
            .deflate(self.deflate);
        client_builder
    }

    fn client_builder(&self) -> reqwest::ClientBuilder {
        let client_builder = reqwest::Client::builder();
        self.apply(client_builder)
    }
}

impl RyHttpClient {
    pub fn new(cfg: Option<ClientConfig>) -> PyResult<Self> {
        let cfg = cfg.unwrap_or_default();
        let client_builder = cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
        Ok(Self { client, cfg })
    }
}

#[pymethods]
impl RyHttpClient {
    #[expect(clippy::too_many_arguments)]
    #[new]
    #[pyo3(
        signature = (
            *,
            headers = None,
            user_agent = None,
            timeout = None,
            read_timeout = None,
            connect_timeout = None,
            gzip = true,
            brotli = true,
            deflate = true
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        user_agent: Option<String>,
        timeout: Option<ryo3_std::PyDuration>,
        read_timeout: Option<ryo3_std::PyDuration>,
        connect_timeout: Option<ryo3_std::PyDuration>,
        gzip: Option<bool>,
        brotli: Option<bool>,
        deflate: Option<bool>,
    ) -> PyResult<Self> {
        let user_agent = parse_user_agent(user_agent)?;
        let headers = headers.map(PyHeaders::try_from).transpose()?;
        let client_cfg = ClientConfig {
            headers,
            user_agent: Some(user_agent.into()),
            timeout,
            read_timeout,
            connect_timeout,
            gzip: gzip.unwrap_or(true),
            brotli: brotli.unwrap_or(true),
            deflate: deflate.unwrap_or(true),
        };
        debug!("reqwest-client-config: {:#?}", client_cfg);
        let client_builder = client_cfg.client_builder();
        let client = client_builder
            .build()
            .map_err(|e| PyValueError::new_err(format!("{e}")))?;
        Ok(Self {
            client,
            cfg: client_cfg,
        })
    }

    fn __repr__(&self) -> String {
        format!("HttpClient<{:?}>", self.cfg)
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.cfg.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let kwargs = self.cfg.into_bound_py_any(py)?;
        Ok(kwargs)
    }

    fn __eq__(&self, other: &RyHttpClient) -> bool {
        self.cfg == other.cfg
    }

    fn __ne__(&self, other: &RyHttpClient) -> bool {
        self.cfg != other.cfg
    }

    #[pyo3(
      signature = (url, *, headers = None),
    )]
    fn get<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let mut req = self.client.get(url);
        // fing-fang-foom make de headers...
        if let Some(headers) = headers {
            req = req.headers(HeaderMap::try_from(headers)?);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
      signature = (url, *, body, headers = None),
    )]
    fn post<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: ryo3_bytes::PyBytes,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let body_bytes = body.into_inner();
        let mut req = self.client.post(url).body(body_bytes);
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
      signature = (url, *, body, headers = None),
    )]
    fn put<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: ryo3_bytes::PyBytes,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let body_bytes = body.into_inner();
        let mut req = self.client.put(url).body(body_bytes);
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
      signature = (url, *, body, headers = None),
    )]
    fn patch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: ryo3_bytes::PyBytes,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let body_bytes = body.into_inner();
        let mut req = self.client.patch(url).body(body_bytes);
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
      signature = (url, *, body=None, headers = None),
    )]
    fn delete<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let mut req = self.client.delete(url);
        if let Some(body) = body {
            let body_bytes = body.into_inner();
            req = req.body(body_bytes);
        }
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[pyo3(
      signature = (url, *, headers = None),
    )]
    fn head<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        headers: Option<PyHeadersLike>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let mut req = self.client.head(url);
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
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
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let method = method.unwrap_or(ryo3_http::HttpMethod(reqwest::Method::GET));
        let url = extract_url(url)?;
        let mut req = self.client.request(method.0, url);
        if let Some(body) = body {
            let body_bytes = body.into_inner();
            req = req.body(body_bytes);
        }
        if let Some(headers) = headers {
            let mut default_headers = HeaderMap::new();
            for (k, v) in headers {
                let k = k.to_string();
                let v = v.to_string();
                let header_name = reqwest::header::HeaderName::from_bytes(k.as_bytes())
                    .map_err(|e| PyValueError::new_err(format!("header-name-error: {e}")))?;
                let header_value = HeaderValue::from_str(&v)
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
