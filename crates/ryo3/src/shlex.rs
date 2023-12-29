use pyo3::prelude::*;
use shlex;

#[pyfunction]
pub fn shplit(in_str: &str) -> Option<Vec<String>> {
    shlex::split(in_str)
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    Ok(())
}
