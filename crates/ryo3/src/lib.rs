#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod fmts;
pub mod fs;
pub mod libs;
pub mod sh;
pub mod sleep;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_std::pymod_add(m)?;
    ryo3_dev::pymod_add(m)?;
    ryo3_quick_maths::pymod_add(m)?;
    sleep::pymod_add(m)?;
    fmts::pymod_add(m)?;
    sh::pymod_add(m)?;
    fs::pymod_add(m)?;
    libs::pymod_add(m)?;
    Ok(())
}
