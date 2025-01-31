use crate::PyBytes;
use pyo3::prelude::*;
use pyo3::{Bound, PyAny, PyResult};

/// Extract a `&[u8]` from a `PyAny` object.
///
/// This is useful for when you have either a `bytes` or a `PyBytes` object
/// as defined in this crate, but you only need a reference to the bytes.
/// It is considerably faster than just using `PyBytes` as the param for a
/// function as it avoids the overhead of creating a new `PyBytes` object.
pub fn extract_vecu8_ref<'py>(obj: &'py Bound<'py, PyAny>) -> PyResult<&'py [u8]> {
    if let Ok(bytes) = obj.extract::<&[u8]>() {
        Ok(bytes)
    } else if let Ok(custom) = obj.downcast::<PyBytes>() {
        let a = custom.get();
        Ok(a.as_ref())
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes, bytearray, or pyo3-bytes object",
        ))
    }
}
