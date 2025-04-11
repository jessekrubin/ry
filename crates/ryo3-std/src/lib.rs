use pyo3::prelude::*;

pub mod fs;
pub mod net;
pub mod time;

pub use fs::*;
pub use time::duration::PyDuration;
pub use time::instant::PyInstant;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    fs::pymod_add(m)?;
    net::pymod_add(m)?;
    time::pymod_add(m)?;
    Ok(())
}
