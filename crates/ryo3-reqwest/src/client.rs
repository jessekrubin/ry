use std::time::Duration;

use crate::RyResponse;
use crate::errors::map_reqwest_err;
use crate::proxy::PyProxies;
#[cfg(feature = "experimental-async")]
use crate::response::RyAsyncResponse;
use crate::response::RyBlockingResponse;
use crate::tls::{PyCertificate, PyCertificateRevocationList, PyIdentity};
use crate::tls_version::TlsVersion;
use crate::user_agent::{DEFAULT_USER_AGENT, parse_user_agent};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, RequestBuilder};
use ryo3_http::{
    HttpMethod as PyHttpMethod, HttpVersion as PyHttpVersion, PyHeaders, PyHeadersLike,
};
use ryo3_macro_rules::py_value_error;
use ryo3_macro_rules::{py_type_err, py_value_err, pytodo};
use ryo3_std::time::PyDuration;
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

#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::struct_excessive_bools)]
pub struct ClientConfig {
    headers: Option<PyHeaders>,
    cookies: bool,
    user_agent: Option<ryo3_http::HttpHeaderValue>,
    hickory_dns: bool,
    redirect: Option<usize>,
    resolve: Option<PyResolveMap>,
    // misspelled of course :/
    referer: bool,
    // -- http preferences --
    http1_only: bool,
    https_only: bool,
    // -- http1 --
    http1_title_case_headers: bool,
    http1_allow_obsolete_multiline_headers_in_responses: bool,
    http1_allow_spaces_after_header_name_in_responses: bool,
    http1_ignore_invalid_headers_in_responses: bool,
    // -- http2 --
    http2_prior_knowledge: bool,
    http2_initial_stream_window_size: Option<u32>,
    http2_initial_connection_window_size: Option<u32>,
    http2_adaptive_window: bool,
    http2_max_frame_size: Option<u32>,
    http2_max_header_list_size: Option<u32>,
    http2_keep_alive_interval: Option<PyDuration>,
    http2_keep_alive_timeout: Option<PyDuration>,
    http2_keep_alive_while_idle: bool,
    // -- timeout(s) --
    timeout: Option<PyDuration>,
    read_timeout: Option<PyDuration>,
    connect_timeout: Option<PyDuration>,
    // -- compression --
    gzip: bool,
    brotli: bool,
    deflate: bool,
    zstd: bool,
    // -- pool --
    pool_max_idle_per_host: usize,
    pool_idle_timeout: Option<PyDuration>,
    // -- tcp --
    tcp_keepalive: Option<PyDuration>, // default: 15 seconds
    tcp_keepalive_interval: Option<PyDuration>, // default: 15 seconds
    tcp_keepalive_retries: Option<u32>, // default: 3
    tcp_nodelay: bool,                 // default: true
    // -- tls --
    identity: Option<PyIdentity>,
    tls_certs_merge: Option<Vec<PyCertificate>>,
    tls_certs_only: Option<Vec<PyCertificate>>,
    tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
    tls_info: bool, // default: false
    tls_sni: bool,  // default: true
    tls_version_max: Option<TlsVersion>,
    tls_version_min: Option<TlsVersion>,
    // -- tls danger zone --
    tls_danger_accept_invalid_certs: bool,
    tls_danger_accept_invalid_hostnames: bool,
    proxy: Option<PyProxies>,
    // == CLIENT BUILDER OPTIONS TODO ==
    // connector_layer
    // cookie_provider
    // cookie_store
    // dns_resolver
    // dns_resolver2
    // http09_responses
    // interface
    // local_address
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            headers: None,
            cookies: false,
            user_agent: Some(HeaderValue::from_static(DEFAULT_USER_AGENT).into()),
            hickory_dns: true,
            redirect: Some(10),
            resolve: None,
            referer: true,
            // compression
            gzip: true,
            brotli: true,
            deflate: true,
            zstd: true,
            // http prefs
            http1_only: false,
            https_only: false,
            // http/1.x
            http1_title_case_headers: false,
            http1_allow_obsolete_multiline_headers_in_responses: false,
            http1_allow_spaces_after_header_name_in_responses: false,
            http1_ignore_invalid_headers_in_responses: false,
            // http/2
            http2_prior_knowledge: false,
            http2_initial_stream_window_size: None,
            http2_initial_connection_window_size: None,
            http2_adaptive_window: false,
            http2_max_frame_size: None,
            http2_max_header_list_size: None,
            http2_keep_alive_interval: None,
            http2_keep_alive_timeout: None,
            http2_keep_alive_while_idle: false,
            // timeouts
            timeout: None,
            read_timeout: None,
            connect_timeout: None,
            // pool
            pool_max_idle_per_host: usize::MAX,
            pool_idle_timeout: Some(PyDuration::from_secs(90)),
            // tcp
            tcp_keepalive: Some(PyDuration::from_secs(15)),
            tcp_keepalive_interval: Some(PyDuration::from_secs(15)),
            tcp_keepalive_retries: Some(3),
            tcp_nodelay: true,
            // tls
            identity: None,
            tls_certs_merge: None,
            tls_certs_only: None,
            tls_crls_only: None,
            tls_info: false,
            tls_sni: true,
            tls_version_max: None,
            tls_version_min: None,
            // tls-danger
            tls_danger_accept_invalid_certs: false,
            tls_danger_accept_invalid_hostnames: false,
            proxy: None,
        }
    }
}

