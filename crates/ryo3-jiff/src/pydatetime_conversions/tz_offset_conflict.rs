use crate::jiff_types::JiffTzOffsetConflict;
use pyo3::prelude::*;
use pyo3::types::PyString;

const JIFF_TZ_OFFSET_CONFLICTS: &str =
    "'always_offset', 'always_timezone', 'prefer_offset', 'reject'";
impl FromPyObject<'_> for JiffTzOffsetConflict {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffTzOffsetConflict> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
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
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Invalid type for era",
            ))
        }
    }
}
