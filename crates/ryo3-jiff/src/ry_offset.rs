use crate::errors::map_py_value_err;
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use jiff::tz::{Offset, OffsetArithmetic};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};
use pyo3::IntoPyObjectExt;
use ryo3_std::PyDuration;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "Offset", module = "ry.ryo3", frozen)]
pub struct RyOffset(pub(crate) Offset);

#[pymethods]
impl RyOffset {
    #[new]
    #[pyo3(signature = (hours = None, seconds = None))]
    fn py_new(hours: Option<i8>, seconds: Option<i32>) -> PyResult<Self> {
        match (hours, seconds) {
            (Some(h), None) => Offset::from_hours(h)
                .map(RyOffset::from)
                .map_err(map_py_value_err),
            (None, Some(s)) => Offset::from_seconds(s)
                .map(RyOffset::from)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Offset() takes either hours or seconds",
            )),
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![
                py.None().into_bound_py_any(py)?,
                self.0.seconds().into_bound_py_any(py)?,
            ],
        )
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        RyOffset::from(Offset::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        RyOffset::from(Offset::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        RyOffset::from(Offset::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UTC() -> Self {
        RyOffset::from(Offset::UTC)
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        RyOffset::from(Offset::UTC)
    }

    #[classmethod]
    fn from_hours(_cls: &Bound<'_, PyType>, hours: i8) -> PyResult<RyOffset> {
        Offset::from_hours(hours)
            .map(RyOffset::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn from_seconds(_cls: &Bound<'_, PyType>, seconds: i32) -> PyResult<Self> {
        Offset::from_seconds(seconds)
            .map(RyOffset::from)
            .map_err(map_py_value_err)
    }

    fn to_py(&self) -> &Offset {
        &self.0
    }

    fn to_pytzinfo(&self) -> &Offset {
        &self.0
    }

    #[classmethod]
    fn from_pytzinfo(_cls: &Bound<'_, PyType>, d: Offset) -> Self {
        Self::from(d)
    }

    #[must_use]
    fn string(&self) -> String {
        self.0.to_string()
    }

    #[must_use]
    fn __str__(&self) -> String {
        self.__repr__()
    }

    #[must_use]
    fn __repr__(&self) -> String {
        let s = self.0.seconds();
        // if it is hours then use hours for repr
        if s % 3600 == 0 {
            format!("Offset(hours={})", s / 3600)
        } else {
            format!("Offset(seconds={s})")
        }
    }

    #[getter]
    #[must_use]
    fn seconds(&self) -> i32 {
        self.0.seconds()
    }

    #[must_use]
    fn negate(&self) -> Self {
        RyOffset::from(self.0.negate())
    }

    fn __neg__(&self) -> Self {
        self.negate()
    }

    #[getter]
    #[must_use]
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    #[getter]
    #[must_use]
    fn is_positive(&self) -> bool {
        !self.0.is_negative()
    }

    #[must_use]
    fn to_datetime(&self, timestamp: &RyTimestamp) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(timestamp.0))
    }

    fn to_timestamp(&self, datetime: &RyDateTime) -> PyResult<RyTimestamp> {
        self.0
            .to_timestamp(datetime.0)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[must_use]
    fn until(&self, other: &RyOffset) -> RySpan {
        let s = self.0.until(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn since(&self, other: &RyOffset) -> RySpan {
        let s = self.0.since(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn duration_until(&self, other: &RyOffset) -> RySignedDuration {
        let s = self.0.duration_until(other.0);
        RySignedDuration::from(s)
    }

    #[must_use]
    fn duration_since(&self, other: &RyOffset) -> RySignedDuration {
        let s = self.0.duration_since(other.0);
        RySignedDuration::from(s)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn to_timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.to_time_zone())
    }

    fn checked_add(&self, other: IntoOffsetArithmetic) -> PyResult<Self> {
        self.0
            .checked_add(other)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn checked_sub(&self, other: IntoOffsetArithmetic) -> PyResult<Self> {
        self.0
            .checked_sub(other)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn saturating_add(&self, other: IntoOffsetArithmetic) -> Self {
        Self::from(self.0.saturating_add(other))
    }

    fn saturating_sub(&self, other: IntoOffsetArithmetic) -> Self {
        Self::from(self.0.saturating_sub(other))
    }
}

impl From<Offset> for RyOffset {
    fn from(value: Offset) -> Self {
        RyOffset(value)
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum IntoOffsetArithmetic {
    Duration(PyDuration),
    SignedDuration(RySignedDuration),
    Span(RySpan),
}

impl From<IntoOffsetArithmetic> for OffsetArithmetic {
    fn from(val: IntoOffsetArithmetic) -> Self {
        match val {
            IntoOffsetArithmetic::Duration(d) => OffsetArithmetic::from(d.0),
            IntoOffsetArithmetic::SignedDuration(d) => OffsetArithmetic::from(d.0),
            IntoOffsetArithmetic::Span(s) => OffsetArithmetic::from(s.0),
        }
    }
}
