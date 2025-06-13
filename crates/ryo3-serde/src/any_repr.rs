use pyo3::prelude::*;
use pyo3::types::PyString;

pub(crate) fn any_repr(obj: &Bound<'_, PyAny>) -> String {
    let typ = obj.get_type();
    let name = typ
        .name()
        .unwrap_or_else(|_| PyString::new(obj.py(), "unknown"));
    match obj.repr() {
        Ok(repr) => format!("{repr} ({name})"),
        Err(_) => name.to_string(),
    }
}
