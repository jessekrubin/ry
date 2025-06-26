use pyo3::prelude::*;
use pyo3::{intern, types::PyDict};

#[cfg(feature = "dirs")]
#[pymodule(gil_used = false, submodule, name = "dirs")]
pub fn dirs_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_dirs::pymod_add(m)?;
    Ok(())
}

// #[cfg(feature = "http")]
// #[pymodule(gil_used = false, name = "http")]
// pub fn http(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     ryo3_http::pymod_add(m)?;
//     Ok(())
// }

// #[cfg(feature = "jiter")]
// #[pymodule(gil_used = false, name = "JSON")]
// pub fn json(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(ryo3_json::stringify, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_json::dumps, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiter::parse, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiter::loads, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiter::cache_clear, m)?)?;
//     m.add_function(wrap_pyfunction!(ryo3_jiter::cache_usage, m)?)?;
//     Ok(())
// }
//
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

#[cfg(feature = "xxhash")]
#[pymodule(gil_used = false, submodule, name = "xxhash")]
pub fn xxhash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_xxhash::pymod_add(m)?;
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
    let sys_modules = sys
        .getattr(intern!(py, "modules"))?
        .downcast_into::<PyDict>()?;

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

    #[cfg(feature = "xxhash")]
    m.add_wrapped(pyo3::wrap_pymodule!(xxhash))?;

    sys_modules.set_item(intern!(py, "ry.dirs"), m.getattr(intern!(py, "dirs"))?)?;
    let attr = m.getattr(intern!(py, "dirs"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.dirs"))?;

    // // http
    // sys_modules.set_item(intern!(py, "ry.http"), m.getattr(intern!(py, "http"))?)?;
    // let attr = m.getattr(intern!(py, "http"))?;
    // attr.setattr(intern!(py, "__name__"), intern!(py, "ry.http"))?;

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
