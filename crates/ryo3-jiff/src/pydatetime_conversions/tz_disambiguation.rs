use pyo3::prelude::*;
use ryo3_macro_rules::{py_type_err, py_value_err};

use crate::jiff_types::JiffTzDisambiguation;

const JIFF_TZ_DISAMBIGUATION_STRINGS: &str = "'compatible', 'earlier', 'later', 'reject'";
impl<'py> FromPyObject<'_, 'py> for JiffTzDisambiguation {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "compatible" => Ok(Self::COMPATIBLE),
                "earlier" => Ok(Self::EARLIER),
                "later" => Ok(Self::LATER),
                "reject" => Ok(Self::REJECT),
                _ => py_value_err!(
                    "Invalid tz disambiguation: {s} (options: {JIFF_TZ_DISAMBIGUATION_STRINGS})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for tz disambiguation, expected a string (options: {JIFF_TZ_DISAMBIGUATION_STRINGS})"
            )
        }
    }
}
