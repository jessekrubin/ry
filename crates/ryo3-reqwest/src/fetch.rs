//! python `reqwest` based `fetch` implementation

use crate::RyHttpClient;
use crate::default_client::default_client;
use pyo3::{prelude::*, pybacked::PyBackedStr};
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
        basic_auth = None,
        bearer_auth = None,
        version = None,
    )
)]
#[expect(clippy::too_many_arguments)]
pub(crate) fn fetch<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyAny>,
    client: Option<&'py RyHttpClient>,
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
    if let Some(c) = client {
        c.fetch(
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
    } else {
        let obj: Py<PyAny> = {
            let guard = default_client().lock();
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
}
