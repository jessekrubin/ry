//! Common code for xxhash3-64 and xxhash3-128.
//!
//! - secret
//!    - must be at least 136 bytes long
//!    - use readable-buf to extract
use pyo3::prelude::*;
use ryo3_core::{py_type_err, py_value_err};

const XXH3_SECRET_MIN_LEN: usize = 136;

/// msg for expect
pub(crate) const XXH3_SECRET_EXPECT_MSG: &str =
    "wenodis: secret already validated to be at least 136 bytes long";

pub struct PyXxHash3Secret(ryo3_bytes::PyBytes);

impl<'py> FromPyObject<'_, 'py> for PyXxHash3Secret {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(rb) = ob.extract::<ryo3_bytes::ReadableBuffer>() {
            if rb.len() < XXH3_SECRET_MIN_LEN {
                return py_value_err!(
                    "xxhash3-secret must be at least {} bytes long",
                    XXH3_SECRET_MIN_LEN
                );
            }
            Ok(Self(rb.to_rybytes()))
        } else {
            py_type_err!(
                "xxhash3-secret must be readable-buffer with of at least {} bytes",
                XXH3_SECRET_MIN_LEN
            )
        }
    }
}

impl AsRef<[u8]> for PyXxHash3Secret {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<PyXxHash3Secret> for Box<[u8]> {
    #[inline]
    fn from(value: PyXxHash3Secret) -> Self {
        value.as_ref().into()
    }
}
