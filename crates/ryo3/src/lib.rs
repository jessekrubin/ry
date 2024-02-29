#![deny(clippy::all)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::correctness)]
#![warn(clippy::must_use_candidate)]

use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

pub mod dev;
pub mod fmts;
pub mod fnv;
pub mod fs;
pub mod libs;
pub mod sh;
pub mod shlex;
pub mod sleep;
pub mod sp;
pub mod walkdir;
pub mod which;

pub fn madd(py: Python, m: &PyModule) -> PyResult<()> {
    dev::madd(py, m)?;
    sleep::madd(py, m)?;
    shlex::madd(py, m)?;
    which::madd(py, m)?;
    fmts::madd(py, m)?;
    sh::madd(py, m)?;
    fs::madd(py, m)?;
    sp::madd(py, m)?;
    walkdir::madd(py, m)?;
    fnv::madd(py, m)?;
    libs::madd(py, m)?;
    Ok(())
}
