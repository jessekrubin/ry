//! Utils for `pyo3::types::PyDict`
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyString};

use crate::dict::BorrowedDictIter;
use crate::pystring::fast_pystr_read;

pub struct KwargsIter<'a, 'py>(BorrowedDictIter<'a, 'py>);

impl<'a, 'py> Iterator for KwargsIter<'a, 'py> {
    type Item = (&'a str, Borrowed<'a, 'py, PyAny>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(move |(key, value)| {
            #[expect(unsafe_code)]
            let borrowed_str = unsafe { key.cast_unchecked::<PyString>() };
            let s = fast_pystr_read(borrowed_str).expect("kwarg is (theoretically) a valid str");
            (s, value)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }
}

impl ExactSizeIterator for KwargsIter<'_, '_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, 'py> KwargsIter<'a, 'py> {
    #[must_use]
    pub fn new(dict: Borrowed<'a, 'py, PyDict>) -> Self {
        let iter = BorrowedDictIter::new(dict);
        KwargsIter(iter)
    }
}
