use crate::jiff_types::JiffTzOffsetConflict;
use pyo3::prelude::*;
use pyo3::types::PyString;
// /// When the offset and time zone are in conflict, this will always use
// /// the offset to interpret the date time.
// ///
// /// When resolving to a [`AmbiguousZoned`], the time zone attached
// /// to the timestamp will still be the same as the time zone given. The
// /// difference here is that the offset will be adjusted such that it is
// /// correct for the given time zone. However, the timestamp itself will
// /// always match the datetime and offset given (and which is always
// /// unambiguous).
// ///
// /// Basically, you should use this option when you want to keep the exact
// /// time unchanged (as indicated by the datetime and offset), even if it
// /// means a change to civil time.
// AlwaysOffset,
// /// When the offset and time zone are in conflict, this will always use
// /// the time zone to interpret the date time.
// ///
// /// When resolving to an [`AmbiguousZoned`], the offset attached to the
// /// timestamp will always be determined by only looking at the time zone.
// /// This in turn implies that the timestamp returned could be ambiguous,
// /// since this conflict resolution strategy specifically ignores the
// /// offset. (And, we're only at this point because the offset is not
// /// possible for the given time zone, so it can't be used in concert with
// /// the time zone anyway.) This is unlike the `AlwaysOffset` strategy where
// /// the timestamp returned is guaranteed to be unambiguous.
// ///
// /// You should use this option when you want to keep the civil time
// /// unchanged even if it means a change to the exact time.
// AlwaysTimeZone,
// /// Always attempt to use the offset to resolve a datetime to a timestamp,
// /// unless the offset is invalid for the provided time zone. In that case,
// /// use the time zone. When the time zone is used, it's possible for an
// /// ambiguous datetime to be returned.
// ///
// /// See [`ZonedWith::offset_conflict`](crate::ZonedWith::offset_conflict)
// /// for an example of when this strategy is useful.
// PreferOffset,
// /// When the offset and time zone are in conflict, this strategy always
// /// results in conflict resolution returning an error.
// ///
// /// This is the default since a conflict between the offset and the time
// /// zone usually implies an invalid datetime in some way.
// #[default]
// Reject,
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
