use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use reqwest::header::HeaderValue;

pub(crate) fn parse_user_agent(user_agent: Option<String>) -> PyResult<HeaderValue> {
    let ua_str = user_agent.unwrap_or_else(|| {
        format!(
            "ry/{} - OSS (github.com/jessekrubin/ry)",
            env!("CARGO_PKG_VERSION")
        )
    });
    ua_str
        .parse()
        .map_err(|e| PyValueError::new_err(format!("{e}")))
}
