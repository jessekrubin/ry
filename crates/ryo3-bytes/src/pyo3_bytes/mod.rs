//! `pyo3-bytes` + ryo3-bytes-method implementation(s)
//!
//! The `pyo3-bytes` crate which provides a wrapper for the `bytes::Bytes` type
//! is contained in this sub-module and also has additional methods mirroring
//! the python `bytes` type.
//!
//! NOTE: Using this module requires the pyo3 feature `multiple-pymethods`
//!
//! A separate version of this struct/python-type is in this crate as well
//! and does not require the `multiple-pymethods` feature.
#![expect(clippy::doc_markdown)]
#![expect(clippy::unused_self)]
#![expect(clippy::cast_sign_loss)]
#![expect(clippy::ptr_as_ptr)]
#![expect(clippy::needless_pass_by_value)]
#![expect(clippy::similar_names)]
#![expect(clippy::cast_possible_wrap)]
#![expect(clippy::use_self)]
mod bytes;
mod bytes_ext;
pub use bytes::PyBytes;
