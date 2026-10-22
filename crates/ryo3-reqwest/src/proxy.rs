use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ryo3_core::py_dict::KwargsIter;
use ryo3_http::PyHeaders;
use ryo3_macro_rules::{py_type_err, py_value_err, py_value_error};
use ryo3_url::UrlLike;

#[pyclass(name = "Proxy", frozen, immutable_type)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyProxy {
    pub(crate) proxy: ::reqwest::Proxy,
    pub(crate) inner: ProxyInner,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PyProxyType {
    Http,
    Https,
    All,
}

const PY_PROXY_TYPE: &str = "'http', 'https', 'all'";

impl<'py> FromPyObject<'_, 'py> for PyProxyType {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode {
                "http" => Ok(Self::Http),
                "https" => Ok(Self::Https),
                "all" => Ok(Self::All),
                _ => py_value_err!(
                    "Invalid proxy-type, expected a string (options: {PY_PROXY_TYPE})"
                ),
            }
        } else {
            py_type_err!("Invalid proxy-type, expected a string (options: {PY_PROXY_TYPE})")
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for &PyProxyType {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode {
                "http" => Ok(&PyProxyType::Http),
                "https" => Ok(&PyProxyType::Https),
                "all" => Ok(&PyProxyType::All),
                _ => py_value_err!(
                    "Invalid proxy-type, expected a string (options: {PY_PROXY_TYPE})"
                ),
            }
        } else {
            py_type_err!("Invalid proxy-type, expected a string (options: {PY_PROXY_TYPE})")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ProxyScheme<T> {
    Http(T),
    Https(T),
    All(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ProxyInner {
    pub scheme: ProxyScheme<String>,
    pub basic_auth: Option<(String, String)>,
    pub no_proxy: Vec<String>,
    // pub headers: Vec<(String, String)>,
    pub headers: Option<PyHeaders>,
}

impl ProxyInner {
    fn new(scheme: ProxyScheme<String>) -> Self {
        Self {
            scheme,
            basic_auth: None,
            no_proxy: Vec::new(),
            headers: None,
        }
    }
}

#[derive(Default)]
pub(crate) struct ProxyKwargs {
    basic_auth: Option<(String, String)>,
    no_proxy: Option<String>,
    headers: Option<ryo3_http::PyHeaders>,
}

impl<'a, 'py> FromPyObject<'a, 'py> for ProxyKwargs {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let d = obj
            .cast_exact::<pyo3::types::PyDict>()
            .map_err(|e| py_value_error!("Expected dict for Proxy kwargs: {e}"))?;
        let mut kwargs = Self::default();
        for kwarg in KwargsIter::new(d) {
            let (key, value) = kwarg?;
            match key {
                "basic_auth" => kwargs.basic_auth = value.extract::<Option<(String, String)>>()?,
                "no_proxy" => kwargs.no_proxy = value.extract::<Option<String>>()?,
                "headers" => {
                    kwargs.headers = value
                        .extract::<Option<ryo3_http::PyHeadersLike>>()?
                        .map(PyHeaders::from);
                }
                s => {
                    return py_type_err!("unknown Proxy option: {}", s);
                }
            }
        }
        Ok(kwargs)
    }
}

impl RyProxy {
    #[inline]
    fn apply_kwargs(mut self, kwds: Option<ProxyKwargs>) -> Self {
        if let Some(kwds) = kwds {
            if let Some((u, p)) = kwds.basic_auth {
                self = self.basic_auth(u, p);
            }
            if let Some(np) = kwds.no_proxy {
                self = self.no_proxy(np);
            }
            if let Some(h) = kwds.headers {
                self = self.headers(ryo3_http::PyHeadersLike::Headers(h));
            }
        }
        self
    }
}

impl std::hash::Hash for RyProxy {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[pymethods]
impl RyProxy {
    #[new]
    #[pyo3(
        signature = (url, ptype = &PyProxyType::All, **kwds),
        text_signature = "(url, \"all\", *, basic_auth=None, headers=None, no_proxy=None)"
    )]
    fn py_new(url: UrlLike, ptype: &PyProxyType, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        match ptype {
            PyProxyType::Http => Self::http(url, kwds),
            PyProxyType::Https => Self::https(url, kwds),
            PyProxyType::All => Self::all(url, kwds),
        }
    }

    #[staticmethod]
    #[pyo3(
        signature = (url, **kwds),
        text_signature = "(url, *, basic_auth=None, headers=None, no_proxy=None)"
    )]
    fn http(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = ProxyInner::new(ProxyScheme::Http(url.to_string()));
        let proxy = ::reqwest::Proxy::http(url.into_inner()).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        Ok(p.apply_kwargs(kwds))
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn https(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = ProxyInner::new(ProxyScheme::Https(url.to_string()));
        let proxy =
            ::reqwest::Proxy::https(url.into_inner()).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        Ok(p.apply_kwargs(kwds))
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn all(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = ProxyInner::new(ProxyScheme::All(url.to_string()));
        let proxy = ::reqwest::Proxy::all(url.into_inner()).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        Ok(p.apply_kwargs(kwds))
    }

    fn basic_auth(&self, username: String, password: String) -> Self {
        let proxy = self.proxy.clone().basic_auth(&username, &password);
        let mut inner = self.inner.clone();
        inner.basic_auth = Some((username, password));
        Self { proxy, inner }
    }

    fn no_proxy(&self, url: String) -> Self {
        let mut inner = self.inner.clone();
        inner.no_proxy.push(url);

        let combined = inner.no_proxy.join(",");
        let no_proxy = ::reqwest::NoProxy::from_string(&combined);
        let proxy = self.proxy.clone().no_proxy(no_proxy);
        Self { proxy, inner }
    }

    fn headers(&self, headers: ryo3_http::PyHeadersLike) -> Self {
        let headers = PyHeaders::from(headers);
        let proxy = self.proxy.clone().headers(headers.py_read().clone());
        let mut inner = self.inner.clone();
        inner.headers = Some(headers);
        Self { proxy, inner }
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        std::hash::Hash::hash(self, &mut hasher);
        hasher.finish()
    }
}

impl std::fmt::Display for RyProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let base = match &self.inner.scheme {
            ProxyScheme::Http(u) => format!("Proxy({u:?}, \"http\")"),
            ProxyScheme::Https(u) => format!("Proxy({u:?}, \"https\")"),
            ProxyScheme::All(u) => format!("Proxy({u:?})"),
        };
        let mut res = base;
        if let Some((u, p)) = &self.inner.basic_auth {
            write!(res, ".basic_auth({u:?}, {p:?})").expect("write to string failed");
        }
        for np in &self.inner.no_proxy {
            write!(res, ".no_proxy({np:?})").expect("write to string failed");
        }
        if let Some(headers) = &self.inner.headers {
            write!(res, ".headers({headers:#})").expect("write to string failed");
        }
        write!(f, "{res}")
    }
}

