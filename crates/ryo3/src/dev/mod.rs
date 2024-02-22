use pyo3::types::PyModule;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, PyResult, Python};

#[pyfunction]
pub fn quick_maths() -> i32 {
    // 2 + 2 that's 4, minus 1 that's 3, quick maths
    let mut qm = 2 + 2;
    qm -= 1;
    qm
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quick_maths, m)?)?;
    Ok(())
}
