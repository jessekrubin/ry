use crate::errors::map_reqwest_err;
use crate::RyResponse;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{intern, IntoPyObjectExt};
use reqwest::header::{HeaderMap, HeaderValue};
use ryo3_http::{HttpVersion, PyHeaders, PyHeadersLike};
use ryo3_macros::err_py_not_impl;
use ryo3_url::extract_url;
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Clone)]
#[pyclass(name = "HttpClient", module = "ry.ryo3.reqwest", frozen)]
pub struct RyHttpClient {
    client: reqwest::Client,
    cfg: ClientConfig,
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
#[expect(clippy::struct_excessive_bools)]
pub struct ClientConfig {
    headers: Option<PyHeaders>,
    cookies: bool,
    user_agent: Option<ryo3_http::HttpHeaderValue>,
    timeout: Option<ryo3_std::PyDuration>,
    read_timeout: Option<ryo3_std::PyDuration>,
    connect_timeout: Option<ryo3_std::PyDuration>,
    gzip: bool,
    brotli: bool,
    deflate: bool,
    zstd: bool,
    http1_only: bool,
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
        dict.set_item(intern!(py, "zstd"), self.zstd)?;
        dict.set_item(intern!(py, "http1_only"), self.http1_only)?;
        Ok(dict)
    }
}

impl ClientConfig {
    fn apply(&self, client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let mut client_builder = client_builder
            .gzip(false)
            .brotli(true)
            .deflate(false)
            .zstd(false)
            .cookie_store(true);
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
        if self.http1_only {
            client_builder = client_builder.http1_only();
        }
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
            cookies = false,
            user_agent = None,
            timeout = None,
            read_timeout = None,
            connect_timeout = None,
            gzip = true,
            brotli = true,
            deflate = true,
            zstd = true,
            http1_only = false,
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        cookies: bool,
        user_agent: Option<String>,
        timeout: Option<ryo3_std::PyDuration>,
        read_timeout: Option<ryo3_std::PyDuration>,
        connect_timeout: Option<ryo3_std::PyDuration>,
        gzip: Option<bool>,
        brotli: Option<bool>,
        deflate: Option<bool>,
        zstd: Option<bool>,
        http1_only: Option<bool>,
    ) -> PyResult<Self> {
        let user_agent = parse_user_agent(user_agent)?;
        let headers = headers.map(PyHeaders::try_from).transpose()?;
        let client_cfg = ClientConfig {
            headers,
            cookies,
            user_agent: Some(user_agent.into()),
            timeout,
            read_timeout,
            connect_timeout,
            gzip: gzip.unwrap_or(true),
            brotli: brotli.unwrap_or(true),
            deflate: deflate.unwrap_or(true),
            zstd: zstd.unwrap_or(true),
            http1_only: http1_only.unwrap_or(false),
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
      signature = (url, *, headers = None, timeout = None, ),
    )]
    fn get<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        headers: Option<PyHeadersLike>,
        timeout: Option<&ryo3_std::PyDuration>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let url = extract_url(url)?;
        let mut req = self.client.get(url);
        // fing-fang-foom make de headers...
        if let Some(headers) = headers {
            req = req.headers(HeaderMap::try_from(headers)?);
        }
        if let Some(timeout) = timeout {
            req = req.timeout(timeout.0);
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
            headers = None,
            query = None,
            multipart = None,
            form = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    #[expect(clippy::needless_pass_by_value)]
    pub fn fetch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        timeout: Option<&ryo3_std::PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let method = method.unwrap_or(ryo3_http::HttpMethod(reqwest::Method::GET));
        let url = extract_url(url)?;

        let mut req = self.client.request(method.0, url);
        if let Some(ref version) = version {
            req = req.version(version.0);
        }
        if let Some(query) = query {
            let q = QueryLike::extract_bound(query)?;
            req = req.query(&q);
        }
        if let Some(_multipart) = multipart {
            return err_py_not_impl!("multipart not implemented (yet)");
        }
        if let Some(_form) = form {
            return err_py_not_impl!("form not implemented (yet)");
        }
        if let Some(body) = body {
            let body_bytes = body.into_inner();
            req = req.body(body_bytes);
        }
        if let Some(headers) = headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        if let Some(timeout) = timeout {
            req = req.timeout(timeout.0);
        }
        debug!("reqwest-client-fetch: {:#?}", req);
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            req.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }
}

#[derive(Serialize, FromPyObject)]
#[serde(untagged)] // I think this is the way????????? but not sure....
enum QueryValue {
    Bool(bool),
    I64(i64),
    Float(f64),
    String(String),
}
#[derive(Serialize)]
struct QueryLike(Vec<(String, QueryValue)>);

impl FromPyObject<'_> for QueryLike {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pairs) = ob.downcast::<PyDict>() {
            let mut vec = Vec::new();
            for (k, v) in pairs.iter() {
                if k.is_none() || v.is_none() {
                    continue;
                }
                let k = k.extract::<String>()?;
                let v = v.extract::<QueryValue>()?;
                vec.push((k, v));
            }
            Ok(QueryLike(vec))
        } else if let Ok(pairs) = ob.downcast::<PyTuple>() {
            let mut vec = Vec::new();
            for item in pairs.iter() {
                if item.is_none() {
                    continue;
                }
                let item = item.extract::<(String, QueryValue)>()?;
                vec.push(item);
            }
            return Ok(QueryLike(vec));
        } else if let Ok(pairs) = ob.downcast::<PyList>() {
            let mut vec = Vec::new();
            for item in pairs.iter() {
                if item.is_none() {
                    continue;
                }
                let item = item.extract::<(String, QueryValue)>()?;
                vec.push(item);
            }
            return Ok(QueryLike(vec));
        } else {
            println!("Invalid query: {ob:?}");
            Err(PyValueError::new_err("Invalid query"))
        }
    }
}
