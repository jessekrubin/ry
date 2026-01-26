use pyo3::prelude::*;

use crate::proxy::PyProxies;
use crate::tls::{PyCertificate, PyCertificateRevocationList, PyIdentity};
use crate::tls_version::TlsVersion;
use crate::types::Timeout;
use crate::user_agent::DEFAULT_USER_AGENT;
use pyo3::intern;
use pyo3::types::PyDict;
use reqwest::header::HeaderValue;
use ryo3_http::{PyHeaders, PyHeadersLike};
use ryo3_macro_rules::py_type_err;
use ryo3_macro_rules::py_value_error;
use ryo3_std::time::PyDuration;

#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::struct_excessive_bools)]
pub struct ClientConfig {
    pub(crate) headers: Option<PyHeaders>,
    pub(crate) cookies: bool,
    pub(crate) user_agent: Option<ryo3_http::HttpHeaderValue>,
    pub(crate) hickory_dns: bool,
    pub(crate) redirect: Option<usize>,
    pub(crate) resolve: Option<PyResolveMap>,
    pub(crate) connection_verbose: bool,
    // misspelled of course :/
    pub(crate) referer: bool,
    // -- http preferences --
    pub(crate) http1_only: bool,
    pub(crate) https_only: bool,
    // -- http1 --
    pub(crate) http1_title_case_headers: bool,
    pub(crate) http1_allow_obsolete_multiline_headers_in_responses: bool,
    pub(crate) http1_allow_spaces_after_header_name_in_responses: bool,
    pub(crate) http1_ignore_invalid_headers_in_responses: bool,
    // -- http2 --
    pub(crate) http2_prior_knowledge: bool,
    pub(crate) http2_initial_stream_window_size: Option<u32>,
    pub(crate) http2_initial_connection_window_size: Option<u32>,
    pub(crate) http2_adaptive_window: bool,
    pub(crate) http2_max_frame_size: Option<u32>,
    pub(crate) http2_max_header_list_size: Option<u32>,
    pub(crate) http2_keep_alive_interval: Option<PyDuration>,
    pub(crate) http2_keep_alive_timeout: Option<PyDuration>,
    pub(crate) http2_keep_alive_while_idle: bool,
    // -- timeout(s) --
    pub(crate) timeout: Option<PyDuration>,
    pub(crate) read_timeout: Option<PyDuration>,
    pub(crate) connect_timeout: Option<PyDuration>,
    // -- compression --
    pub(crate) gzip: bool,
    pub(crate) brotli: bool,
    pub(crate) deflate: bool,
    pub(crate) zstd: bool,
    // -- pool --
    pub(crate) pool_max_idle_per_host: usize,
    pub(crate) pool_idle_timeout: Option<PyDuration>,
    // -- tcp --
    pub(crate) tcp_keepalive: Option<PyDuration>, // default: 15 seconds
    pub(crate) tcp_keepalive_interval: Option<PyDuration>, // default: 15 seconds
    pub(crate) tcp_keepalive_retries: Option<u32>, // default: 3
    pub(crate) tcp_nodelay: bool,                 // default: true
    // -- tls --
    pub(crate) identity: Option<PyIdentity>,
    pub(crate) tls_certs_merge: Option<Vec<PyCertificate>>,
    pub(crate) tls_certs_only: Option<Vec<PyCertificate>>,
    pub(crate) tls_crls_only: Option<Vec<PyCertificateRevocationList>>,
    pub(crate) tls_info: bool, // default: false
    pub(crate) tls_sni: bool,  // default: true
    pub(crate) tls_version_max: Option<TlsVersion>,
    pub(crate) tls_version_min: Option<TlsVersion>,
    // -- tls danger zone --
    pub(crate) tls_danger_accept_invalid_certs: bool,
    pub(crate) tls_danger_accept_invalid_hostnames: bool,
    pub(crate) proxy: Option<PyProxies>,
    // == CLIENT BUILDER OPTIONS TODO ==
    // connector_layer
    // cookie_provider
    // cookie_store
    // dns_resolver
    // dns_resolver2
    // http09_responses
    // interface
    // local_address

