use std::time::Instant;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use super::{RyDuration, RyInstant};

#[pyfunction]
#[pyo3(signature = (secs, interval = 100))]
pub fn sleep(py: Python<'_>, secs: f64, interval: u64) -> PyResult<f64> {
    if secs < 0.0 {
        Err(PyValueError::new_err("sleep ~ secs must be >= 0."))
    } else {
        let returned = secs;
        let py_duration = RyDuration::try_from_secs_f64(secs)?;
        py_duration.sleep(py, interval)?;
        Ok(returned)
    }
}

#[must_use]
#[pyfunction]
#[pyo3(name = "instant")]
pub fn py_instant() -> RyInstant {
    RyInstant::from(Instant::now())
}

#[pyfunction]
#[pyo3(name = "duration", signature = (secs = 0, nanos = 0))]
pub fn py_duration(secs: u64, nanos: u32) -> PyResult<RyDuration> {
    RyDuration::new(secs, nanos)
}
