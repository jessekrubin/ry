use pyo3::prelude::*;

#[pyfunction]
#[must_use]
pub fn sleep(secs: u64) -> f64 {
    let start = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_secs(secs));
    let end = std::time::Instant::now();
    let duration = end - start;
    duration.as_secs_f64()
}

#[pyfunction]
fn sleep_async(py: Python, secs: u64) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
        // return
        Ok(secs)
        // Ok(())
    })
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
