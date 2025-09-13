//! ryo3-jiff difference options
//!
//!
//!
//! `SpanRound` is used internally by several of the underlying difference structs
//! so for ref, here are the span-round defaults:
//!
//! - `SpanRound`
//!   - smallest-max: None
//!   - smallest-min: None
//!   - defaults
//!     - smallest:  Nanosecond
//!     - largest:   None
//!     - mode:      `HalfExpand`
//!     - increment: 1
//!     - relative:  None
//!
//! # DEFAULTS
//!
//! - `DateDifference`
//!   - smallest-max: ???
//!   - defaults
//!     - smallest:  Day
//!     - mode:      Trunc
//!     - increment: 1
//! - `DateTimeDifference`
//!   - smallest-max: ???
//!   - defaults
//!     - smallest:  Nanosecond
//!     - largest:   None
//!     - mode:      Trunc
//!     - increment: 1
//! - `ZonedDifference`
//!   - smallest-max: ???
//!   - defaults
//!     - smallest:  Nanosecond
//!     - largest:   None
//!     - mode:      Trunc
//!     - increment: 1
//! - `TimeDifference`
//!   - smallest-max: Hour
//!   - defaults
//!     - smallest:  Nanosecond
//!     - largest:   None (defaults to Hour if not set)
//!     - mode:      Trunc
//!     - increment: 1
//! - `TimestampDifference`
//!   - smallest-max: Day
//!   - largest-max:  Day
//!   - defaults
//!    - smallest:  Nanosecond
//!    - largest:   None
//!    - mode:      Trunc
//!    - increment: 1

use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_zoned::RyZoned;
use crate::{JiffRoundMode, JiffUnit, RyDate, RyTimestamp};
use jiff::TimestampDifference;
use jiff::civil::{DateDifference, DateTimeDifference, TimeDifference};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use pyo3::{IntoPyObjectExt, intern};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DifferenceOptions {
    smallest: JiffUnit,
    largest: Option<JiffUnit>,
    mode: JiffRoundMode,
    increment: i64,
}

impl DifferenceOptions {
    fn pydict_options<'py>(
        &self,
        py: Python<'py>,
        pydict: &'py Bound<'py, PyDict>,
    ) -> PyResult<()> {
        use crate::interns;
        pydict.set_item(interns::smallest(py), self.smallest)?;
        if let Some(largest) = self.largest {
            pydict.set_item(interns::largest(py), largest)?;
        } else {
            pydict.set_item(interns::largest(py), py.None())?;
        }
        pydict.set_item(interns::mode(py), self.mode)?;
        pydict.set_item(interns::increment(py), self.increment)?;
        Ok(())
    }
}

// ============================================================================
// DateDifference
// ============================================================================
#[derive(Debug, Clone, Copy)]
#[pyclass(name = "DateDifference", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateDifference {
    date: RyDate,
    options: DifferenceOptions,
    pub(crate) diff: DateDifference,
}

#[pymethods]
impl RyDateDifference {
    #[new]
    #[pyo3(
        signature = (
            date,
            *,
            smallest = None,
            largest = None,
            mode = None,
            increment = 1
        ),
    )]
    #[must_use]
    fn py_new(
        date: &RyDate,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> Self {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::Trunc));
        let mut diff = DateDifference::new(date.0)
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        let options = DifferenceOptions {
            smallest,
            largest,
            mode,
            increment,
        };
        Self {
            date: *date,
            options,
            diff,
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.date == other.date
            && self.options.smallest == other.options.smallest
            && self.options.largest == other.options.largest
            && self.options.mode == other.options.mode
            && self.options.increment == other.options.increment
    }

    #[getter]
    fn date(&self) -> RyDate {
        self.date
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn largest(&self) -> Option<JiffUnit> {
        self.options.largest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: unit,
            largest: self.options.largest,
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.smallest(unit.0);
        Self {
            date: self.date,
            options,
            diff,
        }
    }

    fn _largest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: Some(unit),
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.largest(unit.0);
        Self {
            date: self.date,
            options,
            diff,
        }
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode,
            increment: self.options.increment,
        };
        let diff = self.diff.mode(mode.0);
        Self {
            date: self.date,
            options,
            diff,
        }
    }

    fn _increment(&self, increment: i64) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode: self.options.mode,
            increment,
        };
        let diff = self.diff.increment(increment);
        Self {
            date: self.date,
            options,
            diff,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "date"), self.date)?;
        self.options.pydict_options(py, &dict)?;
        Ok(dict)
    }
}

