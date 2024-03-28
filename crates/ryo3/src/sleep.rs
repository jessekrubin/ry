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

// #[pyfunction]
// pub fn sleep_async(py: Python<'_>, secs: u64) -> PyResult<&PyAny> {
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let start = tokio::time::Instant::now();
//         tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
//         let end = tokio::time::Instant::now();
//         let duration = end - start;
//         Ok(Python::with_gil(|_py| duration.as_secs_f64()))
//     })
// }

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    // m.add_function(wrap_pyfunction!(sleep_async, m)?)?;
    Ok(())
}
