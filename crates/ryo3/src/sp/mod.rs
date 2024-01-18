use pyo3::prelude::PyModule;
use pyo3::{wrap_pyfunction, PyResult, Python};

mod done;
mod pydone;
pub mod run;

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run::run, m)?)?;
    Ok(())
}
