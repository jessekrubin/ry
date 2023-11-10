// use tracing::{info};
use tracing_subscriber::EnvFilter;
// export the macros;
pub use tracing::*;

/// Return the EnvFilter directive to use for initializing the tracing subscriber,
/// Looks for the following environment variables, in order:
///   "RYDEBUG" - truthy value enables debug logging
///   "RYLOG" - returns
/// otherwise using 'RUST_LOG' if set.
fn env_filter_directives() -> String {
  if let Ok(ry_debug) = std::env::var("RYDEBUG") {
    if ry_debug == "1" || ry_debug == "true" || ry_debug == "TRUE" {
      return "debug".to_string();
    }
  }

  // use "RYLOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
  if let Ok(ry_log) = std::env::var("RYLOG") {
    let ry_log_lower = ry_log.to_lowercase();
    if ry_log_lower == "1" || ry_log_lower == "true" || ry_log_lower == "on" || ry_log_lower == "yes" || ry_log_lower == "y" {
      return "debug".to_string();
    }
  }

  let rust_log = std::env::var("RUST_LOG").unwrap_or("warn".to_string());
  rust_log
}

pub fn tracing_init() {
  // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
  let env_filter_directives_string = env_filter_directives();
  let filter = EnvFilter::new(
    &env_filter_directives_string
  );
  info!("tracing_init - env_filter_directives_string: {}", env_filter_directives_string);
  // Install the global collector configured based on the filter.
  tracing_subscriber::fmt()
      .with_env_filter(filter)
      .with_writer(std::io::stderr)
      .init();
}
