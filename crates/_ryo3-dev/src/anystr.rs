/// Rust version of python's typing.AnyStr
/// TBD if this is a good idea or not :/
use pyo3::prelude::*;
use pyo3::types::PyBytes;

// #[derive(Debug, FromPyObject)]
// pub enum AnyStr<'a> {
//     Str(String),
//     Bytes(&'a Bound<'a, PyBytes>),
// }
//
// impl IntoPy<PyObject> for AnyStr<'_> {
//     fn into_py(self, py: Python) -> PyObject {
//         match self {
//             AnyStr::Str(s) => s.into_py(py),
//
//             AnyStr::Bytes(b) => b.into_py(py),
//         }
//     }
// }
//
// #[pyfunction]
// pub fn anystr_noop(s: AnyStr) -> PyResult<AnyStr> {
//     Ok(s)
// }
#[pyfunction]
pub fn anystr_noop<'a>(s: &'a Bound<'a, PyAny>) -> PyResult<&'a Bound<'a, PyAny>> {
    Ok(s)
}

#[pyfunction]
pub fn string_noop(s: String) -> PyResult<String> {
    Ok(s)
}

#[pyfunction]
pub fn bytes_noop<'a>(
    _py: Python<'a>,
    b: &'a Bound<'a, PyBytes>,
) -> PyResult<&'a Bound<'a, PyBytes>> {
    Ok(b)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(anystr_noop, m)?)?;
    m.add_function(wrap_pyfunction!(string_noop, m)?)?;
    m.add_function(wrap_pyfunction!(bytes_noop, m)?)?;
    Ok(())
}
