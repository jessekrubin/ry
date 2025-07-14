#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![expect(clippy::doc_markdown)]
#![expect(clippy::unwrap_used)]
#![expect(clippy::unused_self)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::ptr_as_ptr)]
#![expect(clippy::needless_pass_by_value)]
#![expect(clippy::similar_names)]
#![expect(clippy::cast_possible_wrap)]
#![expect(clippy::use_self)]
mod pyo3_bytes;
mod ry_bytes;
use pyo3::intern;
use pyo3::prelude::*;
pub use pyo3_bytes::Pyo3Bytes;
pub mod bytes;
// pub mod bytes_dev;
mod anybytes;
mod bytes_ext;
mod bytes_like;

pub use crate::bytes::PyBytes;
pub use bytes_like::{extract_bytes_ref, extract_bytes_ref_str};
pub use ry_bytes::RyBytes;

/// ryo3-bytes python module registration
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBytes>()?;

    // rename bytes module to `ry`
    m.getattr(intern!(m.py(), "Bytes"))?
        .setattr(intern!(m.py(), "__module__"), intern!(m.py(), "ry.ryo3"))?;

    Ok(())
}
