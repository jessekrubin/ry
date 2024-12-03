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
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]
#![allow(clippy::unwrap_used)]

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod anystr;
pub mod sp;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    anystr::pymod_add(m)?;
    sp::pymod_add(m)?;
    Ok(())
}
