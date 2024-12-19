use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_time_difference::{IntoTimeDifference, RyTimeDifference};
use crate::{JiffRoundMode, JiffTime, JiffUnit};
use jiff::civil::TimeRound;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTime, PyTuple, PyType};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[pyclass(name = "Time", module = "ryo3")]
#[derive(Debug, Clone)]
pub struct RyTime(pub(crate) jiff::civil::Time);

#[pymethods]
impl RyTime {
    #[new]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
    pub fn py_new(
        hour: Option<i8>,
        minute: Option<i8>,
        second: Option<i8>,
        nanosecond: Option<i32>,
    ) -> PyResult<Self> {
        jiff::civil::Time::new(
            hour.unwrap_or(0),
            minute.unwrap_or(0),
            second.unwrap_or(0),
            nanosecond.unwrap_or(0),
        )
        .map(crate::RyTime::from)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    // ========================================================================
    // CLASS ATTRS
    // ========================================================================
    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(jiff::civil::Time::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(jiff::civil::Time::MAX)
    }

    // ========================================================================
    // CLASS METHODS
    // ========================================================================
    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Time::from(Zoned::now());
        Self::from(z)
    }

    #[classmethod]
    fn midnight(_cls: &Bound<'_, PyType>) -> Self {
        Self(jiff::civil::Time::midnight())
    }
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        jiff::civil::Time::from_str(s)
            .map(crate::RyTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    // ========================================================================
    // STRPTIME/STRFTIME
    // ========================================================================

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, format: &str, input: &str) -> PyResult<Self> {
        jiff::civil::Time::strptime(format, input)
            .map(crate::RyTime::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn strftime(&self, format: &str) -> String {
        self.0.strftime(format).to_string()
    }

    // ========================================================================
    // STRING
    // ========================================================================
    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        format!(
            "Time(hour={}, minute={}, second={}, nanosecond={})",
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
            self.0.nanosecond()
        )
    }

    // ========================================================================
    // OPERATORS/DUNDERS
    // ========================================================================
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyTimeArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            RyTimeArithmeticSub::Time(other) => {
                let span = self.0 - other.0;
                let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
                Ok(obj)
            }
            RyTimeArithmeticSub::Span(other) => {
                let t = self.0.checked_sub(other.0).map_err(map_py_overflow_err)?;

                RyTime::from(t).into_pyobject(py).map(Bound::into_any)
            }
            RyTimeArithmeticSub::SignedDuration(other) => {
                let t = self.0 - other.0;
                RyTime::from(t).into_pyobject(py).map(Bound::into_any)
            }
        }
    }

    fn __isub__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0 - other.0,
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0 - other.0,
            RyDeltaArithmeticSelf::Duration(other) => self.0 - other.0,
        };
        self.0 = t;
        Ok(())
    }

    fn __add__(&self, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.checked_add(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.checked_add(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.checked_add(other.0),
        }
        .map_err(map_py_overflow_err)?;
        Ok(RyTime::from(t))
    }

    fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0.checked_add(other.0),
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0.checked_add(other.0),
            RyDeltaArithmeticSelf::Duration(other) => self.0.checked_add(other.0),
        }
        .map_err(map_py_overflow_err)?;
        self.0 = t;
        Ok(())
    }
    fn checked_add(&self, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        self.__add__(other)
    }

    fn checked_sub<'py>(
        &self,
        py: Python<'py>,
        other: RyTimeArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.__sub__(py, other)
    }

    // ========================================================================
    // PROPERTIES
    // ========================================================================
    #[getter]
    fn hour(&self) -> i8 {
        self.0.hour()
    }

    #[getter]
    fn minute(&self) -> i8 {
        self.0.minute()
    }
    #[getter]
    fn second(&self) -> i8 {
        self.0.second()
    }

    #[getter]
    fn millisecond(&self) -> i16 {
        self.0.millisecond()
    }

    #[getter]
    fn microsecond(&self) -> i16 {
        self.0.microsecond()
    }

    #[getter]
    fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    #[getter]
    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    // =====================================================================
    // PYTHON CONVERSIONS
    // =====================================================================
    fn to_pytime<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTime>> {
        JiffTime(self.0).into_pyobject(py)
    }
    #[classmethod]
    fn from_pytime(_cls: &Bound<'_, PyType>, py_time: &Bound<'_, PyTime>) -> PyResult<Self> {
        py_time.extract::<JiffTime>().map(RyTime::from)
    }

    // =====================================================================
    // INSTANCE METHODS
    // =====================================================================

    fn on(&self, year: i16, month: i8, day: i8) -> RyDateTime {
        RyDateTime::from(self.0.on(year, month, day))
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                i32::from(self.0.hour()),
                i32::from(self.0.minute()),
                i32::from(self.0.second()),
                self.0.subsec_nanosecond(),
            ],
        )
    }

    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "hour"), self.0.hour())?;
        dict.set_item(intern!(py, "minute"), self.0.minute())?;
        dict.set_item(intern!(py, "second"), self.0.second())?;
        dict.set_item(intern!(py, "nanosecond"), self.0.subsec_nanosecond())?;
        Ok(dict)
    }

    fn series(&self, period: &RySpan) -> PyResult<RyTimeSeries> {
        let ser = self.0.series(period.0);
        Ok(RyTimeSeries { series: ser })
    }
    fn duration_since(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_since(other.0))
    }
    fn duration_until(&self, other: &Self) -> RySignedDuration {
        RySignedDuration::from(self.0.duration_until(other.0))
    }

    fn saturating_add(&self, other: RyDeltaArithmeticSelf) -> RyTime {
        match other {
            RyDeltaArithmeticSelf::Span(other) => Self::from(self.0.saturating_add(other.0)),
            RyDeltaArithmeticSelf::SignedDuration(other) => {
                Self::from(self.0.saturating_add(other.0))
            }
            RyDeltaArithmeticSelf::Duration(other) => Self::from(self.0.saturating_add(other.0)),
        }
    }
    fn saturating_sub(&self, other: RyDeltaArithmeticSelf) -> RyTime {
        match other {
            RyDeltaArithmeticSelf::Span(other) => Self::from(self.0.saturating_sub(other.0)),
            RyDeltaArithmeticSelf::SignedDuration(other) => {
                Self::from(self.0.saturating_sub(other.0))
            }
            RyDeltaArithmeticSelf::Duration(other) => Self::from(self.0.saturating_sub(other.0)),
        }
    }
    fn wrapping_add(&self, other: RyDeltaArithmeticSelf) -> RyTime {
        match other {
            RyDeltaArithmeticSelf::Span(other) => Self::from(self.0.wrapping_add(other.0)),
            RyDeltaArithmeticSelf::SignedDuration(other) => {
                Self::from(self.0.wrapping_add(other.0))
            }
            RyDeltaArithmeticSelf::Duration(other) => Self::from(self.0.wrapping_add(other.0)),
        }
    }

    fn wrapping_sub(&self, other: RyDeltaArithmeticSelf) -> RyTime {
        match other {
            RyDeltaArithmeticSelf::Span(other) => Self::from(self.0.wrapping_sub(other.0)),
            RyDeltaArithmeticSelf::SignedDuration(other) => {
                Self::from(self.0.wrapping_sub(other.0))
            }
            RyDeltaArithmeticSelf::Duration(other) => Self::from(self.0.wrapping_sub(other.0)),
        }
    }

    fn to_datetime(&self, date: &crate::RyDate) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(date.0))
    }

    #[pyo3(signature = (smallest = None, mode = None, increment = None))]
    fn round(
        &self,
        smallest: Option<JiffUnit>,
        mode: Option<JiffRoundMode>,
        increment: Option<i64>,
    ) -> PyResult<Self> {
        let mut timeround = TimeRound::new();
        if let Some(smallest) = smallest {
            timeround = timeround.smallest(smallest.0);
        }
        if let Some(mode) = mode {
            timeround = timeround.mode(mode.0);
        }
        if let Some(increment) = increment {
            timeround = timeround.increment(increment);
        }
        self.0
            .round(timeround)
            .map(RyTime::from)
            .map_err(map_py_value_err)
    }

    // ------------------------------------------------------------------------
    // SINCE/UNTIL
    // ------------------------------------------------------------------------
    fn since(&self, other: IntoTimeDifference) -> PyResult<RySpan> {
        self.0
            .since(other)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }
    fn until(&self, other: IntoTimeDifference) -> PyResult<RySpan> {
        self.0
            .until(other)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }

    fn _since(&self, other: &RyTimeDifference) -> PyResult<RySpan> {
        self.0
            .since(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }
    fn _until(&self, other: &RyTimeDifference) -> PyResult<RySpan> {
        self.0
            .until(other.0)
            .map(RySpan::from)
            .map_err(map_py_value_err)
    }
}

impl Display for RyTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time<{}>", self.0)
    }
}
impl From<jiff::civil::Time> for RyTime {
    fn from(value: jiff::civil::Time) -> Self {
        Self(value)
    }
}

impl From<JiffTime> for RyTime {
    fn from(value: JiffTime) -> Self {
        Self(value.0)
    }
}

#[pyclass]
#[pyo3(name = "TimeSeries", module = "ryo3")]
pub struct RyTimeSeries {
    pub(crate) series: jiff::civil::TimeSeries,
}

#[pymethods]
impl RyTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<RyTime> {
        slf.series.next().map(RyTime::from)
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyTimeArithmeticSub {
    Time(RyTime),
    Span(RySpan),
    SignedDuration(RySignedDuration),
}
