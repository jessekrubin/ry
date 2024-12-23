use crate::time::instant::PyInstant;
use crate::PyDuration;
use pyo3::prelude::*;

pub mod duration;
pub mod instant;
pub mod sleep;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDuration>()?;
    m.add_class::<PyInstant>()?;
    m.add_function(wrap_pyfunction!(instant::instant, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;
    Ok(())
}
