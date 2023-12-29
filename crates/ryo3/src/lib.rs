use pyo3::prelude::PyModule;
use pyo3::{wrap_pyfunction, PyResult, Python};

pub mod fmts;
pub mod shlex;
pub mod sleep;

pub fn madd(py: Python, m: &PyModule) -> PyResult<()> {
    shlex::madd(py, m)?;
    sleep::madd(py, m)?;
    Ok(())
}
