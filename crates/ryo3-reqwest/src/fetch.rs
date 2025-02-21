//! python `reqwest` based `fetch` implementation

use crate::default_client::default_client;
use crate::RyHttpClient;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::IntoPyObjectExt;

// global fetch
#[pyfunction]
#[pyo3(
    signature = (
        url,
        *,
        client = None,
        method = None,
        body = None,
        headers = None
    )
)]
pub(crate) fn fetch<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyAny>,
    client: Option<RyHttpClient>,
    method: Option<ryo3_http::HttpMethod>,
    body: Option<&[u8]>,
    headers: Option<Bound<'py, PyDict>>,
) -> PyResult<Py<PyAny>> {
    if let Some(client) = client {
        let bound_pyany = client.fetch(py, url, method, body, headers)?;
        bound_pyany.into_py_any(py)
    } else {
        let client = default_client();
        // boom lock that up!
        let client = client
            .lock()
            .map_err(|e| PyValueError::new_err(format!("default-client-lock-err: {e}")))?;
        let bound_pyany = client.fetch(py, url, method, body, headers)?;
        bound_pyany.into_py_any(py)
    }
}
