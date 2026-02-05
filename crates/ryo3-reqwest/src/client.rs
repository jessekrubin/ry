use crate::errors::map_reqwest_err;
use crate::request::{BlockingReqwestKwargs, ReqwestKwargs};
#[cfg(feature = "experimental-async")]
use crate::response::RyAsyncResponse;
use crate::response::RyBlockingResponse;
use crate::{ClientConfig, RyResponse};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use reqwest::{Method, RequestBuilder};
use ryo3_http::PyHttpMethod;
use ryo3_url::UrlLike;

//============================================================================

//============================================================================

#[derive(Debug, Clone)]
#[pyclass(name = "HttpClient", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyHttpClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

#[cfg(feature = "experimental-async")]
#[derive(Debug, Clone)]
#[pyclass(name = "Client", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

#[derive(Debug, Clone)]
#[pyclass(name = "BlockingClient", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyBlockingClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

impl RyHttpClient {
    #[inline]
    pub fn new(cfg: Option<ClientConfig>) -> PyResult<Self> {
        let cfg = cfg.unwrap_or_default();
        let client_builder = cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
        Ok(Self { client, cfg })
    }

    #[inline]
    fn send_sync(req: RequestBuilder) -> PyResult<RyBlockingResponse> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            req.send()
                .await
                .map(RyBlockingResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[inline]
    fn request_builder(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RequestBuilder> {
        // we can avoid the weird little hackyh query serde song and dance
        // TODO: FIX THIS?
        // Cases are:
        //    - query for the url is set from the UrlLike and query in kwargs is None -- we are done
        //    - query in kwargs is Some -- and the url already has a query -- here we do the song and dance
        //    - query in kwargs is Some -- and the url has NO query so we can just set the string I think
        // url is empty and the kwargs do not contain a
        let url = url.0;
        if let Some(kwargs) = kwargs {
            kwargs.apply(self.client.request(method, url))
        } else {
            Ok(self.client.request(method, url))
        }
    }

    #[inline]
    fn blocking_request_builder(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RequestBuilder> {
        // we can avoid the weird little hackyh query serde song and dance
        // TODO: FIX THIS?
        // Cases are:
        //    - query for the url is set from the UrlLike and query in kwargs is None -- we are done
        //    - query in kwargs is Some -- and the url already has a query -- here we do the song and dance
        //    - query in kwargs is Some -- and the url has NO query so we can just set the string I think
        // url is empty and the kwargs do not contain a
        let url = url.0;
        if let Some(kwargs) = kwargs {
            kwargs.apply(self.client.request(method, url))
        } else {
            Ok(self.client.request(method, url))
        }
    }

    #[inline]
    fn request<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rb = self.request_builder(url, method, kwargs)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            rb.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[inline]
    fn request_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        let rb = self.blocking_request_builder(url, method, kwargs)?;
        Self::send_sync(rb)
    }
}

#[cfg(feature = "experimental-async")]
impl RyClient {
    #[inline]
    pub fn new(cfg: Option<ClientConfig>) -> PyResult<Self> {
        let cfg = cfg.unwrap_or_default();
        let client_builder = cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
        Ok(Self { client, cfg })
    }

    #[inline]
    fn send_sync(req: RequestBuilder) -> PyResult<RyBlockingResponse> {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async {
            req.send()
                .await
                .map(RyBlockingResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    /// Return the reqwest builder instance...
    #[inline]
    fn request_builder(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RequestBuilder> {
        // we can avoid the weird little hackyh query serde song and dance
        // TODO: FIX THIS?
        // Cases are:
        //    - query for the url is set from the UrlLike and query in kwargs is None -- we are done
        //    - query in kwargs is Some -- and the url already has a query -- here we do the song and dance
        //    - query in kwargs is Some -- and the url has NO query so we can just set the string I think
        // url is empty and the kwargs do not contain a
        let url = url.0;
        if let Some(kwargs) = kwargs {
            kwargs.apply(self.client.request(method, url))
        } else {
            Ok(self.client.request(method, url))
        }
    }

    #[inline]
    fn request_builder_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RequestBuilder> {
        let url = url.0;
        if let Some(kwargs) = kwargs {
            kwargs.apply(self.client.request(method, url))
        } else {
            Ok(self.client.request(method, url))
        }
    }

    #[inline]
    async fn request(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        use ryo3_macro_rules::py_runtime_error;

        let req = self.request_builder(url, method, kwargs)?;

        let rt = pyo3_async_runtimes::tokio::get_runtime();
        let r = rt
            .spawn(async move { req.send().await })
            .await
            .map_err(|e| py_runtime_error!("Join error: {e}"))?
            .map(RyAsyncResponse::from)
            .map_err(crate::RyReqwestError::from)?;
        Ok(r)
    }

    #[inline]
    fn request_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        let req = self.request_builder_sync(url, method, kwargs)?;
        Self::send_sync(req)
    }
}

impl RyBlockingClient {
    #[inline]
    pub fn new(cfg: Option<ClientConfig>) -> PyResult<Self> {
        let cfg = cfg.unwrap_or_default();
        let client_builder = cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
        Ok(Self { client, cfg })
    }

    #[inline]
    fn send_sync(req: RequestBuilder) -> PyResult<RyBlockingResponse> {
        let a = pyo3_async_runtimes::tokio::get_runtime().block_on(async { req.send().await });
        a.map(RyBlockingResponse::from).map_err(map_reqwest_err)
    }

    #[inline]
    fn request_builder_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RequestBuilder> {
        let url = url.0;
        if let Some(kwargs) = kwargs {
            kwargs.apply(self.client.request(method, url))
        } else {
            Ok(self.client.request(method, url))
        }
    }

    #[inline]
    fn request_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        self.request_builder_sync(url, method, kwargs)
            .map(Self::send_sync)?
    }
}

#[pymethods]
impl RyHttpClient {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn py_new(py: Python<'_>, kwargs: Option<ClientConfig>) -> PyResult<Self> {
        let client_cfg = kwargs.unwrap_or_default();
        let client = py
            .detach(|| client_cfg.client_builder().build())
            .map_err(map_reqwest_err)?;
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

    fn config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.cfg.into_pyobject(py)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.cfg == other.cfg
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.cfg != other.cfg
    }

    #[pyo3(signature = (url, **kwargs))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::GET, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn post<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::POST, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn put<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::PUT, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::DELETE, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn head<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::HEAD, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn options<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::OPTIONS, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn patch<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, Method::PATCH, kwargs)
    }

    #[pyo3(
        signature = (url, *, method=PyHttpMethod::GET, **kwargs)
    )]
    pub(crate) fn fetch<'py>(
        &'py self,
        py: Python<'py>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, method.into(), kwargs)
    }

