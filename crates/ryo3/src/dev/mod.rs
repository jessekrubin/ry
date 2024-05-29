#![allow(clippy::unwrap_used)]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod anystr;
pub mod quick_maths;
pub mod sp;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    quick_maths::madd(m)?;
    anystr::madd(m)?;
    sp::madd(m)?;
    Ok(())
}
