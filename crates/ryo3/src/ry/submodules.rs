use pyo3::prelude::*;
use pyo3::types::PyDict;

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

#[cfg(feature = "jiter")]
#[pymodule(gil_used = false, name = "JSON")]
pub fn json(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_jiter::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "uuid")]
#[pymodule(gil_used = false, name = "uuid")]
pub fn uuid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_uuid::pymod_add(m)?;
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

    #[cfg(feature = "dirs")]
    m.add_wrapped(pyo3::wrap_pymodule!(dirs_module))?;

    // #[cfg(feature = "http")]
    // m.add_wrapped(wrap_pymodule!(http))?;

    #[cfg(feature = "jiter")]
    m.add_wrapped(pyo3::wrap_pymodule!(json))?;

    #[cfg(feature = "uuid")]
    m.add_wrapped(pyo3::wrap_pymodule!(uuid))?;

    #[cfg(feature = "xxhash")]
    m.add_wrapped(pyo3::wrap_pymodule!(xxhash))?;

    // renaming
    let sys = PyModule::import(py, pyo3::intern!(py, "sys"))?;
    let sys_modules = sys
        .getattr(pyo3::intern!(py, "modules"))?
        .downcast_into::<PyDict>()?;

    sys_modules.set_item(
        pyo3::intern!(py, "ry.dirs"),
        m.getattr(pyo3::intern!(py, "dirs"))?,
    )?;
    let attr = m.getattr(pyo3::intern!(py, "dirs"))?;
    attr.setattr(pyo3::intern!(py, "__name__"), pyo3::intern!(py, "ry.dirs"))?;

    // // http
    // sys_modules.set_item(intern!(py, "ry.http"), m.getattr(intern!(py, "http"))?)?;
    // let attr = m.getattr(intern!(py, "http"))?;
    // attr.setattr(intern!(py, "__name__"), intern!(py, "ry.http"))?;

    // JSON
    sys_modules.set_item(
        pyo3::intern!(py, "ry.JSON"),
        m.getattr(pyo3::intern!(py, "JSON"))?,
    )?;
    let attr = m.getattr(pyo3::intern!(py, "JSON"))?;
    attr.setattr(pyo3::intern!(py, "__name__"), pyo3::intern!(py, "ry.JSON"))?;

    // uuid
    sys_modules.set_item(
        pyo3::intern!(py, "ry.uuid"),
        m.getattr(pyo3::intern!(py, "uuid"))?,
    )?;
    let attr = m.getattr(pyo3::intern!(py, "uuid"))?;
    attr.setattr(pyo3::intern!(py, "__name__"), pyo3::intern!(py, "ry.uuid"))?;

    // xxhash
    sys_modules.set_item(
        pyo3::intern!(py, "ry.xxhash"),
        m.getattr(pyo3::intern!(py, "xxhash"))?,
    )?;
    let attr = m.getattr(pyo3::intern!(py, "xxhash"))?;
    attr.setattr(
        pyo3::intern!(py, "__name__"),
        pyo3::intern!(py, "ry.xxhash"),
    )?;

    // zstd
    #[cfg(feature = "zstd")]
    m.add_wrapped(pyo3::wrap_pymodule!(zstd))?;
    sys_modules.set_item(
        pyo3::intern!(py, "ry.zstd"),
        m.getattr(pyo3::intern!(py, "zstd"))?,
    )?;
    let attr = m.getattr(pyo3::intern!(py, "zstd"))?;
    attr.setattr(pyo3::intern!(py, "__name__"), pyo3::intern!(py, "ry.zstd"))?;
    Ok(())
}
