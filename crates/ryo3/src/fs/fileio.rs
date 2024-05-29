use std::path::Path;

use pyo3::exceptions::{PyFileNotFoundError, PyNotADirectoryError, PyUnicodeDecodeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

#[pyfunction]
pub fn read_vec_u8(s: &str) -> PyResult<Vec<u8>> {
    let fpath = Path::new(s);
    let fbytes = std::fs::read(fpath);
    match fbytes {
        Ok(b) => Ok(b),
        Err(e) => {
            let emsg = format!("read_vec_u8 - path: {s} - {e}");
            Err(PyFileNotFoundError::new_err(emsg))
        }
    }
}

#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: &str) -> PyResult<PyObject> {
    let bvec = read_vec_u8(s)?;
    Ok(PyBytes::new_bound(py, &bvec).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: &str) -> PyResult<String> {
    let bvec = read_vec_u8(s)?;
    let r = std::str::from_utf8(&bvec);
    match r {
        Ok(s) => Ok(s.to_string()),
        Err(e) => {
            let decode_err = PyUnicodeDecodeError::new_utf8_bound(py, &bvec, e)?;
            Err(decode_err.into())
        }
    }
}

#[pyfunction]
pub fn write_bytes(fspath: &str, b: Vec<u8>) -> PyResult<()> {
    let fpath = Path::new(fspath);
    let write_res = std::fs::write(fpath, b);
    match write_res {
        Ok(()) => Ok(()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

#[pyfunction]
pub fn write_text(fspath: &str, string: &str) -> PyResult<()> {
    let fpath = Path::new(fspath);
    let write_result = std::fs::write(fpath, string);
    match write_result {
        Ok(()) => Ok(()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

// #[instrument(level = "warn", err, fields(s = module_path!()), ret, skip(_py))]
pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(write_text, m)?)?;
    m.add_function(wrap_pyfunction!(write_bytes, m)?)?;
    Ok(())
}
