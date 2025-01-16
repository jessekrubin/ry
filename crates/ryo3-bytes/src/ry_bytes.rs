use crate::bytes::PyBytes;
use pyo3::prelude::*;

#[pyclass(extends=PyBytes, subclass, name="Bytes", module = "ry.ryo3")]
pub struct RyBytes {}

#[pymethods]
impl RyBytes {
    #[new]
    fn py_new(buf: PyBytes) -> (Self, PyBytes) {
        (RyBytes {}, buf)
    }
}