// struct RequestKwargs<'py> {
//     url: &'py Bound<'py, PyAny>,
//     method: Method,
//     body: Option<&'py Bound<'py, PyAny>>,
//     headers: Option<PyHeadersLike>,
//     query: Option<&'py Bound<'py, PyAny>>,
//     json: Option<&'py Bound<'py, PyAny>>,
//     multipart: Option<&'py Bound<'py, PyAny>>,
//     form: Option<&'py Bound<'py, PyAny>>,
//     timeout: Option<&'py PyDuration>,
//     basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
//     bearer_auth: Option<PyBackedStr>,
//     version: Option<PyHttpVersion>,
// }

// fn client_request_builder(
//     client: &reqwest::Client,
//     options: RequestKwargs,
// ) -> PyResult<RequestBuilder> {
//     let url = options.url.extract::<UrlLike>()?.0;
//     let mut req = client.request(options.method, url);
//     if let Some((username, password)) = options.basic_auth {
//         req = req.basic_auth(username, password);
//     }
//     if let Some(token) = options.bearer_auth {
//         req = req.bearer_auth(token);
//     }
//     if let Some(ref version) = options.version {
//         req = req.version(version.0);
//     }
//     if let Some(headers) = options.headers {
//         let headers = HeaderMap::try_from(headers)?;
//         req = req.headers(headers);
//     }
//     if let Some(timeout) = options.timeout {
//         req = req.timeout(timeout.0);
//     }
//     if let Some(query) = options.query {
//         let pyser = ryo3_serde::PyAnySerializer::new(query.into(), None);
//         req = req.query(&pyser);
//     }

//     // version 2
//     match (options.body, options.json, options.form, options.multipart) {
//         (Some(_), Some(_), _, _)
//         | (Some(_), _, Some(_), _)
//         | (Some(_), _, _, Some(_))
//         | (_, Some(_), Some(_), _)
//         | (_, Some(_), _, Some(_))
//         | (_, _, Some(_), Some(_)) => {
//             return py_value_err!("body, json, form, multipart are mutually exclusive");
//         }
//         (Some(body), None, None, None) => {
//             use crate::body::PyBody;
//             let bod = body.extract::<PyBody>()?;
//             req = req.body(bod);
//         }
//         (None, Some(json), None, None) => {
//             let wrapped = ryo3_serde::PyAnySerializer::new(json.into(), None);
//             req = req.json(&wrapped);
//         }
//         (None, None, Some(form), None) => {
//             let pyser = ryo3_serde::PyAnySerializer::new(form.into(), None);
//             req = req.form(&pyser);
//         }
//         (None, None, None, Some(_multipart)) => {
//             pytodo!("multipart not implemented (yet)");
//         }
//         (None, None, None, None) => {}
//     }
//     Ok(req)
// }

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

    // TODO: replace this with custom python-y builder pattern that does not
    //       crudely wrap the reqwest::RequestBuilder
    // #[inline]
    // fn build_request<'py>(&'py self, options: RequestKwargs<'py>) -> PyResult<RequestBuilder> {
    //     client_request_builder(&self.client, options)
    // }

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
    #[expect(
        clippy::fn_params_excessive_bools,
        clippy::similar_names,
        clippy::too_many_arguments
    )]
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
            redirect = Some(10),
            resolve = None,
            referer = true,
            identity = None,

            gzip = true,
            brotli = true,
            deflate = true,
            zstd = true,
            hickory_dns = true,

            http1_only = false,
            https_only = false,

            http1_title_case_headers = false,
            http1_allow_obsolete_multiline_headers_in_responses = false,
            http1_allow_spaces_after_header_name_in_responses = false,
            http1_ignore_invalid_headers_in_responses = false,

            http2_prior_knowledge = false,
            http2_initial_stream_window_size = None,
            http2_initial_connection_window_size = None,
            http2_adaptive_window = false,
            http2_max_frame_size = None,
            http2_max_header_list_size = None,
            http2_keep_alive_interval = None,
            http2_keep_alive_timeout = None,
            http2_keep_alive_while_idle = false,

            pool_idle_timeout = Some(PyDuration::from_secs(90)),
            pool_max_idle_per_host = usize::MAX,

            tcp_keepalive = Some(PyDuration::from_secs(15)),
            tcp_keepalive_interval = Some(PyDuration::from_secs(15)),
            tcp_keepalive_retries = Some(3),
            tcp_nodelay = true,

            tls_certs_merge = None,
            tls_certs_only = None,
            tls_crls_only = None,
            tls_info = false,
            tls_sni = true,
            tls_version_max = None,
            tls_version_min = None,
            tls_danger_accept_invalid_certs = false,
            tls_danger_accept_invalid_hostnames = false,
            proxy = None,
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        cookies: bool,
        user_agent: Option<String>,
        timeout: Option<PyDuration>,
        read_timeout: Option<PyDuration>,
        connect_timeout: Option<PyDuration>,
        redirect: Option<usize>,
        resolve: Option<PyResolveMap>,
        referer: bool,
        identity: Option<PyIdentity>,

        gzip: bool,
        brotli: bool,
        deflate: bool,
        zstd: bool,
        hickory_dns: bool,
        http1_only: bool,
        https_only: bool,

        // -- http1 --
        http1_title_case_headers: bool,
        http1_allow_obsolete_multiline_headers_in_responses: bool,
        http1_allow_spaces_after_header_name_in_responses: bool,
        http1_ignore_invalid_headers_in_responses: bool,

        // -- http2 --
        http2_prior_knowledge: bool,
        http2_initial_stream_window_size: Option<u32>,
        http2_initial_connection_window_size: Option<u32>,
        http2_adaptive_window: bool,
        http2_max_frame_size: Option<u32>,
        http2_max_header_list_size: Option<u32>,
        http2_keep_alive_interval: Option<PyDuration>,
        http2_keep_alive_timeout: Option<PyDuration>,
        http2_keep_alive_while_idle: bool,

        // -- pool --
        pool_idle_timeout: Option<PyDuration>,
        pool_max_idle_per_host: usize,

        // -- tcp --
        tcp_keepalive: Option<PyDuration>,
        tcp_keepalive_interval: Option<PyDuration>,
        tcp_keepalive_retries: Option<u32>,
        tcp_nodelay: bool,

        // -- tls --
        tls_certs_merge: Option<Vec<PyCertificate>>,
        tls_certs_only: Option<Vec<PyCertificate>>,
        tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
        tls_info: bool,
        tls_sni: bool,
        tls_version_max: Option<TlsVersion>,
        tls_version_min: Option<TlsVersion>,
        tls_danger_accept_invalid_certs: bool,
        tls_danger_accept_invalid_hostnames: bool,
        proxy: Option<PyProxies>,
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
            redirect,
            resolve,
            referer,
            gzip,
            brotli,
            deflate,
            zstd,
            hickory_dns,
            http1_only,
            https_only,
            // -- http1 --
            http1_title_case_headers,
            http1_allow_obsolete_multiline_headers_in_responses,
            http1_allow_spaces_after_header_name_in_responses,
            http1_ignore_invalid_headers_in_responses,
            // -- http2 --
            http2_prior_knowledge,
            http2_initial_stream_window_size,
            http2_initial_connection_window_size,
            http2_adaptive_window,
            http2_max_frame_size,
            http2_max_header_list_size,
            http2_keep_alive_interval,
            http2_keep_alive_timeout,
            http2_keep_alive_while_idle,
            // --- pool ---
            pool_idle_timeout,
            pool_max_idle_per_host,
            // --- tcp ---
            tcp_keepalive,
            tcp_keepalive_interval,
            tcp_keepalive_retries,
            tcp_nodelay,
            // --- TLS ---
            identity,
            tls_certs_merge,
            tls_certs_only,
            tls_crls_only,
            tls_info,
            tls_sni,
            tls_version_max,
            tls_version_min,
            tls_danger_accept_invalid_certs,
            tls_danger_accept_invalid_hostnames,
            proxy,
        };
        let client_builder = client_cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
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
        self.request(py, url, method.0, kwargs)
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
        self.request(py, url, method.0, kwargs)
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
        py.detach(|| self.request_sync(url, method.0, kwargs))
    }
}

