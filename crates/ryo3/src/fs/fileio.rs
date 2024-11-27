use std::path::Path;

use crate::fs::fspath::PyFsPath;
use pyo3::exceptions::{PyFileNotFoundError, PyNotADirectoryError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};
use pyo3::{pyfunction, wrap_pyfunction, PyResult};
use ryo3_types::PathLike;

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
pub fn read_bytes(py: Python<'_>, s: PathLike) -> PyResult<PyObject> {
    let fspath = PyFsPath::from(s);
    let bvec = fspath.read_vec_u8()?;
    Ok(PyBytes::new(py, &bvec).into())
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: PathLike) -> PyResult<String> {
    let fspath = PyFsPath::from(s);
    fspath.read_text(py)
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

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_text, m)?)?;
    m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(write_text, m)?)?;
    m.add_function(wrap_pyfunction!(write_bytes, m)?)?;
    Ok(())
}
