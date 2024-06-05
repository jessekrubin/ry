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

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod dev;
pub mod fmts;
pub mod fs;
pub mod libs;
pub mod sh;
pub mod sleep;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    dev::madd(m)?;
    sleep::madd(m)?;
    fmts::madd(m)?;
    sh::madd(m)?;
    fs::madd(m)?;
    libs::madd(m)?;
    Ok(())
}
