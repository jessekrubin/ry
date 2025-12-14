use pyo3::prelude::*;
use pyo3::types::PyString;

#[inline]
pub(crate) fn any_repr(obj: Borrowed<'_, '_, PyAny>) -> String {
    obj.get_type()
        .name()
        .unwrap_or_else(|_| PyString::new(obj.py(), "unknown"))
        .to_string()
}
