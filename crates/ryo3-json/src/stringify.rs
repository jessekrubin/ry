use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use ryo3_serde::ser::SerializePyAny;

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
        let mut bytes: Vec<u8> = vec![];
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
