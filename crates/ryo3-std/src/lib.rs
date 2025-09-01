use pyo3::prelude::*;

#[cfg(feature = "std-fs")]
pub mod fs;
#[cfg(feature = "std-net")]
pub mod net;
#[cfg(feature = "std-time")]
pub mod time;

#[allow(unused_variables)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "std-fs")]
    fs::pymod_add(m)?;
    #[cfg(feature = "std-net")]
    net::pymod_add(m)?;
    #[cfg(feature = "std-time")]
    time::pymod_add(m)?;
    Ok(())
}
