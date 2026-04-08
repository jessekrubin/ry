//! dev module
//!
//! makes a submodule `_dev` and renames it to `ry.ryo3._dev` containing all
//! dev exports
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
    let sys_modules = sys.getattr(intern!(py, "modules"))?.cast_into::<PyDict>()?;

    // dev module
    let pystr_dev = intern!(py, "_dev");
    let pystr_ryo3_dev = intern!(py, "ry.ryo3._dev");
    sys_modules.set_item(pystr_ryo3_dev, m.getattr(pystr_dev)?)?;
    let attr = m.getattr(pystr_dev)?;
    attr.setattr(intern!(py, "__name__"), pystr_ryo3_dev)?;

    Ok(())
}
