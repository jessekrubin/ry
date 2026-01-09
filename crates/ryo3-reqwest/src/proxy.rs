use pyo3::{IntoPyObjectExt, prelude::*};
use ryo3_macro_rules::py_value_error;
use ryo3_url::UrlLike;

#[pyclass(name = "Proxy", frozen, immutable_type, skip_from_py_object)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PyProxy {
    pub(crate) proxy: ::reqwest::Proxy,
    pub(crate) inner: PyProxyInner,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ProxyScheme {
    Http(String),
    Https(String),
    All(String),
    #[allow(dead_code)]
    Unix(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PyProxyInner {
    pub scheme: ProxyScheme,
    pub basic_auth: Option<(String, String)>,
    pub no_proxy: Vec<String>,
    pub headers: Vec<(String, String)>,
}

impl PyProxyInner {
    fn new(scheme: ProxyScheme) -> Self {
        Self {
            scheme,
            basic_auth: None,
            no_proxy: Vec::new(),
            headers: Vec::new(),
        }
    }
}

#[derive(FromPyObject, Default)]
pub(crate) struct ProxyKwargs {
    basic_auth: Option<(String, String)>,
    no_proxy: Option<String>,
    headers: Option<ryo3_http::PyHeadersLike>,
}

impl PyProxy {
    fn apply_kwargs(mut self, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        if let Some(kwds) = kwds {
            if let Some((u, p)) = kwds.basic_auth {
                self = self.basic_auth(u, p);
            }
            if let Some(np) = kwds.no_proxy {
                self = self.no_proxy(np);
            }
            if let Some(headers) = kwds.headers {
                self = self.headers(headers)?;
            }
        }
        Ok(self)
    }
}

impl std::hash::Hash for PyProxy {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

#[pymethods]
impl PyProxy {
    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn http(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = PyProxyInner::new(ProxyScheme::Http(url.to_string()));
        let proxy = ::reqwest::Proxy::http(url.0).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        p.apply_kwargs(kwds)
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn https(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = PyProxyInner::new(ProxyScheme::Https(url.to_string()));
        let proxy = ::reqwest::Proxy::https(url.0).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        p.apply_kwargs(kwds)
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn all(url: UrlLike, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let inner = PyProxyInner::new(ProxyScheme::All(url.to_string()));
        let proxy = ::reqwest::Proxy::all(url.0).map_err(|e| py_value_error!("{e}"))?;
        let p = Self { proxy, inner };
        p.apply_kwargs(kwds)
    }

    /// Creates a new UNIX domain socket proxy.
    #[allow(unused)]
    #[staticmethod]
    #[pyo3(signature = (path, **kwds))]
    #[allow(clippy::needless_pass_by_value)]
    fn unix(path: String, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        use ryo3_macro_rules::py_not_implemented_err;
        py_not_implemented_err!("unix socket proxy not yet implemented")
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

    fn headers(&self, headers: ryo3_http::PyHeadersLike) -> PyResult<Self> {
        let headers_map = reqwest::header::HeaderMap::try_from(headers)?;
        let proxy = self.proxy.clone().headers(headers_map.clone());
        let mut inner = self.inner.clone();
        for (k, v) in &headers_map {
            inner.headers.push((
                k.to_string(),
                v.to_str()
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
                    .to_string(),
            ));
        }
        Ok(Self { proxy, inner })
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

impl std::fmt::Display for PyProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let base = match &self.inner.scheme {
            ProxyScheme::Http(u) => format!("Proxy.http({u:?})"),
            ProxyScheme::Https(u) => format!("Proxy.https({u:?})"),
            ProxyScheme::All(u) => format!("Proxy.all({u:?})"),
            ProxyScheme::Unix(u) => format!("Proxy.unix({u:?})"),
        };
        let mut res = base;
        if let Some((u, p)) = &self.inner.basic_auth {
            write!(res, ".basic_auth({u:?}, {p:?})").expect("write to string failed");
        }
        for np in &self.inner.no_proxy {
            write!(res, ".no_proxy({np:?})").expect("write to string failed");
        }
        if !self.inner.headers.is_empty() {
            write!(res, ".headers({:?})", self.inner.headers).expect("write to string failed");
        }
        write!(f, "{res}")
    }
}

impl PartialEq for PyProxy {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl From<&PyProxy> for ::reqwest::Proxy {
    fn from(value: &PyProxy) -> Self {
        value.proxy.clone()
    }
}

impl Eq for PyProxy {}

impl<'a, 'py> FromPyObject<'a, 'py> for PyProxy {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(proxy) = obj.cast_exact::<Self>() {
            Ok(proxy.get().clone())
        } else if let Ok(url) = obj.extract::<UrlLike>() {
            Self::all(url, None)
        } else {
            Err(py_value_error!("Expected Proxy object or string URL"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PyProxies(Vec<PyProxy>);

impl<'a, 'py> FromPyObject<'a, 'py> for PyProxies {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(proxy) = obj.cast_exact::<PyProxy>() {
            return Ok(Self::from(proxy.get()));
        }
        // If it's a string, assume Proxy::all(url)
        if let Ok(url) = obj.cast_exact::<pyo3::types::PyString>() {
            let pyprox = url.extract::<PyProxy>()?;
            return Ok(Self::from(pyprox));
        }

        Err(py_value_error!("Expected Proxy object or string URL"))
    }
}

impl PyProxies {
    pub(crate) fn apply2client(&self, cb: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
        self.0.iter().fold(cb, |cb, el| cb.proxy(el.into()))
    }
}

impl From<PyProxy> for PyProxies {
    fn from(value: PyProxy) -> Self {
        Self(vec![value])
    }
}

impl From<&PyProxy> for PyProxies {
    fn from(value: &PyProxy) -> Self {
        Self(vec![value.clone()])
    }
}

impl<'py> IntoPyObject<'py> for &PyProxies {
    type Target = pyo3::types::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0.len() {
            0 => {
                unreachable!()
            }
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
