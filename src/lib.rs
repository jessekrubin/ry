use pyo3::prelude::*;
mod sleep;
mod fmts;
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn subry(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;
    Ok(())
}

#[pyfunction]
fn nbytes_str(nbytes: u64) -> PyResult<String> {
  Ok(fmts::nbytes_str(nbytes, Option::from(1)).unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_ry")]
fn ry(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_function(wrap_pyfunction!(nbytes_str, m)?)?;

    m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;

    Ok(())
}
