use pyo3::exceptions::{PyNotADirectoryError, PyUnicodeDecodeError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ryo3_types::PathLike;

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn read_bytes(py: Python<'_>, s: PathLike) -> PyResult<PyObject> {
    let fbytes = std::fs::read(s)?;
    Ok(PyBytes::new(py, &fbytes).into())
}

#[allow(clippy::needless_pass_by_value)]
#[pyfunction]
pub fn read_text(py: Python<'_>, s: PathLike) -> PyResult<String> {
    let fbytes = std::fs::read(s)?;
    let r = std::str::from_utf8(&fbytes);
    match r {
        Ok(s) => Ok(s.to_string()),
        Err(e) => {
            let decode_err = PyUnicodeDecodeError::new_utf8(py, &fbytes, e)?;
            Err(decode_err.into())
        }
    }
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
