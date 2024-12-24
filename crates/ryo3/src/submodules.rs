use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{intern, wrap_pymodule};

#[cfg(feature = "dirs")]
#[pymodule(gil_used = false, submodule, name = "dirs")]
pub fn dirs_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_dirs::pymod_add(m)?;
    Ok(())
}
#[cfg(feature = "http")]
#[pymodule(gil_used = false, name = "http")]
pub fn http(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_http::pymod_add(m)?;
    Ok(())
}

#[cfg(feature = "jiter")]
#[pymodule(gil_used = false, name = "JSON")]
pub fn json(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_jiter::pymod_add(m)?;
    Ok(())
}

// #[pymodule(gil_used = false, submodule, name = "reqwest")]
// pub fn reqwest(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     ryo3_reqwest::pymod_add(m)?;
//     Ok(())
// }
#[cfg(feature = "xxhash")]
#[pymodule(gil_used = false, submodule, name = "xxhash")]
pub fn xxhash(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ryo3_xxhash::pymod_add(m)?;
    Ok(())
}
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    #[cfg(feature = "dirs")]
    m.add_wrapped(wrap_pymodule!(dirs_module))?;

    #[cfg(feature = "http")]
    m.add_wrapped(wrap_pymodule!(http))?;

    #[cfg(feature = "jiter")]
    m.add_wrapped(wrap_pymodule!(json))?;

    // #[cfg(feature = "reqwest")]
    // m.add_wrapped(wrap_pymodule!(reqwest))?;

    #[cfg(feature = "xxhash")]
    m.add_wrapped(wrap_pymodule!(xxhash))?;

    // renaming
    let sys = PyModule::import(py, intern!(py, "sys"))?;
    let sys_modules = sys
        .getattr(intern!(py, "modules"))?
        .downcast_into::<PyDict>()?;

    sys_modules.set_item(intern!(py, "ry.dirs"), m.getattr(intern!(py, "dirs"))?)?;
    let attr = m.getattr(intern!(py, "dirs"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.dirs"))?;

    // http
    sys_modules.set_item(intern!(py, "ry.http"), m.getattr(intern!(py, "http"))?)?;
    let attr = m.getattr(intern!(py, "http"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.http"))?;

    // JSON
    sys_modules.set_item(intern!(py, "ry.JSON"), m.getattr(intern!(py, "JSON"))?)?;
    let attr = m.getattr(intern!(py, "JSON"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.JSON"))?;

    // reqwest (TO MOVE)
    // sys_modules.set_item(
    //     intern!(py, "ry.reqwest"),
    //     m.getattr(intern!(py, "reqwest"))?,
    // )?;
    // let attr = m.getattr(intern!(py, "reqwest"))?;
    // attr.setattr(intern!(py, "__name__"), intern!(py, "ry.reqwest"))?;

    // xxhash
    sys_modules.set_item(intern!(py, "ry.xxhash"), m.getattr(intern!(py, "xxhash"))?)?;
    let attr = m.getattr(intern!(py, "xxhash"))?;
    attr.setattr(intern!(py, "__name__"), intern!(py, "ry.xxhash"))?;

    Ok(())
}
