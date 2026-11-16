pub(crate) mod duration;
pub(crate) mod functions;
pub(crate) mod instant;
pub(crate) mod timeout;
pub use duration::RyDuration;
pub use functions::{py_duration, py_instant, sleep};
pub use instant::RyInstant;
use pyo3::prelude::*;
pub use timeout::PyTimeout;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    std::sync::LazyLock::force(&instant::REPR_ANCHOR);
    m.add_class::<RyDuration>()?;
    m.add_class::<RyInstant>()?;
    m.add_function(wrap_pyfunction!(functions::py_instant, m)?)?;
    m.add_function(wrap_pyfunction!(functions::py_duration, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sleep, m)?)?;
    Ok(())
}
