use std::sync::Arc;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use ryo3_tokio_rt::future_into_py;
use tokio::sync::{AcquireError, Semaphore};

//   #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
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

#[pymethods]
impl PySemaphore {
    #[new]
    #[pyo3(signature = (value = 1))]
    fn py_new(value: usize) -> PyResult<Self> {
        if value == 0 {
            return Err(PyValueError::new_err("value must be >= 1"));
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
            let permit = sem.acquire_owned().await.map_err(|e| PyAcquireError(e))?;
            permit.forget();
            Ok(true)
        })
    }

    #[cfg(feature = "experimental-async")]
    async fn acquire(&self) -> PyResult<()> {
        let permit = self
            .0
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| PyAcquireError(e))?;
        permit.forget();
        Ok(())
    }

    #[pyo3(signature = (n = 1))]
    fn release(&self, n: usize) -> PyResult<()> {
        if n == 0 {
            return Err(PyValueError::new_err("n must be >= 1"));
        }
        self.0.add_permits(n);
        Ok(())
    }

    fn __aenter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let sem = self.0.clone();
        future_into_py(py, async move {
            let permit = sem.acquire_owned().await.map_err(|e| PyAcquireError(e))?;
            permit.forget();
            Ok(())
        })
    }

    #[pyo3(name = "__aexit__")]
    fn __aexit__<'py>(
        &self,
        py: Python<'py>,
        _exc_type: Py<PyAny>,
        _exc_value: Py<PyAny>,
        _traceback: Py<PyAny>,
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
        self.0.add_permits(n);
        Ok(())
    }

    fn close(&self) -> PyResult<()> {
        self.0.close();
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    fn available_permits(&self) -> usize {
        self.0.available_permits()
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