impl std::fmt::Display for RyDateDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateDifference(")?;
        write!(f, "{}", self.date)?;
        write!(f, ", smallest=\"{}\"", self.options.smallest)?;
        if let Some(largest) = self.options.largest {
            write!(f, ", largest=\"{largest}\"")?;
        } else {
            write!(f, ", largest=None")?;
        }

        write!(f, ", mode=\"{}\"", self.options.mode)?;
        write!(f, ", increment={}", self.options.increment)?;
        write!(f, ")")
    }
}
#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum DateDifferenceArg {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

impl DateDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> DateDifference {
        let mut diff = match self {
            Self::Zoned(zoned) => DateDifference::from(zoned.0),
            Self::Date(date) => DateDifference::from(date.0),
            Self::DateTime(date_time) => DateDifference::from(date_time.0),
        };
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        diff
    }
}

// ============================================================================
// DateTimeDifference
// ============================================================================

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "DateTimeDifference", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateTimeDifference {
    datetime: RyDateTime,
    options: DifferenceOptions,
    pub(crate) diff: DateTimeDifference,
}

#[pymethods]
impl RyDateTimeDifference {
    #[new]
    #[pyo3(
        signature = (
            datetime,
            *,
            smallest = None,
            largest = None,
            mode = None,
            increment = 1
        ),
    )]
    #[must_use]
    fn py_new(
        datetime: &RyDateTime,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> Self {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::Trunc));
        let mut diff = DateTimeDifference::new(datetime.0)
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        let options = DifferenceOptions {
            smallest,
            largest,
            mode,
            increment,
        };
        Self {
            datetime: *datetime,
            options,
            diff,
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.datetime == other.datetime && self.options == other.options
    }

    #[getter]
    fn datetime(&self) -> RyDateTime {
        self.datetime
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn largest(&self) -> Option<JiffUnit> {
        self.options.largest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: unit,
            largest: self.options.largest,
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.smallest(unit.0);
        Self {
            datetime: self.datetime,
            options,
            diff,
        }
    }

    fn _largest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: Some(unit),
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.largest(unit.0);
        Self {
            datetime: self.datetime,
            options,
            diff,
        }
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode,
            increment: self.options.increment,
        };
        let diff = self.diff.mode(mode.0);
        Self {
            datetime: self.datetime,
            options,
            diff,
        }
    }

    fn _increment(&self, increment: i64) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode: self.options.mode,
            increment,
        };
        let diff = self.diff.increment(increment);
        Self {
            datetime: self.datetime,
            options,
            diff,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "datetime"), self.datetime)?;
        self.options.pydict_options(py, &dict)?;
        Ok(dict)
    }
}

impl std::fmt::Display for RyDateTimeDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTimeDifference(")?;
        write!(f, "{}", self.datetime)?;
        write!(f, ", smallest=\"{}\"", self.options.smallest)?;
        if let Some(largest) = self.options.largest {
            write!(f, ", largest=\"{largest}\"")?;
        } else {
            write!(f, ", largest=None")?;
        }

        write!(f, ", mode=\"{}\"", self.options.mode)?;
        write!(f, ", increment={}", self.options.increment)?;
        write!(f, ")")
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum DateTimeDifferenceArg {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

impl DateTimeDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> DateTimeDifference {
        let mut diff = match self {
            Self::Zoned(other) => DateTimeDifference::from(other.0),
            Self::DateTime(other) => DateTimeDifference::from(other.0),
            Self::Date(other) => DateTimeDifference::from(other.0),
        };
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        diff
    }
}

// ============================================================================
// TimeDifference
// ============================================================================
#[derive(Debug, Clone, Copy)]
#[pyclass(name = "TimeDifference", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimeDifference {
    time: RyTime,
    options: DifferenceOptions,
    pub(crate) diff: TimeDifference,
}

