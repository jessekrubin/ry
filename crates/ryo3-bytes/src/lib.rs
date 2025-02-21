#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::unused_self)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_wrap)]
mod pyo3_bytes;
mod ry_bytes;

use pyo3::prelude::*;
pub use pyo3_bytes::Pyo3Bytes;
pub mod bytes;
pub mod bytes_dev;
mod bytes_ext;
mod bytes_like;

pub use crate::bytes::PyBytes;
pub use bytes_like::{extract_bytes_ref, extract_bytes_ref_str};
pub use ry_bytes::RyBytes;

/// ryo3-bytes python module registration
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBytes>()?;

    // rename bytes module to `ry`
    m.getattr("Bytes")?.setattr("__module__", "ryo3")?;

    Ok(())
}
