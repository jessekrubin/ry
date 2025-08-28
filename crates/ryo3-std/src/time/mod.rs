pub(crate) mod duration;
pub(crate) mod functions;
pub(crate) mod instant;
pub use duration::PyDuration;
pub use functions::{py_instant, sleep};
pub use instant::PyInstant;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDuration>()?;
    m.add_class::<PyInstant>()?;
    m.add_function(wrap_pyfunction!(functions::py_instant, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sleep, m)?)?;
    Ok(())
}
