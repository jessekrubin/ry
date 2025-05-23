use crate::jiff_types::JiffTzDisambiguation;
use pyo3::prelude::*;
use pyo3::types::PyString;

const JIFF_ERA_STRINGS: &str = "'compatible', 'earlier', 'later', 'reject'";
impl FromPyObject<'_> for JiffTzDisambiguation {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffTzDisambiguation> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "compatible" => Ok(::jiff::tz::Disambiguation::Compatible.into()),
                "earlier" => Ok(::jiff::tz::Disambiguation::Earlier.into()),
                "later" => Ok(::jiff::tz::Disambiguation::Later.into()),
                "reject" => Ok(::jiff::tz::Disambiguation::Reject.into()),
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
