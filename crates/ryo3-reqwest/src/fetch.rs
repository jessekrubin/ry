//! python `reqwest` based `fetch` implementation

use crate::{RyBlockingResponse, RyHttpClient};
use pyo3::{prelude::*, pybacked::PyBackedStr};
use ryo3_http::{HttpVersion, PyHeadersLike};
use std::sync::OnceLock;

static DEFAULT_CLIENT: OnceLock<RyHttpClient> = OnceLock::new();

#[inline]
pub(crate) fn default_client() -> &'static RyHttpClient {
    DEFAULT_CLIENT.get_or_init(|| {
        RyHttpClient::new(None)
            .expect("Failed to create default client. This should never happen.")
    })
}

// global fetch
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
) -> PyResult<Bound<'py, PyAny>> {
    let obj: Py<PyAny> = {
        let guard = default_client();
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
    let guard = default_client();
    // let bound =
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
    // bound.unbind()
}
