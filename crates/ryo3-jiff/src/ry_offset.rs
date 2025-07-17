use crate::JiffOffset;
use crate::errors::{map_py_overflow_err, map_py_value_err};
use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_timezone::RyTimeZone;
use crate::spanish::Spanish;
use jiff::tz::Offset;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::{PyTuple, PyType};
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
                .map(Self::from)
                .map_err(map_py_value_err),
            (None, Some(s)) => Offset::from_seconds(s)
                .map(Self::from)
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
        Self::from(Offset::MIN)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self::from(Offset::MAX)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        Self::from(Offset::ZERO)
    }

    #[expect(non_snake_case)]
    #[classattr]
    fn UTC() -> Self {
        Self::from(Offset::UTC)
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Offset::UTC)
    }

    #[classmethod]
    fn from_hours(_cls: &Bound<'_, PyType>, hours: i8) -> PyResult<Self> {
        Offset::from_hours(hours)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn from_seconds(_cls: &Bound<'_, PyType>, seconds: i32) -> PyResult<Self> {
        Offset::from_seconds(seconds)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    fn to_py(&self) -> &Offset {
        &self.0
    }

    fn to_pytzinfo(&self) -> &Offset {
        &self.0
    }

    #[classmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_pytzinfo(_cls: &Bound<'_, PyType>, d: JiffOffset) -> Self {
        Self::from(d.0)
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
        Self::from(self.0.negate())
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
    fn until(&self, other: &Self) -> RySpan {
        let s = self.0.until(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn since(&self, other: &Self) -> RySpan {
        let s = self.0.since(other.0);
        RySpan::from(s)
    }

    #[must_use]
    fn duration_until(&self, other: &Self) -> RySignedDuration {
        let s = self.0.duration_until(other.0);
        RySignedDuration::from(s)
    }

    #[must_use]
    fn duration_since(&self, other: &Self) -> RySignedDuration {
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
    fn __add__<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        self.0
            .checked_add(spanish)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn __sub__<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        self.0
            .checked_sub(spanish)
            .map(Self::from)
            .map_err(map_py_overflow_err)
    }

    fn add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        self.__add__(other)
    }

    fn sub<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        self.__sub__(other)
    }

    fn to_timezone(&self) -> RyTimeZone {
        RyTimeZone::from(self.0.to_time_zone())
    }

    fn saturating_add<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(self.0.saturating_add(spanish).into())
    }

    fn saturating_sub<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<Self> {
        let spanish = Spanish::try_from(other)?;
        Ok(self.0.saturating_sub(spanish).into())
    }
}

impl From<Offset> for RyOffset {
    fn from(value: Offset) -> Self {
        Self(value)
    }
}

impl From<JiffOffset> for RyOffset {
    fn from(value: JiffOffset) -> Self {
        Self::from(value.0)
    }
}

pub(crate) fn print_isoformat_offset<W: std::fmt::Write>(
    offset: &Offset,
    w: &mut W,
) -> std::fmt::Result {
    if offset.is_zero() {
        return write!(w, "+00:00");
    }
    // total number of seconds
    let sign = if offset.is_negative() { "-" } else { "+" };
    let total_seconds = offset.seconds();
    // calculate hours and minutes, and seconds
    let hours = total_seconds.abs() / 3600;
    let minutes = (total_seconds.abs() % 3600) / 60;
    let seconds = total_seconds.abs() % 60;

    // write the formatted string
    if seconds == 0 {
        write!(w, "{sign}{hours:02}:{minutes:02}")
    } else {
        write!(w, "{sign}{hours:02}:{minutes:02}:{seconds:02}")
    }
}
