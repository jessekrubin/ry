use core::marker::PhantomData;

use pyo3::{Bound, PyAny};

use crate::ob_type_cache::PyTypeCache;
use crate::ser::{PySerializeTarget, SerdeTarget};

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PySerializeContext<'py, T = SerdeTarget>
where
    T: PySerializeTarget,
{
    pub(crate) default: Option<&'py Bound<'py, PyAny>>,
    pub(crate) typeref: &'py PyTypeCache,
    pub(crate) numpy: bool,
    _target: PhantomData<T>,
}

impl<'py, T> PySerializeContext<'py, T>
where
    T: PySerializeTarget,
{
    pub(crate) fn new(default: Option<&'py Bound<'py, PyAny>>, typeref: &'py PyTypeCache) -> Self {
        Self {
            default,
            typeref,
            numpy: false,
            _target: PhantomData,
        }
    }

    pub(crate) fn new_with_numpy(
        default: Option<&'py Bound<'py, PyAny>>,
        typeref: &'py PyTypeCache,
    ) -> Self {
        Self {
            default,
            typeref,
            numpy: true,
            _target: PhantomData,
        }
    }
}
