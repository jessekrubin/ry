#![doc = include_str!("../README.md")]
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult};
#[cfg(feature = "xxh3")]
pub mod xxh3;
#[cfg(feature = "xxh32")]
pub mod xxh32;
#[cfg(feature = "xxh64")]
pub mod xxh64;

#[cfg_attr(
    not(any(feature = "xxh32", feature = "xxh64", feature = "xxh3")),
    allow(unused_variables)
)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "xxh32")]
    xxh32::pymod_add(m)?;
    #[cfg(feature = "xxh64")]
    xxh64::pymod_add(m)?;
    #[cfg(feature = "xxh3")]
    xxh3::pymod_add(m)?;
    Ok(())
}
