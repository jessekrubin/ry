use crate::{
    RyDate, RyDateTime, RyISOWeekDate, RyOffset, RySignedDuration, RySpan, RyTime, RyTimeZone,
    RyTimestamp, RyZoned, isoformat::parse_iso_week_date,
};
use jiff::{civil::DateTime, tz::TimeZone};
use ryo3_core::PyFromStr;
use std::str::FromStr;

macro_rules! impl_ry_jiff_from_str {
    (
        $ryo3_type:ty
    ) => {
        impl FromStr for $ryo3_type {
            type Err = jiff::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let jiff_ob = s.parse()?;
                Ok(Self(jiff_ob))
            }
        }
    };
}

impl_ry_jiff_from_str!(RyDate);
impl_ry_jiff_from_str!(RySignedDuration);
impl_ry_jiff_from_str!(RySpan);
impl_ry_jiff_from_str!(RyTime);
impl_ry_jiff_from_str!(RyTimestamp);
impl_ry_jiff_from_str!(RyZoned);

// WTF is up here... I fugue-state-jesse do not remember..
// I think they don't have FromStr impls in jiff? herm...

impl PyFromStr for RyISOWeekDate {
    #[inline]
    fn py_from_str(s: &str) -> pyo3::PyResult<Self> {
        let iwd = parse_iso_week_date(s).map(Self::from);
        // try parse as jiff::civil::Date
        if let Ok(date) = jiff::civil::Date::from_str(s) {
            Ok(Self::from(date.iso_week_date()))
        } else {
            iwd.map_err(crate::errors::map_py_value_err)
        }
    }
}

impl PyFromStr for RyTimeZone {
    #[inline]
    fn py_from_str(s: &str) -> pyo3::PyResult<Self> {
        TimeZone::get(s)
            .map(Self::from)
            .map_err(crate::errors::map_py_value_err)
    }
}

impl FromStr for RyOffset {
    type Err = jiff::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::constants::DATETIME_PARSER;
        let o = DATETIME_PARSER.parse_time_zone(s)?;
        o.to_fixed_offset().map(Self::from)
    }
}

impl FromStr for RyDateTime {
    type Err = jiff::Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if ends with 'Z', parse via timezone...
        if s.ends_with('Z') {
            jiff::Timestamp::from_str(s)
                .map(|ts| ts.to_zoned(TimeZone::UTC).datetime())
                .map(Self::from)
        } else {
            DateTime::from_str(s).map(Self::from)
        }
    }
}
