#![doc = include_str!("../README.md")]
use pyo3::types::PyModule;
use pyo3::{Bound, PyResult};
mod py_digest;
#[cfg(feature = "xxhash32")]
pub mod xxhash32;
#[cfg(feature = "xxhash3_128")]
pub mod xxhash3_128;
#[cfg(feature = "xxhash3_64")]
pub mod xxhash3_64;
#[cfg(feature = "xxhash64")]
pub mod xxhash64;

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
    xxhash32::pymod_add(m)?;
    #[cfg(feature = "xxhash64")]
    xxhash64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_64")]
    xxhash3_64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_128")]
    xxhash3_128::pymod_add(m)?;
    Ok(())
}
