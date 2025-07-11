//! python `reqwest` based `fetch` implementation

use crate::RyHttpClient;
use crate::default_client::default_client;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use ryo3_http::{HttpVersion, PyHeadersLike};

// global fetch
#[pyfunction(
    signature = (
        url,
        *,
        client = None,
        method = None,
        body = None,
        headers = None,
        query = None,
        json = None,
        form = None,
        multipart = None,
        timeout = None,
        version = None,
    )
)]
#[expect(clippy::too_many_arguments)]
pub(crate) fn fetch<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyAny>,
    client: Option<&RyHttpClient>,
    method: Option<ryo3_http::HttpMethod>,
    body: Option<ryo3_bytes::PyBytes>,
    headers: Option<PyHeadersLike>,
    query: Option<&Bound<'py, PyAny>>,
    json: Option<&Bound<'py, PyAny>>,
    form: Option<&Bound<'py, PyAny>>,
    multipart: Option<&Bound<'py, PyAny>>,
    timeout: Option<&ryo3_std::PyDuration>,
    version: Option<HttpVersion>,
) -> PyResult<Py<PyAny>> {
    let default_client_mut_guard;
    let client_ref: &RyHttpClient = if let Some(c) = client {
        c
    } else {
        let guard = default_client().lock();
        default_client_mut_guard = guard; // "stayin-alive" (ah ah ah ah, stayin-alive)
        &default_client_mut_guard
    };

    client_ref
        .fetch(
            py, url, method, body, headers, query, json, multipart, form, timeout, version,
        )
        .map(|x| x.into_py_any(py))?
}
