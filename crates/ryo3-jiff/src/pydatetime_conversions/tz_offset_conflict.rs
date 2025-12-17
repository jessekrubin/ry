use crate::jiff_types::JiffTzOffsetConflict;
use pyo3::prelude::*;
use ryo3_macro_rules::{py_type_err, py_value_err};

const JIFF_TZ_OFFSET_CONFLICTS: &str = "'always-offset', 'always-timezone', 'prefer-offset', 'reject' (underscores and hyphens are interchangeable)";
impl<'py> FromPyObject<'_, 'py> for JiffTzOffsetConflict {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "always_offset" | "always-offset" => {
                    Ok(jiff::tz::OffsetConflict::AlwaysOffset.into())
                }
                "always_timezone" | "always-timezone" => {
                    Ok(jiff::tz::OffsetConflict::AlwaysTimeZone.into())
                }
                "prefer_offset" | "prefer-offset" => {
                    Ok(jiff::tz::OffsetConflict::PreferOffset.into())
                }
                "reject" => Ok(jiff::tz::OffsetConflict::Reject.into()),
                _ => py_value_err!("Invalid era: {s} (options: {JIFF_TZ_OFFSET_CONFLICTS})"),
            }
        } else {
            py_type_err!(
                "Invalid type for tz offset conflict, expected a string (options: {JIFF_TZ_OFFSET_CONFLICTS})"
            )
        }
    }
}
