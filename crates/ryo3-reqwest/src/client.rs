use crate::RyResponse;
use crate::errors::map_reqwest_err;
use crate::tls_version::TlsVersion;
use crate::user_agent::parse_user_agent;
use bytes::Bytes;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use reqwest::header::HeaderMap;
use reqwest::{Method, RequestBuilder};
use ryo3_http::{HttpVersion, PyHeaders, PyHeadersLike};
use ryo3_macro_rules::{py_type_err, py_value_err, pytodo};
use ryo3_std::time::PyDuration;
use ryo3_url::extract_url;

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
    hickory_dns: bool,
    redirect: Option<usize>,
    // misspelled of course
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
    tls_max_version: Option<TlsVersion>,
    tls_min_version: Option<TlsVersion>,
    tls_info: bool, // default: false
    tls_sni: bool,  // default: true
    // -- danger zone --
    danger_accept_invalid_certs: bool,
    danger_accept_invalid_hostnames: bool,
    // == CLIENT BUILDER OPTIONS TODO ==
    // add_crl
    // add_crls
    // add_root_certificate
    // connector_layer
    // cookie_provider
    // cookie_store
    // dns_resolver
    // dns_resolver2
    // http09_responses
    // identity
    // interface
    // local_address
    // proxy
    // referer
    // resolve
    // resolve_to_addrs
    // retry
}

