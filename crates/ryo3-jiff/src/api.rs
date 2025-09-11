pub use crate::difference::{
    RyDateDifference, RyDateTimeDifference, RyTimeDifference, RyTimestampDifference,
    RyZonedDifference,
};
pub use crate::functions::*;
use crate::round::RyOffsetRound;
pub use crate::round::{
    RyDateTimeRound, RySignedDurationRound, RyTimeRound, RyTimestampRound, RyZonedDateTimeRound,
};
pub use crate::ry_date::RyDate;
pub use crate::ry_datetime::RyDateTime;
pub use crate::ry_offset::RyOffset;
pub use crate::ry_signed_duration::RySignedDuration;
pub use crate::ry_span::RySpan;
pub use crate::ry_time::RyTime;
pub use crate::ry_timestamp::RyTimestamp;
pub use crate::ry_timezone::RyTimeZone;
use crate::ry_timezone_database::RyTimeZoneDatabase;
pub use crate::ry_zoned::RyZoned;

use crate::ry_iso_week_date::RyISOWeekDate;
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

    // timezone database
    m.add_class::<RyTimeZoneDatabase>()?;

    // difference
    m.add_class::<RyDateDifference>()?;
    m.add_class::<RyDateTimeDifference>()?;
    m.add_class::<RyTimeDifference>()?;
    m.add_class::<RyTimestampDifference>()?;
    m.add_class::<RyZonedDifference>()?;

    // round
    m.add_class::<RyDateTimeRound>()?;
    m.add_class::<RySignedDurationRound>()?;
    m.add_class::<RyTimeRound>()?;
    m.add_class::<RyOffsetRound>()?;
    m.add_class::<RyTimestampRound>()?;
    m.add_class::<RyZonedDateTimeRound>()?;

    // functions
    m.add_function(wrap_pyfunction!(now, m)?)?;
    m.add_function(wrap_pyfunction!(utcnow, m)?)?;
    m.add_function(wrap_pyfunction!(date, m)?)?;
    m.add_function(wrap_pyfunction!(datetime, m)?)?;
    m.add_function(wrap_pyfunction!(offset, m)?)?;
    m.add_function(wrap_pyfunction!(time, m)?)?;
    m.add_function(wrap_pyfunction!(timespan, m)?)?;
    m.add_function(wrap_pyfunction!(zoned, m)?)?;

    // okee-dokey
    Ok(())
}