#[cfg(feature = "experimental-async")]
#[pymethods]
impl RyClient {
    #[expect(
        clippy::fn_params_excessive_bools,
        clippy::similar_names,
        clippy::too_many_arguments
    )]
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
            redirect = Some(10),
            resolve = None,
            referer = true,
            identity = None,

            gzip = true,
            brotli = true,
            deflate = true,
            zstd = true,
            hickory_dns = true,

            http1_only = false,
            https_only = false,

            http1_title_case_headers = false,
            http1_allow_obsolete_multiline_headers_in_responses = false,
            http1_allow_spaces_after_header_name_in_responses = false,
            http1_ignore_invalid_headers_in_responses = false,

            http2_prior_knowledge = false,
            http2_initial_stream_window_size = None,
            http2_initial_connection_window_size = None,
            http2_adaptive_window = false,
            http2_max_frame_size = None,
            http2_max_header_list_size = None,
            http2_keep_alive_interval = None,
            http2_keep_alive_timeout = None,
            http2_keep_alive_while_idle = false,

            pool_idle_timeout = Some(PyDuration::from_secs(90)),
            pool_max_idle_per_host = usize::MAX,

            tcp_keepalive = Some(PyDuration::from_secs(15)),
            tcp_keepalive_interval = Some(PyDuration::from_secs(15)),
            tcp_keepalive_retries = Some(3),
            tcp_nodelay = true,

            tls_certs_merge = None,
            tls_certs_only = None,
            tls_crls_only = None,
            tls_info = false,
            tls_sni = true,
            tls_version_max = None,
            tls_version_min = None,
            tls_danger_accept_invalid_certs = false,
            tls_danger_accept_invalid_hostnames = false,
            proxy = None,
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        cookies: bool,
        user_agent: Option<String>,
        timeout: Option<PyDuration>,
        read_timeout: Option<PyDuration>,
        connect_timeout: Option<PyDuration>,
        redirect: Option<usize>,
        resolve: Option<PyResolveMap>,
        referer: bool,
        identity: Option<PyIdentity>,

        gzip: bool,
        brotli: bool,
        deflate: bool,
        zstd: bool,
        hickory_dns: bool,
        http1_only: bool,
        https_only: bool,

        // -- http1 --
        http1_title_case_headers: bool,
        http1_allow_obsolete_multiline_headers_in_responses: bool,
        http1_allow_spaces_after_header_name_in_responses: bool,
        http1_ignore_invalid_headers_in_responses: bool,

        // -- http2 --
        http2_prior_knowledge: bool,
        http2_initial_stream_window_size: Option<u32>,
        http2_initial_connection_window_size: Option<u32>,
        http2_adaptive_window: bool,
        http2_max_frame_size: Option<u32>,
        http2_max_header_list_size: Option<u32>,
        http2_keep_alive_interval: Option<PyDuration>,
        http2_keep_alive_timeout: Option<PyDuration>,
        http2_keep_alive_while_idle: bool,

        // -- pool --
        pool_idle_timeout: Option<PyDuration>,
        pool_max_idle_per_host: usize,

        // -- tcp --
        tcp_keepalive: Option<PyDuration>,
        tcp_keepalive_interval: Option<PyDuration>,
        tcp_keepalive_retries: Option<u32>,
        tcp_nodelay: bool,

        // -- tls --
        tls_certs_merge: Option<Vec<PyCertificate>>,
        tls_certs_only: Option<Vec<PyCertificate>>,
        tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
        tls_info: bool,
        tls_sni: bool,
        tls_version_max: Option<TlsVersion>,
        tls_version_min: Option<TlsVersion>,
        tls_danger_accept_invalid_certs: bool,
        tls_danger_accept_invalid_hostnames: bool,
        proxy: Option<PyProxies>,
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
            redirect,
            resolve,
            referer,
            gzip,
            brotli,
            deflate,
            zstd,
            hickory_dns,
            http1_only,
            https_only,
            // -- http1 --
            http1_title_case_headers,
            http1_allow_obsolete_multiline_headers_in_responses,
            http1_allow_spaces_after_header_name_in_responses,
            http1_ignore_invalid_headers_in_responses,
            // -- http2 --
            http2_prior_knowledge,
            http2_initial_stream_window_size,
            http2_initial_connection_window_size,
            http2_adaptive_window,
            http2_max_frame_size,
            http2_max_header_list_size,
            http2_keep_alive_interval,
            http2_keep_alive_timeout,
            http2_keep_alive_while_idle,
            // --- pool ---
            pool_idle_timeout,
            pool_max_idle_per_host,
            // --- tcp ---
            tcp_keepalive,
            tcp_keepalive_interval,
            tcp_keepalive_retries,
            tcp_nodelay,
            // --- TLS ---
            identity,
            tls_certs_merge,
            tls_certs_only,
            tls_crls_only,
            tls_info,
            tls_sni,
            tls_version_max,
            tls_version_min,
            tls_danger_accept_invalid_certs,
            tls_danger_accept_invalid_hostnames,
            proxy,
        };
        let client_builder = client_cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
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
    #[expect(
        clippy::fn_params_excessive_bools,
        clippy::similar_names,
        clippy::too_many_arguments
    )]
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
            redirect = Some(10),
            resolve = None,
            referer = true,
            identity = None,

            gzip = true,
            brotli = true,
            deflate = true,
            zstd = true,
            hickory_dns = true,

            http1_only = false,
            https_only = false,

            http1_title_case_headers = false,
            http1_allow_obsolete_multiline_headers_in_responses = false,
            http1_allow_spaces_after_header_name_in_responses = false,
            http1_ignore_invalid_headers_in_responses = false,

            http2_prior_knowledge = false,
            http2_initial_stream_window_size = None,
            http2_initial_connection_window_size = None,
            http2_adaptive_window = false,
            http2_max_frame_size = None,
            http2_max_header_list_size = None,
            http2_keep_alive_interval = None,
            http2_keep_alive_timeout = None,
            http2_keep_alive_while_idle = false,

            pool_idle_timeout = Some(PyDuration::from_secs(90)),
            pool_max_idle_per_host = usize::MAX,

            tcp_keepalive = Some(PyDuration::from_secs(15)),
            tcp_keepalive_interval = Some(PyDuration::from_secs(15)),
            tcp_keepalive_retries = Some(3),
            tcp_nodelay = true,

            tls_certs_merge = None,
            tls_certs_only = None,
            tls_crls_only = None,
            tls_info = false,
            tls_sni = true,
            tls_version_max = None,
            tls_version_min = None,
            tls_danger_accept_invalid_certs = false,
            tls_danger_accept_invalid_hostnames = false,
            proxy = None,
        )
    )]
    fn py_new(
        headers: Option<PyHeadersLike>,
        cookies: bool,
        user_agent: Option<String>,
        timeout: Option<PyDuration>,
        read_timeout: Option<PyDuration>,
        connect_timeout: Option<PyDuration>,
        redirect: Option<usize>,
        resolve: Option<PyResolveMap>,
        referer: bool,
        identity: Option<PyIdentity>,

        gzip: bool,
        brotli: bool,
        deflate: bool,
        zstd: bool,
        hickory_dns: bool,
        http1_only: bool,
        https_only: bool,

        // -- http1 --
        http1_title_case_headers: bool,
        http1_allow_obsolete_multiline_headers_in_responses: bool,
        http1_allow_spaces_after_header_name_in_responses: bool,
        http1_ignore_invalid_headers_in_responses: bool,

        // -- http2 --
        http2_prior_knowledge: bool,
        http2_initial_stream_window_size: Option<u32>,
        http2_initial_connection_window_size: Option<u32>,
        http2_adaptive_window: bool,
        http2_max_frame_size: Option<u32>,
        http2_max_header_list_size: Option<u32>,
        http2_keep_alive_interval: Option<PyDuration>,
        http2_keep_alive_timeout: Option<PyDuration>,
        http2_keep_alive_while_idle: bool,

        // -- pool --
        pool_idle_timeout: Option<PyDuration>,
        pool_max_idle_per_host: usize,

        // -- tcp --
        tcp_keepalive: Option<PyDuration>,
        tcp_keepalive_interval: Option<PyDuration>,
        tcp_keepalive_retries: Option<u32>,
        tcp_nodelay: bool,

        // -- tls --
        tls_certs_merge: Option<Vec<PyCertificate>>,
        tls_certs_only: Option<Vec<PyCertificate>>,
        tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
        tls_info: bool,
        tls_sni: bool,
        tls_version_max: Option<TlsVersion>,
        tls_version_min: Option<TlsVersion>,
        tls_danger_accept_invalid_certs: bool,
        tls_danger_accept_invalid_hostnames: bool,
        proxy: Option<PyProxies>,
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
            redirect,
            resolve,
            referer,
            gzip,
            brotli,
            deflate,
            zstd,
            hickory_dns,
            http1_only,
            https_only,
            // -- http1 --
            http1_title_case_headers,
            http1_allow_obsolete_multiline_headers_in_responses,
            http1_allow_spaces_after_header_name_in_responses,
            http1_ignore_invalid_headers_in_responses,
            // -- http2 --
            http2_prior_knowledge,
            http2_initial_stream_window_size,
            http2_initial_connection_window_size,
            http2_adaptive_window,
            http2_max_frame_size,
            http2_max_header_list_size,
            http2_keep_alive_interval,
            http2_keep_alive_timeout,
            http2_keep_alive_while_idle,
            // --- pool ---
            pool_idle_timeout,
            pool_max_idle_per_host,
            // --- tcp ---
            tcp_keepalive,
            tcp_keepalive_interval,
            tcp_keepalive_retries,
            tcp_nodelay,
            // --- TLS ---
            identity,
            tls_certs_merge,
            tls_certs_only,
            tls_crls_only,
            tls_info,
            tls_sni,
            tls_version_max,
            tls_version_min,
            tls_danger_accept_invalid_certs,
            tls_danger_accept_invalid_hostnames,
            proxy,
        };
        let client_builder = client_cfg.client_builder();
        let client = client_builder.build().map_err(map_reqwest_err)?;
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

