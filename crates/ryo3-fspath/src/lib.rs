//! `FsPath` python module
use pyo3::prelude::*;
mod fspath;
mod pathlib;
pub use fspath::PyFsPath;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
