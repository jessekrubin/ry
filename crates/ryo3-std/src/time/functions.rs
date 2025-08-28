use super::{PyDuration, PyInstant};
use pyo3::exceptions::{PyOverflowError, PyValueError};
use pyo3::prelude::*;
use std::time::{Duration, Instant};

#[pyfunction]
pub fn sleep(py: Python<'_>, secs: f64) -> PyResult<f64> {
    if secs < 0.0 {
        Err(PyValueError::new_err("sleep ~ secs must be >= 0."))
    } else {
        let py_duration = Duration::try_from_secs_f64(secs)
            .map(PyDuration::from)
            // overflow error here b/c negative handled above
            .map_err(|e| PyOverflowError::new_err(format!("{e}")))?;
        py_duration.sleep(py, None)?;
        Ok(py_duration.0.as_secs_f64())
    }
}

#[pyfunction]
#[pyo3(name = "instant")]
#[must_use]
pub fn py_instant() -> PyInstant {
    PyInstant::from(Instant::now())
}
