//! Development place
use jiff::civil::DateTimeRound;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::types::{PyAnyMethods, PyString};
use pyo3::{pyclass, pymethods, Bound, FromPyObject, IntoPyObject, PyAny, PyResult, Python};
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffUnit(pub(crate) jiff::Unit);
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

const JIFF_UNIT_ERROR: &str = "Invalid unit, should be `'year'`, `'month'`, `'week'`, `'day'`, `'hour'`, `'minute'`, `'second'`, `'millisecond'`, `'microsecond'` or `'nanosecond'`";

impl<'py> FromPyObject<'py> for JiffUnit {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode {
                "year" => Ok(Self(jiff::Unit::Year)),
                "month" => Ok(Self(jiff::Unit::Month)),
                "week" => Ok(Self(jiff::Unit::Week)),
                "day" => Ok(Self(jiff::Unit::Day)),
                "hour" => Ok(Self(jiff::Unit::Hour)),
                "minute" => Ok(Self(jiff::Unit::Minute)),
                "second" => Ok(Self(jiff::Unit::Second)),
                "millisecond" => Ok(Self(jiff::Unit::Millisecond)),
                "microsecond" => Ok(Self(jiff::Unit::Microsecond)),
                "nanosecond" => Ok(Self(jiff::Unit::Nanosecond)),
                _ => Err(PyValueError::new_err(JIFF_UNIT_ERROR)),
            }
        } else {
            Err(PyTypeError::new_err(JIFF_UNIT_ERROR))
        }
    }
}

impl<'py> IntoPyObject<'py> for JiffUnit {
    type Target = PyString; // the Python type
    type Output = Bound<'py, Self::Target>; // in most cases this will be `Bound`
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let unit = match self.0 {
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
        };
        let string = PyString::new(py, unit);
        Ok(string)
    }
}

impl Display for JiffUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.static_str();
        write!(f, "{s}")
    }
}

//
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JiffRoundMode(pub(crate) jiff::RoundMode);
const JIFF_ROUND_MODE_ERROR: &str = "Invalid round mode, should be `'ceil'`, `'floor'`, `'expand'`, `'trunc'`, `'half_ceil'`, `'half_floor'`, `'half_expand'`, `'half_trunc'` or `'half_even'`";

impl<'py> FromPyObject<'py> for JiffRoundMode {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode {
                "ceil" => Ok(Self(jiff::RoundMode::Ceil)),
                "floor" => Ok(Self(jiff::RoundMode::Floor)),
                "expand" => Ok(Self(jiff::RoundMode::Expand)),
                "trunc" => Ok(Self(jiff::RoundMode::Trunc)),
                "half_ceil" => Ok(Self(jiff::RoundMode::HalfCeil)),
                "half_floor" => Ok(Self(jiff::RoundMode::HalfFloor)),
                "half_expand" => Ok(Self(jiff::RoundMode::HalfExpand)),
                "half_trunc" => Ok(Self(jiff::RoundMode::HalfTrunc)),
                "half_even" => Ok(Self(jiff::RoundMode::HalfEven)),
                _ => Err(PyValueError::new_err(JIFF_ROUND_MODE_ERROR)),
            }
        } else {
            Err(PyTypeError::new_err(JIFF_ROUND_MODE_ERROR))
        }
    }
}
impl<'py> IntoPyObject<'py> for JiffRoundMode {
    type Target = PyString; // the Python type
    type Output = Bound<'py, Self::Target>; // in most cases this will be `Bound`
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = self.static_str();
        let string = PyString::new(py, s);
        Ok(string)
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
