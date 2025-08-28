pub use duration::PyDuration;
pub use instant::PyInstant;
use pyo3::prelude::*;
pub(crate) mod duration;
pub(crate) mod instant;
pub(crate) mod sleep;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDuration>()?;
    m.add_class::<PyInstant>()?;
    m.add_function(wrap_pyfunction!(instant::instant, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;
    Ok(())
}
