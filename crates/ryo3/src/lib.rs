#![deny(clippy::all)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::correctness)]
#![warn(clippy::must_use_candidate)]

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::PyResult;

pub mod dev;
pub mod fmts;
pub mod fs;
pub mod libs;
pub mod sh;
pub mod shlex;
pub mod sleep;
pub mod sp;
pub mod walkdir;

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    dev::madd(m)?;
    sleep::madd(m)?;
    shlex::madd(m)?;
    fmts::madd(m)?;
    sh::madd(m)?;
    fs::madd(m)?;
    sp::madd(m)?;
    walkdir::madd(m)?;
    libs::madd(m)?;
    Ok(())
}
