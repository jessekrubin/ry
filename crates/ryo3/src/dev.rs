use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{intern, wrap_pymodule};

#[pymodule(gil_used = false, submodule, name = "_dev")]
pub fn dev(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_dev::pymod_add(m)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    m.add_wrapped(wrap_pymodule!(dev))?;

    // renaming
    let sys = PyModule::import(py, intern!(py, "sys"))?;
    let sys_modules = sys
        .getattr(intern!(py, "modules"))?
        .downcast_into::<PyDict>()?;

    sys_modules.set_item("ry.ryo3._dev", m.getattr("_dev")?)?;
    let attr = m.getattr("_dev")?;
    attr.setattr("__name__", "ry.ryo3._dev")?;

    Ok(())
}
