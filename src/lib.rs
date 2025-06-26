//! ry = rust + python (entry point)

use pyo3::exceptions::PyRuntimeError;
use pyo3::intern;
use pyo3::prelude::*;
use tracing::debug;
mod lager;

const PACKAGE: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
const TARGET: &str = env!("TARGET");

// #[pyfunction]
// pub fn python_main(ext_filepath: &str) -> PyResult<String> {
//     //     get file size of the extension
//     let size = std::fs::metadata(ext_filepath)?.len();
//     let size_str = nbytes_u64(size, None);
//     let ryo3_json = json!({
//         "abspath": std::fs::canonicalize(ext_filepath).unwrap(),
//         "fsize": size,
//         "fsize_str": size_str,
//         "build_profile": BUILD_PROFILE,
//         "build_timestamp": BUILD_TIMESTAMP,
//     });
//     let v = json!(
//         {
//             "package": PACKAGE,
//             "version": VERSION,
//             "authors": AUTHORS,
//             "ryo3": ryo3_json,
//         }
//     );
//     // jsonify
//     serde_json::to_string_pretty(&v).map_err(|e| PyOSError::new_err(e.to_string()))
// }

/// Raise RuntimeWarning for debug build(s)
///
/// Taken from `obstore` pyo3 library (obstore)[https://github.com/developmentseed/obstore.git]
#[cfg(debug_assertions)]
#[pyfunction]
fn warn_debug_build(_py: Python) -> PyResult<()> {
    use pyo3::exceptions::PyRuntimeWarning;
    use pyo3::intern;
    use pyo3::types::PyTuple;
    let warnings_mod = _py.import(intern!(_py, "warnings"))?;
    let warning = PyRuntimeWarning::new_err("ry not compiled in release mode");
    let args = PyTuple::new(_py, vec![warning])?;
    warnings_mod.call_method1(intern!(_py, "warn"), args)?;
    Ok(())
}

/// ry = rust + python
///
/// `ry` is a kitchen-sink collection of wrappers for well vetted and popular rust crates
#[pymodule(gil_used = false)]
#[pyo3(name = "ryo3")]
fn ry(m: &Bound<'_, PyModule>) -> PyResult<()> {
    lager::tracing_init()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to initialize logging: {e}")))?;
    let ti = std::time::Instant::now();
    #[cfg(debug_assertions)]
    warn_debug_build(m.py())?;
    debug!("version: {}", VERSION);
    debug!("build_profile: {}", BUILD_PROFILE);
    debug!("build_timestamp: {}", BUILD_TIMESTAMP);

    let py = m.py();
    m.add(intern!(py, "__pkg_name__"), PACKAGE)?;
    m.add(intern!(py, "__description__"), DESCRIPTION)?;
    m.add(intern!(py, "__version__"), VERSION)?;
    m.add(intern!(py, "__build_profile__"), BUILD_PROFILE)?;
    m.add(intern!(py, "__build_timestamp__"), BUILD_TIMESTAMP)?;
    m.add(intern!(py, "__authors__"), AUTHORS)?;
    m.add(intern!(py, "__target__"), TARGET)?;

    // register/add core lib from ryo3
    ryo3::ry::pymod_add(m)?;

    debug!("ryo3-init: {:?}", ti.elapsed());
    Ok(())
}
