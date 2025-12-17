use crate::jiff_types::JiffTzDisambiguation;
use jiff::tz;
use pyo3::prelude::*;
use ryo3_macro_rules::{py_type_err, py_value_err};

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
                _ => py_value_err!("Invalid era: {s} (options: {JIFF_ERA_STRINGS})"),
            }
        } else {
            py_type_err!("Invalid type for era, expected a string (options: {JIFF_ERA_STRINGS})")
        }
    }
}
