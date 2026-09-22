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
        // if self.remaining == 0 {
        //     return None;
        // }
        let mut key_ptr = std::mem::MaybeUninit::<*mut ffi::PyObject>::uninit();
        let mut val_ptr = std::mem::MaybeUninit::<*mut ffi::PyObject>::uninit();
        // pydict-next returns 1 if more items, 0 if donezo
        #[expect(unsafe_code)]
        // Safety: self.dict lives long enuf that the ptr aint dangling
        let found = unsafe {
            ffi::PyDict_Next(
                self.dict.as_ptr(),
                &raw mut self.ppos,
                key_ptr.as_mut_ptr(),
                val_ptr.as_mut_ptr(),
            )
        } != 0;
        if found {
            self.remaining -= 1;
            let py = self.dict.py();
            #[expect(unsafe_code)]
            // Safety:
            // - PyDict_Next returns borrowed values
            // - we have already checked that `PyDict_Next` succeeded, so we can assume these to be non-null
            let map_kv = unsafe {
                let key_ptr = key_ptr.assume_init();
                let val_ptr = val_ptr.assume_init();
                let map_key = Borrowed::from_ptr(py, key_ptr);
                let map_val = Borrowed::from_ptr(py, val_ptr);
                (map_key, map_val)
            };
            Some(map_kv)
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
pub struct KwargsIter<'a, 'py>(BorrowedDictIter<'a, 'py>);

impl<'a, 'py> KwargsIter<'a, 'py> {
    #[must_use]
    pub fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
        Self(BorrowedDictIter::new(dict))
    }
}

/// Iterator over a kwarg-dict which cpython should (afaict) ensure string keys
///
/// previous version didnt yield `PyResult`, it just yielded the tuple:
///
/// ```rust,ignore
/// impl<'a, 'py> Iterator for KwargsIter<'a, 'py> {
/// type Item = (&'a str, Borrowed<'a, 'py, PyAny>);
///
/// fn next(&mut self) -> Option<Self::Item> {
///         let (key, val) = self.0.next()?;
///         let pys = key::cast_exact_opt::<pyo3::types::PyString>(key);
///         if let Some(pys) = pys {
///             #[expect(unsafe_code)]
///             let key_str = unsafe { crate::pystr_read_fast_opt(pys) }?;
///             Some((key_str, val))
///         } else {
///             None
///         }
///     }
/// }
/// ```
impl<'a, 'py> Iterator for KwargsIter<'a, 'py> {
    type Item = PyResult<(&'a str, Borrowed<'a, 'py, PyAny>)>;
    fn next(&mut self) -> Option<Self::Item> {
        use crate::PyCastExactOpt;
        self.0.next().map(|(key, value)| {
            key.cast_exact_opt::<pyo3::types::PyString>()
                .ok_or_else(|| pyo3::exceptions::PyTypeError::new_err("non-str kwarg"))
                .and_then(crate::pystr_read_fast)
                .map(|key_str| (key_str, value))
        })
    }
}
