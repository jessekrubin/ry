//! fallible allocation for outputs whose size comes from python
use pyo3::PyErr;
use pyo3::exceptions::{PyMemoryError, PyOverflowError};

/// why an output buffer couldn't be allocated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AllocError {
    /// size can't exist (`> isize::MAX`) -> `OverflowError` (same split as cpython)
    Overflow,
    /// allocator says "no-way-jose" -> `MemoryError`
    Memory,
}

impl std::error::Error for AllocError {}
impl std::fmt::Display for AllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "allocation size overflow"),
            Self::Memory => write!(f, "memory allocation failed"),
        }
    }
}

impl From<AllocError> for PyErr {
    fn from(e: AllocError) -> Self {
        match e {
            AllocError::Overflow => PyOverflowError::new_err(()),
            AllocError::Memory => PyMemoryError::new_err(()),
        }
    }
}

/// `Vec<u8>` w/ capacity for exactly `len` bytes w/o panicking/aborting
///
/// TODO: match on `TryReserveError::kind()` once `try_reserve_kind` is stable
pub(crate) fn try_with_capacity(len: usize) -> Result<Vec<u8>, AllocError> {
    if isize::try_from(len).is_err() {
        return Err(AllocError::Overflow);
    }
    let mut v = Vec::new();
    v.try_reserve_exact(len).map_err(|_| AllocError::Memory)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::{AllocError, try_with_capacity};

    #[test]
    fn exact_capacity() {
        assert!(try_with_capacity(0).is_ok_and(|v| v.capacity() == 0));
        assert!(try_with_capacity(1024).is_ok_and(|v| v.capacity() == 1024 && v.is_empty()));
    }

    #[test]
    fn too_big_is_err() {
        assert_eq!(
            try_with_capacity(isize::MAX.unsigned_abs() + 1),
            Err(AllocError::Overflow)
        );
        assert_eq!(try_with_capacity(usize::MAX), Err(AllocError::Overflow));
        // no allocator hands out isize::MAX bytes
        assert_eq!(
            try_with_capacity(isize::MAX.unsigned_abs()),
            Err(AllocError::Memory)
        );
    }
}
