use crate::jiff_types::JiffTzDisambiguation;
use jiff::tz;
use pyo3::prelude::*;

const JIFF_ERA_STRINGS: &str = "'compatible', 'earlier', 'later', 'reject'";
impl<'py> FromPyObject<'_, 'py> for JiffTzDisambiguation {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "compatible" => Ok(tz::Disambiguation::Compatible.into()),
                "earlier" => Ok(tz::Disambiguation::Earlier.into()),
                "later" => Ok(tz::Disambiguation::Later.into()),
                "reject" => Ok(tz::Disambiguation::Reject.into()),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid era: {s} (options: {JIFF_ERA_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Invalid type for era",
            ))
        }
    }
}
