//! bytes reexport

#[cfg(feature = "multiple-pymethods")]
pub use crate::pyo3_bytes::PyBytes;
#[cfg(not(feature = "multiple-pymethods"))]
pub use crate::ryo3_bytes::PyBytes;
