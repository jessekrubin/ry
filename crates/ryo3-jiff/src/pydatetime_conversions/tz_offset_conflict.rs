use pyo3::prelude::*;
use ryo3_macro_rules::{py_type_err, py_value_err};

use crate::jiff_types::JiffTzOffsetConflict;

const JIFF_TZ_OFFSET_CONFLICTS: &str = "'always-offset', 'always-timezone', 'prefer-offset', 'reject' (underscores and hyphens are interchangeable)";
impl<'py> FromPyObject<'_, 'py> for JiffTzOffsetConflict {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "always-offset" | "always_offset" => Ok(Self::ALWAYS_OFFSET),
                "always-timezone" | "always_timezone" => Ok(Self::ALWAYS_TIME_ZONE),
                "prefer-offset" | "prefer_offset" => Ok(Self::PREFER_OFFSET),
                "reject" => Ok(Self::REJECT),
                _ => py_value_err!(
                    "Invalid tz offset conflict: {s} (options: {JIFF_TZ_OFFSET_CONFLICTS})"
                ),
            }
        } else {
            py_type_err!(
                "Invalid type for tz offset conflict, expected a string (options: {JIFF_TZ_OFFSET_CONFLICTS})"
            )
        }
    }
}
