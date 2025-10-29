//! ry = rust + python
use pyo3::{IntoPyObjectExt, prelude::*};
mod lager;

const PACKAGE: &str = env!("CARGO_PKG_NAME");

const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const OPT_LEVEL_STR: &str = env!("OPT_LEVEL");
const TARGET: &str = env!("TARGET");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
const ALLOCATOR: &str = "mimalloc";

#[cfg(not(feature = "mimalloc"))]
const ALLOCATOR: &str = "system";

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
    let py = m.py();
    let opt_level = match OPT_LEVEL_STR {
        "0" => 0.into_py_any(py),
        "1" => 1.into_py_any(py),
        "2" => 2.into_py_any(py),
        "3" => 3.into_py_any(py),
        _ => OPT_LEVEL_STR.into_py_any(py),
    }?;
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
    m.add("__opt_level__", opt_level)?;
    m.add("__authors__", AUTHORS)?;
    m.add("__target__", TARGET)?;
    m.add("__allocator__", ALLOCATOR)?;
    // ------------------------------------------------------------------------
    ryo3::ry::pymod_add(m)?;
    // ------------------------------------------------------------------------
    let ryo3_init_time = ti.elapsed();
    tracing::debug!(
        build.profile = %BUILD_PROFILE,
        build.target = %TARGET,
        build.timestamp = %BUILD_TIMESTAMP,
        build.version = %VERSION,
        "ryo3-v{VERSION} initialized [{ryo3_init_time:?}]",
    );

    Ok(())
}
