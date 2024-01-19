use pyo3::prelude::PyModule;
use pyo3::{PyResult, Python};

pub mod anystr;
pub mod fmts;
pub mod fs;
pub mod sh;
pub mod shlex;
pub mod sleep;
pub mod sp;
pub mod which;

pub fn madd(py: Python, m: &PyModule) -> PyResult<()> {
    sleep::madd(py, m)?;
    shlex::madd(py, m)?;
    which::madd(py, m)?;
    fmts::madd(py, m)?;
    sh::madd(m)?;
    fs::pymod(m)?;
    sp::madd(py, m)?;
    anystr::madd(py, m)?;
    Ok(())
}
