use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use ryo3_serde::ser::SerializePyAny;

#[pyfunction(
    signature = (obj, fmt = false, sort_keys = false)
)]
pub(crate) fn stringify_to_vec<'py>(
    py: Python<'py>,
    obj: Bound<'py, PyAny>,
    fmt: bool,
    sort_keys: bool,
) -> PyResult<ryo3_bytes::PyBytes> {
    if sort_keys {
        return Err(PyNotImplementedError::new_err(
            "`sort_keys` not implemented (yet)",
        ));
    }
    Python::with_gil(|py| {
        let s = SerializePyAny::new(py, obj, None);
        if fmt {
            let b = serde_json::to_vec_pretty(&s)
                .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            Ok(ryo3_bytes::PyBytes::from(b))

            // serde_json::to_writer_pretty(&mut bytes, &s)
            //     .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
        } else {
            let b = serde_json::to_vec(&s)
                .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            Ok(ryo3_bytes::PyBytes::from(b))
        }
    })
}
#[pyfunction(
    signature = (obj, fmt = false, sort_keys = false)
)]
pub(crate) fn stringify<'py>(
    py: Python<'py>,
    obj: Bound<'py, PyAny>,
    fmt: bool,
    sort_keys: bool,
) -> PyResult<ryo3_bytes::PyBytes> {
    if sort_keys {
        return Err(PyNotImplementedError::new_err(
            "`sort_keys` not implemented (yet)",
        ));
    }
    Python::with_gil(|py| {
        let s = SerializePyAny::new(py, obj, None);
        let mut bytes: Vec<u8> = Vec::with_capacity(32768); // 4k is a reasonable default size for JSON serialization
        if fmt {
            serde_json::to_writer_pretty(&mut bytes, &s)
                .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
        } else {
            serde_json::to_writer(&mut bytes, &s)
                .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
        }
        Ok(ryo3_bytes::PyBytes::from(bytes))
    })
}
