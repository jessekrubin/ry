use pyo3::intern;
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
    let sys = PyModule::import(py, intern!(py, "sys"))?;
    let sys_modules = sys.getattr(intern!(py, "modules"))?.cast_into::<PyDict>()?;

    #[cfg(feature = "jiter")]
    m.add_wrapped(pyo3::wrap_pymodule!(ryo3_json::json_py_module))?;
    sys_modules.set_item(intern!(py, "ry.JSON"), m.getattr(intern!(py, "JSON"))?)?;
    let attr = m.getattr(intern!(py, "JSON"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.JSON"))?;

    #[cfg(feature = "twox-hash")]
    m.add_wrapped(pyo3::wrap_pymodule!(twox_hash))?;

    // JSON
    sys_modules.set_item(intern!(py, "ry.JSON"), m.getattr(intern!(py, "JSON"))?)?;
    let attr = m.getattr(intern!(py, "JSON"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.JSON"))?;

    // xxhash
    sys_modules.set_item(intern!(py, "ry.xxhash"), m.getattr(intern!(py, "xxhash"))?)?;
    let attr = m.getattr(intern!(py, "xxhash"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.xxhash"))?;

    // zstd
    #[cfg(feature = "zstd")]
    m.add_wrapped(pyo3::wrap_pymodule!(zstd))?;
    sys_modules.set_item(intern!(py, "ry.zstd"), m.getattr(intern!(py, "zstd"))?)?;
    let attr = m.getattr(intern!(py, "zstd"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.zstd"))?;
    Ok(())
}
