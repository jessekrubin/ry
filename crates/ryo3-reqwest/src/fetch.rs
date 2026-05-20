//! ry `reqwest` based global `fetch` and `fetch_sync` functions
use std::sync::OnceLock;

#[cfg(feature = "experimental-async")]
use pyo3::coroutine::CancelHandle;
use pyo3::prelude::*;
use ryo3_http::PyHttpMethod;

#[cfg(feature = "experimental-async")]
use crate::RyResponse;
use crate::request::{BlockingReqwestKwargs, ReqwestKwargs};
use crate::{RyBlockingResponse, RyClient};

static FETCH_CLIENT: OnceLock<RyClient> = OnceLock::new();

pub(crate) fn fetch_client() -> &'static RyClient {
    FETCH_CLIENT.get_or_init(|| {
        RyClient::new(None).expect("Failed to create fetch client. This should never happen.")
    })
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) fn fetch(
    py: Python<'_>,
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<ReqwestKwargs>,
) -> PyResult<Bound<'_, PyAny>> {
    fetch_client().fetch(py, url, method, kwargs)
}

#[cfg(feature = "experimental-async")]
#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) async fn fetch(
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<ReqwestKwargs>,
    #[pyo3(cancel_handle)] cancel: CancelHandle,
) -> PyResult<RyResponse> {
    fetch_client().fetch(url, method, kwargs, cancel).await
}

#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) fn fetch_sync(
    py: Python<'_>,
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<BlockingReqwestKwargs>,
) -> PyResult<RyBlockingResponse> {
    fetch_client().fetch_sync(py, url, method, kwargs)
}
