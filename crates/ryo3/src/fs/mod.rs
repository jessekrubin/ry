use pyo3::exceptions::{PyFileNotFoundError, PyUnicodeDecodeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use std::path::Path;

pub mod fspath;

#[pyfunction]
pub fn read_vec_u8(s: &str) -> PyResult<Vec<u8>> {
    let p = Path::new(s);
    let b = std::fs::read(p);
    match b {
        Ok(b) => Ok(b),
        Err(e) => Err(PyFileNotFoundError::new_err(format!(
            "{}: {}",
            p.to_str().unwrap(),
            format!("{}: {:?}", e.to_string(), p.to_str().unwrap())
        ))),
    }
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: &str) -> PyResult<PyObject> {
    let bvec = read_vec_u8(s)?;
    Ok(PyBytes::new(py, &bvec).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: &str) -> PyResult<String> {
    let bvec = read_vec_u8(s)?;
    let r = std::str::from_utf8(&*bvec);

    match r {
        Ok(s) => Ok(s.to_string()),
        Err(e) => {
            let decode_err = PyUnicodeDecodeError::new_utf8(py, &*bvec, e).unwrap();
            Err(decode_err.into())
        }
    }
}

pub fn pymod(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_class::<fspath::PyPath>()?;
    Ok(())
}
