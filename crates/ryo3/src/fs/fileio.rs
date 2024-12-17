use std::path::Path;

use crate::fs::fspath::PyFsPath;
use pyo3::exceptions::{PyFileNotFoundError, PyNotADirectoryError};
use pyo3::prelude::*;
use pyo3::types::PyModule;
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
    fspath.read_bytes(py)
}

#[pyfunction]
pub fn read_text(py: Python<'_>, s: PathLike) -> PyResult<String> {
    let fspath = PyFsPath::from(s);
    fspath.read_text(py)
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_bytes(fspath: PathLike, b: &[u8]) -> PyResult<usize> {
    let write_res = std::fs::write(fspath.as_ref(), b);
    match write_res {
        Ok(()) => Ok(b.len()),
        Err(e) => Err(PyNotADirectoryError::new_err(format!(
            "write_bytes - parent: {fspath} - {e}"
        ))),
    }
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn write_text(fspath: PathLike, string: &str) -> PyResult<usize> {
    let str_bytes = string.as_bytes();
    match std::fs::write(fspath.as_ref(), str_bytes) {
        Ok(()) => Ok(str_bytes.len()),
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
