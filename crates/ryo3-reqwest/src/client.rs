use crate::RyResponse;
use crate::errors::map_reqwest_err;
use crate::user_agent::parse_user_agent;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use reqwest::header::HeaderMap;
use reqwest::{Method, RequestBuilder};
use ryo3_http::{HttpVersion, PyHeaders, PyHeadersLike};
use ryo3_macro_rules::{py_value_err, pytodo};
use ryo3_std::time::PyDuration;
use ryo3_url::extract_url;
use tracing::debug;

#[derive(Debug, Clone)]
#[pyclass(name = "HttpClient", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyHttpClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[expect(clippy::struct_excessive_bools)]
pub struct ClientConfig {
    headers: Option<PyHeaders>,
    cookies: bool,
    user_agent: Option<ryo3_http::HttpHeaderValue>,
    timeout: Option<PyDuration>,
    read_timeout: Option<PyDuration>,
    connect_timeout: Option<PyDuration>,
    gzip: bool,
    brotli: bool,
    deflate: bool,
    zstd: bool,
    hickory_dns: bool,
    http1_only: bool,
}

struct RequestKwargs<'py> {
    url: &'py Bound<'py, PyAny>,
    method: Method,
    body: Option<ryo3_bytes::PyBytes>,
    headers: Option<PyHeadersLike>,
    query: Option<&'py Bound<'py, PyAny>>,
    json: Option<&'py Bound<'py, PyAny>>,
    multipart: Option<&'py Bound<'py, PyAny>>,
    form: Option<&'py Bound<'py, PyAny>>,
    timeout: Option<&'py PyDuration>,
    version: Option<HttpVersion>,
}

impl RyHttpClient {
    pub fn new(cfg: Option<ClientConfig>) -> PyResult<Self> {
        let cfg = cfg.unwrap_or_default();
        let client_builder = cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
        Ok(Self { client, cfg })
    }

    // TODO: replace this with custom python-y builder pattern that does not
    //       crudely wrap the reqwest::RequestBuilder
    fn build_request<'py>(&'py self, options: RequestKwargs<'py>) -> PyResult<RequestBuilder> {
        let url = extract_url(options.url)?;
        let mut req = self.client.request(options.method, url);
        if let Some(ref version) = options.version {
            req = req.version(version.0);
        }
        if let Some(headers) = options.headers {
            let headers = HeaderMap::try_from(headers)?;
            req = req.headers(headers);
        }
        if let Some(timeout) = options.timeout {
            req = req.timeout(timeout.0);
        }
        if let Some(query) = options.query {
            let pyser = ryo3_serde::SerializePyAny::new(query, None);
            req = req.query(&pyser);
        }

        // make sure only one of body, json, form, multipart is set
        if u8::from(options.body.is_some())
            + u8::from(options.json.is_some())
            + u8::from(options.form.is_some())
            + u8::from(options.multipart.is_some())
            > 1
        {
            return py_value_err!("body, json, form, multipart are mutually exclusive");
        }

        if let Some(_multipart) = options.multipart {
            pytodo!("multipart not implemented (yet)");
        }
        if let Some(json) = options.json {
            let wrapped = ryo3_serde::SerializePyAny::new(json, None);
            req = req.json(&wrapped);
        }
        if let Some(form) = options.form {
            let pyser = ryo3_serde::SerializePyAny::new(form, None);
            req = req.form(&pyser);
        }
        if let Some(body) = options.body {
            let body_bytes = body.into_inner();
            req = req.body(body_bytes);
        }
        debug!("reqwest-client-fetch: {:#?}", req);
        Ok(req)
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
            hickory_dns = true,
            http1_only = false,
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        cookies: bool,
        user_agent: Option<String>,
        timeout: Option<PyDuration>,
        read_timeout: Option<PyDuration>,
        connect_timeout: Option<PyDuration>,
        gzip: Option<bool>,
        brotli: Option<bool>,
        deflate: Option<bool>,
        zstd: Option<bool>,
        hickory_dns: Option<bool>,
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
            hickory_dns: hickory_dns.unwrap_or(true),
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

    fn __eq__(&self, other: &Self) -> bool {
        self.cfg == other.cfg
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.cfg != other.cfg
    }

    #[pyo3(
        signature = (
            url,
            *,
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn get<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::GET,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn post<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::POST,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn put<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::PUT,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn patch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::PATCH,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn delete<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::DELETE,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn head<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::HEAD,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            body = None,
            headers = None,
            query = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn options<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let opts = RequestKwargs {
            url,
            method: Method::OPTIONS,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };
        let req = self.build_request(opts)?;
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
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    pub(crate) fn fetch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let method = method.map_or_else(|| Method::GET, |m| m.0);
        let opts = RequestKwargs {
            url,
            method,
            body,
            headers,
            query,
            json,
            multipart,
            form,
            timeout,
            version,
        };

        let req = self.build_request(opts)?;
        debug!("reqwest-client-fetch: {:#?}", req);
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
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn __call__<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<ryo3_bytes::PyBytes>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.fetch(
            py, url, method, body, headers, query, json, form, multipart, timeout, version,
        )
    }
}

impl<'py> IntoPyObject<'py> for &ClientConfig {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "headers"), self.headers.clone())?;
        dict.set_item(intern!(py, "user_agent"), self.user_agent.clone())?;
        dict.set_item(intern!(py, "timeout"), self.timeout)?;
        dict.set_item(intern!(py, "read_timeout"), self.read_timeout)?;
        dict.set_item(intern!(py, "connect_timeout"), self.connect_timeout)?;
        dict.set_item(intern!(py, "gzip"), self.gzip)?;
        dict.set_item(intern!(py, "brotli"), self.brotli)?;
        dict.set_item(intern!(py, "deflate"), self.deflate)?;
        dict.set_item(intern!(py, "zstd"), self.zstd)?;
        dict.set_item(intern!(py, "http1_only"), self.http1_only)?;
        dict.set_item(intern!(py, "hickory_dns"), self.hickory_dns)?;
        Ok(dict)
    }
}

impl ClientConfig {
    fn apply(&self, client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let mut client_builder = client_builder
            .gzip(self.gzip)
            .brotli(self.brotli)
            .deflate(self.deflate)
            .zstd(self.zstd)
            .cookie_store(self.cookies)
            .hickory_dns(self.hickory_dns);
        if let Some(user_agent) = &self.user_agent {
            client_builder = client_builder.user_agent(user_agent.clone());
        }
        if let Some(headers) = &self.headers {
            client_builder = client_builder.default_headers(headers.0.lock().clone());
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
