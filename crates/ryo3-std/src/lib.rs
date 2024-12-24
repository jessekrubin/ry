use pyo3::prelude::*;

pub mod fs;
pub mod time;

pub use fs::*;
pub use time::duration::PyDuration;
pub use time::instant::PyInstant;
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    time::pymod_add(m)?;
    fs::pymod_add(m)?;
    Ok(())
}
