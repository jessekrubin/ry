use pyo3::prelude::*;

mod py_semaphore;
pub use py_semaphore::PySemaphore;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySemaphore>()?;
    Ok(())
}