impl<'py> IntoPyObject<'py> for &ClientConfig {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.as_pydict(py)
    }
}

impl ClientConfig {
    #[inline]
    fn apply_http2_opts(
        &self,
        mut client_builder: reqwest::ClientBuilder,
    ) -> reqwest::ClientBuilder {
        if self.http2_prior_knowledge {
            client_builder = client_builder.http2_prior_knowledge();
        }
        if let Some(http2_initial_stream_window_size) = self.http2_initial_stream_window_size {
            client_builder =
                client_builder.http2_initial_stream_window_size(http2_initial_stream_window_size);
        }
        if let Some(http2_initial_connection_window_size) =
            self.http2_initial_connection_window_size
        {
            client_builder = client_builder
                .http2_initial_connection_window_size(http2_initial_connection_window_size);
        }
        if self.http2_adaptive_window {
            client_builder = client_builder.http2_adaptive_window(true);
        }
        if let Some(http2_max_frame_size) = self.http2_max_frame_size {
            client_builder = client_builder.http2_max_frame_size(http2_max_frame_size);
        }
        if let Some(http2_max_header_list_size) = self.http2_max_header_list_size {
            client_builder = client_builder.http2_max_header_list_size(http2_max_header_list_size);
        }
        if let Some(http2_keep_alive_interval) = &self.http2_keep_alive_interval {
            client_builder = client_builder.http2_keep_alive_interval(http2_keep_alive_interval.0);
        }
        if let Some(http2_keep_alive_timeout) = &self.http2_keep_alive_timeout {
            client_builder = client_builder.http2_keep_alive_timeout(http2_keep_alive_timeout.0);
        }
        if self.http2_keep_alive_while_idle {
            client_builder = client_builder.http2_keep_alive_while_idle(true);
        }
        client_builder
    }

