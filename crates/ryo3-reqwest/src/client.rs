#[cfg(feature = "experimental-async")]
use crate::body::PyBody;
use crate::errors::map_reqwest_err;
#[cfg(feature = "experimental-async")]
use crate::response::RyAsyncResponse;
use crate::response::RyBlockingResponse;
use crate::types::Timeout;
#[cfg(feature = "experimental-async")]
use crate::types::{PyQuery, PyRequestJson};
use crate::{ClientConfig, RyResponse};
use cookie::time::Time;
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Method, RequestBuilder};
use ryo3_http::{PyHeadersLike, PyHttpMethod, PyHttpVersion};
use ryo3_macro_rules::py_value_error;
use ryo3_macro_rules::{py_type_err, py_value_err, pytodo};
use ryo3_url::UrlLike;
use std::time::Duration;

//============================================================================
use std::{
    future::Future,
    pin::{Pin, pin},
    task::{Context, Poll},
};

macro_rules! reqwest_kwargs {
    () => {
        let opts = ReqwestKwargsBuilder::new()
            .headers(headers)
            .query(query)
            .body(body)
            .json(json)
            .form(form)
            .multipart(multipart.map(|m| m.to_string()))
            .timeout(timeout.map(|t| t.into()))
            .basic_auth(basic_auth)
            .bearer_auth(bearer_auth)
            .version(version)
            .build();
        self.request(url, method.into(), Some(opts)).await
    };
}
struct AllowThreads<F>(F);

impl<F> Future for AllowThreads<F>
where
    F: Future + Unpin + Send,
    F::Output: Send,
{
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker();
        Python::attach(|py| py.detach(|| pin!(&mut self.0).poll(&mut Context::from_waker(waker))))
    }
}
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

    // #[pyo3(signature = (url, **kwargs))]
    // async fn get(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
    //     self.request(url, Method::GET, kwargs).await
    // }

    // #[pyo3(signature = (url, **kwargs))]
    // async fn post(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
    //     self.request(url, Method::POST, kwargs).await
    // }

    // #[pyo3(signature = (url, **kwargs))]
    // async fn put(&self, url: UrlLike, kwargs: Option<ReqwestKwargs>) -> PyResult<RyAsyncResponse> {
    //     self.request(url, Method::PUT, kwargs).await
    // }

    // #[pyo3(signature = (url, **kwargs))]
    // async fn patch(
    //     &self,
    //     url: UrlLike,
    //     kwargs: Option<ReqwestKwargs>,
    // ) -> PyResult<RyAsyncResponse> {
    //     self.request(url, Method::PATCH, kwargs).await
    // }

    // #[pyo3(signature = (url, **kwargs))]
    // async fn delete(
    //     &self,
    //     url: UrlLike,
    //     kwargs: Option<ReqwestKwargs>,
    // ) -> PyResult<RyAsyncResponse> {
    //     self.request(url, Method::DELETE, kwargs).await
    // }

    // #[pyo3(signature = (url, **kwargs))]
    // async fn options(&self, url: UrlLike, kwargs: Option<Py<PyDict>>) -> PyResult<RyAsyncResponse> {
    //     let kwargs2: Option<ReqwestKwargs> = Python::attach(|py| {
    //         let t = kwargs.map(|k| {
    //             // let b = k.bind(py);
    //             let b = k.bind(py);
    //             let rk = b.extract::<ReqwestKwargs>().unwrap();
    //             rk
    //         });
    //         t
    //     });
    //     // let kwargs = kwargs.map(|k| k.as_ref(py).extract::<ReqwestKwargs>().unwrap());
    //     self.request(url, Method::OPTIONS, kwargs2).await
    // }
    #[pyo3(
        signature = (
            url,
            *,
            headers = None,
            query = None,
            body = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            basic_auth = None,
            bearer_auth = None,
            version = None
        )
    )]
    pub(crate) async fn get(
        &self,
        url: UrlLike,
        headers: Option<PyHeadersLike>,
        query: Option<PyQuery>,
        body: Option<PyBody>,
        json: Option<PyRequestJson>,
        form: Option<String>,
        multipart: Option<String>,
        timeout: Option<Timeout>,
        basic_auth: Option<BasicAuth>,
        bearer_auth: Option<PyBackedStr>,
        version: Option<PyHttpVersion>,
    ) -> PyResult<RyAsyncResponse> {
        let kw = reqwest_kwargs_from_parts(
            headers,
            query,
            body,
            json,
            form,
            multipart.is_some(),
            timeout,
            basic_auth,
            bearer_auth,
            version,
        )?;
        self.request(url, Method::GET, Some(kw)).await
    }

    #[pyo3(
        signature = (
            url,
            *,
            method = PyHttpMethod::GET,
            headers = None,
            query = None,
            body = None,
            json = None,
            form = None,
            multipart = None,
            timeout = None,
            basic_auth = None,
            bearer_auth = None,
            version = None
        )
    )]
    pub(crate) async fn fetch(
        &self,
        url: UrlLike,
        method: PyHttpMethod,
        headers: Option<PyHeadersLike>,
        query: Option<PyQuery>,
        body: Option<PyBody>,
        json: Option<PyRequestJson>,
        form: Option<String>,
        multipart: Option<Py<PyAny>>,
        timeout: Option<Timeout>,
        basic_auth: Option<BasicAuth>,
        bearer_auth: Option<PyBackedStr>,
        version: Option<PyHttpVersion>,
    ) -> PyResult<RyAsyncResponse> {
        // macro that does all the below...
        let opts = ReqwestKwargsBuilder::new()
            .headers(headers)
            .query(query)
            .body(body)
            .json(json)
            .form(form)
            .multipart(multipart.map(|m| m.to_string()))
            .timeout(timeout.map(|t| t.into()))
            .basic_auth(basic_auth)
            .bearer_auth(bearer_auth)
            .version(version)
            .build();
        self.request(url, method.into(), Some(opts)).await
    }

    // #[pyo3(signature = (url, *, method = PyHttpMethod::GET, **kwargs))]
    // async fn __call__(
    //     &self,
    //     url: UrlLike,
    //     method: PyHttpMethod,
    //     kwargs: Option<ReqwestKwargs>,
    // ) -> PyResult<RyAsyncResponse> {
    //     self.request(url, method.into(), kwargs).await
    // }

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

