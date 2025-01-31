use crate::PyBytes;
use pyo3::prelude::*;
use pyo3::{Bound, PyAny, PyResult};

pub(crate) fn extract_vecu8_ref<'py>(obj: &'py Bound<'py, PyAny>) -> PyResult<&'py [u8]> {
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