    // -- UNSTABLE ~ RY ONLY) -
    /// (unstable) use cached native system certs
    pub(crate) _tls_cached_native_certs: bool,
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
            connection_verbose: false,
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
            // (UNSTABLE)
            _tls_cached_native_certs: false,
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for ClientConfig {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dict = obj.cast_exact::<PyDict>()?;
        let mut cfg = ClientConfig::default();

        for (k, v) in dict.iter() {
            let key_str = k.extract::<&str>()?;
            match key_str {
                "headers" => {
                    cfg.headers = v.extract::<Option<PyHeadersLike>>()?.map(PyHeaders::from);
                }
                "cookies" => {
                    cfg.cookies = v.extract::<bool>()?;
                }
                "user_agent" => {
                    cfg.user_agent = v.extract::<Option<ryo3_http::HttpHeaderValue>>()?;
                }
                "timeout" => {
                    cfg.timeout = v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "read_timeout" => {
                    cfg.read_timeout = v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "connect_timeout" => {
                    cfg.connect_timeout = v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "redirect" => {
                    cfg.redirect = v.extract::<Option<usize>>()?;
                }
                "resolve" => {
                    cfg.resolve = v.extract::<Option<PyResolveMap>>()?;
                }
                "referer" => {
                    cfg.referer = v.extract::<bool>()?;
                }
                "gzip" => {
                    cfg.gzip = v.extract::<bool>()?;
                }
                "brotli" => {
                    cfg.brotli = v.extract::<bool>()?;
                }
                "deflate" => {
                    cfg.deflate = v.extract::<bool>()?;
                }
                "zstd" => {
                    cfg.zstd = v.extract::<bool>()?;
                }
                "connection_verbose" => {
                    cfg.connection_verbose = v.extract::<bool>()?;
                }
                "hickory_dns" => {
                    cfg.hickory_dns = v.extract::<bool>()?;
                }
                "http1_only" => {
                    cfg.http1_only = v.extract::<bool>()?;
                }
                "https_only" => {
                    cfg.https_only = v.extract::<bool>()?;
                }
                // -- http1 --
                "http1_title_case_headers" => {
                    cfg.http1_title_case_headers = v.extract::<bool>()?;
                }
                "http1_allow_obsolete_multiline_headers_in_responses" => {
                    cfg.http1_allow_obsolete_multiline_headers_in_responses =
                        v.extract::<bool>()?;
                }
                "http1_allow_spaces_after_header_name_in_responses" => {
                    cfg.http1_allow_spaces_after_header_name_in_responses = v.extract::<bool>()?;
                }
                "http1_ignore_invalid_headers_in_responses" => {
                    cfg.http1_ignore_invalid_headers_in_responses = v.extract::<bool>()?;
                }
                // -- http2 --
                "http2_prior_knowledge" => {
                    cfg.http2_prior_knowledge = v.extract::<bool>()?;
                }
                "http2_initial_stream_window_size" => {
                    cfg.http2_initial_stream_window_size = v.extract::<Option<u32>>()?;
                }
                "http2_initial_connection_window_size" => {
                    cfg.http2_initial_connection_window_size = v.extract::<Option<u32>>()?;
                }
                "http2_adaptive_window" => {
                    cfg.http2_adaptive_window = v.extract::<bool>()?;
                }
                "http2_max_frame_size" => {
                    cfg.http2_max_frame_size = v.extract::<Option<u32>>()?;
                }
                "http2_max_header_list_size" => {
                    cfg.http2_max_header_list_size = v.extract::<Option<u32>>()?;
                }
                "http2_keep_alive_interval" => {
                    cfg.http2_keep_alive_interval =
                        v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "http2_keep_alive_timeout" => {
                    cfg.http2_keep_alive_timeout =
                        v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "http2_keep_alive_while_idle" => {
                    cfg.http2_keep_alive_while_idle = v.extract::<bool>()?;
                }
                // --- pool ---
                "pool_idle_timeout" => {
                    cfg.pool_idle_timeout = v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "pool_max_idle_per_host" => {
                    cfg.pool_max_idle_per_host = v.extract::<usize>()?;
                }
                // --- tcp ---
                "tcp_keepalive" => {
                    cfg.tcp_keepalive = v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "tcp_keepalive_interval" => {
                    cfg.tcp_keepalive_interval =
                        v.extract::<Option<Timeout>>()?.map(PyDuration::from);
                }
                "tcp_keepalive_retries" => {
                    cfg.tcp_keepalive_retries = v.extract::<Option<u32>>()?;
                }
                "tcp_nodelay" => {
                    cfg.tcp_nodelay = v.extract::<bool>()?;
                }
                // --- TLS ---
                "identity" => {
                    cfg.identity = v.extract::<Option<PyIdentity>>()?;
                }
                "tls_certs_merge" => {
                    cfg.tls_certs_merge = v.extract::<Option<Vec<PyCertificate>>>()?;
                }
                "tls_certs_only" => {
                    cfg.tls_certs_only = v.extract::<_>()?;
                }
                "tls_crls_only" => {
                    cfg.tls_crls_only = v.extract::<_>()?;
                }
                "tls_info" => {
                    cfg.tls_info = v.extract::<_>()?;
                }
                "tls_sni" => {
                    cfg.tls_sni = v.extract::<_>()?;
                }
                "tls_version_max" => {
                    cfg.tls_version_max = v.extract::<_>()?;
                }
                "tls_version_min" => {
                    cfg.tls_version_min = v.extract::<_>()?;
                }
                "tls_danger_accept_invalid_certs" => {
                    cfg.tls_danger_accept_invalid_certs = v.extract::<_>()?;
                }
                "tls_danger_accept_invalid_hostnames" => {
                    cfg.tls_danger_accept_invalid_hostnames = v.extract::<_>()?;
                }
                "proxy" => {
                    cfg.proxy = v.extract::<_>()?;
                }
                "_tls_cached_native_certs" => {
                    cfg._tls_cached_native_certs = v.extract::<_>()?;
                }
                _ => {
                    return py_type_err!("unknown ClientConfig option: {}", key_str);
                }
            }
        }

        Ok(cfg)
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
    #[expect(clippy::used_underscore_binding)]
    fn apply_tls_opts(&self, mut client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        // based on the reqwest logic for system cert loading, but adapted to allow for
        // caching the system certs once per process instead of loading them every time a client is built
        if let Some(tls_certs_only) = &self.tls_certs_only {
            client_builder = client_builder
                .tls_certs_only(tls_certs_only.iter().map(|py_cert| py_cert.cert.clone()));
        } else if self._tls_cached_native_certs && self.tls_certs_merge.is_none() {
            let cached = crate::tls::py_load_native_certs();
            if !cached.is_empty() {
                client_builder = client_builder.tls_certs_only(cached.iter().cloned());
            }
        }

        if let Some(tls_certs_merge) = &self.tls_certs_merge {
            client_builder = client_builder
                .tls_certs_merge(tls_certs_merge.iter().map(|py_cert| py_cert.cert.clone()));
        }

        // CRL
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
    pub(crate) fn apply(&self, client_builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        let mut client_builder = client_builder
            .connection_verbose(self.connection_verbose)
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

    pub(crate) fn as_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
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
            "connection_verbose" => self.connection_verbose,
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
        // unstable
        #[expect(clippy::used_underscore_binding)]
        {
            set_item!("_tls_cached_native_certs", self._tls_cached_native_certs);
        }

        Ok(dict)
    }

    #[inline]
    pub(crate) fn client_builder(&self) -> reqwest::ClientBuilder {
        let client_builder = reqwest::Client::builder();
        self.apply(client_builder)
    }
}

// ============================================================================
// RESOLVE EXTRACT
// ============================================================================
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

pub(crate) enum PyResolveMapValue {
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
pub(crate) struct PyResolveMap(Vec<(String, Vec<std::net::SocketAddr>)>);

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
