//! `FsPath` python module
use pyo3::prelude::*;
mod fspath;
pub use fspath::PyFsPath;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
