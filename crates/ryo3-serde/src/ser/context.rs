use crate::ob_type_cache::PyTypeCache;
use pyo3::{Bound, PyAny};

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PySerializeContext<'py> {
    pub(crate) default: Option<&'py Bound<'py, PyAny>>,
    pub(crate) typeref: &'py PyTypeCache,
}

impl<'py> PySerializeContext<'py> {
    pub(crate) fn new(default: Option<&'py Bound<'py, PyAny>>, typeref: &'py PyTypeCache) -> Self {
        Self { default, typeref }
    }
}
