use pyo3::prelude::*;
use ryo3_core::py_type_err;
use ryo3_std::time::PyDuration;
use std::time::Duration;

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
        PyDuration::from(t.0)
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
