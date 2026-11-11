#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::PathLike;

/// Return `True` if pathlike points to same file
#[pyfunction]
pub fn is_same_file(py: Python<'_>, left: PathLike, right: PathLike) -> PyResult<bool> {
    py.detach(move || {
        let l = same_file::Handle::from_path(&left).map_err(|e| (e, left))?;
        let r = same_file::Handle::from_path(&right).map_err(|e| (e, right))?;
        Ok(l == r)
    })
    .or_else(|(e, path)| Err(e).map_py_io_err_with_path(path))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_same_file, m)?)?;
    Ok(())
}
