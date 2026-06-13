//! ry module registration
//!
//! `ry` does all registration of pyo3 types/fns/classes/mods here

use pyo3::prelude::*;

use crate::libs;

#[cfg(feature = "dev")]
pub mod dev;
pub mod sh;
pub mod submodules;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_core::errors::pymod_add(m)?;
    ryo3_std::pymod_add(m)?;
    ryo3_fspath::pymod_add(m)?;
    ryo3_quick_maths::pymod_add(m)?;
    ryo3_json::pymod_add(m)?;
    sh::pymod_add(m)?;
    libs::pymod_add(m)?;
    // register submodules
    submodules::pymod_add(m)?;
    // dev submodule
    #[cfg(feature = "dev")]
    dev::pymod_add(m)?;
    Ok(())
}
