/// Rust version of python's typing.AnyStr
/// TBD if this is a good idea or not :/
use std::fmt::Debug;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::PyNativeType;
use pyo3::PyTypeInfo;

#[derive(Debug, FromPyObject)]
pub enum AnyStr<'a> {
    Str(String),
    Bytes(&'a PyBytes),
}

impl IntoPy<PyObject> for AnyStr<'_> {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            AnyStr::Str(s) => s.into_py(py),

            AnyStr::Bytes(b) => {
                // b.into()
                b.into_py(py)
                // PyBytes::new(py, &b).into()
            }
        }
    }
}

#[pyfunction]
pub fn anystr_noop(s: AnyStr) -> PyResult<AnyStr> {
    Ok(s)
}

#[pyfunction]
pub fn string_noop(s: String) -> PyResult<String> {
    Ok(s)
}

#[pyfunction]
pub fn bytes_noop<'a>(py: Python<'a>, b: &'a PyBytes) -> PyResult<&'a PyBytes> {
    Ok(b)
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(anystr_noop, m)?)?;
    m.add_function(wrap_pyfunction!(string_noop, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_noop, m)?)?;
    // m.add_class::<AnyStr>()?;
    Ok(())
}
