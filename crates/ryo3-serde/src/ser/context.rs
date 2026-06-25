use pyo3::{Bound, PyAny};

use crate::ob_type_cache::PyTypeCache;

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PySerializeContext<'py, T: SerializeTarget> {
    pub(crate) default: Option<&'py Bound<'py, PyAny>>,
    pub(crate) typeref: &'py PyTypeCache,
    pub(crate) _target: T,
}

impl<'py, T: SerializeTarget> PySerializeContext<'py, T> {
    pub(crate) fn new(
        default: Option<&'py Bound<'py, PyAny>>,
        typeref: &'py PyTypeCache,
        target: T,
    ) -> Self {
        Self {
            default,
            typeref,
            _target: target,
        }
    }
}

pub trait SerializeTarget: Copy {
    const SORT_KEYS: bool;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SerdeTarget;

impl SerializeTarget for SerdeTarget {
    const SORT_KEYS: bool = false;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JsonTarget<const SORT_KEYS: bool>;

impl<const SORT_KEYS: bool> SerializeTarget for JsonTarget<SORT_KEYS> {
    const SORT_KEYS: bool = SORT_KEYS;
}
