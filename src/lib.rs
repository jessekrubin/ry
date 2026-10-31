//! ry = rust + python
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyDict;
mod lager;

const PACKAGE: &str = env!("CARGO_PKG_NAME");

const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const OPT_LEVEL_STR: &str = env!("OPT_LEVEL");
const TARGET: &str = env!("TARGET");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HOST: &str = env!("HOST");
const RUSTC_VERSION: &str = env!("RUSTC_VERSION");
const LLVM_VERSION: &str = env!("LLVM_VERSION");
const RUSTFLAGS: &str = env!("RUSTFLAGS");
const LINKER: &str = env!("LINKER");
const CC_INFO: &str = env!("CC_INFO");

const GIT_SHA: &str = env!("GIT_SHA");
const GIT_REPO: &str = "git@github.com:jessekrubin/ry.git";

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
const ALLOCATOR: &str = "mimalloc";

#[cfg(not(feature = "mimalloc"))]
const ALLOCATOR: &str = "system";

// if not ring
#[cfg(not(feature = "ring"))]
const CRYPTO_PROVIDER: &str = "aws-lc-rs";

#[cfg(feature = "ring")]
const CRYPTO_PROVIDER: &str = "ring";

/// Raise `pyo3::exceptions::PyRuntimeWarning` for debug build(s)
///
/// Taken from `obstore` pyo3 library [obstore](https://github.com/developmentseed/obstore.git)
#[cfg(debug_assertions)]
#[pyfunction]
fn warn_debug_build(py: Python) -> PyResult<()> {
    use pyo3::exceptions::PyRuntimeWarning;
    use pyo3::types::PyTuple;
    let warnings_mod = py.import("warnings")?;
    let warning = PyRuntimeWarning::new_err("ry not compiled in release mode");
    let args = PyTuple::new(py, vec![warning])?;
    warnings_mod.call_method1("warn", args)?;
    tracing::warn!("ry not compiled in release mode");
    Ok(())
}

/// build info for `python -m ry` (private; not part of the api)
#[pyfunction]
fn _build_info(py: Python<'_>) -> PyResult<Bound<'_, PyDict>> {
    let d = PyDict::new(py);
    d.set_item("host", HOST)?;
    d.set_item("rustc", RUSTC_VERSION)?;
    d.set_item("llvm", LLVM_VERSION)?;
    d.set_item("linker", LINKER)?;
    d.set_item("rustflags", (!RUSTFLAGS.is_empty()).then_some(RUSTFLAGS))?;
    d.set_item("cc", CC_INFO)?;
    Ok(d)
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
    m.add("__git_sha__", GIT_SHA)?;
    m.add("__git_repo__", GIT_REPO)?;
    m.add("__opt_level__", opt_level)?;
    m.add("__authors__", AUTHORS)?;
    m.add("__target__", TARGET)?;
    m.add("__allocator__", ALLOCATOR)?;
    m.add("__crypto_provider__", CRYPTO_PROVIDER)?;
    m.add_function(wrap_pyfunction!(_build_info, m)?)?;

    // ------------------------------------------------------------------------
    ryo3::ry::pymod_add(m)?;
    // ------------------------------------------------------------------------
    let ryo3_init_time = ti.elapsed();
    tracing::debug!(
        build.profile = %BUILD_PROFILE,
        build.git_sha = %GIT_SHA,
        build.git_repo = %GIT_REPO,
        build.target = %TARGET,
        build.host = %HOST,
        build.rustc = %RUSTC_VERSION,
        build.cc = %CC_INFO,
        build.linker = %LINKER,
        build.timestamp = %BUILD_TIMESTAMP,
        build.version = %VERSION,
        build.opt_level = %OPT_LEVEL_STR,
        "ryo3-v{VERSION} initialized [{ryo3_init_time:?}]",
    );
    Ok(())
}
