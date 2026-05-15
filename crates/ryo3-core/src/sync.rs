use std::sync::{Mutex, MutexGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use pyo3::sync::{MutexExt, RwLockExt};
use pyo3::{PyResult, Python};
use ryo3_macro_rules::py_runtime_error;

pub trait PyLock<T> {
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>>;
    fn py_with_lock<R>(&self, f: impl FnOnce(&mut T) -> PyResult<R>) -> PyResult<R> {
        self.py_lock().map(|mut g| f(&mut *g))?
    }
}

pub trait PyRead<T> {
    fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>>;
    fn py_with_read<R>(&self, f: impl FnOnce(&T) -> PyResult<R>) -> PyResult<R> {
        self.py_read().map(|g| f(&*g))?
    }
}

pub trait PyWrite<T> {
    fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>>;
    fn py_with_write<R>(&self, f: impl FnOnce(&mut T) -> PyResult<R>) -> PyResult<R> {
        self.py_write().map(|mut g| f(&mut *g))?
    }
}

pub trait PyLockAttached<T> {
    fn py_lock_attached(&self, py: Python<'_>) -> PyResult<MutexGuard<'_, T>>;
    fn py_with_lock_attached<R>(
        &self,
        py: Python<'_>,
        f: impl FnOnce(&mut T) -> PyResult<R>,
    ) -> PyResult<R> {
        self.py_lock_attached(py).map(|mut g| f(&mut *g))?
    }
}

pub trait PyReadAttached<T> {
    fn py_read_attached(&self, py: Python<'_>) -> PyResult<RwLockReadGuard<'_, T>>;
    fn py_with_read_attached<R>(
        &self,
        py: Python<'_>,
        f: impl FnOnce(&T) -> PyResult<R>,
    ) -> PyResult<R> {
        self.py_read_attached(py).map(|g| f(&*g))?
    }
}

pub trait PyWriteAttached<T> {
    fn py_write_attached(&self, py: Python<'_>) -> PyResult<RwLockWriteGuard<'_, T>>;
    fn py_with_write_attached<R>(
        &self,
        py: Python<'_>,
        f: impl FnOnce(&mut T) -> PyResult<R>,
    ) -> PyResult<R> {
        self.py_write_attached(py).map(|mut g| f(&mut *g))?
    }
}

// ===========================================================================

const MUTEX_POISONED_ERR_MSG: &str = "Mutex poisoned error";
const RWLOCK_READ_POISONED_ERR_MSG: &str = "RwLock<Read> poisoned error";
const RWLOCK_WRITE_POISONED_ERR_MSG: &str = "RwLock<Write> poisoned error";

impl<T> PyLock<T> for Mutex<T> {
    #[inline]
    fn py_lock(&self) -> PyResult<MutexGuard<'_, T>> {
        self.lock()
            .map_err(|_| py_runtime_error!(MUTEX_POISONED_ERR_MSG))
    }
}

impl<T> PyRead<T> for RwLock<T> {
    #[inline]
    fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>> {
        self.read()
            .map_err(|_| py_runtime_error!(RWLOCK_READ_POISONED_ERR_MSG))
    }
}

impl<T> PyWrite<T> for RwLock<T> {
    #[inline]
    fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>> {
        self.write()
            .map_err(|_| py_runtime_error!(RWLOCK_WRITE_POISONED_ERR_MSG))
    }
}

impl<T> PyLockAttached<T> for Mutex<T> {
    #[inline]
    fn py_lock_attached(&self, py: Python<'_>) -> PyResult<MutexGuard<'_, T>> {
        self.lock_py_attached(py)
            .map_err(|_| py_runtime_error!(MUTEX_POISONED_ERR_MSG))
    }
}

impl<T> PyReadAttached<T> for RwLock<T> {
    #[inline]
    fn py_read_attached(&self, py: Python<'_>) -> PyResult<RwLockReadGuard<'_, T>> {
        self.read_py_attached(py)
            .map_err(|_| py_runtime_error!(RWLOCK_READ_POISONED_ERR_MSG))
    }
}

impl<T> PyWriteAttached<T> for RwLock<T> {
    #[inline]
    fn py_write_attached(&self, py: Python<'_>) -> PyResult<RwLockWriteGuard<'_, T>> {
        self.write_py_attached(py)
            .map_err(|_| py_runtime_error!(RWLOCK_WRITE_POISONED_ERR_MSG))
    }
}

// ==========================================================================
// Mutex wrapper that optionally throws on poisoning
// ==========================================================================

/// Python friendly `std::sync::Mutex` wrapper that optionally throws on
/// poisoning
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

    pub fn py_with_lock<R>(&self, f: impl FnOnce(&mut T) -> PyResult<R>) -> PyResult<R> {
        self.py_lock().map(|mut g| f(&mut *g))?
    }

    pub fn py_lock_attached(&self, py: Python<'_>) -> PyResult<MutexGuard<'_, T>> {
        self.0.py_lock_attached(py)
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

/// Python friendly `std::sync::RwLock` wrapper that optionally throws on
/// poisoning
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

    pub fn py_read_attached(&self, py: Python<'_>) -> PyResult<RwLockReadGuard<'_, T>> {
        self.0.py_read_attached(py)
    }

    pub fn py_write_attached(&self, py: Python<'_>) -> PyResult<RwLockWriteGuard<'_, T>> {
        self.0.py_write_attached(py)
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
