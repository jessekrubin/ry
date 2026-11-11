#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use ryo3_core::PyIoResultExt;
use ryo3_core::types::PathLike;

/// Return `True` if pathlike points to same file
#[pyfunction]
pub fn is_same_file(py: Python<'_>, left: PathLike, right: PathLike) -> PyResult<bool> {
    let (l, r) = py.detach(|| {
        (
            same_file::Handle::from_path(&left),
            same_file::Handle::from_path(&right),
        )
    });
    let l = l.map_py_io_err_with_path(left)?;
    let r = r.map_py_io_err_with_path(right)?;
    Ok(l == r)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_same_file, m)?)?;
    Ok(())
}
