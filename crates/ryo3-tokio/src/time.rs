use pyo3::prelude::*;

// #[pyfunction]
// pub fn sleep_async(py: Python, secs: f64) -> PyResult<Bound<PyAny>> {
//     pyo3_async_runtimes::tokio::future_into_py(py, async move {
//         let dur = std::time::Duration::from_secs_f64(secs);
//
//         tokio::time::sleep(dur).await;
//         Ok(secs)
//     })
// }

// 2 functions macros

// a macro that defines the sleep function as both `sleep_async` and `asleep`

macro_rules! py_sleep_fn {
    ($($name:ident),*) => {
        $(
            #[pyfunction]
            pub fn $name(py: Python, secs: f64) -> PyResult<Bound<PyAny>> {
                pyo3_async_runtimes::tokio::future_into_py(py, async move {
                    let dur = std::time::Duration::from_secs_f64(secs);

                    tokio::time::sleep(dur).await;
                    Ok(secs)
                })
            }
        )*
    };
}

py_sleep_fn!(sleep_async, asleep);
