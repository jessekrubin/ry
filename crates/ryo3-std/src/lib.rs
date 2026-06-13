use pyo3::prelude::*;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "net")]
pub mod net;
pub mod primitive;
#[cfg(feature = "time")]
pub mod time;

#[allow(unused_variables)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    primitive::pymod_add(m)?;
    #[cfg(feature = "fs")]
    fs::pymod_add(m)?;
    #[cfg(feature = "net")]
    net::pymod_add(m)?;
    #[cfg(feature = "time")]
    time::pymod_add(m)?;
    Ok(())
}
