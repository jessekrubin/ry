use pyo3::prelude::*;
mod sleep;
mod fmts;

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
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__build_profile__", env!("PROFILE"))?;
    m.add("__build_timestamp__", env!("BUILD_TIMESTAMP"))?;

    m.add_function(wrap_pyfunction!(nbytes_str, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;

    Ok(())
}
