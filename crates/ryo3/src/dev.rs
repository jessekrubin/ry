use pyo3::types::PyModule;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult, Python};

#[pyfunction]
pub fn rydev() -> PyResult<String> {
    Ok("RYDEV".to_string())
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rydev, m)?)?;
    Ok(())
}
