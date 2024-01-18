use pyo3::{PyResult, Python, wrap_pyfunction};
use pyo3::prelude::PyModule;

mod done;
mod pydone;
pub mod run;

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run::run, m)?)?;
    Ok(())
}