use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod anystr;
pub mod quick_maths;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    quick_maths::madd(m)?;
    anystr::madd(m)?;
    Ok(())
}