    #[inline]
    fn apply_tls_opts(&self, mut client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        if self.tls_certs_only.is_none() {
            client_builder =
                client_builder.tls_certs_only(crate::tls::py_load_native_certs().iter().cloned());
        }
        if let Some(tls_certs_merge) = &self.tls_certs_merge {
            client_builder = client_builder
                .tls_certs_merge(tls_certs_merge.iter().map(|py_cert| py_cert.cert.clone()));
        }
        if let Some(tls_crls_only) = &self.tls_crls_only {
            client_builder = client_builder
                .tls_crls_only(tls_crls_only.clone().into_iter().map(|py_crl| py_crl.crl));
        }

        if let Some(tls_version_min) = &self.tls_version_min {
            client_builder = client_builder.tls_version_min(tls_version_min.into());
        }
        if let Some(tls_version_max) = &self.tls_version_max {
            client_builder = client_builder.tls_version_max(tls_version_max.into());
        }
        client_builder
    }

    #[inline]
    fn apply(&self, client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let mut client_builder = client_builder
            .gzip(self.gzip)
            .brotli(self.brotli)
            .deflate(self.deflate)
            .zstd(self.zstd)
            .cookie_store(self.cookies)
            .hickory_dns(self.hickory_dns)
            .referer(self.referer)
            .redirect(
                self.redirect
                    .map_or_else(reqwest::redirect::Policy::none, |max| {
                        reqwest::redirect::Policy::limited(max)
                    }),
            )
            .https_only(self.https_only)
            .http1_allow_obsolete_multiline_headers_in_responses(
                self.http1_allow_obsolete_multiline_headers_in_responses,
            )
            .http1_allow_spaces_after_header_name_in_responses(
                self.http1_allow_spaces_after_header_name_in_responses,
            )
            .http1_ignore_invalid_headers_in_responses(
                self.http1_ignore_invalid_headers_in_responses,
            )
            .pool_idle_timeout(self.pool_idle_timeout.map(|d| d.0))
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .tcp_keepalive(self.tcp_keepalive.map(|d| d.0))
            .tcp_keepalive_interval(self.tcp_keepalive_interval.map(|d| d.0))
            .tcp_keepalive_retries(self.tcp_keepalive_retries)
            .tcp_nodelay(self.tcp_nodelay)
            .tls_sni(self.tls_sni)
            .tls_info(self.tls_info)
            .tls_danger_accept_invalid_certs(self.tls_danger_accept_invalid_certs)
            .tls_danger_accept_invalid_hostnames(self.tls_danger_accept_invalid_hostnames);

        if let Some(user_agent) = &self.user_agent {
            client_builder = client_builder.user_agent(user_agent.clone());
        }
        if let Some(headers) = &self.headers {
            client_builder = client_builder.default_headers(headers.0.py_read().clone());
        }
        if let Some(timeout) = &self.timeout {
            client_builder = client_builder.timeout(timeout.into());
        }
        if let Some(read_timeout) = &self.read_timeout {
            client_builder = client_builder.read_timeout(read_timeout.into());
        }
        if let Some(connect_timeout) = &self.connect_timeout {
            client_builder = client_builder.connect_timeout(connect_timeout.into());
        }

        if let Some(resolve) = &self.resolve {
            for (domain, addrs) in &resolve.0 {
                client_builder = client_builder.resolve_to_addrs(domain, addrs);
            }
        }

        if let Some(proxy) = &self.proxy {
            client_builder = proxy.apply2client(client_builder);
        }

        // http1
        if self.http1_only {
            client_builder = client_builder.http1_only();
        }
        if self.http1_title_case_headers {
            client_builder = client_builder.http1_title_case_headers();
        }

        // http2
        client_builder = self.apply_http2_opts(client_builder);
        // apply_tls
        client_builder = self.apply_tls_opts(client_builder);
        client_builder
    }

    fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        macro_rules! set_item {
            ($key:expr, $value:expr) => {
                dict.set_item(intern!(py, $key), $value)?
            };
        }
        macro_rules! set_items {
            ($( $key:expr => $value:expr),* ) => {
                $(
                    set_item!($key, $value);
                )*
            };
        }
        let resolve = self.resolve.into_pyobject(py)?;

        set_items! {
            "headers" => self.headers.clone(),
            "cookies" => self.cookies,
            "user_agent" => self.user_agent.clone(),
            "timeout" => self.timeout,
            "read_timeout" => self.read_timeout,
            "connect_timeout" => self.connect_timeout,
            "redirect" => self.redirect,
            "resolve" => resolve,
            "referer" => self.referer,
            "gzip" => self.gzip,
            "brotli" => self.brotli,
            "deflate" => self.deflate,
            "zstd" => self.zstd,
            "hickory_dns" => self.hickory_dns,
            "http1_only" => self.http1_only,
            "https_only" => self.https_only,
            // -- http1 --
            "http1_title_case_headers" => self.http1_title_case_headers,
            "http1_allow_obsolete_multiline_headers_in_responses" => self.http1_allow_obsolete_multiline_headers_in_responses,
            "http1_allow_spaces_after_header_name_in_responses" => self.http1_allow_spaces_after_header_name_in_responses,
            "http1_ignore_invalid_headers_in_responses" => self.http1_ignore_invalid_headers_in_responses,
            // -- http2 --
            "http2_prior_knowledge" => self.http2_prior_knowledge,
            "http2_initial_stream_window_size" => self.http2_initial_stream_window_size,
            "http2_initial_connection_window_size" => self.http2_initial_connection_window_size,
            "http2_adaptive_window" => self.http2_adaptive_window,
            "http2_max_frame_size" => self.http2_max_frame_size,
            "http2_max_header_list_size" => self.http2_max_header_list_size,
            "http2_keep_alive_interval" => self.http2_keep_alive_interval,
            "http2_keep_alive_timeout" => self.http2_keep_alive_timeout,
            "http2_keep_alive_while_idle" => self.http2_keep_alive_while_idle,
            // -- pool --
            "pool_idle_timeout" => self.pool_idle_timeout,
            "pool_max_idle_per_host" => self.pool_max_idle_per_host,
            // -- tcp --
            "tcp_keepalive" => self.tcp_keepalive,
            "tcp_keepalive_interval" => self.tcp_keepalive_interval,
            "tcp_keepalive_retries" => self.tcp_keepalive_retries,
            "tcp_nodelay" => self.tcp_nodelay,
            // -- tls --
            "identity" => self.identity.clone(),
            "tls_crls_only" => self.tls_crls_only.clone(),
            "tls_certs_merge" => self.tls_certs_merge.clone(),
            "tls_certs_only" => self.tls_certs_only.clone(),
            "tls_info" => self.tls_info,
            "tls_sni" => self.tls_sni,
            "tls_version_max" => self.tls_version_max,
            "tls_version_min" => self.tls_version_min,
            "tls_danger_accept_invalid_certs" => self.tls_danger_accept_invalid_certs,
            "tls_danger_accept_invalid_hostnames" => self.tls_danger_accept_invalid_hostnames,
            "proxy" => &self.proxy
        }
        Ok(dict)
    }

    #[inline]
    fn client_builder(&self) -> reqwest::ClientBuilder {
        let client_builder = reqwest::Client::builder();
        self.apply(client_builder)
    }
}

