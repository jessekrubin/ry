use crate::delta_arithmetic_self::RyDeltaArithmeticSelf;
use crate::errors::map_py_value_err;
use crate::ry_span::RySpan;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::{Timestamp, Zoned};
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::PyType;
use ryo3_macros::err_py_not_impl;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Timestamp", module = "ryo3")]
pub struct RyTimestamp(pub(crate) Timestamp);

#[pymethods]
impl RyTimestamp {
    #[new]
    #[pyo3(signature = (second = None, nanosecond = None))]
    pub fn new(second: Option<i64>, nanosecond: Option<i32>) -> PyResult<Self> {
        let s = second.unwrap_or(0);
        let ns = nanosecond.unwrap_or(0);
        Timestamp::new(s, ns)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        Self(Timestamp::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        Self(Timestamp::MAX)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UNIX_EPOCH() -> Self {
        Self(Timestamp::UNIX_EPOCH)
    }

    #[classmethod]
    fn now(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(Timestamp::now())
    }

    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Timestamp::from_str(s)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_millisecond(_cls: &Bound<'_, PyType>, millisecond: i64) -> PyResult<RyTimestamp> {
        Timestamp::from_millisecond(millisecond)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn to_zoned(&self, time_zone: RyTimeZone) -> RyZoned {
        RyZoned::from(Zoned::new(self.0, time_zone.0))
    }

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

    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        format!("Timestamp<{}>", self.string())
    }

    fn __repr__(&self) -> String {
        format!(
            "Timestamp({:?}, {:?})",
            self.0.as_second(),
            self.0.subsec_nanosecond()
        )
    }
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        // use nanosecond as hash as it is lossless
        self.0.as_nanosecond().hash(&mut hasher);
        hasher.finish()
    }

    fn __sub__<'py>(
        &self,
        py: Python<'py>,
        other: RyTimestampArithmeticSub,
    ) -> PyResult<Bound<'py, PyAny>> {
        match other {
            RyTimestampArithmeticSub::Timestamp(other) => {
                let span = self.0 - other.0;
                let obj = RySpan::from(span).into_pyobject(py).map(Bound::into_any)?;
                Ok(obj)
            }
            RyTimestampArithmeticSub::Delta(other) => {
                let t = match other {
                    RyDeltaArithmeticSelf::Span(other) => self.0 - other.0,
                    RyDeltaArithmeticSelf::SignedDuration(other) => self.0 - other.0,
                    RyDeltaArithmeticSelf::Duration(other) => self.0 - other.0,
                };
                RyTimestamp::from(t).into_pyobject(py).map(Bound::into_any)
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

    fn __add__(&self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<Self> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::Duration(other) => self.0 + other.0,
        };
        Ok(Self::from(t))
    }

    fn __iadd__(&mut self, _py: Python<'_>, other: RyDeltaArithmeticSelf) -> PyResult<()> {
        let t = match other {
            RyDeltaArithmeticSelf::Span(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::SignedDuration(other) => self.0 + other.0,
            RyDeltaArithmeticSelf::Duration(other) => self.0 + other.0,
        };
        self.0 = t;
        Ok(())
    }
    fn as_second(&self) -> i64 {
        self.0.as_second()
    }

    fn as_microsecond(&self) -> i64 {
        self.0.as_microsecond()
    }

    fn as_millisecond(&self) -> i64 {
        self.0.as_millisecond()
    }

    fn as_nanosecond(&self) -> i128 {
        self.0.as_nanosecond()
    }
    fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }
    fn subsec_microsecond(&self) -> i32 {
        self.0.subsec_microsecond()
    }
    fn subsec_millisecond(&self) -> i32 {
        self.0.subsec_millisecond()
    }

    fn series(&self, period: &RySpan) -> RyTimestampSeries {
        RyTimestampSeries {
            series: self.0.series(period.0),
        }
    }

    #[getter]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn intz(&self, time_zone_name: &str) -> PyResult<RyZoned> {
        self.0
            .intz(time_zone_name)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    #[classmethod]
    fn from_microsecond(_cls: &Bound<'_, PyType>, microsecond: i64) -> PyResult<RyTimestamp> {
        Timestamp::from_microsecond(microsecond)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_nanosecond(_cls: &Bound<'_, PyType>, nanosecond: i128) -> PyResult<RyTimestamp> {
        Timestamp::from_nanosecond(nanosecond)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn from_second(_cls: &Bound<'_, PyType>, second: i64) -> PyResult<RyTimestamp> {
        Timestamp::from_second(second)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }
    fn signum(&self) -> i8 {
        self.0.signum()
    }
    fn strftime(&self, format: &str) -> PyResult<String> {
        Ok(self.0.strftime(format).to_string())
    }

    #[classmethod]
    fn strptime(_cls: &Bound<'_, PyType>, s: &str, format: &str) -> PyResult<RyTimestamp> {
        Timestamp::strptime(s, format)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn checked_add(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn checked_sub(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn display_with_offset(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn duration_since(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn duration_until(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn since(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn round(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn saturating_add(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn saturating_sub(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn until(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
}
impl Display for RyTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<Timestamp> for RyTimestamp {
    fn from(value: Timestamp) -> Self {
        RyTimestamp(value)
    }
}

#[pyclass]
#[pyo3(name = "TimestampSeries", module = "ryo3")]
pub struct RyTimestampSeries {
    pub(crate) series: jiff::TimestampSeries,
}

#[pymethods]
impl RyTimestampSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<RyTimestamp> {
        slf.series.next().map(RyTimestamp::from)
    }
}

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RyTimestampArithmeticSub {
    Timestamp(RyTimestamp),
    Delta(RyDeltaArithmeticSelf),
}
