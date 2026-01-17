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

#[derive(Debug)]
pub struct RyMutex<T, const THROW: bool = true>(pub Mutex<T>);

impl<T, const THROW: bool> RyMutex<T, THROW> {
    pub fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }
}

impl<T> RyMutex<T, true> {
    pub fn py_lock(&self) -> PyResult<MutexGuard<'_, T>> {
        self.0.lock().map_err(|e| map_poison_error(&e))
    }
}

impl<T> RyMutex<T, false> {
    pub fn py_lock(&self) -> MutexGuard<'_, T> {
        // yolo ~ ignore poisoning bc we don't care
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

// ==========================================================================
// FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM FROM
// ==========================================================================
impl<T> From<Mutex<T>> for RyMutex<T, true> {
    fn from(mutex: Mutex<T>) -> Self {
        Self(mutex)
    }
}

impl<T> From<Mutex<T>> for RyMutex<T, false> {
    fn from(mutex: Mutex<T>) -> Self {
        Self(mutex)
    }
}
impl<T, const THROW: bool> From<T> for RyMutex<T, THROW> {
    fn from(value: T) -> Self {
        Self(Mutex::new(value))
    }
}
