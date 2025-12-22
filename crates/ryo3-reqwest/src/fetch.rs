//! python `reqwest` based `fetch` implementation

use crate::RyBlockingResponse;
use ryo3_http::HttpMethod as PyHttpMethod;

#[cfg(feature = "experimental-async")]
use crate::RyClient;
#[cfg(not(feature = "experimental-async"))]
use crate::RyHttpClient;
#[cfg(feature = "experimental-async")]
use crate::response_parking_lot::RyAsyncResponse;
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_http::{HttpVersion, PyHeadersLike};
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

// TODO move to using the new client version...

// global fetch
#[cfg(not(feature = "experimental-async"))]
#[pyfunction(
    signature = (
        url,
        *,
        method = None,
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
    method: Option<PyHttpMethod>,
    body: Option<&Bound<'py, PyAny>>,
    headers: Option<PyHeadersLike>,
    query: Option<&Bound<'py, PyAny>>,
    json: Option<&Bound<'py, PyAny>>,
    form: Option<&Bound<'py, PyAny>>,
    multipart: Option<&Bound<'py, PyAny>>,
    timeout: Option<&ryo3_std::time::PyDuration>,
    basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<HttpVersion>,
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
#[pyfunction(signature = (url, method = PyHttpMethod::GET, **kwargs))]
pub(crate) async fn fetch(
    url: ryo3_url::UrlLike,
    method: PyHttpMethod,
    kwargs: Option<crate::client::ReqwestKwargs>,
) -> PyResult<RyAsyncResponse> {
    let guard = fetch_client();
    guard.fetch(url, method, kwargs).await
}

#[pyfunction(
    signature = (
        url,
        *,
        method = None,
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
    method: Option<ryo3_http::HttpMethod>,
    body: Option<&Bound<'py, PyAny>>,
    headers: Option<PyHeadersLike>,
    query: Option<&Bound<'py, PyAny>>,
    json: Option<&Bound<'py, PyAny>>,
    form: Option<&Bound<'py, PyAny>>,
    multipart: Option<&Bound<'py, PyAny>>,
    timeout: Option<&ryo3_std::time::PyDuration>,
    basic_auth: Option<(PyBackedStr, Option<PyBackedStr>)>,
    bearer_auth: Option<PyBackedStr>,
    version: Option<HttpVersion>,
) -> PyResult<RyBlockingResponse> {
    let guard = fetch_client();
    guard.fetch_sync(
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
