use std::time::Duration;

#[cfg(feature = "experimental-async")]
use pyo3::coroutine::CancelHandle;
use pyo3::exceptions::{PyOverflowError, PyValueError};
use pyo3::prelude::*;
#[cfg(not(feature = "experimental-async"))]
use ryo3_tokio_rt::future_into_py;
#[cfg(feature = "experimental-async")]
use ryo3_tokio_rt::on_tokio_py_cancel;

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

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn asleep(py: Python, secs: SleepSecs) -> PyResult<Bound<PyAny>> {
    let SleepSecs { secs, dur } = secs;
    future_into_py(py, async move {
        tokio::time::sleep(dur).await;
        Ok(secs)
    })
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn sleep_async(py: Python, secs: SleepSecs) -> PyResult<Bound<PyAny>> {
    let SleepSecs { secs, dur } = secs;
    future_into_py(py, async move {
        tokio::time::sleep(dur).await;
        Ok(secs)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn asleep(secs: SleepSecs, #[pyo3(cancel_handle)] cancel: CancelHandle) -> PyResult<f64> {
    let SleepSecs { secs, dur } = secs;
    on_tokio_py_cancel(
        async move {
            tokio::time::sleep(dur).await;
            Ok(secs)
        },
        cancel,
    )
    .await
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn sleep_async(
    secs: SleepSecs,
    #[pyo3(cancel_handle)] cancel: CancelHandle,
) -> PyResult<f64> {
    let SleepSecs { secs, dur } = secs;
    on_tokio_py_cancel(
        async move {
            tokio::time::sleep(dur).await;
            Ok(secs)
        },
        cancel,
    )
    .await
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(asleep, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
