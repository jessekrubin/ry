use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use reqwest::header::HeaderValue;

const DEFAULT_USER_AGENT: &str = concat!("ry/", env!("CARGO_PKG_VERSION"));

pub(crate) fn parse_user_agent(user_agent: Option<String>) -> PyResult<HeaderValue> {
    let ua_str = user_agent.unwrap_or_else(|| DEFAULT_USER_AGENT.into());
    ua_str
        .parse()
        .map_err(|e| PyValueError::new_err(format!("{e}")))
}
