//! ry `reqwest` based global `fetch` and `fetch_sync` functions
use std::sync::OnceLock;

use pyo3::prelude::*;
use ryo3_http::PyHttpMethod;
use ryo3_tokio_rt::RyCoroutine;

use crate::request::{BlockingReqwestKwargs, ReqwestKwargs};
use crate::{RyBlockingResponse, RyClient};

static FETCH_CLIENT: OnceLock<RyClient> = OnceLock::new();

pub(crate) fn fetch_client() -> &'static RyClient {
    FETCH_CLIENT.get_or_init(|| {
        RyClient::new(None).expect("Failed to create fetch client. This should never happen.")
    })
}

#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) fn fetch(
    py: Python<'_>,
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<ReqwestKwargs>,
) -> PyResult<Bound<'_, RyCoroutine>> {
    fetch_client().request(py, "fetch", url, method.into(), kwargs)
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
