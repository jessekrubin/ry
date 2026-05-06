#![doc = include_str!("../README.md")]

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "time")]
pub mod time;

use pyo3::prelude::*;

#[cfg_attr(
    not(any(feature = "fs", feature = "time", feature = "sync")),
    expect(unused_variables)
)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "time")]
    time::pymod_add(m)?;

    #[cfg(feature = "fs")]
    fs::pymod_add(m)?;

    #[cfg(feature = "sync")]
    sync::pymod_add(m)?;

    Ok(())
}
