use pyo3::types::PyModule;
use pyo3::{PyResult, Python};
pub mod quick_maths;
pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    quick_maths::madd(_py, m)?;
    Ok(())
}
