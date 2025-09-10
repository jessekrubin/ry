#![doc = include_str!("../README.md")]
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult};
mod py_digest;
// #[cfg(feature = "xxh3")]
// pub mod xxh3;
// #[cfg(feature = "xxhash32")]
// pub mod xxh32;
#[cfg(feature = "xxhash32")]
pub mod xxh32;
#[cfg(feature = "xxhash3_128")]
pub mod xxh3_128;
#[cfg(feature = "xxhash3_64")]
pub mod xxh3_64;
#[cfg(feature = "xxhash64")]
pub mod xxh64;

#[cfg_attr(
    not(any(
        feature = "xxhash32",
        feature = "xxhash64",
        feature = "xxhash3_64",
        feature = "xxhash3_128"
    )),
    allow(unused_variables)
)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "xxhash32")]
    xxh32::pymod_add(m)?;
    #[cfg(feature = "xxhash64")]
    xxh64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_64")]
    xxh3_64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_128")]
    xxh3_128::pymod_add(m)?;
    Ok(())
}
