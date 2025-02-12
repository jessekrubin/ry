//! `FsPath` python module
mod fspath;

use crate::fspath::PyFsPath;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
