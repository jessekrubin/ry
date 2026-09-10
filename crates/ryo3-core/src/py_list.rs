//! Borrowed Python list iteration.

use std::iter::FusedIterator;

use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

/// Iterate over a Python list without changing its elements' reference counts.
///
/// Copy-pasta-ed from pyo3's `BorrowedTupleIterator` and modified for lists
///
/// The list MUST NOT modified while this iterator or an item yielded by it
/// is being used!
pub struct BorrowedListIter<'a, 'py> {
    list: Borrowed<'a, 'py, PyList>,
    index: usize,
    length: usize,
}

impl<'a, 'py> BorrowedListIter<'a, 'py> {
    #[must_use]
    pub fn new(list: Borrowed<'a, 'py, PyList>) -> Self {
        let len = list.len();
        Self::new_with_len(list, len)
    }

    #[must_use]
    pub fn new_with_len(list: Borrowed<'a, 'py, PyList>, len: usize) -> Self {
        Self {
            list,
            index: 0,
            length: len,
        }
    }
}

impl<'a, 'py> Iterator for BorrowedListIter<'a, 'py> {
    type Item = Borrowed<'a, 'py, PyAny>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            None
        } else {
            let index = self.index;
            self.index += 1;
            #[expect(unsafe_code)]
            // Safety:
            // - `index` is less than the list length captured at construction
            // - the caller guarantees that the list aint modified during iteration
            unsafe {
                #[cfg(not(Py_LIMITED_API))]
                let ptr = ffi::PyList_GET_ITEM(self.list.as_ptr(), index.cast_signed());
                #[cfg(Py_LIMITED_API)]
                let ptr = ffi::PyList_GetItem(self.list.as_ptr(), index.cast_signed());
                Some(Borrowed::from_ptr(self.list.py(), ptr))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl ExactSizeIterator for BorrowedListIter<'_, '_> {
    #[inline]
    fn len(&self) -> usize {
        self.length - self.index
    }
}

impl FusedIterator for BorrowedListIter<'_, '_> {}
