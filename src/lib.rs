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

/// ry = rust + python
///
/// `ry` is a kitchen-sink of utils and wrappers around popular rust crates
#[pymodule]
#[pyo3(name = "_ry")]

fn ry(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // fn ry(py: Python, m: &PyModule) -> PyResult<()> {
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
    ryo3::madd(m)?;
    Ok(())
}
