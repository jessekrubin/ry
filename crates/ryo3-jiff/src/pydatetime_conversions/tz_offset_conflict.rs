use crate::jiff_types::JiffTzOffsetConflict;
use pyo3::prelude::*;

const JIFF_TZ_OFFSET_CONFLICTS: &str = "'always-offset', 'always-timezone', 'prefer-offset', 'reject' (case-insensitive; underscores and hyphens are interchangeable)";
impl FromPyObject<'_> for JiffTzOffsetConflict {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // downcast to string...
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
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid era: {s} (options: {JIFF_TZ_OFFSET_CONFLICTS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid type, expected str (options: {JIFF_TZ_OFFSET_CONFLICTS})"
            )))
        }
    }
}
