#![warn(clippy::missing_const_for_fn)]
//! `ryo3-jiff-core`
//!
//! jiff new-types + their python conversions; this is how things get
//! extracted from (and converted into) python for the jiff library...
//!
//! - `newts.rs`   ~ the new-types (+ consts/`Display`)
//! - `fns.rs`     ~ raw conversion functions (the actual logic) + tz-db lookup
//! - `to_py.rs`   ~ `IntoPyObject` impls (jiff -> python)
//! - `from_py.rs` ~ `FromPyObject` impls (python -> jiff)
//! - `fmt.rs`     ~ shared jiff parsers/printers (`ryo3_jiff_core::fmt::*`)
//!
//! Why not use the `pyo3/jiff` feature?
//!
//! - i have found a few bugs (which I plan to PR into pyo3)
//! - the ry jiff conversions actually predate the `pyo3/jiff` feature, however
//!   they have been influenced by the official pyo3 code
//!
//! Nothing in here should depend on the `Ry*` pyclasses (those live in
//! `ryo3-jiff`).
pub mod fmt;
mod fns;
mod from_py_object;
mod into_py_object;
mod newts;

pub use fns::*;
pub use newts::*;

// TODO: jiff error new-type?
// #[derive(Debug, Clone)]
// pub struct JiffError(jiff::Error);
//
// impl From<JiffError> for pyo3::PyErr {
//     fn from(e: JiffError) -> Self {
//         if  (e.inner().is_range()) {
//             pyo3::exceptions::PyOverflowError::new_err(e.inner().to_string())
//         } else {
//             pyo3::exceptions::PyValueError::new_err(e.inner().to_string())
//         }
//     }
// }
