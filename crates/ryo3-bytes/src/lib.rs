#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![expect(clippy::ptr_as_ptr)]
#![expect(clippy::needless_pass_by_value)]
use pyo3::prelude::*;
pub mod bytes;
mod readable_buffer;
mod replace;

mod python_bytes_methods;
mod ryo3_bytes;
mod search;
pub use ::bytes::Bytes;
pub use readable_buffer::{ExactReadableBuffer, ReadableBuffer};

pub use crate::bytes::PyBytes;
// export alias `RyBytes` to avoid confusion with `pyo3::types::PyBytes`
pub use crate::bytes::PyBytes as RyBytes;

/// ryo3-bytes python module registration
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBytes>()?;
    Ok(())
}
