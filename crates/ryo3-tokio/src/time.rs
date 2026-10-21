use std::time::Duration;

use pyo3::exceptions::{PyOverflowError, PyValueError};
use pyo3::prelude::*;
use ryo3_tokio_rt::{RyCoroutine, tokio_coro};

/// neg/nan -> `ValueError`
/// inf     -> `OverflowError`
#[derive(Debug, Clone, Copy)]
pub struct SleepSecs {
    secs: f64,
    dur: Duration,
}

impl<'py> FromPyObject<'_, 'py> for SleepSecs {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let secs: f64 = obj.extract()?;
        if secs.is_nan() || secs < 0.0 {
            return Err(PyValueError::new_err("sleep secs must be >= 0"));
        }
        let dur = Duration::try_from_secs_f64(secs)
            .map_err(|e| PyOverflowError::new_err(e.to_string()))?;
        Ok(Self { secs, dur })
    }
}

#[pyfunction]
pub fn asleep(py: Python<'_>, secs: SleepSecs) -> PyResult<Bound<'_, RyCoroutine>> {
    let SleepSecs { secs, dur } = secs;
    tokio_coro(py, "asleep", async move {
        tokio::time::sleep(dur).await;
        Ok(secs)
    })
}

#[pyfunction]
pub fn sleep_async(py: Python<'_>, secs: SleepSecs) -> PyResult<Bound<'_, RyCoroutine>> {
    let SleepSecs { secs, dur } = secs;
    tokio_coro(py, "sleep_async", async move {
        tokio::time::sleep(dur).await;
        Ok(secs)
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(asleep, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
