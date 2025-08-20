use std::str::FromStr;
use tracing::debug;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;
/// List of environment variables to check for logging level
const LOG_ENV_VARS: [&str; 3] = ["RYLOG", "RY_LOG", "RUST_LOG"];

fn env_var_is_falsey(s: &str) -> bool {
    let s_lower = s.trim().to_lowercase();
    matches!(s_lower.as_str(), "" | "0" | "false" | "off" | "no" | "n")
}

fn env_var_str_is_truthy(s: &str) -> bool {
    !env_var_is_falsey(s)
}

/// Return the EnvFilter directive to use for initializing the tracing subscriber,
/// Looks for the following environment variables, in order:
///   "RYTRACE" - truthy value enables trace logging
///   "RYDEBUG" - truthy value enables debug logging
///   "RYLOG" - returns
/// otherwise using 'RUST_LOG' if set.
fn env_log_level() -> LevelFilter {
    // use "RYTRACE" if set to a truthy value
    if let Ok(ry_trace) = std::env::var("RYTRACE")
        && env_var_str_is_truthy(&ry_trace)
    {
        return LevelFilter::TRACE;
    }

    if let Ok(ry_debug) = std::env::var("RYDEBUG")
        && env_var_str_is_truthy(&ry_debug)
    {
        return LevelFilter::DEBUG;
    }

    for env_var in LOG_ENV_VARS.iter() {
        if let Ok(value) = std::env::var(env_var) {
            if value.is_empty() {
                continue;
            }
            if env_var_is_falsey(&value) {
                continue;
            }
            match LevelFilter::from_str(&value) {
                Ok(level) => return level,
                Err(_) => {
                    return LevelFilter::DEBUG;
                }
            }
        }
    }
    LevelFilter::WARN
}

pub fn tracing_init() -> Result<(), Box<dyn std::error::Error>> {
    // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
    LogTracer::init()?;
    let env_log_level = env_log_level();
    debug!(
        "tracing_init - env_filter_directives_string: {}",
        env_log_level
    );
    let subscriber = tracing_subscriber::fmt()
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::CLOSE
                | tracing_subscriber::fmt::format::FmtSpan::ENTER,
        )
        .with_writer(std::io::stderr)
        .with_max_level(env_log_level)
        .finish();
    let set_subscriber_result = tracing::subscriber::set_global_default(subscriber);
    match set_subscriber_result {
        Ok(()) => {
            debug!("tracing_init - set_global_default succeeded");
        }
        Err(e) => {
            debug!("tracing_init - set_global_default failed: {:?}", e);
        }
    }
    Ok(())
}

// TODO: add ability to reload tracing subscriber...
// pub fn tracing_reload() {
//     todo!("tracing_reload")
// }
