mod jiter_ry;

use pyo3::prelude::PyModule;
use pyo3::{wrap_pyfunction, PyResult, Python};

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    jiter_ry::madd(_py, m)?;
    Ok(())
}
