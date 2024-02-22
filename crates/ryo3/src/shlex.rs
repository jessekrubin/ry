use pyo3::prelude::*;
use shlex;

#[pyfunction(signature = (string), text_signature = "(string: str) -> list[str] | None")]
pub fn shplit(string: &str) -> Option<Vec<String>> {
    shlex::split(string)
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shplit, m)?)?;
    Ok(())
}
