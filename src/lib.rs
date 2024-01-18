use pyo3::prelude::*;
use tracing::debug;

use ryo3;

mod fmts;
// mod fs;
mod lager;
// mod sh;
mod sp;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &'static str = env!("PROFILE");
const BUILD_TIMESTAMP: &'static str = env!("BUILD_TIMESTAMP");

// #[pymodule]
// fn subry(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
//     m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;
//     Ok(())
// }

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_ry")]
fn ry(py: Python, m: &PyModule) -> PyResult<()> {
    lager::tracing_init();

    debug!("version: {}", VERSION);
    debug!("build_profile: {}", BUILD_PROFILE);
    debug!("build_timestamp: {}", BUILD_TIMESTAMP);
    debug!("build_timestamp\n: {}", BUILD_TIMESTAMP);
    m.add("__version__", VERSION)?;
    m.add("__build_profile__", BUILD_PROFILE)?;
    m.add("__build_timestamp__", BUILD_TIMESTAMP)?;

    // register core lib from ryo3
    ryo3::madd(py, m)?;

    m.add_function(wrap_pyfunction!(fmts::nbytes_str, m)?)?;
    m.add_function(wrap_pyfunction!(sp::run::run, m)?)?;
    // sh::madd(m)?;
    // fs::pymod(m)?;
    Ok(())
}