// maybe dont actually need this?

struct BasicAuth(PyBackedStr, Option<PyBackedStr>);

impl<'py> FromPyObject<'_, 'py> for BasicAuth {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let tuple: (PyBackedStr, Option<PyBackedStr>) = obj.extract()?;
        Ok(Self(tuple.0, tuple.1))
    }
}

pub(crate) struct ReqwestKwargs<const BLOCKING: bool = false> {
    headers: Option<HeaderMap>,
    query: Option<String>,
    body: PyReqwestBody,
    timeout: Option<Duration>,
    basic_auth: Option<BasicAuth>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<PyHttpVersion>,
}

pub(crate) type BlockingReqwestKwargs = ReqwestKwargs<true>;

impl<const BLOCKING: bool> ReqwestKwargs<BLOCKING> {
    /// Apply the kwargs to the `reqwest::RequestBuilder`
    #[inline]
    fn apply(self, req: reqwest::RequestBuilder) -> PyResult<reqwest::RequestBuilder> {
        let mut req = req;

        // headers
        if let Some(headers) = self.headers {
            req = req.headers(headers);
        }

        // query
        if let Some(query) = self.query {
            // temp hack we know that the query is already url-encoded so we
            // decode it and then re-encode it...
            let decoded: Vec<(&str, &str)> = serde_urlencoded::from_str(&query)
                .map_err(|err| py_value_error!("failed to decode query params: {err}"))?;
            req = req.query(&decoded);
        }

        // body
        req = match self.body {
            PyReqwestBody::Bytes(b) => req.body(b),
            PyReqwestBody::Stream(s) => req.body(s),
            PyReqwestBody::Json(j) => req.body(j).header(
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            ),
            PyReqwestBody::Form(f) => req.body(f).header(
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            ),
            PyReqwestBody::Multipart(_m) => {
                pytodo!("multipart not implemented (yet)");
            }
            PyReqwestBody::None => req,
        };

        // timeout
        if let Some(timeout) = self.timeout {
            req = req.timeout(timeout);
        }

        // basic auth
        if let Some(BasicAuth(username, password)) = self.basic_auth {
            req = req.basic_auth(username, password);
        }

        // bearer auth
        if let Some(token) = self.bearer_auth {
            req = req.bearer_auth(token);
        }

        // version
        if let Some(version) = self.version {
            req = req.version(version.into());
        }

        Ok(req)
    }
}

#[derive(Debug)]
enum PyReqwestBody {
    Bytes(bytes::Bytes),
    Stream(crate::body::PyBodyStream),
    Json(Vec<u8>),
    Form(String),
    #[allow(dead_code)]
    Multipart(bool), // placeholder
    None,
}

impl<'py, const BLOCKING: bool> FromPyObject<'_, 'py> for ReqwestKwargs<BLOCKING> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        let dict = obj.cast_exact::<PyDict>()?;

        // body parts...
        let body = dict.get_item(intern!(py, "body"))?;
        let json = dict.get_item(intern!(py, "json"))?;
        let form = dict.get_item(intern!(py, "form"))?;
        let multipart = dict.get_item(intern!(py, "multipart"))?;

        // let query: PyResult<Option<String>> =
        let query: Option<String> = dict.get_item(intern!(py, "query")).map(|e| {
            if let Some(q) = e {
                let py_any_serializer = ryo3_serde::PyAnySerializer::new(q.as_borrowed(), None);
                let url_encoded_query = serde_urlencoded::to_string(py_any_serializer)
                    .map_err(|err| py_value_error!("failed to serialize query params: {err}"))?;
                // have to annotate the err type...
                Ok::<_, PyErr>(Some(url_encoded_query))
            } else {
                Ok(None)
            }
        })??;

        let body: PyReqwestBody = match (body, json, form, multipart) {
            (Some(_), Some(_), _, _)
            | (Some(_), _, Some(_), _)
            | (Some(_), _, _, Some(_))
            | (_, Some(_), Some(_), _)
            | (_, Some(_), _, Some(_))
            | (_, _, Some(_), Some(_)) => {
                return py_value_err!("body, json, form, multipart are mutually exclusive");
            }
            (Some(body), None, None, None) => {
                let py_body = body.extract::<crate::body::PyBody>()?;
                match py_body {
                    crate::body::PyBody::Bytes(bs) => PyReqwestBody::Bytes(bs.into_inner()),
                    crate::body::PyBody::Stream(s) => {
                        // using an async stream with blocking client is a no-go (yo)
                        if BLOCKING {
                            if s.is_async() {
                                return py_type_err!(
                                    "cannot use async stream body with blocking client"
                                );
                            }
                            PyReqwestBody::Stream(s)
                        } else {
                            PyReqwestBody::Stream(s)
                        }
                    }
                }
            }
            (None, Some(json), None, None) => {
                let b = ryo3_json::to_vec(&json)?;
                PyReqwestBody::Json(b)
            }
            (None, None, Some(form), None) => {
                use ryo3_macro_rules::py_value_error;

                let py_any_serializer = ryo3_serde::PyAnySerializer::new(form.as_borrowed(), None);
                let url_encoded_form = serde_urlencoded::to_string(py_any_serializer)
                    .map_err(|e| py_value_error!("failed to serialize form data: {e}"))?;
                PyReqwestBody::Form(url_encoded_form)
            }
            (None, None, None, Some(_multipart)) => {
                pytodo!("multipart not implemented (yet)");

                // (None, None, None, Some(true))
            }
            (None, None, None, None) => PyReqwestBody::None,
        };

        let timeout = dict
            .get_item(intern!(py, "timeout"))?
            .map(|t| t.extract::<Timeout>())
            .transpose()?
            .map(|d| d.0);
        let headers = dict
            .get_item(intern!(py, "headers"))?
            .map(|h| h.extract::<PyHeadersLike>())
            .transpose()?
            .map(HeaderMap::try_from)
            .transpose()?;
        let bearer_auth: Option<PyBackedStr> = dict
            .get_item(intern!(py, "bearer_auth"))?
            .map(|b| b.extract())
            .transpose()?;
        let version: Option<PyHttpVersion> = dict
            .get_item(intern!(py, "version"))?
            .map(|v| v.extract())
            .transpose()?;
        Ok(Self {
            body,
            headers,
            query,
            timeout,
            basic_auth: dict
                .get_item(intern!(obj.py(), "basic_auth"))?
                .map(|b| b.extract())
                .transpose()?,
            bearer_auth,
            version,
        })
    }
}

