use pyo3::prelude::*;
use tracing::{debug};
use tracing_subscriber::EnvFilter;
mod sleep;
mod fmts;
mod sp;
mod lager;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BUILD_PROFILE: &'static str = env!("PROFILE");
const BUILD_TIMESTAMP: &'static str = env!("BUILD_TIMESTAMP");

#[pymodule]
fn subry(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;
    Ok(())
}

#[pyfunction]
fn nbytes_str(nbytes: u64) -> PyResult<String> {
  Ok(fmts::nbytes_str(nbytes, Option::from(1)).unwrap())
}

fn get_env_filter() -> EnvFilter {
  // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
  if let Ok(ry_log) = std::env::var("RY_LOG") {
    if ry_log == "1" || ry_log == "true" || ry_log == "TRUE" {
      std::env::set_var("RUST_LOG", "debug");
    }else{
      std::env::set_var("RUST_LOG", "warn");
    }
  }

  EnvFilter::from_default_env()
}

fn tracing_init() {
  // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
  let filter = get_env_filter();
  // Install the global collector configured based on the filter.
  tracing_subscriber::fmt()
      .with_env_filter(filter)
      .with_writer(std::io::stderr)
      .init();
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_ry")]
fn ry(_py: Python, m: &PyModule) -> PyResult<()> {
    lager::tracing_init();

    debug!("version: {}", VERSION);
    debug!("build_profile: {}", BUILD_PROFILE);
    debug!("build_timestamp: {}", BUILD_TIMESTAMP);
    debug!("build_timestamp\n: {}", BUILD_TIMESTAMP);

    // m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    // m.add("__build_profile__", env!("PROFILE"))?;
    // m.add("__build_timestamp__", env!("BUILD_TIMESTAMP"))?;
    m.add("__version__", VERSION)?;
    m.add("__build_profile__", BUILD_PROFILE)?;
    m.add("__build_timestamp__",  BUILD_TIMESTAMP)?;


    m.add_function(wrap_pyfunction!(nbytes_str, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep_async, m)?)?;
    m.add_function(wrap_pyfunction!(sleep::sleep, m)?)?;

    m.add_function(wrap_pyfunction!(sp::run, m)?)?;

    Ok(())
}
