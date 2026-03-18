use pyo3::prelude::*;

mod done;
mod pydone;
pub mod run;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run::run, m)?)?;
    Ok(())
}
