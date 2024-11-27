//! ry = rust + python (entry point)
use pyo3::prelude::*;
use tracing::debug;
mod lager;

const PACKAGE: &str = "ry";
const AUTHORS: &str = "jesse rubin <jessekrubin@gmail.com>";
const DESCRIPTION: &str = "ry ~ rush & python & wrappers oh my!";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

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

/// ry = rust + python
///
/// `ry` is a kitchen-sink collection of wrappers for well vetted and popular rust crates
#[pymodule(gil_used = false)]
#[pyo3(name = "ryo3")] // possibly change to `ryo3`?
fn ry(m: &Bound<'_, PyModule>) -> PyResult<()> {
    lager::tracing_init();
    debug!("version: {}", VERSION);
    debug!("build_profile: {}", BUILD_PROFILE);
    debug!("build_timestamp: {}", BUILD_TIMESTAMP);

    m.add("__pkg_name__", PACKAGE)?;
    m.add("__description__", DESCRIPTION)?;
    m.add("__version__", VERSION)?;
    m.add("__build_profile__", BUILD_PROFILE)?;
    m.add("__build_timestamp__", BUILD_TIMESTAMP)?;
    m.add("__authors__", AUTHORS)?;
    // register/add core lib from ryo3
    ryo3::pymod_add(m)?;

    Ok(())
}
