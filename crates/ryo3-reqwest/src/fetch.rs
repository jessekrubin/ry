//! python `reqwest` based `fetch` implementation

use crate::default_client::default_client;
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
        method = None,
        body = None,
        headers = None
    )
)]
pub(crate) fn fetch<'py>(
    py: Python<'py>,
    url: &str,
    method: Option<ryo3_http::HttpMethod>,
    body: Option<&[u8]>,
    headers: Option<Bound<'py, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let client = default_client();
    // boom lock that up!
    let mut client = client
        .lock()
        .map_err(|e| PyValueError::new_err(format!("default-client-lock-err: {e}")))?;

    // client.fetch(...) presumably returns something like PyResult<Bound<'py, PyAny>>
    let borrowed_pyany = client.fetch(py, url, method, body, headers)?;
    borrowed_pyany.into_py_any(py)
    // let borrowed_pyany = client.fetch(py, url, method, body, headers)?;
    // borrowed_pyany
    //
    // // Convert the borrowed reference to an owned PyObject
    // let owned: PyObject = borrowed_pyany.into_py(py);
    //
    // Ok(owned)
}
