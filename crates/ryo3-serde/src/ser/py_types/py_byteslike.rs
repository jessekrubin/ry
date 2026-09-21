use core::marker::PhantomData;

use pyo3::prelude::*;
use serde::ser::{Serialize, Serializer};

use crate::errors::pyerr2sererr;
use crate::ser::{PySerializeTarget, SerdeTarget};

pub(crate) struct PyBytesLikeSerializer<'a, 'py, T = SerdeTarget>
where
    T: PySerializeTarget,
{
    obj: Borrowed<'a, 'py, PyAny>,
    _target: PhantomData<T>,
}

impl<'a, 'py, T> PyBytesLikeSerializer<'a, 'py, T>
where
    T: PySerializeTarget,
{
    #[inline]
    pub(crate) fn new(obj: Borrowed<'a, 'py, PyAny>) -> Self {
        Self {
            obj,
            _target: PhantomData,
        }
    }
}

impl<T> Serialize for PyBytesLikeSerializer<'_, '_, T>
where
    T: PySerializeTarget,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obj.extract::<&[u8]>() {
            Ok(v) => v.serialize(serializer),
            Err(e) => Err(pyerr2sererr(e)),
        }
    }
}
