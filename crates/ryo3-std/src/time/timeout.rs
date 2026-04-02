use super::PyDuration;
use pyo3::prelude::*;
use ryo3_core::{py_type_err, py_value_err};
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct PyTimeout(Duration);

impl PyTimeout {
    #[must_use]
    pub const fn from_secs(secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }
}

impl From<PyTimeout> for Duration {
    fn from(t: PyTimeout) -> Self {
        t.0
    }
}

impl From<PyTimeout> for PyDuration {
    fn from(t: PyTimeout) -> Self {
        Self::from(t.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for PyTimeout {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(pydur) = obj.cast_exact::<PyDuration>() {
            Ok(Self(pydur.get().into()))
        } else if let Ok(pydelta) = obj.cast_exact::<pyo3::types::PyDelta>() {
            pydelta.extract::<Duration>().map(Self)
        } else if let Ok(seconds) = obj.extract::<f64>() {
            if !seconds.is_finite() || seconds < 0.0 {
                py_value_err!("timeout must be a positive-finite-number of seconds")
            } else {
                Ok(Self(Duration::from_secs_f64(seconds)))
            }
        } else {
            py_type_err!(
                "timeout must be a Duration | datetime.timedelta | positive number of seconds"
            )
        }
    }
}
