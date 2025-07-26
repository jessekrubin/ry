#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![expect(clippy::unwrap_used)]
#![expect(clippy::unused_self)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::ptr_as_ptr)]
#![expect(clippy::needless_pass_by_value)]
#![expect(clippy::similar_names)]
#![expect(clippy::cast_possible_wrap)]
// #![expect(clippy::use_self)]
use pyo3::intern;
use pyo3::prelude::*;
mod anybytes;
pub mod bytes;
mod bytes_like;

#[cfg(feature = "multiple-pymethods")]
mod pyo3_bytes;

mod python_bytes_methods;
mod ryo3_bytes;
pub use crate::bytes::PyBytes;
pub use bytes_like::{extract_bytes_ref, extract_bytes_ref_str};

/// ryo3-bytes python module registration
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBytes>()?;

    // rename bytes module to `ry`
    m.getattr(intern!(m.py(), "Bytes"))?
        .setattr(intern!(m.py(), "__module__"), intern!(m.py(), "ry.ryo3"))?;

    Ok(())
}
