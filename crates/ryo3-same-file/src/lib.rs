#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::PathLike;

/// Return `True` if pathlike points to same file
#[pyfunction]
pub fn is_same_file(py: Python<'_>, left: PathLike, right: PathLike) -> PyResult<bool> {
    let left_path = left.as_ref();
    let right_path = right.as_ref();
    py.detach(|| {
        let l = same_file::Handle::from_path(left_path).map_err(|e| (e, left_path))?;
        let r = same_file::Handle::from_path(right_path).map_err(|e| (e, right_path))?;
        Ok(l == r)
    })
    .or_else(|(e, path)| Err(e).map_py_io_err_with_path(path))
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_same_file, m)?)?;
    Ok(())
}
