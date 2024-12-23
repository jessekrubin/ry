//! `FsPath` python module
#![allow(clippy::needless_pass_by_value)] // TODO: remove in future? if possible?

mod fspath;

use crate::fspath::PyFsPath;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFsPath>()?;
    Ok(())
}
