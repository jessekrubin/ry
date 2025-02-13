pub use crate::functions::*;
pub use crate::ry_date::RyDate;
pub use crate::ry_date_difference::RyDateDifference;
pub use crate::ry_datetime::RyDateTime;
pub use crate::ry_datetime_difference::RyDateTimeDifference;
pub use crate::ry_datetime_round::RyDateTimeRound;
pub use crate::ry_offset::RyOffset;
pub use crate::ry_signed_duration::RySignedDuration;
pub use crate::ry_span::RySpan;
pub use crate::ry_time::RyTime;
pub use crate::ry_time_difference::RyTimeDifference;
pub use crate::ry_timestamp::RyTimestamp;
pub use crate::ry_timestamp_difference::RyTimestampDifference;
pub use crate::ry_timestamp_round::RyTimestampRound;
pub use crate::ry_timezone::RyTimeZone;
pub use crate::ry_zoned::RyZoned;
pub use crate::ry_zoned_difference::RyZonedDifference;

use crate::ry_iso_week_date::RyISOWeekDate;
use crate::ry_zoned_round::RyZonedDateTimeRound;
use pyo3::prelude::*;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    m.add_class::<RyDate>()?;
    m.add_class::<RyDateTime>()?;
    m.add_class::<RyISOWeekDate>()?;
    m.add_class::<RyOffset>()?;
    m.add_class::<RySignedDuration>()?;
    m.add_class::<RySpan>()?;
    m.add_class::<RyTime>()?;
    m.add_class::<RyTimeZone>()?;
    m.add_class::<RyTimestamp>()?;
    m.add_class::<RyZoned>()?;

    // difference
    m.add_class::<RyDateDifference>()?;
    m.add_class::<RyDateTimeDifference>()?;
    m.add_class::<RyTimeDifference>()?;
    m.add_class::<RyTimestampDifference>()?;
    m.add_class::<RyZonedDifference>()?;

    // round
    m.add_class::<RyDateTimeRound>()?;
    m.add_class::<RyZonedDateTimeRound>()?;
    m.add_class::<RyTimestampRound>()?;

    // functions
    m.add_function(wrap_pyfunction!(date, m)?)?;
    m.add_function(wrap_pyfunction!(time, m)?)?;
    m.add_function(wrap_pyfunction!(datetime, m)?)?;
    m.add_function(wrap_pyfunction!(offset, m)?)?;
    m.add_function(wrap_pyfunction!(timespan, m)?)?;

    // okee-dokey
    Ok(())
}
