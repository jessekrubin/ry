#![doc = include_str!("../README.md")]
#![expect(clippy::unnecessary_wraps)]
pub mod const_fns;
pub mod xxhashers;

use pyo3::types::PyModule;
use pyo3::{Bound, PyResult};

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    const_fns::pymod_add(m)?;
    xxhashers::pymod_add(m)?;
    Ok(())
}
