use pyo3::prelude::*;
use pyo3::{intern, types::PyDict};

#[cfg(feature = "dirs")]
#[pymodule(gil_used = false, submodule, name = "dirs")]
pub fn dirs_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_dirs::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "uuid")]
#[pymodule(gil_used = false, name = "uuid")]
pub fn uuid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_uuid::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "ulid")]
#[pymodule(gil_used = false, name = "ulid")]
pub fn ulid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_ulid::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "twox-hash")]
#[pymodule(gil_used = false, submodule, name = "xxhash")]
pub fn twox_hash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_twox_hash::pymod_add(m)?;
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

    #[cfg(feature = "dirs")]
    m.add_wrapped(pyo3::wrap_pymodule!(dirs_module))?;

    #[cfg(feature = "ulid")]
    m.add_wrapped(pyo3::wrap_pymodule!(ulid))?;

    #[cfg(feature = "uuid")]
    m.add_wrapped(pyo3::wrap_pymodule!(uuid))?;

    // #[cfg(feature = "xxhash")]
    // m.add_wrapped(pyo3::wrap_pymodule!(xxhash))?;
    #[cfg(feature = "twox-hash")]
    m.add_wrapped(pyo3::wrap_pymodule!(twox_hash))?;

    sys_modules.set_item(intern!(py, "ry.dirs"), m.getattr(intern!(py, "dirs"))?)?;
    let attr = m.getattr(intern!(py, "dirs"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.dirs"))?;

    // JSON
    sys_modules.set_item(intern!(py, "ry.JSON"), m.getattr(intern!(py, "JSON"))?)?;
    let attr = m.getattr(intern!(py, "JSON"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.JSON"))?;

    // ulid
    sys_modules.set_item(intern!(py, "ry.ulid"), m.getattr(intern!(py, "ulid"))?)?;
    let attr = m.getattr(intern!(py, "ulid"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.ulid"))?;

    // uuid
    sys_modules.set_item(intern!(py, "ry.uuid"), m.getattr(intern!(py, "uuid"))?)?;
    m.getattr(intern!(py, "uuid"))
        .and_then(|attr| attr.setattr(intern!(py, "__name__"), intern!(py, "ry.uuid")))?;

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
