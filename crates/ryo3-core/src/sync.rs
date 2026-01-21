use pyo3::{PyErr, PyResult};
use ryo3_macro_rules::py_runtime_error;
use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait PyLock<T> {
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>>;
}

pub trait PyRead<T> {
    fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>>;
}

pub trait PyWrite<T> {
    fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>>;
}

// ===========================================================================

impl<T> PyLock<T> for Mutex<T> {
    #[inline]
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>> {
        self.lock().map_err(|e| map_mutex_poison_error(e))
    }
}

impl<T> PyRead<T> for RwLock<T> {
    #[inline]
    fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>> {
        self.read().map_err(|e| map_rwlock_read_poison_error(e))
    }
}

impl<T> PyWrite<T> for RwLock<T> {
    #[inline]
    fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>> {
        self.write().map_err(|e| map_rwlock_write_poison_error(e))
    }
}

// ==========================================================================
// MAPERR
// ==========================================================================

/// Maps a poisoned `Mutex` error to a Python `RuntimeError`.
#[must_use]
fn map_mutex_poison_error<T>(e: PoisonError<T>) -> PyErr {
    py_runtime_error!("Mutex poisoned: {e:?}")
}

/// Maps a poisoned `RwLock` error to a Python `RuntimeError`.
#[must_use]
fn map_rwlock_read_poison_error<T>(e: PoisonError<RwLockReadGuard<'_, T>>) -> PyErr {
    py_runtime_error!("RwLock<Read> poisoned: {e:?}")
}

#[must_use]
fn map_rwlock_write_poison_error<T>(e: PoisonError<RwLockWriteGuard<'_, T>>) -> PyErr {
    py_runtime_error!("RwLock<Write> poisoned: {e:?}")
}

// ==========================================================================
// Mutex wrapper that optionally throws on poisoning
// ==========================================================================

/// Python friendly Mutex wrapper that optionally throws on poisoning
// if debug
#[derive(Debug)]
pub struct RyMutex<T, const THROW: bool = true>(pub Mutex<T>);

impl<T, const THROW: bool> RyMutex<T, THROW> {
    pub fn new(value: T) -> Self {
        Self(Mutex::new(value))
    }
}

impl<T> RyMutex<T, true> {
    pub fn py_lock(&self) -> PyResult<MutexGuard<'_, T>> {
        self.0.py_lock()
    }
}

impl<T> RyMutex<T, false> {
    pub fn py_lock(&self) -> MutexGuard<'_, T> {
        // yolo ~ ignore poisoning bc we don't care bc `THROW` is false
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

// ==========================================================================
// RwLock wrapper that optionally throws on poisoning
// ==========================================================================

/// Python friendly RwLock wrapper that optionally throws on poisoning
#[derive(Debug)]
pub struct RyRwLock<T, const THROW: bool = true>(pub RwLock<T>);

impl<T, const THROW: bool> RyRwLock<T, THROW> {
    pub fn new(value: T) -> Self {
        Self(RwLock::new(value))
    }
}

impl<T> RyRwLock<T, true> {
    pub fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>> {
        self.0.py_read()
    }

    pub fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>> {
        self.0.py_write()
    }
}

impl<T> RyRwLock<T, false> {
    pub fn py_read(&self) -> RwLockReadGuard<'_, T> {
        // yolo ~ ignore poisoning bc we don't care bc `THROW` is false
        self.0.read().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn py_write(&self) -> RwLockWriteGuard<'_, T> {
        // yolo ~ ignore poisoning bc we don't care bc `THROW` is false
        self.0.write().unwrap_or_else(PoisonError::into_inner)
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

impl<T> From<RwLock<T>> for RyRwLock<T, true> {
    fn from(mutex: RwLock<T>) -> Self {
        Self(mutex)
    }
}

impl<T> From<RwLock<T>> for RyRwLock<T, false> {
    fn from(mutex: RwLock<T>) -> Self {
        Self(mutex)
    }
}
impl<T, const THROW: bool> From<T> for RyRwLock<T, THROW> {
    fn from(value: T) -> Self {
        Self(RwLock::new(value))
    }
}
