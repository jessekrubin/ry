use pyo3::prelude::*;
use ryo3_macro_rules::py_value_error;

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
                self = self.basic_auth(u, p)?;
            }
            if let Some(np) = kwds.no_proxy {
                self = self.no_proxy(np)?;
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
    fn http(url: String, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let proxy = ::reqwest::Proxy::http(&url)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let inner = PyProxyInner::new(ProxyScheme::Http(url));
        let p = PyProxy { proxy, inner };
        p.apply_kwargs(kwds)
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn https(url: String, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let proxy = ::reqwest::Proxy::https(&url)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let inner = PyProxyInner::new(ProxyScheme::Https(url));
        let p = PyProxy { proxy, inner };
        p.apply_kwargs(kwds)
    }

    #[staticmethod]
    #[pyo3(signature = (url, **kwds))]
    fn all(url: String, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        let proxy = ::reqwest::Proxy::all(&url)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let inner = PyProxyInner::new(ProxyScheme::All(url));
        let p = PyProxy { proxy, inner };
        p.apply_kwargs(kwds)
    }

    /// Creates a new UNIX domain socket proxy.
    #[allow(unused)]
    #[staticmethod]
    #[pyo3(signature = (path, **kwds))]
    fn unix(path: String, kwds: Option<ProxyKwargs>) -> PyResult<Self> {
        use ryo3_macro_rules::py_not_implemented_err;
        py_not_implemented_err!("unix socket proxy not yet implemented")
    }

    fn basic_auth(&self, username: String, password: String) -> PyResult<Self> {
        let proxy = self.proxy.clone().basic_auth(&username, &password);
        let mut inner = self.inner.clone();
        inner.basic_auth = Some((username, password));
        Ok(PyProxy { proxy, inner })
    }

    fn no_proxy(&self, url: String) -> PyResult<Self> {
        let mut inner = self.inner.clone();
        inner.no_proxy.push(url);

        let combined = inner.no_proxy.join(",");
        let no_proxy = ::reqwest::NoProxy::from_string(&combined);
        let proxy = self.proxy.clone().no_proxy(no_proxy);
        Ok(PyProxy { proxy, inner })
    }

    fn headers(&self, headers: ryo3_http::PyHeadersLike) -> PyResult<Self> {
        let headers_map = reqwest::header::HeaderMap::try_from(headers)?;
        let proxy = self.proxy.clone().headers(headers_map.clone());
        let mut inner = self.inner.clone();
        for (k, v) in headers_map.iter() {
            inner.headers.push((
                k.to_string(),
                v.to_str()
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
                    .to_string(),
            ));
        }
        Ok(PyProxy { proxy, inner })
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
        let base = match &self.inner.scheme {
            ProxyScheme::Http(u) => format!("Proxy.http({:?})", u),
            ProxyScheme::Https(u) => format!("Proxy.https({:?})", u),
            ProxyScheme::All(u) => format!("Proxy.all({:?})", u),
            ProxyScheme::Unix(u) => format!("Proxy.unix({:?})", u),
        };
        let mut res = base;
        if let Some((u, p)) = &self.inner.basic_auth {
            res.push_str(&format!(".basic_auth({:?}, {:?})", u, p));
        }
        for np in &self.inner.no_proxy {
            res.push_str(&format!(".no_proxy({:?})", np));
        }
        if !self.inner.headers.is_empty() {
            res.push_str(&format!(".headers({:?})", self.inner.headers));
        }
        write!(f, "{}", res)
    }
}

impl PartialEq for PyProxy {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for PyProxy {}

impl<'a, 'py> FromPyObject<'a, 'py> for PyProxy {
    type Error = PyErr;
    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        // If it's a PyProxy, extract it.
        if let Ok(proxy) = obj.cast_exact::<PyProxy>() {
            return Ok(proxy.get().clone());
        }
        // If it's a string, assume Proxy::all(url)
        if let Ok(url) = obj.extract::<String>() {
            return PyProxy::all(url, None);
        }
        Err(py_value_error!("Expected Proxy object or string URL"))
    }
}
