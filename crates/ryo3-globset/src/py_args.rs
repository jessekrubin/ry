use pyo3::prelude::*;
use pyo3::types::PyTuple;

pub(crate) struct VarArgs<'a, 'py>(pub(crate) Borrowed<'a, 'py, PyTuple>);

impl<'a, 'py> VarArgs<'a, 'py> {
    pub(crate) fn into_inner(self) -> Borrowed<'a, 'py, PyTuple> {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for VarArgs<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        Ok(Self(obj.cast_exact()?))
    }
}
