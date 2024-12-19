use pyo3::prelude::*;
use ryo3_std::PyDuration;
use std::time::Duration;

#[pyfunction]
pub fn sleep(py: Python<'_>, secs: u64) -> PyResult<f64> {
    let py_duration = PyDuration(Duration::from_secs(secs));
    py_duration.sleep(py, None)?;

    Ok(py_duration.0.as_secs_f64())
}

#[pyfunction]
fn sleep_async(py: Python, secs: f64) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let dur = Duration::from_secs_f64(secs);

        tokio::time::sleep(dur).await;
        Ok(secs)
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