impl<'py> IntoPyObject<'py> for &ClientConfig {
    type Target = PyDict;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.as_pydict(py)
    }
}
// maybe dont actually need this?

pub(crate) struct BasicAuth(PyBackedStr, Option<PyBackedStr>);

impl<'py> FromPyObject<'_, 'py> for BasicAuth {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let tuple: (PyBackedStr, Option<PyBackedStr>) = obj.extract()?;
        Ok(Self(tuple.0, tuple.1))
    }
}

pub(crate) struct ReqwestKwargsBuilder<const BLOCKING: bool = false> {
    headers: Option<PyHeadersLike>,
    query: Option<PyQuery>,
    body: Option<PyBody>,
    json: Option<PyRequestJson>,
    form: Option<String>,
    multipart: Option<String>,
    timeout: Option<Timeout>,
    basic_auth: Option<BasicAuth>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<PyHttpVersion>,
}

macro_rules! impl_reqwest_kwargs_builder_field {
    ($field:ident, $ty:ty) => {
        fn $field(self, $field: Option<$ty>) -> Self {
            Self { $field, ..self }
        }
    };
}
impl<const BLOCKING: bool> ReqwestKwargsBuilder<BLOCKING> {
    fn new() -> Self {
        Self {
            headers: None,
            query: None,
            body: None,
            json: None,
            form: None,
            multipart: None,
            timeout: None,
            basic_auth: None,
            bearer_auth: None,
            version: None,
        }
    }
    impl_reqwest_kwargs_builder_field!(headers, PyHeadersLike);
    impl_reqwest_kwargs_builder_field!(query, PyQuery);
    impl_reqwest_kwargs_builder_field!(body, PyBody);
    impl_reqwest_kwargs_builder_field!(json, PyRequestJson);
    impl_reqwest_kwargs_builder_field!(form, String);
    impl_reqwest_kwargs_builder_field!(multipart, String);
    impl_reqwest_kwargs_builder_field!(timeout, Timeout);
    impl_reqwest_kwargs_builder_field!(basic_auth, BasicAuth);
    impl_reqwest_kwargs_builder_field!(bearer_auth, PyBackedStr);
    impl_reqwest_kwargs_builder_field!(version, PyHttpVersion);

    fn build(self) -> ReqwestKwargs<BLOCKING> {
        let body = reqwest_body_from_parts::<BLOCKING>(
            self.body,
            self.json,
            self.form,
            self.multipart.is_some(),
        )
        .unwrap_or(PyReqwestBody::None);
        ReqwestKwargs {
            headers: self.headers.map(|h| h.into()),
            query: self.query.map(|q| q.into()),
            body,
            timeout: self.timeout.map(|t| t.into()),
            basic_auth: self.basic_auth,
            bearer_auth: self.bearer_auth,
            version: self.version,
        }
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

fn reqwest_body_from_parts<const BLOCKING: bool>(
    body: Option<PyBody>,
    json: Option<PyRequestJson>,
    form: Option<String>,
    multipart_present: bool,
) -> PyResult<PyReqwestBody> {
    match (body, json, form, multipart_present) {
        (Some(_), Some(_), _, _)
        | (Some(_), _, Some(_), _)
        | (Some(_), _, _, true)
        | (_, Some(_), Some(_), _)
        | (_, Some(_), _, true)
        | (_, _, Some(_), true) => {
            return py_value_err!("body, json, form, multipart are mutually exclusive");
        }
        (Some(body), None, None, false) => match body {
            PyBody::Bytes(b) => Ok(PyReqwestBody::Bytes(b.into())),
            PyBody::Stream(s) => {
                if BLOCKING && s.is_async() {
                    return py_type_err!("cannot use async stream body with blocking client");
                }
                Ok(PyReqwestBody::Stream(s))
            }
        },
        (None, Some(json), None, false) => Ok(PyReqwestBody::Json(json.into())),
        (None, None, Some(form), false) => Ok(PyReqwestBody::Form(form)),
        (None, None, None, true) => {
            pytodo!("multipart not implemented (yet)");
        }
        (None, None, None, false) => Ok(PyReqwestBody::None),
    }
}

#[allow(clippy::too_many_arguments)]
fn reqwest_kwargs_from_parts<const BLOCKING: bool>(
    headers: Option<PyHeadersLike>,
    query: Option<PyQuery>,
    body: Option<PyBody>,
    json: Option<PyRequestJson>,
    form: Option<String>,
    multipart_present: bool,
    timeout: Option<Timeout>,
    basic_auth: Option<BasicAuth>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<PyHttpVersion>,
) -> PyResult<ReqwestKwargs<BLOCKING>> {
    Ok(ReqwestKwargs {
        headers: headers.map(Into::into),
        query: query.map(String::from),
        body: reqwest_body_from_parts::<BLOCKING>(body, json, form, multipart_present)?,
        timeout: timeout.map(Duration::from),
        basic_auth,
        bearer_auth,
        version,
    })
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
            .map(Duration::from);
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

fn extract_request_kwargs<'py>(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<ReqwestKwargs> {
    obj.extract::<ReqwestKwargs>()
}
