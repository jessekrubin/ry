use pyo3::prelude::*;
use pyo3::types::PyDict;

#[cfg(feature = "twox-hash")]
#[pymodule(gil_used = false, submodule, name = "xxhash")]
pub fn twox_hash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_twox_hash::pysubmod_add(m)?;
    Ok(())
}

#[cfg(feature = "zstd")]
#[pymodule(gil_used = false, submodule, name = "zstd")]
pub fn zstd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_zstd::pysubmod_register(m)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    // renaming
    let sys = PyModule::import(py, "sys")?;
    let sys_modules = sys.getattr("modules")?.cast_into::<PyDict>()?;

    #[cfg(feature = "jiter")]
    m.add_wrapped(pyo3::wrap_pymodule!(ryo3_json::json_py_module))?;
    sys_modules.set_item("ry.JSON", m.getattr("JSON")?)?;
    let attr = m.getattr("JSON")?;
    attr.setattr("__name__", "ry.JSON")?;

    #[cfg(feature = "twox-hash")]
    m.add_wrapped(pyo3::wrap_pymodule!(twox_hash))?;

    // JSON
    sys_modules.set_item("ry.JSON", m.getattr("JSON")?)?;
    let attr = m.getattr("JSON")?;
    attr.setattr("__name__", "ry.JSON")?;

    // xxhash
    sys_modules.set_item("ry.xxhash", m.getattr("xxhash")?)?;
    let attr = m.getattr("xxhash")?;
    attr.setattr("__name__", "ry.xxhash")?;

    // zstd
    #[cfg(feature = "zstd")]
    m.add_wrapped(pyo3::wrap_pymodule!(zstd))?;
    sys_modules.set_item("ry.zstd", m.getattr("zstd")?)?;
    let attr = m.getattr("zstd")?;
    attr.setattr("__name__", "ry.zstd")?;
    Ok(())
}
