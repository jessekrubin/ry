//! ry = rust + python
use pyo3::prelude::*;
mod lager;

const PACKAGE: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const TARGET: &str = env!("TARGET");

/// Raise `pyo3::exceptions::PyRuntimeWarning` for debug build(s)
///
/// Taken from `obstore` pyo3 library [obstore](https://github.com/developmentseed/obstore.git)
#[cfg(debug_assertions)]
#[pyfunction]
fn warn_debug_build(py: Python) -> PyResult<()> {
    use pyo3::exceptions::PyRuntimeWarning;
    use pyo3::intern;
    use pyo3::types::PyTuple;
    let warnings_mod = py.import(intern!(py, "warnings"))?;
    let warning = PyRuntimeWarning::new_err("ry not compiled in release mode");
    let args = PyTuple::new(py, vec![warning])?;
    warnings_mod.call_method1(intern!(py, "warn"), args)?;
    tracing::warn!("ry not compiled in release mode");
    Ok(())
}

/// ry = rust + python
#[pymodule(gil_used = false)]
#[pyo3(name = "ryo3")]
fn ry(m: &Bound<'_, PyModule>) -> PyResult<()> {
    lager::tracing_init().map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to initialize logging: {e}"))
    })?;
    // ------------------------------------------------------------------------
    #[cfg(debug_assertions)]
    warn_debug_build(m.py())?;
    // ------------------------------------------------------------------------
    let ti = std::time::Instant::now();
    m.add("__pkg_name__", PACKAGE)?;
    m.add("__description__", DESCRIPTION)?;
    m.add("__version__", VERSION)?;
    m.add("__build_profile__", BUILD_PROFILE)?;
    m.add("__build_timestamp__", BUILD_TIMESTAMP)?;
    m.add("__authors__", AUTHORS)?;
    m.add("__target__", TARGET)?;
    // ------------------------------------------------------------------------
    // register/add core lib from ryo3
    ryo3::ry::pymod_add(m)?;
    // ------------------------------------------------------------------------
    tracing::debug!(
        build.version = %VERSION,
        build.profile = %BUILD_PROFILE,
        build.timestamp = %BUILD_TIMESTAMP,
        "ryo3-init: {:?}", ti.elapsed()
    );

    Ok(())
}