#[pymethods]
impl RyTimeDifference {
    #[new]
    #[pyo3(
        signature = (
            time,
            *,
            smallest = None,
            largest = None,
            mode = None,
            increment = 1
        ),
    )]
    #[must_use]
    fn py_new(
        time: &RyTime,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> Self {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::Trunc));
        let mut diff = TimeDifference::new(time.0)
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        let options = DifferenceOptions {
            smallest,
            largest,
            mode,
            increment,
        };
        Self {
            time: *time,
            options,
            diff,
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.time == other.time
            && self.options.smallest == other.options.smallest
            && self.options.largest == other.options.largest
            && self.options.mode == other.options.mode
            && self.options.increment == other.options.increment
    }

    #[getter]
    fn time(&self) -> RyTime {
        self.time
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn largest(&self) -> Option<JiffUnit> {
        self.options.largest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        let time = self.time;
        let options = DifferenceOptions {
            smallest: unit,
            largest: self.options.largest,
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.smallest(unit.0);
        Self {
            time,
            options,
            diff,
        }
    }

    fn _largest(&self, unit: JiffUnit) -> Self {
        let time = self.time;
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: Some(unit),
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.largest(unit.0);
        Self {
            time,
            options,
            diff,
        }
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode,
            increment: self.options.increment,
        };
        let diff = self.diff.mode(mode.0);
        Self {
            time: self.time,
            options,
            diff,
        }
    }

    fn _increment(&self, increment: i64) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode: self.options.mode,
            increment,
        };
        let diff = self.diff.increment(increment);
        Self {
            time: self.time,
            options,
            diff,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "time"), self.time)?;
        self.options.pydict_options(py, &dict)?;
        Ok(dict)
    }
}

impl std::fmt::Display for RyTimeDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeDifference(")?;
        write!(f, "{}", self.time)?;
        write!(f, ", smallest=\"{}\"", self.options.smallest)?;
        if let Some(largest) = self.options.largest {
            write!(f, ", largest=\"{largest}\"")?;
        } else {
            write!(f, ", largest=None")?;
        }

        write!(f, ", mode=\"{}\"", self.options.mode)?;
        write!(f, ", increment={}", self.options.increment)?;
        write!(f, ")")
    }
}

// ============================================================================
// Zoned/Time/DateTime
// ============================================================================

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum TimeDifferenceArg {
    Zoned(RyZoned),
    Time(RyTime),
    DateTime(RyDateTime),
}

impl TimeDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> TimeDifference {
        let mut diff = match self {
            Self::Time(other) => TimeDifference::from(other.0),
            Self::Zoned(other) => TimeDifference::from(other.0),
            Self::DateTime(other) => TimeDifference::from(other.0),
        };
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        diff
    }
}

// ============================================================================
// TimestampDifference
// ============================================================================
#[derive(Debug, Clone, Copy)]
#[pyclass(name = "TimestampDifference", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimestampDifference {
    timestamp: RyTimestamp,
    options: DifferenceOptions,
    pub(crate) diff: TimestampDifference,
}

#[pymethods]
impl RyTimestampDifference {
    #[new]
    #[pyo3(
        signature = (
            timestamp,
            *,
            smallest = None,
            largest = None,
            mode = None,
            increment = 1
        ),
    )]
    #[must_use]
    fn py_new(
        timestamp: &RyTimestamp,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> Self {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::Trunc));
        let mut diff = TimestampDifference::new(timestamp.0)
            .smallest(smallest.0)
            .mode(mode.0)
            .increment(increment);
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        let options = DifferenceOptions {
            smallest,
            largest,
            mode,
            increment,
        };
        Self {
            timestamp: *timestamp,
            options,
            diff,
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }
    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
            && self.options.smallest == other.options.smallest
            && self.options.largest == other.options.largest
            && self.options.mode == other.options.mode
            && self.options.increment == other.options.increment
    }

    #[getter]
    fn timestamp(&self) -> RyTimestamp {
        self.timestamp
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn largest(&self) -> Option<JiffUnit> {
        self.options.largest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: unit,
            largest: self.options.largest,
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.smallest(unit.0);
        Self {
            timestamp: self.timestamp,
            options,
            diff,
        }
    }

    fn _largest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: Some(unit),
            mode: self.options.mode,
            increment: self.options.increment,
        };
        let diff = self.diff.largest(unit.0);
        Self {
            timestamp: self.timestamp,
            options,
            diff,
        }
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode,
            increment: self.options.increment,
        };
        let diff = self.diff.mode(mode.0);
        Self {
            timestamp: self.timestamp,
            options,
            diff,
        }
    }

    fn _increment(&self, increment: i64) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode: self.options.mode,
            increment,
        };
        let diff = self.diff.increment(increment);
        Self {
            timestamp: self.timestamp,
            options,
            diff,
        }
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "timestamp"), self.timestamp)?;
        self.options.pydict_options(py, &dict)?;
        Ok(dict)
    }
}

