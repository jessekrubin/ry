use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use ryo3_serde::SerializePyAny;

fn map_serde_json_err<E: std::fmt::Display>(e: E) -> PyErr {
    PyTypeError::new_err(format!("Failed to serialize: {e}"))
}

#[pyfunction(
    signature = (obj, fmt = false, sort_keys = false, append_newline = false)
)]
pub(crate) fn stringify(
    py: Python<'_>,
    obj: Bound<'_, PyAny>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
) -> PyResult<ryo3_bytes::PyBytes> {
    if sort_keys {
        // TODO: This is a very hacky way of handling sorting the keys...
        //       ideally this would be part of the serialization process
        //       I think
        let s = SerializePyAny::new(py, obj, None);
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);
        let value = serde_json::to_value(&s).map_err(|e| {
            PyTypeError::new_err(format!("Failed to (de)serialize to json-value: {e}"))
        })?;

        if fmt {
            serde_json::to_writer_pretty(&mut bytes, &value).map_err(map_serde_json_err)?;
        } else {
            serde_json::to_writer(&mut bytes, &value).map_err(map_serde_json_err)?;
        }
        if append_newline {
            bytes.push(b'\n');
        }
        Ok(ryo3_bytes::PyBytes::from(bytes))
    } else {
        let s = SerializePyAny::new(py, obj, None);
        // 4k seeeems is a reasonable default size for JSON serialization?
        let mut bytes: Vec<u8> = Vec::with_capacity(4096);
        if fmt {
            serde_json::to_writer_pretty(&mut bytes, &s).map_err(map_serde_json_err)?;
        } else {
            serde_json::to_writer(&mut bytes, &s).map_err(map_serde_json_err)?;
        }
        if append_newline {
            bytes.push(b'\n');
        }
        Ok(ryo3_bytes::PyBytes::from(bytes))
    }
}
