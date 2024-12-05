use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

fn env_var_str_is_truthy(s: &str) -> bool {
    let s_lower = s.to_lowercase();
    matches!(s_lower.as_str(), "1" | "true" | "on" | "yes" | "y")
}

/// Return the EnvFilter directive to use for initializing the tracing subscriber,
/// Looks for the following environment variables, in order:
///   "RYTRACE" - truthy value enables trace logging
///   "RYDEBUG" - truthy value enables debug logging
///   "RYLOG" - returns
/// otherwise using 'RUST_LOG' if set.
fn env_filter_directives() -> String {
    // use "RYTRACE" if set to a truthy value
    if let Ok(ry_trace) = std::env::var("RYTRACE") {
        if env_var_str_is_truthy(&ry_trace) {
            return "trace".to_string();
        }
    }

    if let Ok(ry_debug) = std::env::var("RYDEBUG") {
        if env_var_str_is_truthy(&ry_debug) {
            return "debug".to_string();
        }
    }

    // use "RYLOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
    if let Ok(ry_log) = std::env::var("RYLOG") {
        let ry_log_lower = ry_log.to_lowercase();
        if env_var_str_is_truthy(&ry_log_lower) {
            return "debug".to_string();
        }
        return ry_log_lower;
    }

    std::env::var("RUST_LOG").unwrap_or("warn".to_string())
}

pub fn tracing_init() {
    // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
    let env_filter_directives_string = env_filter_directives();
    debug!(
        "tracing_init - env_filter_directives_string: {}",
        env_filter_directives_string
    );
    let filter = EnvFilter::new(&env_filter_directives_string);
    info!(
        "tracing_init - env_filter_directives_string: {}",
        env_filter_directives_string
    );
    // Install the global collector configured based on the filter.
    // TODO: add the json and other format(s)...
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::CLOSE
                | tracing_subscriber::fmt::format::FmtSpan::ENTER,
        )
        .with_writer(std::io::stderr)
        .init();
}

// TODO: add ability to reload tracing subscriber...
// pub fn tracing_reload() {
//     todo!("tracing_reload")
// }
