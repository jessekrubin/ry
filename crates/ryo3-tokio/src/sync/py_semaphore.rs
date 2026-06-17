use std::sync::Arc;

#[cfg(feature = "experimental-async")]
use pyo3::coroutine::CancelHandle;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use ryo3_core::macros::py_value_err;
use ryo3_tokio_rt::future_into_py;
use tokio::sync::{AcquireError, Semaphore};

#[derive(Clone)]
#[pyclass(name = "Semaphore", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySemaphore(Arc<Semaphore>);

pub struct PyAcquireError(AcquireError);

impl From<PyAcquireError> for PyErr {
    fn from(_: PyAcquireError) -> Self {
        PyRuntimeError::new_err("Semaphore is closed")
    }
}

const MAX_PERMITS: usize = tokio::sync::Semaphore::MAX_PERMITS;
const SEMAPHORE_RANGE: std::ops::RangeInclusive<usize> = 1..=MAX_PERMITS;
const INVALID_SEMAPHORE_ERROR: &str = concat!(
    "Invalid semaphore value (range 1..=",
    stringify!(MAX_SEMAPHORE_PERMITS),
    ")"
);

#[pymethods]
impl PySemaphore {
    #[new]
    #[pyo3(signature = (value = 1))]
    fn py_new(value: usize) -> PyResult<Self> {
        // more than 0 and less than MAX_PERMITS
        if !SEMAPHORE_RANGE.contains(&value) {
            return py_value_err!("{INVALID_SEMAPHORE_ERROR}");
        }

        Ok(Self(Arc::new(Semaphore::new(value))))
    }

    #[getter]
    fn value(&self) -> usize {
        self.0.available_permits()
    }

    fn locked(&self) -> bool {
        self.0.available_permits() == 0
    }

    #[cfg(not(feature = "experimental-async"))]
    fn acquire<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let sem = self.0.clone();
        future_into_py(py, async move {
            let permit = sem.acquire_owned().await.map_err(PyAcquireError)?;
            permit.forget();
            Ok(true)
        })
    }

    #[cfg(feature = "experimental-async")]
    async fn acquire(&self, #[pyo3(cancel_handle)] cancel: CancelHandle) -> PyResult<()> {
        use ryo3_tokio_rt::on_tokio_py_cancel;
        let sem = self.0.clone();
        on_tokio_py_cancel(
            async move {
                sem.acquire_owned().await.map_err(PyAcquireError)?;
                Ok(())
            },
            cancel,
        )
        .await
    }

    #[pyo3(signature = (n = 1))]
    fn release(&self, n: usize) -> PyResult<()> {
        self.add_permits(n)
    }

    fn __aenter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let sem = self.0.clone();
        future_into_py(py, async move {
            let permit = sem.acquire_owned().await.map_err(PyAcquireError)?;
            permit.forget();
            Ok(())
        })
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: &Bound<'py, PyAny>,
        _exc_value: &Bound<'py, PyAny>,
        _traceback: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let sem = self.0.clone();
        future_into_py(py, async move {
            sem.add_permits(1);
            Ok(())
        })
    }

    fn forget_permits(&self, n: usize) -> usize {
        self.0.forget_permits(n)
    }

    fn add_permits(&self, n: usize) -> PyResult<()> {
        if (self.0.available_permits() + n) > MAX_PERMITS {
            return py_value_err!("add_permits({n}) would exceed maximum permits ({MAX_PERMITS})");
        }
        self.0.add_permits(n);
        Ok(())
    }

    fn close(&self) {
        self.0.close();
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    fn available_permits(&self) -> usize {
        self.0.available_permits()
    }

    fn __int__(&self) -> usize {
        self.available_permits()
    }
}

// FUTURE?------------------
// pub struct PyTryAcquireError(TryAcquireError);

// impl From<PyTryAcquireError> for PyErr {
//     fn from(err: PyTryAcquireError) -> Self {
//         match err.0 {
//             TryAcquireError::NoPermits => PyRuntimeError::new_err("No permits available"),
//             TryAcquireError::Closed => PyRuntimeError::new_err("Semaphore is closed"),
//         }
//     }
// }
