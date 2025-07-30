use pyo3::{PyErr, PyResult, exceptions::PyRuntimeError};
use std::sync::{Mutex, MutexGuard, PoisonError};

#[must_use]
pub fn map_poison_error<T>(e: &PoisonError<MutexGuard<'_, T>>) -> PyErr {
    PyErr::new::<PyRuntimeError, _>(format!("Mutex poisoned: {e:?}"))
}

pub trait PyLock<T> {
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>>;
}

impl<T> PyLock<T> for Mutex<T> {
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>> {
        self.lock().map_err(|e| map_poison_error(&e))
    }
}
