use crate::type_cache::PyTypeCache;
use pyo3::{Bound, PyAny};

#[derive(Copy, Clone, Debug)]
pub struct PySerializeContext<'py> {
    pub(crate) default: Option<&'py Bound<'py, PyAny>>,
    pub(crate) typeref: &'py PyTypeCache,
}

impl<'py> PySerializeContext<'py> {
    pub fn new(default: Option<&'py Bound<'py, PyAny>>, typeref: &'py PyTypeCache) -> Self {
        Self { default, typeref }
    }

    pub fn with_default(&self, default: Option<&'py Bound<'py, PyAny>>) -> Self {
        Self {
            default,
            typeref: self.typeref,
        }
    }
}
