//! ry `reqwest` based global `fetch` and `fetch_sync` functions
use crate::RyBlockingResponse;
use ryo3_http::HttpMethod as PyHttpMethod;

#[cfg(feature = "experimental-async")]
use crate::RyClient;
#[cfg(not(feature = "experimental-async"))]
use crate::RyHttpClient;
#[cfg(feature = "experimental-async")]
use crate::response_parking_lot::RyAsyncResponse;
use pyo3::prelude::*;
use std::sync::OnceLock;

#[cfg(not(feature = "experimental-async"))]
static FETCH_CLIENT: OnceLock<RyHttpClient> = OnceLock::new();
#[cfg(feature = "experimental-async")]
static FETCH_CLIENT: OnceLock<RyClient> = OnceLock::new();

#[cfg(not(feature = "experimental-async"))]
pub(crate) fn fetch_client() -> &'static RyHttpClient {
    FETCH_CLIENT.get_or_init(|| {
        RyHttpClient::new(None).expect("Failed to create fetch client. This should never happen.")
    })
}

#[cfg(feature = "experimental-async")]
pub(crate) fn fetch_client() -> &'static RyClient {
    FETCH_CLIENT.get_or_init(|| {
        RyClient::new(None).expect("Failed to create fetch client. This should never happen.")
    })
}

// TODO move to using the new client dersion...

// global fetch
#[cfg(not(feature = "experimental-async"))]
#[pyfunction(
    signature = (
        url,
        *,
        method = PyHttpMethod::GET,
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
    ),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
#[expect(clippy::too_many_arguments)]
pub(crate) fn fetch<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyAny>,
    method: PyHttpMethod,
    body: Option<&Bound<'py, PyAny>>,
    headers: Option<ryo3_http::PyHeadersLike>,
    query: Option<&Bound<'py, PyAny>>,
    json: Option<&Bound<'py, PyAny>>,
    form: Option<&Bound<'py, PyAny>>,
    multipart: Option<&Bound<'py, PyAny>>,
    timeout: Option<&ryo3_std::time::PyDuration>,
    basic_auth: Option<(
        pyo3::pybacked::PyBackedStr,
        Option<pyo3::pybacked::PyBackedStr>,
    )>,
    bearer_auth: Option<pyo3::pybacked::PyBackedStr>,
    version: Option<ryo3_http::HttpVersion>,
) -> PyResult<Bound<'py, PyAny>> {
    let obj: Py<PyAny> = {
        let guard = fetch_client();
        let bound = guard.fetch(
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
        )?;
        bound.unbind()
    };
    Ok(obj.into_bound(py))
}

#[cfg(feature = "experimental-async")]
#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) async fn fetch(
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<crate::client::ReqwestKwargs<false>>,
) -> PyResult<RyAsyncResponse> {
    fetch_client().fetch(url, method, kwargs).await
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction(
    signature = (
        url,
        *,
        method = PyHttpMethod::GET,
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
    ),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
#[expect(clippy::too_many_arguments)]
pub(crate) fn fetch_sync<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyAny>,
    method: PyHttpMethod,
    body: Option<&Bound<'py, PyAny>>,
    headers: Option<ryo3_http::PyHeadersLike>,
    query: Option<&Bound<'py, PyAny>>,
    json: Option<&Bound<'py, PyAny>>,
    form: Option<&Bound<'py, PyAny>>,
    multipart: Option<&Bound<'py, PyAny>>,
    timeout: Option<&ryo3_std::time::PyDuration>,
    basic_auth: Option<(
        pyo3::pybacked::PyBackedStr,
        Option<pyo3::pybacked::PyBackedStr>,
    )>,
    bearer_auth: Option<pyo3::pybacked::PyBackedStr>,
    version: Option<ryo3_http::HttpVersion>,
) -> PyResult<RyBlockingResponse> {
    fetch_client().fetch_sync(
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

#[cfg(feature = "experimental-async")]
#[pyfunction(
    signature = (url, *, method = PyHttpMethod::GET, **kwargs),
    text_signature = "(url, *, method=\"GET\", body=None, headers=None, query=None, json=None, form=None, multipart=None, timeout=None, basic_auth=None, bearer_auth=None, version=None)"
)]
pub(crate) fn fetch_sync(
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<crate::client::ReqwestKwargs<true>>,
) -> PyResult<RyBlockingResponse> {
    fetch_client().fetch_sync(url, method, kwargs)
}
