use pyo3::prelude::*;
#[cfg(not(feature = "experimental-async"))]
use ryo3_tokio_rt::future_into_py;
#[cfg(feature = "experimental-async")]
use ryo3_tokio_rt::on_tokio_py;

async fn sleep_impl(secs: f64) {
    let dur = std::time::Duration::from_secs_f64(secs);
    tokio::time::sleep(dur).await;
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn asleep(py: Python, secs: f64) -> PyResult<Bound<PyAny>> {
    future_into_py(py, async move {
        sleep_impl(secs).await;
        Ok(secs)
    })
}

#[cfg(not(feature = "experimental-async"))]
#[pyfunction]
pub fn sleep_async(py: Python, secs: f64) -> PyResult<Bound<PyAny>> {
    future_into_py(py, async move {
        sleep_impl(secs).await;
        Ok(secs)
    })
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn asleep(secs: f64) -> PyResult<f64> {
    on_tokio_py(async move {
        sleep_impl(secs).await;
        Ok(secs)
    })
    .await
}

#[cfg(feature = "experimental-async")]
#[pyfunction]
pub async fn sleep_async(secs: f64) -> PyResult<f64> {
    on_tokio_py(async move {
        sleep_impl(secs).await;
        Ok(secs)
    })
    .await
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(asleep, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
