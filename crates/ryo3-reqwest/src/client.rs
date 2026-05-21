use pyo3::IntoPyObjectExt;
#[cfg(feature = "experimental-async")]
use pyo3::coroutine::CancelHandle;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use reqwest::{Method, RequestBuilder};
use ryo3_http::PyHttpMethod;
use ryo3_tokio_rt::get_tokio_runtime;
use ryo3_url::UrlLike;

use crate::errors::map_reqwest_err;
use crate::request::{BlockingReqwestKwargs, ReqwestKwargs};
use crate::response::RyBlockingResponse;
use crate::{ClientConfig, RyResponse};

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
        get_tokio_runtime().block_on(async {
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
        //    - query for the url is set from the UrlLike and query in kwargs is None --
        //      we are done
        //    - query in kwargs is Some -- and the url already has a query -- here we do
        //      the song and dance
        //    - query in kwargs is Some -- and the url has NO query so we can just set
        //      the string I think
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

    #[cfg(not(feature = "experimental-async"))]
    #[inline]
    fn request<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let rb = self.request_builder(url, method, kwargs)?;
        ryo3_tokio_rt::future_into_py(py, async move {
            rb.send()
                .await
                .map(RyResponse::from)
                .map_err(map_reqwest_err)
        })
    }

    #[cfg(feature = "experimental-async")]
    async fn send_request(req: RequestBuilder, mut cancel: CancelHandle) -> PyResult<RyResponse> {
        tokio::select! {
            res = req.send() => {
                res.map(RyResponse::from).map_err(map_reqwest_err)
            }
            _ = cancel.cancelled() => {
                Err(pyo3::exceptions::asyncio::CancelledError::new_err("Request was cancelled"))
            }
        }
    }

    #[cfg(feature = "experimental-async")]
    #[inline]
    async fn request(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
        cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        use ryo3_tokio_rt::on_tokio_py;
        let req = self.request_builder(url, method, kwargs)?;
        on_tokio_py(async move { Self::send_request(req, cancel).await }).await
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
        let a = get_tokio_runtime().block_on(async { req.send().await });
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
    async fn get(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::GET, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn post(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::POST, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn put(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::PUT, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn patch(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::PATCH, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn delete(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::DELETE, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn head(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::HEAD, kwargs, cancel).await
    }

    #[pyo3(signature = (url, **kwargs))]
    async fn options(
        &self,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, Method::OPTIONS, kwargs, cancel).await
    }

    #[pyo3(
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
    )]
    pub(crate) async fn fetch(
        &self,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, method.into(), kwargs, cancel).await
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    async fn __call__(
        &self,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
        #[pyo3(cancel_handle)] cancel: CancelHandle,
    ) -> PyResult<RyResponse> {
        self.request(url, method.into(), kwargs, cancel).await
    }

    #[pyo3(
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
    )]
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

#[cfg(not(feature = "experimental-async"))]
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
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
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

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
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
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
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

    #[pyo3(
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
    )]
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