struct Timeout(Duration);

impl From<Timeout> for Duration {
    fn from(t: Timeout) -> Self {
        t.0
    }
}

impl<'py> FromPyObject<'_, 'py> for Timeout {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(pydur) = obj.cast_exact::<PyDuration>() {
            Ok(Self(pydur.get().into()))
        } else if let Ok(dur) = obj.extract::<Duration>() {
            Ok(Self(dur))
        } else {
            py_type_err!("timeout must be a Duration | datetime.timedelta")
        }
    }
}

// RESOLVE-MAP
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PySocketAddrLike(std::net::SocketAddr);

impl<'py> FromPyObject<'_, 'py> for PySocketAddrLike {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(addr) = obj.cast_exact::<ryo3_std::net::PySocketAddr>() {
            Ok(Self(addr.get().into()))
        } else if let Ok(addr_str) = obj.cast_exact::<ryo3_std::net::PySocketAddrV4>() {
            Ok(Self(addr_str.get().into()))
        } else if let Ok(addr_str) = obj.cast_exact::<ryo3_std::net::PySocketAddrV6>() {
            Ok(Self(addr_str.get().into()))
        } else if let Ok(s) = obj.extract::<pyo3::pybacked::PyBackedStr>() {
            let addr: std::net::SocketAddr = s
                .parse()
                .map_err(|err| py_value_error!("failed to parse socket addr '{s}': {err}"))?;
            Ok(Self(addr))
        } else {
            py_type_err!("expected SocketAddr, SocketAddrV4, SocketAddrV6, or str")
        }
    }
}

enum PyResolveMapValue {
    Single(std::net::SocketAddr),
    Multiple(Vec<std::net::SocketAddr>),
}

impl<'py> FromPyObject<'_, 'py> for PyResolveMapValue {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(addr) = obj.extract::<PySocketAddrLike>() {
            Ok(Self::Single(addr.0))
        } else if let Ok(addr_list) = obj.extract::<Vec<PySocketAddrLike>>() {
            let mut addrs = addr_list.into_iter().map(|a| a.0).collect::<Vec<_>>();
            addrs.sort_unstable();
            addrs.dedup();

            Ok(Self::Multiple(addrs))
        } else if let Ok(addr_list) = obj.extract::<std::collections::HashSet<PySocketAddrLike>>() {
            let addrs = addr_list.into_iter().map(|a| a.0).collect();
            Ok(Self::Multiple(addrs))
        } else {
            py_type_err!("expected SocketAddr, SocketAddrV4, SocketAddrV6, str, or list of these")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PyResolveMap(Vec<(String, Vec<std::net::SocketAddr>)>);

impl<'py> FromPyObject<'_, 'py> for PyResolveMap {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dict = obj.cast_exact::<PyDict>()?;
        let mut map: std::collections::HashMap<String, Vec<std::net::SocketAddr>> =
            std::collections::HashMap::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<pyo3::pybacked::PyBackedStr>()?.to_string();
            let resolve_value = value.extract::<PyResolveMapValue>()?;
            match resolve_value {
                PyResolveMapValue::Single(addr) => {
                    map.entry(key_str).or_default().push(addr);
                }
                PyResolveMapValue::Multiple(addrs) => {
                    map.entry(key_str).or_default().extend(addrs);
                }
            }
        }
        let vecify = map
            .into_iter()
            .filter(
                |(_domain, addrs)| !addrs.is_empty(), // filter out empty addr lists
            )
            .collect::<Vec<(String, Vec<std::net::SocketAddr>)>>();
        Ok(Self(vecify))
    }
}

impl<'py> IntoPyObject<'py> for &PyResolveMap {
    type Target = PyDict;
    type Output = Bound<'py, PyDict>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dict = PyDict::new(py);
        for (key, addrs) in &self.0 {
            // map 2 PySocketAddr,
            let py_sock_addrs = addrs
                .iter()
                .map(ryo3_std::net::PySocketAddr::from)
                .collect::<Vec<_>>();
            dict.set_item(key, py_sock_addrs)?;
        }
        Ok(dict)
    }
}