impl PartialEq for RyProxy {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl From<&RyProxy> for ::reqwest::Proxy {
    fn from(value: &RyProxy) -> Self {
        value.proxy.clone()
    }
}

impl Eq for RyProxy {}

impl<'a, 'py> FromPyObject<'a, 'py> for RyProxy {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(proxy) = obj.cast_exact::<Self>() {
            Ok(proxy.get().clone())
        } else if let Ok(url) = obj.extract::<UrlLike>() {
            Self::all(url, None)
        } else {
            Err(py_value_error!("Expected Proxy, URL, or str"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PyProxies(Vec<RyProxy>);

impl<'a, 'py> FromPyObject<'a, 'py> for PyProxies {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(p) = obj.extract::<_>() {
            Ok(Self(vec![p]))
        } else if let Ok(v) = obj.extract::<Vec<_>>() {
            Ok(Self(v))
        } else {
            Err(py_value_error!(
                "Expected Proxy, URL, or str, or sequence thereof"
            ))
        }
    }
}

impl PyProxies {
    pub(crate) fn apply2client(&self, cb: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        self.0.iter().fold(cb, |cb, el| cb.proxy(el.into()))
    }
}

impl From<RyProxy> for PyProxies {
    fn from(value: RyProxy) -> Self {
        Self(vec![value])
    }
}

impl From<&RyProxy> for PyProxies {
    fn from(value: &RyProxy) -> Self {
        Self(vec![value.clone()])
    }
}

impl<'py> IntoPyObject<'py> for &PyProxies {
    type Target = pyo3::types::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0.len() {
            0 => pyo3::types::PyNone::get(py).into_bound_py_any(py),
            1 => self
                .0
                .first()
                .expect("wenodis")
                .clone()
                .into_bound_py_any(py),
            _ => self.0.clone().into_bound_py_any(py),
        }
    }
}
