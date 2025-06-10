//! url-encoded form support

use pyo3::prelude::*;
use pythonize::depythonize;

// global fetch
#[pyfunction]
pub fn url_encode(data: &Bound<'_, PyAny>) -> PyResult<String> {
    let json_value: serde_json::Value = depythonize(data)?;
    let encoded = serde_urlencoded::to_string(json_value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(encoded)
}

#[pyfunction]
pub fn url_decode<'py>(py: Python<'py>, data: &str) -> PyResult<Bound<'py, PyAny>> {
    let decoded: serde_json::Value = serde_urlencoded::from_str(data)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    let py_value = pythonize::pythonize(py, &decoded)?;
    Ok(py_value)
}
