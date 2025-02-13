//! ry module registration
//!
//! `ry` does all registration of pyo3 types/fns/classes/mods here

use crate::libs;
use pyo3::prelude::PyModule;
use pyo3::{Bound, PyResult};

pub mod dev;
pub mod sh;
pub mod submodules;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_std::pymod_add(m)?;
    ryo3_fspath::pymod_add(m)?;
    ryo3_quick_maths::pymod_add(m)?;
    sh::pymod_add(m)?;
    libs::pymod_add(m)?;
    // register submodules
    submodules::pymod_add(m)?;
    // dev submodule
    dev::pymod_add(m)?;
    Ok(())
}
