use pyo3::prelude::*;

use tracing::debug;
mod lager;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &str = env!("PROFILE");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

/// Python utils and common wrappers written in rust!
#[pymodule]
#[pyo3(name = "_ry")]
fn ry(py: Python, m: &PyModule) -> PyResult<()> {
    lager::tracing_init();
    debug!("version: {}", VERSION);
    debug!("build_profile: {}", BUILD_PROFILE);
    debug!("build_timestamp: {}", BUILD_TIMESTAMP);

    m.add("__version__", VERSION)?;
    m.add("__build_profile__", BUILD_PROFILE)?;
    m.add("__build_timestamp__", BUILD_TIMESTAMP)?;

    // register/add core lib from ryo3
    ryo3::madd(py, m)?;
    Ok(())
}
