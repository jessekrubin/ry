//! Development place
use crate::{JiffRoundMode, JiffUnit};
use jiff::civil::DateTimeRound;
use pyo3::{pyclass, pymethods, PyResult};
use std::fmt::Display;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RyWeekday(pub(crate) jiff::civil::Weekday);

#[pymethods]
impl RyWeekday {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn string(&self) -> &'static str {
        match self.0 {
            jiff::civil::Weekday::Sunday => "sunday",
            jiff::civil::Weekday::Monday => "monday",
            jiff::civil::Weekday::Tuesday => "tuesday",
            jiff::civil::Weekday::Wednesday => "wednesday",
            jiff::civil::Weekday::Thursday => "thursday",
            jiff::civil::Weekday::Friday => "friday",
            jiff::civil::Weekday::Saturday => "saturday",
        }
    }
}

// Year = 9,
// Month = 8,
// Week = 7,
// Day = 6,
// Hour = 5,
// Minute = 4,
// Second = 3,
// Millisecond = 2,
// Microsecond = 1,
// Nanosecond = 0,

impl JiffUnit {
    #[must_use]
    pub fn static_str(self) -> &'static str {
        match self.0 {
            jiff::Unit::Year => "year",
            jiff::Unit::Month => "month",
            jiff::Unit::Week => "week",
            jiff::Unit::Day => "day",
            jiff::Unit::Hour => "hour",
            jiff::Unit::Minute => "minute",
            jiff::Unit::Second => "second",
            jiff::Unit::Millisecond => "millisecond",
            jiff::Unit::Microsecond => "microsecond",
            jiff::Unit::Nanosecond => "nanosecond",
        }
    }
}

impl Display for JiffUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}

impl JiffRoundMode {
    fn static_str(self) -> &'static str {
        match self.0 {
            jiff::RoundMode::Ceil => "ceil",
            jiff::RoundMode::Floor => "floor",
            jiff::RoundMode::Expand => "expand",
            jiff::RoundMode::Trunc => "trunc",
            jiff::RoundMode::HalfCeil => "half_ceil",
            jiff::RoundMode::HalfFloor => "half_floor",
            jiff::RoundMode::HalfExpand => "half_expand",
            jiff::RoundMode::HalfTrunc => "half_trunc",
            jiff::RoundMode::HalfEven => "half_even",
            _ => "round_unknown",
        }
    }
}
impl Display for JiffRoundMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "DateTimeRound", module = "ryo3")]
pub struct RyDateTimeRound {
    pub smallest: JiffUnit,
    pub mode: JiffRoundMode,
    pub increment: i64,
    // internal
    pub round: DateTimeRound,
}

#[pymethods]
impl RyDateTimeRound {
    #[new]
    #[pyo3(signature = (smallest=None, mode=None, increment=1))]
    pub fn new(
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> PyResult<Self> {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::HalfExpand));
        let round = DateTimeRound::new()
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        Ok(Self {
            smallest,
            mode,
            increment,
            round,
        })
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!(
            "DateTimeRound(smallest=\"{}\", mode=\"{}\", increment={})",
            self.smallest, self.mode, self.increment
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.smallest == other.smallest
            && self.increment == other.increment
    }

    #[pyo3(signature = (smallest=None, mode=None, increment=None))]
    fn replace(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> Self {
        let smallest = smallest.unwrap_or(self.smallest);
        let mode = mode.unwrap_or(self.mode);
        let increment = increment.unwrap_or(self.increment);
        let round = DateTimeRound::new()
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        Self {
            smallest,
            mode,
            increment,
            round,
        }
    }

    fn smallest(&self, unit: JiffUnit) -> Self {
        self.replace(Some(unit), None, None)
    }

    fn mode(&self, mode: JiffRoundMode) -> Self {
        self.replace(None, Some(mode), None)
    }

    fn increment(&self, increment: i64) -> Self {
        self.replace(None, None, Some(increment))
    }

    fn _smallest(&self) -> JiffUnit {
        self.smallest
    }

    fn _mode(&self) -> JiffRoundMode {
        self.mode
    }

    fn _increment(&self) -> i64 {
        self.increment
    }
}
