use pyo3::prelude::*;

use ryo3::fmts;

#[pyfunction]
pub fn nbytes_str(nbytes: u64) -> PyResult<String> {
    Ok(fmts::nbytes_str(nbytes, Option::from(1)).unwrap())
}
