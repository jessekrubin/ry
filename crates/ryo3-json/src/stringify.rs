use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ryo3_serde::ser::SerializePyAny;

// #[pyfunction(
//     signature = (obj, fmt = false, sort_keys = false)
// )]
// pub(crate) fn stringify_to_vec<'py>(
//     py: Python<'py>,
//     obj: Bound<'py, PyAny>,
//     fmt: bool,
//     sort_keys: bool,
// ) -> PyResult<ryo3_bytes::PyBytes> {
//     if sort_keys {
//         return Err(PyNotImplementedError::new_err(
//             "`sort_keys` not implemented (yet)",
//         ));
//     }
//     Python::with_gil(|py| {
//         let s = SerializePyAny::new(py, obj, None);
//         if fmt {
//             let b = serde_json::to_vec_pretty(&s)
//                 .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
//             Ok(ryo3_bytes::PyBytes::from(b))
//
//             // serde_json::to_writer_pretty(&mut bytes, &s)
//             //     .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
//         } else {
//             let b = serde_json::to_vec(&s)
//                 .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
//             Ok(ryo3_bytes::PyBytes::from(b))
//         }
//     })
// }
#[pyfunction(
    signature = (obj, fmt = false, sort_keys = false, append_newline = false)
)]
pub(crate) fn stringify(
    obj: Bound<'_, PyAny>,
    fmt: bool,
    sort_keys: bool,
    append_newline: bool,
) -> PyResult<ryo3_bytes::PyBytes> {
    if sort_keys {
        // TODO: This is a very hacky way of handling sorting the keys...
        //       ideally this would be part of the serialization process
        //       I think
        Python::with_gil(|py| {
            let s = SerializePyAny::new(py, obj, None);
            let mut bytes: Vec<u8> = Vec::with_capacity(4096);
            let value = serde_json::to_value(&s)
                .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            if fmt {
                serde_json::to_writer_pretty(&mut bytes, &value)
                    .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            } else {
                serde_json::to_writer(&mut bytes, &value)
                    .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            }
            if append_newline {
                bytes.push(b'\n');
            }
            Ok(ryo3_bytes::PyBytes::from(bytes))
        })
    } else {
        Python::with_gil(|py| {
            let s = SerializePyAny::new(py, obj, None);
            // 4k seeeems is a reasonable default size for JSON serialization?
            let mut bytes: Vec<u8> = Vec::with_capacity(4096);
            if fmt {
                serde_json::to_writer_pretty(&mut bytes, &s)
                    .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            } else {
                serde_json::to_writer(&mut bytes, &s)
                    .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
            }
            if append_newline {
                bytes.push(b'\n');
            }
            Ok(ryo3_bytes::PyBytes::from(bytes))
        })
    }
}
//
// #[pyfunction(
//     signature = (obj, fmt = false, sort_keys = false)
// )]
// pub(crate) fn stringify_unsafe<'py>(
//     py: Python<'py>,
//     obj: Bound<'py, PyAny>,
//     fmt: bool,
//     sort_keys: bool,
// ) -> PyResult<ryo3_bytes::PyBytes> {
//     if sort_keys {
//         return Err(PyNotImplementedError::new_err(
//             "`sort_keys` not implemented (yet)",
//         ));
//     }
//     Python::with_gil(|py| {
//         let s = UnsafeSerializePyAny::new(py, obj, None);
//         let mut bytes: Vec<u8> = Vec::with_capacity(32768); // 4k is a reasonable default size for JSON serialization
//         if fmt {
//             serde_json::to_writer_pretty(&mut bytes, &s)
//                 .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
//         } else {
//             serde_json::to_writer(&mut bytes, &s)
//                 .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {e}")))?;
//         }
//         Ok(ryo3_bytes::PyBytes::from(bytes))
//     })
// }