impl std::fmt::Display for RyTimestampDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimestampDifference(")?;
        write!(f, "{}", self.timestamp)?;
        write!(f, ", smallest=\"{}\"", self.options.smallest)?;
        if let Some(largest) = self.options.largest {
            write!(f, ", largest=\"{largest}\"")?;
        } else {
            write!(f, ", largest=None")?;
        }
        write!(f, ", mode=\"{}\"", self.options.mode)?;
        write!(f, ", increment={}", self.options.increment)?;
        write!(f, ")")
    }
}

// ============================================================================
// Zoned/Time/DateTime
// ============================================================================

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum TimestampDifferenceArg {
    Zoned(RyZoned),
    Timestamp(RyTimestamp),
}

impl TimestampDifferenceArg {
    pub(crate) fn build(
        self,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> TimestampDifference {
        let mut diff = match self {
            Self::Zoned(zoned) => TimestampDifference::from(zoned.0),
            Self::Timestamp(date) => TimestampDifference::from(date.0),
        };
        if let Some(smallest) = smallest {
            diff = diff.smallest(smallest.0);
        }
        if let Some(largest) = largest {
            diff = diff.largest(largest.0);
        }
        if let Some(mode) = mode {
            diff = diff.mode(mode.0);
        }
        if let Some(increment) = increment {
            diff = diff.increment(increment);
        }
        diff
    }
}

// ============================================================================
// ZonedDateTimeDifference
// ============================================================================
#[derive(Debug, Clone)]
#[pyclass(name = "ZonedDateTimeDifference", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyZonedDifference {
    zoned: RyZoned,
    options: DifferenceOptions,
}

#[pymethods]
impl RyZonedDifference {
    #[new]
    #[pyo3(
        signature = (
            zoned,
            *,
            smallest = None,
            largest = None,
            mode = None,
            increment = 1
        ),
    )]
    #[must_use]
    fn py_new(
        zoned: &RyZoned,
        smallest: Option<JiffUnit>,
        largest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: i64,
    ) -> Self {
        let smallest = smallest.unwrap_or(JiffUnit(jiff::Unit::Nanosecond));
        let mode = mode.unwrap_or(JiffRoundMode(jiff::RoundMode::Trunc));
        let options = DifferenceOptions {
            smallest,
            largest,
            mode,
            increment,
        };
        Self {
            zoned: zoned.clone(),
            options,
        }
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.zoned == other.zoned
            && self.options.smallest == other.options.smallest
            && self.options.largest == other.options.largest
            && self.options.mode == other.options.mode
            && self.options.increment == other.options.increment
    }

    #[getter]
    fn zoned(&self) -> RyZoned {
        self.zoned.clone()
    }

    #[getter]
    fn smallest(&self) -> JiffUnit {
        self.options.smallest
    }

    #[getter]
    fn largest(&self) -> Option<JiffUnit> {
        self.options.largest
    }

    #[getter]
    fn mode(&self) -> JiffRoundMode {
        self.options.mode
    }

    #[getter]
    fn increment(&self) -> i64 {
        self.options.increment
    }

    fn _smallest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: unit,
            largest: self.options.largest,
            mode: self.options.mode,
            increment: self.options.increment,
        };
        Self {
            zoned: self.zoned.clone(),
            options,
        }
    }

    fn _largest(&self, unit: JiffUnit) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: Some(unit),
            mode: self.options.mode,
            increment: self.options.increment,
        };
        Self {
            zoned: self.zoned.clone(),
            options,
        }
    }

    fn _mode(&self, mode: JiffRoundMode) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode,
            increment: self.options.increment,
        };
        Self {
            zoned: self.zoned.clone(),
            options,
        }
    }

    fn _increment(&self, increment: i64) -> Self {
        let options = DifferenceOptions {
            smallest: self.options.smallest,
            largest: self.options.largest,
            mode: self.options.mode,
            increment,
        };
        Self {
            zoned: self.zoned.clone(),
            options,
        }
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "zoned"), self.zoned.clone())?;
        self.options.pydict_options(py, &dict)?;
        Ok(dict)
    }
}

impl std::fmt::Display for RyZonedDifference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZonedDateTimeDifference(")?;
        write!(f, "{}", self.zoned)?;
        write!(f, ", smallest=\"{}\"", self.options.smallest)?;
        if let Some(largest) = self.options.largest {
            write!(f, ", largest=\"{largest}\"")?;
        } else {
            write!(f, ", largest=None")?;
        }
        write!(f, ", mode=\"{}\"", self.options.mode)?;
        write!(f, ", increment={}", self.options.increment)?;
        write!(f, ")")
    }
}
