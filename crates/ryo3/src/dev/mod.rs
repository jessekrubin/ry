use pyo3::types::PyModule;
use pyo3::{PyResult, Python};
pub mod anystr;
pub mod quick_maths;
pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    quick_maths::madd(_py, m)?;
    anystr::madd(_py, m)?;
    Ok(())
}