struct RequestKwargs<'py> {
    url: &'py Bound<'py, PyAny>,
    method: Method,
    body: Option<&'py Bound<'py, PyAny>>,
    headers: Option<PyHeadersLike>,
    query: Option<&'py Bound<'py, PyAny>>,
    json: Option<&'py Bound<'py, PyAny>>,
    multipart: Option<&'py Bound<'py, PyAny>>,
    form: Option<&'py Bound<'py, PyAny>>,
    timeout: Option<&'py PyDuration>,
    basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
    bearer_auth: Option<PyBackedStr>,
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
        if let Some((username, password)) = options.basic_auth {
            req = req.basic_auth(username, password);
        }
        if let Some(token) = options.bearer_auth {
            req = req.bearer_auth(token);
        }
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

        // version 2
        match (options.body, options.json, options.form, options.multipart) {
            (Some(_), Some(_), _, _)
            | (Some(_), _, Some(_), _)
            | (Some(_), _, _, Some(_))
            | (_, Some(_), Some(_), _)
            | (_, Some(_), _, Some(_))
            | (_, _, Some(_), Some(_)) => {
                return py_value_err!("body, json, form, multipart are mutually exclusive");
            }
            (Some(body), None, None, None) => {
                if let Ok(rsbytes) = body.cast_exact::<ryo3_bytes::PyBytes>() {
                    // short circuit for rs-py-bytes
                    let rsbytes: &Bytes = rsbytes.get().as_ref();
                    req = req.body(rsbytes.to_owned());
                } else if let Ok(bytes) = body.extract::<ryo3_bytes::PyBytes>() {
                    // buffer protocol
                    req = req.body(bytes.into_inner());
                } else {
                    return py_type_err!("body must be bytes-like or string");
                }
            }
            (None, Some(json), None, None) => {
                let wrapped = ryo3_serde::SerializePyAny::new(json, None);
                req = req.json(&wrapped);
            }
            (None, None, Some(form), None) => {
                let pyser = ryo3_serde::SerializePyAny::new(form, None);
                req = req.form(&pyser);
            }
            (None, None, None, Some(_multipart)) => {
                pytodo!("multipart not implemented (yet)");
            }
            (None, None, None, None) => {}
        }

        // // version 1
        // // make sure only one of body, json, form, multipart is set
        // if u8::from(options.body.is_some())
        //     + u8::from(options.json.is_some())
        //     + u8::from(options.form.is_some())
        //     + u8::from(options.multipart.is_some())
        //     > 1
        // {
        //     return py_value_err!("body, json, form, multipart are mutually exclusive");
        // }

        // if let Some(_multipart) = options.multipart {
        //     pytodo!("multipart not implemented (yet)");
        // }
        // if let Some(json) = options.json {
        //     let wrapped = ryo3_serde::SerializePyAny::new(json, None);
        //     req = req.json(&wrapped);
        // }
        // if let Some(form) = options.form {
        //     let pyser = ryo3_serde::SerializePyAny::new(form, None);
        //     req = req.form(&pyser);
        // }
        // if let Some(body) = options.body {}
        Ok(req)
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
            referer = true,
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

            pool_idle_timeout = Some(PyDuration::from(std::time::Duration::from_secs(90))),
            pool_max_idle_per_host = usize::MAX,

            tcp_keepalive = Some(PyDuration::from(std::time::Duration::from_secs(15))),
            tcp_keepalive_interval = Some(PyDuration::from(std::time::Duration::from_secs(15))),
            tcp_keepalive_retries = Some(3),
            tcp_nodelay = true,

            tls_min_version = None,
            tls_max_version = None,
            tls_info = false,
            tls_sni = true,

            danger_accept_invalid_certs = false,
            danger_accept_invalid_hostnames = false,
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
        referer: bool,
        gzip: Option<bool>,
        brotli: Option<bool>,
        deflate: Option<bool>,
        zstd: Option<bool>,
        hickory_dns: Option<bool>,
        http1_only: Option<bool>,
        https_only: Option<bool>,

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
        tls_min_version: Option<TlsVersion>,
        tls_max_version: Option<TlsVersion>,
        tls_info: bool,
        tls_sni: bool,

        // -- danger --
        danger_accept_invalid_certs: bool,
        danger_accept_invalid_hostnames: bool,
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
            referer,
            gzip: gzip.unwrap_or(true),
            brotli: brotli.unwrap_or(true),
            deflate: deflate.unwrap_or(true),
            zstd: zstd.unwrap_or(true),
            hickory_dns: hickory_dns.unwrap_or(true),
            http1_only: http1_only.unwrap_or(false),
            https_only: https_only.unwrap_or(false),
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
            tls_min_version,
            tls_max_version,
            tls_info,
            tls_sni,
            // -- danger --
            danger_accept_invalid_certs,
            danger_accept_invalid_hostnames,
        };
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn get<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn post<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn put<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn patch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn delete<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn head<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn options<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    pub(crate) fn fetch<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
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
            basic_auth,
            bearer_auth,
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
            basic_auth = None,
            bearer_auth = None,
            version = None,
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn __call__<'py>(
        &'py self,
        py: Python<'py>,
        url: &Bound<'py, PyAny>,
        method: Option<ryo3_http::HttpMethod>,
        body: Option<&Bound<'py, PyAny>>,
        headers: Option<PyHeadersLike>,
        query: Option<&Bound<'py, PyAny>>,
        json: Option<&Bound<'py, PyAny>>,
        form: Option<&Bound<'py, PyAny>>,
        multipart: Option<&Bound<'py, PyAny>>,
        timeout: Option<&PyDuration>,
        basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
        bearer_auth: Option<PyBackedStr>,
        version: Option<HttpVersion>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.fetch(
            py,
            url,
            method,
            body,
            headers,
            query,
            json,
            form,
            multipart,
            timeout,
            basic_auth,
            bearer_auth,
            version,
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
        dict.set_item(intern!(py, "cookies"), self.cookies)?;
        dict.set_item(intern!(py, "user_agent"), self.user_agent.clone())?;
        dict.set_item(intern!(py, "timeout"), self.timeout)?;
        dict.set_item(intern!(py, "read_timeout"), self.read_timeout)?;
        dict.set_item(intern!(py, "connect_timeout"), self.connect_timeout)?;
        dict.set_item(intern!(py, "redirect"), self.redirect)?;
        dict.set_item(intern!(py, "referer"), self.referer)?;
        dict.set_item(intern!(py, "gzip"), self.gzip)?;
        dict.set_item(intern!(py, "brotli"), self.brotli)?;
        dict.set_item(intern!(py, "deflate"), self.deflate)?;
        dict.set_item(intern!(py, "zstd"), self.zstd)?;
        dict.set_item(intern!(py, "hickory_dns"), self.hickory_dns)?;
        dict.set_item(intern!(py, "http1_only"), self.http1_only)?;
        dict.set_item(intern!(py, "https_only"), self.https_only)?;
        // -- http1 --
        dict.set_item(
            intern!(py, "http1_title_case_headers"),
            self.http1_title_case_headers,
        )?;
        dict.set_item(
            intern!(py, "http1_allow_obsolete_multiline_headers_in_responses"),
            self.http1_allow_obsolete_multiline_headers_in_responses,
        )?;
        dict.set_item(
            intern!(py, "http1_allow_spaces_after_header_name_in_responses"),
            self.http1_allow_spaces_after_header_name_in_responses,
        )?;
        dict.set_item(
            intern!(py, "http1_ignore_invalid_headers_in_responses"),
            self.http1_ignore_invalid_headers_in_responses,
        )?;
        // -- http2 --
        dict.set_item(
            intern!(py, "http2_prior_knowledge"),
            self.http2_prior_knowledge,
        )?;
        dict.set_item(
            intern!(py, "http2_initial_stream_window_size"),
            self.http2_initial_stream_window_size,
        )?;
        dict.set_item(
            intern!(py, "http2_initial_connection_window_size"),
            self.http2_initial_connection_window_size,
        )?;
        dict.set_item(
            intern!(py, "http2_adaptive_window"),
            self.http2_adaptive_window,
        )?;
        dict.set_item(
            intern!(py, "http2_max_frame_size"),
            self.http2_max_frame_size,
        )?;
        dict.set_item(
            intern!(py, "http2_max_header_list_size"),
            self.http2_max_header_list_size,
        )?;
        dict.set_item(
            intern!(py, "http2_keep_alive_interval"),
            self.http2_keep_alive_interval,
        )?;
        dict.set_item(
            intern!(py, "http2_keep_alive_timeout"),
            self.http2_keep_alive_timeout,
        )?;
        dict.set_item(
            intern!(py, "http2_keep_alive_while_idle"),
            self.http2_keep_alive_while_idle,
        )?;
        // -- pool --
        dict.set_item(intern!(py, "pool_idle_timeout"), self.pool_idle_timeout)?;
        dict.set_item(
            intern!(py, "pool_max_idle_per_host"),
            self.pool_max_idle_per_host,
        )?;
        // -- tcp --
        dict.set_item(intern!(py, "tcp_keepalive"), self.tcp_keepalive)?;
        dict.set_item(
            intern!(py, "tcp_keepalive_interval"),
            self.tcp_keepalive_interval,
        )?;
        dict.set_item(
            intern!(py, "tcp_keepalive_retries"),
            self.tcp_keepalive_retries,
        )?;
        dict.set_item(intern!(py, "tcp_nodelay"), self.tcp_nodelay)?;
        // -- tls --
        dict.set_item(intern!(py, "tls_min_version"), self.tls_min_version)?;
        dict.set_item(intern!(py, "tls_max_version"), self.tls_max_version)?;
        dict.set_item(intern!(py, "tls_info"), self.tls_info)?;
        dict.set_item(intern!(py, "tls_sni"), self.tls_sni)?;
        dict.set_item(
            intern!(py, "danger_accept_invalid_certs"),
            self.danger_accept_invalid_certs,
        )?;
        dict.set_item(
            intern!(py, "danger_accept_invalid_hostnames"),
            self.danger_accept_invalid_hostnames,
        )?;
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
            .danger_accept_invalid_certs(self.danger_accept_invalid_certs)
            .danger_accept_invalid_hostnames(self.danger_accept_invalid_hostnames);

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
        // http1
        if self.http1_only {
            client_builder = client_builder.http1_only();
        }
        if self.http1_title_case_headers {
            client_builder = client_builder.http1_title_case_headers();
        }

        // http2
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

        // tls
        if let Some(tls_min_version) = &self.tls_min_version {
            client_builder = client_builder.min_tls_version(tls_min_version.into());
        }
        if let Some(tls_max_version) = &self.tls_max_version {
            client_builder = client_builder.max_tls_version(tls_max_version.into());
        }
        client_builder
    }

    fn client_builder(&self) -> reqwest::ClientBuilder {
        let client_builder = reqwest::Client::builder();
        self.apply(client_builder)
    }
}
