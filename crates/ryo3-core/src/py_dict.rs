//! BORROWED ITERATORS!!!
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};

// use crate::{PyCastExactOpt, pystr_read_fast_opt};

// ----------------------------------------------------------------------------
// DICT
// ----------------------------------------------------------------------------

/// Modified from `pyo3::types::dict`
///
/// Big advantage is ref counts arent messed w/ so only use if you know the
/// dict is not being modified during iteration
pub struct BorrowedDictIter<'a, 'py> {
    dict: Borrowed<'a, 'py, PyDict>,
    ppos: ffi::Py_ssize_t,
    remaining: usize,
}

impl<'a, 'py> BorrowedDictIter<'a, 'py> {
    #[must_use]
    pub fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
        let len = dict.len();
        Self::new_with_len(dict, len)
    }

    #[must_use]
    pub fn new_with_len(dict: Borrowed<'a, 'py, PyDict>, len: usize) -> Self {
        debug_assert!(len == dict.len(), "dict.len ne expected length {len}");
        BorrowedDictIter {
            dict,
            ppos: 0,
            remaining: len,
        }
    }
}

impl<'a, 'py> Iterator for BorrowedDictIter<'a, 'py> {
    type Item = (Borrowed<'a, 'py, PyAny>, Borrowed<'a, 'py, PyAny>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let mut key_ptr = std::mem::MaybeUninit::<*mut ffi::PyObject>::uninit();
        let mut val_ptr = std::mem::MaybeUninit::<*mut ffi::PyObject>::uninit();
        // let mut key_ptr: *mut ffi::PyObject = std::ptr::null_mut();
        // let mut val_ptr: *mut ffi::PyObject = std::ptr::null_mut();

        #[expect(unsafe_code)]
        // Safety: self.dict lives sufficiently long that the pointer is not dangling
        if unsafe {
            ffi::PyDict_Next(
                self.dict.as_ptr(),
                &raw mut self.ppos,
                key_ptr.as_mut_ptr(),
                val_ptr.as_mut_ptr(),
            )
        } != 0
        {
            self.remaining -= 1;
            let py = self.dict.py();
            // Safety:
            // - PyDict_Next returns borrowed values
            // - we have already checked that `PyDict_Next` succeeded, so we can assume these to be non-null
            let key_ptr = unsafe { key_ptr.assume_init() };
            let val_ptr = unsafe { val_ptr.assume_init() };
            let map_key = unsafe { Borrowed::from_ptr(py, key_ptr) };
            let map_val = unsafe { Borrowed::from_ptr(py, val_ptr) };
            Some((map_key, map_val))
        } else {
            self.remaining = 0;
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

// KWARGS TBD

// pub struct KwargsIter<'a, 'py> {
//     dict_iter: BorrowedDictIter<'a, 'py>,
// }

// impl<'a, 'py> KwargsIter<'a, 'py> {
//     #[must_use]
//     pub fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
//         Self {
//             dict_iter: BorrowedDictIter::new(dict),
//         }
//     }
// }

// impl<'a, 'py> Iterator for KwargsIter<'a, 'py> {
//     type Item = (&'a str, Borrowed<'a, 'py, PyAny>);

//     fn next(&mut self) -> Option<Self::Item> {
//         let (key, val) = self.dict_iter.next()?;
//         let pys = key.cast_exact_opt::<pyo3::types::PyString>()?;
//         let key_str = unsafe { pystr_read_fast_opt(pys) }?;
//         Some((key_str, val))
//     }
// }
