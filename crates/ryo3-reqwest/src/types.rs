use pyo3::prelude::*;
use reqwest::header::HeaderValue;
use ryo3_core::{py_type_err, py_value_error};
use ryo3_http::PyHttpHeaderValue;
use ryo3_std::time::PyDuration;
use std::time::Duration;

use crate::user_agent::DEFAULT_USER_AGENT;

// ============================================================================
// TIMEOUT EXTRACT
// ============================================================================
pub(crate) struct Timeout(Duration);

impl From<Timeout> for Duration {
    fn from(t: Timeout) -> Self {
        t.0
    }
}

impl From<Timeout> for PyDuration {
    fn from(t: Timeout) -> Self {
        Self::from(t.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for Timeout {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(pydur) = obj.cast_exact::<PyDuration>() {
            Ok(Self(pydur.get().into()))
        } else if let Ok(dur) = obj.extract::<Duration>() {
            Ok(Self(dur))
        } else {
            py_type_err!("timeout must be a Duration | datetime.timedelta")
        }
    }
}

// ============================================================================
// USER AGENT EXTRACT
// ============================================================================
pub(crate) enum PyUserAgent {
    Default,
    Disabled,
    Value(PyHttpHeaderValue),
}

impl PyUserAgent {
    #[inline]
    pub(crate) fn default_value() -> PyHttpHeaderValue {
        HeaderValue::from_static(DEFAULT_USER_AGENT).into()
    }
}

impl From<PyUserAgent> for Option<PyHttpHeaderValue> {
    fn from(value: PyUserAgent) -> Self {
        match value {
            PyUserAgent::Default => Some(PyUserAgent::default_value()),
            PyUserAgent::Disabled => None,
            PyUserAgent::Value(value) => Some(value),
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for PyUserAgent {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            return Ok(Self::Default);
        }
        if let Ok(flag) = obj.cast_exact::<pyo3::types::PyBool>() {
            return Ok(if flag.is_true() {
                Self::Default
            } else {
                Self::Disabled
            });
        }
        if let Ok(s) = obj.cast_exact::<pyo3::types::PyString>() {
            let s_str = s.to_str()?;
            return Ok(Self::Value(
                HeaderValue::from_str(s_str)
                    .map_err(|e| py_value_error!("{e}"))?
                    .into(),
            ));
        }
        if let Ok(value) = obj.extract::<&[u8]>() {
            return Ok(Self::Value(
                HeaderValue::from_bytes(value)
                    .map_err(|e| py_value_error!("{e}"))?
                    .into(),
            ));
        }

        py_type_err!("user_agent must be str | http.HeaderValue | bool | None")
    }
}
