use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

#[cfg(feature = "jiter")]
mod jiter_ry;

pub fn madd(_py: Python, _m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "jiter")]
    jiter_ry::madd(_py, _m)?;

    Ok(())
}
