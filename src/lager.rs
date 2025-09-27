use std::str::FromStr;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;
/// List of environment variables to check for logging level
const LOG_ENV_VARS: [&str; 3] = ["RYLOG", "RY_LOG", "RUST_LOG"];

fn env_var_is_falsey(s: &str) -> bool {
    let s_lower: std::borrow::Cow<'_, str> = s.trim().to_ascii_lowercase().into();
    matches!(s_lower.as_ref(), "" | "0" | "false" | "off" | "no")
}

fn env_var_str_is_truthy(s: &str) -> bool {
    !env_var_is_falsey(s)
}

/// Return the `LevelFilter` directive to use for initializing the tracing subscriber,
///
/// Looks for the following environment variables, in order:
///   "RYTRACE" - truthy value enables trace logging
///   "RYDEBUG" - truthy value enables debug logging
///   "RYLOG" - returns
/// otherwise using `RUST_LOG` if set.
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

    LOG_ENV_VARS
        .into_iter()
        .filter_map(|name| std::env::var(name).ok())
        .filter(|v| env_var_str_is_truthy(v))
        .map(|v| LevelFilter::from_str(v.trim()).unwrap_or(LevelFilter::DEBUG))
        .next()
        .unwrap_or(LevelFilter::WARN)
}

pub(crate) fn tracing_init() -> Result<(), Box<dyn std::error::Error>> {
    // use "RY_LOG" if set to a truthy value, otherwise use 'RUST_LOG' if set.
    LogTracer::init()?;
    let env_log_level = env_log_level();
    tracing::trace!(
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
            tracing::trace!("tracing_init - set_global_default succeeded");
        }
        Err(e) => {
            tracing::trace!("tracing_init - set_global_default failed: {:?}", e);
        }
    }
    Ok(())
}

// TODO: add ability to reload tracing subscriber...
// pub fn tracing_reload() {
//     todo!("tracing_reload")
// }
