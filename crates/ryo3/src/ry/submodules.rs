use pyo3::prelude::*;

#[cfg(feature = "zstd")]
#[pymodule(gil_used = false, submodule, name = "zstd")]
pub fn zstd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_zstd::pysubmod_register(m)?;
    Ok(())
}

#[cfg_attr(not(feature = "zstd"), expect(unused_variables))]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // zstd (deprecated submodule ~ removal: v0.0.109)
    #[cfg(feature = "zstd")]
    {
        m.add_wrapped(pyo3::wrap_pymodule!(zstd))?;
        let sys_modules = PyModule::import(m.py(), "sys")?
            .getattr("modules")?
            .cast_into::<pyo3::types::PyDict>()?;
        let attr = m.getattr("zstd")?;
        sys_modules.set_item("ry.zstd", &attr)?;
        attr.setattr("__name__", "ry.zstd")?;
    }
    Ok(())
}
