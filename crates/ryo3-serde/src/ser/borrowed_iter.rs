//! BORROWED ITERATORS!!!
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};

// ----------------------------------------------------------------------------
// DICT
// ----------------------------------------------------------------------------

/// Modified from `pyo3::types::dict`
///
/// Big advantage is ref counts arent messed w/ so only use if you know the
/// dict is not being modified during iteration
pub(crate) struct BorrowedDictIter<'a, 'py> {
    dict: Borrowed<'a, 'py, PyDict>,
    ppos: ffi::Py_ssize_t,
    remaining: usize,
}

impl<'a, 'py> Iterator for BorrowedDictIter<'a, 'py> {
    type Item = (Borrowed<'a, 'py, PyAny>, Borrowed<'a, 'py, PyAny>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut key_ptr: *mut ffi::PyObject = std::ptr::null_mut();
        let mut val_ptr: *mut ffi::PyObject = std::ptr::null_mut();

        #[expect(unsafe_code)]
        // Safety: self.dict lives sufficiently long that the pointer is not dangling
        if unsafe {
            ffi::PyDict_Next(
                self.dict.as_ptr(),
                &raw mut self.ppos,
                &raw mut key_ptr,
                &raw mut val_ptr,
            )
        } != 0
        {
            self.remaining -= 1;
            let py = self.dict.py();
            // Safety:
            // - PyDict_Next returns borrowed values
            // - we have already checked that `PyDict_Next` succeeded, so we can assume these to be non-null
            let map_key = unsafe { Borrowed::from_ptr(py, key_ptr) };
            let map_val = unsafe { Borrowed::from_ptr(py, val_ptr) };
            Some((map_key, map_val))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl ExactSizeIterator for BorrowedDictIter<'_, '_> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a, 'py> BorrowedDictIter<'a, 'py> {
    pub(crate) fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
        let len = dict.len();
        BorrowedDictIter {
            dict,
            ppos: 0,
            remaining: len,
        }
    }
}