    #[pyo3(
        signature = (url, *, method=PyHttpMethod::GET, **kwargs)
    )]
    fn __call__<'py>(
        &'py self,
        py: Python<'py>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.request(py, url, method.into(), kwargs)
    }

    #[pyo3(
        signature = (url, *, method=PyHttpMethod::GET, **kwargs)
    )]
    pub(crate) fn fetch_sync(
        &self,
        py: Python<'_>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, method.into(), kwargs))
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyClient {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn py_new(py: Python<'_>, kwargs: Option<ClientConfig>) -> PyResult<Self> {
        let client_cfg = kwargs.unwrap_or_default();
        let client = py
            .detach(|| client_cfg.client_builder().build())
            .map_err(map_reqwest_err)?;
        Ok(Self {
            client,
            cfg: client_cfg,
        })
    }

    fn __repr__(&self) -> String {
        format!("Client<{:?}>", self.cfg)
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.cfg.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.cfg.into_pyobject(py)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.cfg == other.cfg
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.cfg != other.cfg
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn get(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::GET, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn post(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::POST, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn put(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::PUT, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn patch(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::PATCH, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn delete(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::DELETE, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn head(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::HEAD, kwargs).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn options(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        self.request(url, Method::OPTIONS, kwargs).await
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    pub(crate) async fn fetch(
        &self,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        self.request(url, method.into(), kwargs).await
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    async fn __call__(
        &self,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<RyAsyncResponse> {
        self.request(url, method.into(), kwargs).await
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    pub(crate) fn fetch_sync(
        &self,
        py: Python<'_>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs<true>>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, method.into(), kwargs))
    }
}

#[pymethods]
impl RyBlockingClient {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn py_new(py: Python<'_>, kwargs: Option<ClientConfig>) -> PyResult<Self> {
        let client_cfg = kwargs.unwrap_or_default();
        let client = py
            .detach(|| client_cfg.client_builder().build())
            .map_err(map_reqwest_err)?;
        Ok(Self {
            client,
            cfg: client_cfg,
        })
    }

    fn __repr__(&self) -> String {
        format!("BlockingClient<{:?}>", self.cfg)
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.cfg.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn config<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.cfg.into_pyobject(py)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.cfg == other.cfg
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.cfg != other.cfg
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn get(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::GET, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn post(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::POST, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn put(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::PUT, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn patch(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::PATCH, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn delete(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::DELETE, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn head(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::HEAD, kwargs))
    }

    #[pyo3(signature = (url, **kwargs))]
    pub(crate) fn options(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::OPTIONS, kwargs))
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    pub(crate) fn fetch(
        &self,
        py: Python<'_>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, method.into(), kwargs))
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    pub(crate) fn __call__(
        &self,
        py: Python<'_>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, method.into(), kwargs))
    }
}

// ============================================================================
// PURGATORY
// ============================================================================
// ---- OLD CONSTRUCTOR FOR CLIENT(s) FOR REF ----
// ```
//    #[expect(
//        clippy::fn_params_excessive_bools,
//        clippy::similar_names,
//        clippy::too_many_arguments
//    )]
//    #[new]
//    #[pyo3(
//        signature = (
//            *,
//            headers = None,
//            cookies = false,
//            user_agent = None,
//            timeout = None,
//            read_timeout = None,
//            connect_timeout = None,
//            redirect = Some(10),
//            resolve = None,
//            referer = true,
//            identity = None,
//            connection_verbose = false,
//
//            gzip = true,
//            brotli = true,
//            deflate = true,
//            zstd = true,
//            hickory_dns = true,
//
//            http1_only = false,
//            https_only = false,
//
//            http1_title_case_headers = false,
//            http1_allow_obsolete_multiline_headers_in_responses = false,
//            http1_allow_spaces_after_header_name_in_responses = false,
//            http1_ignore_invalid_headers_in_responses = false,
//
//            http2_prior_knowledge = false,
//            http2_initial_stream_window_size = None,
//            http2_initial_connection_window_size = None,
//            http2_adaptive_window = false,
//            http2_max_frame_size = None,
//            http2_max_header_list_size = None,
//            http2_keep_alive_interval = None,
//            http2_keep_alive_timeout = None,
//            http2_keep_alive_while_idle = false,
//
//            pool_idle_timeout = Some(PyDuration::from_secs(90)),
//            pool_max_idle_per_host = usize::MAX,
//
//            tcp_keepalive = Some(PyDuration::from_secs(15)),
//            tcp_keepalive_interval = Some(PyDuration::from_secs(15)),
//            tcp_keepalive_retries = Some(3),
//            tcp_nodelay = true,
//
//            tls_certs_merge = None,
//            tls_certs_only = None,
//            tls_crls_only = None,
//            tls_info = false,
//            tls_sni = true,
//            tls_version_max = None,
//            tls_version_min = None,
//            tls_danger_accept_invalid_certs = false,
//            tls_danger_accept_invalid_hostnames = false,
//            proxy = None,
//            _tls_cached_native_certs = false,
//        )
//    )]
//    fn py_new(
//        headers: Option<PyHeadersLike>,
//        cookies: bool,
//        user_agent: Option<String>,
//        timeout: Option<PyDuration>,
//        read_timeout: Option<PyDuration>,
//        connect_timeout: Option<PyDuration>,
//        redirect: Option<usize>,
//        resolve: Option<PyResolveMap>,
//        referer: bool,
//        identity: Option<PyIdentity>,
//        connection_verbose: bool,
//
//        gzip: bool,
//        brotli: bool,
//        deflate: bool,
//        zstd: bool,
//        hickory_dns: bool,
//        http1_only: bool,
//        https_only: bool,
//
//        // -- http1 --
//        http1_title_case_headers: bool,
//        http1_allow_obsolete_multiline_headers_in_responses: bool,
//        http1_allow_spaces_after_header_name_in_responses: bool,
//        http1_ignore_invalid_headers_in_responses: bool,
//
//        // -- http2 --
//        http2_prior_knowledge: bool,
//        http2_initial_stream_window_size: Option<u32>,
//        http2_initial_connection_window_size: Option<u32>,
//        http2_adaptive_window: bool,
//        http2_max_frame_size: Option<u32>,
//        http2_max_header_list_size: Option<u32>,
//        http2_keep_alive_interval: Option<PyDuration>,
//        http2_keep_alive_timeout: Option<PyDuration>,
//        http2_keep_alive_while_idle: bool,
//
//        // -- pool --
//        pool_idle_timeout: Option<PyDuration>,
//        pool_max_idle_per_host: usize,
//
//        // -- tcp --
//        tcp_keepalive: Option<PyDuration>,
//        tcp_keepalive_interval: Option<PyDuration>,
//        tcp_keepalive_retries: Option<u32>,
//        tcp_nodelay: bool,
//
//        // -- tls --
//        tls_certs_merge: Option<Vec<PyCertificate>>,
//        tls_certs_only: Option<Vec<PyCertificate>>,
//        tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
//        tls_info: bool,
//        tls_sni: bool,
//        tls_version_max: Option<TlsVersion>,
//        tls_version_min: Option<TlsVersion>,
//        tls_danger_accept_invalid_certs: bool,
//        tls_danger_accept_invalid_hostnames: bool,
//        proxy: Option<PyProxies>,
//        _tls_cached_native_certs: bool,
//    ) -> PyResult<Self> {
//        let user_agent = parse_user_agent(user_agent)?;
//        let headers = headers.map(PyHeaders::try_from).transpose()?;
//        let client_cfg = ClientConfig {
//            headers,
//            cookies,
//            user_agent: Some(user_agent.into()),
//            timeout,
//            read_timeout,
//            connect_timeout,
//            redirect,
//            resolve,
//            referer,
//            connection_verbose,
//            gzip,
//            brotli,
//            deflate,
//            zstd,
//            hickory_dns,
//            http1_only,
//            https_only,
//            // -- http1 --
//            http1_title_case_headers,
//            http1_allow_obsolete_multiline_headers_in_responses,
//            http1_allow_spaces_after_header_name_in_responses,
//            http1_ignore_invalid_headers_in_responses,
//            // -- http2 --
//            http2_prior_knowledge,
//            http2_initial_stream_window_size,
//            http2_initial_connection_window_size,
//            http2_adaptive_window,
//            http2_max_frame_size,
//            http2_max_header_list_size,
//            http2_keep_alive_interval,
//            http2_keep_alive_timeout,
//            http2_keep_alive_while_idle,
//            // --- pool ---
//            pool_idle_timeout,
//            pool_max_idle_per_host,
//            // --- tcp ---
//            tcp_keepalive,
//            tcp_keepalive_interval,
//            tcp_keepalive_retries,
//            tcp_nodelay,
//            // --- TLS ---
//            identity,
//            tls_certs_merge,
//            tls_certs_only,
//            tls_crls_only,
//            tls_info,
//            tls_sni,
//            tls_version_max,
//            tls_version_min,
//            tls_danger_accept_invalid_certs,
//            tls_danger_accept_invalid_hostnames,
//            proxy,
//            #[expect(clippy::used_underscore_binding)]
//            _tls_cached_native_certs,
//        };
//        let client_builder = client_cfg.client_builder();
//        let client = client_builder.build().map_err(map_reqwest_err)?;
//        Ok(Self {
//            client,
//            cfg: client_cfg,
//        })
//    }
// ```
