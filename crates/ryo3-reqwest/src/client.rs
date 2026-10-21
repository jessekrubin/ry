use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use reqwest::{Method, RequestBuilder};
use ryo3_http::PyHttpMethod;
use ryo3_tokio_rt::{RyCoroutine, get_tokio_runtime};
use ryo3_url::UrlLike;

use crate::errors::map_reqwest_err;
use crate::request::{BlockingReqwestKwargs, ReqwestKwargs};
use crate::response::RyBlockingResponse;
use crate::{ClientConfig, RyResponse};

#[derive(Debug, Clone)]
#[pyclass(name = "Client", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyClient {
    client: reqwest::Client,
    cfg: ClientConfig,
}

#[derive(Debug, Clone)]
#[pyclass(name = "BlockingClient", frozen, immutable_type)]
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

    /// the awaitable for a request ~ nothing is sent until it is awaited
    #[inline]
    pub(crate) fn request<'py>(
        &self,
        py: Python<'py>,
        qualname: &'static str,
        url: UrlLike,
        method: Method,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        let primer = kwargs.as_ref().and_then(ReqwestKwargs::body_primer);
        let rb = match build_request(&self.client, url, method, kwargs) {
            Ok(rb) => rb,
            // raised on await like it would be from an `async def`
            Err(e) => return ryo3_tokio_rt::failed_coro(py, qualname, e),
        };
        let send = async move {
            let response = rb.send().await.map_err(map_reqwest_err)?;
            Ok(RyResponse::from_response(response).await)
        };
        // first poll is on the loop thread ~ pull any async body there before
        // the request goes over to tokio (saves a tokio -> loop round trip)
        ryo3_tokio_rt::pyloop_coro(py, qualname, async move {
            if let Some(primer) = primer {
                Python::attach(|py| primer.prime(py));
            }
            ryo3_tokio_rt::on_tokio_py(send).await
        })
    }

    #[inline]
    fn request_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        build_request(&self.client, url, method, kwargs).map(send_sync)?
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
    fn request_sync(
        &self,
        url: UrlLike,
        method: Method,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        build_request(&self.client, url, method, kwargs).map(send_sync)?
    }
}

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
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.get", url, Method::GET, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn post<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.post", url, Method::POST, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn put<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.put", url, Method::PUT, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.delete", url, Method::DELETE, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn head<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.head", url, Method::HEAD, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn options<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.options", url, Method::OPTIONS, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn patch<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.patch", url, Method::PATCH, kwargs)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn query<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.query", url, Method::QUERY, kwargs)
    }

    #[pyo3(
        signature = (url, *, method = PyHttpMethod::GET, **kwargs),
        text_signature = "($self, url, *, method=\"GET\", **kwargs)"
    )]
    pub(crate) fn fetch<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.fetch", url, method.into(), kwargs)
    }

    #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    fn __call__<'py>(
        &self,
        py: Python<'py>,
        url: UrlLike,
        method: PyHttpMethod,
        kwargs: Option<ReqwestKwargs>,
    ) -> PyResult<Bound<'py, RyCoroutine>> {
        self.request(py, "Client.__call__", url, method.into(), kwargs)
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
    pub(crate) fn query(
        &self,
        py: Python<'_>,
        url: UrlLike,
        kwargs: Option<BlockingReqwestKwargs>,
    ) -> PyResult<RyBlockingResponse> {
        py.detach(|| self.request_sync(url, Method::QUERY, kwargs))
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

/// the reqwest builder for one request w/ the kwargs applied
#[inline]
fn build_request<const BLOCKING: bool>(
    client: &reqwest::Client,
    url: UrlLike,
    method: Method,
    kwargs: Option<ReqwestKwargs<BLOCKING>>,
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
    let rb = client.request(method, url.into_inner());
    match kwargs {
        Some(kwargs) => kwargs.apply(rb),
        None => Ok(rb),
    }
}

/// send + wait (blocking; caller has already let go of the gil)
#[inline]
fn send_sync(req: RequestBuilder) -> PyResult<RyBlockingResponse> {
    // `send()` MUST be called inside the async block ~ w/ a timeout set reqwest
    // makes a `tokio::time::sleep` right in `send()`, which panics (== abort in
    // release) when there is no tokio rt context
    get_tokio_runtime().block_on(async {
        let response = req.send().await.map_err(map_reqwest_err)?;
        Ok(RyBlockingResponse::from_response(response).await)
    })
}
