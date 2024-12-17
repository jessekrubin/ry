use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{intern, wrap_pymodule};

#[cfg(feature = "xxhash")]
#[pymodule(gil_used = false, submodule, name = "xxhash")]
pub fn xxhash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_xxhash::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "jiter")]
#[pymodule(gil_used = false, name = "JSON")]
pub fn json(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_jiter::pymod_add(m)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    #[cfg(feature = "xxhash")]
    m.add_wrapped(wrap_pymodule!(xxhash))?;

    #[cfg(feature = "jiter")]
    m.add_wrapped(wrap_pymodule!(json))?;

    // renaming
    let sys = PyModule::import(py, intern!(py, "sys"))?;
    let sys_modules = sys
        .getattr(intern!(py, "modules"))?
        .downcast_into::<PyDict>()?;

    sys_modules.set_item("ry.xxhash", m.getattr("xxhash")?)?;
    let attr = m.getattr("xxhash")?;
    attr.setattr("__name__", "ry.xxhash")?;

    sys_modules.set_item("ry.JSON", m.getattr("JSON")?)?;
    let attr = m.getattr("JSON")?;
    attr.setattr("__name__", "ry.JSON")?;

    Ok(())
}
