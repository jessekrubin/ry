use pyo3::prelude::*;
use pyo3::{IntoPyObjectExt, PyTypeInfo};
use ryo3_core::{PyCastExactOpt, py_type_err, py_value_err};

use crate::spanish::Spanish;
use crate::{RyDate, RyDateTime, RySpan, RyTime, RyTimestamp, RyZoned};

#[derive(Debug, Clone)]
pub(crate) enum PyTemporalTypes<'a, 'py> {
    Date(Borrowed<'a, 'py, RyDate>),
    DateTime(Borrowed<'a, 'py, RyDateTime>),
    Time(Borrowed<'a, 'py, RyTime>),
    Zoned(Borrowed<'a, 'py, RyZoned>),
    Timestamp(Borrowed<'a, 'py, RyTimestamp>),
}

trait PyTemporalTypeName {
    const PY_NAME: &'static str;
}
macro_rules! impl_py_temporal_type_name {
    ($ty:ty, $name:expr) => {
        impl PyTemporalTypeName for $ty {
            const PY_NAME: &'static str = $name;
        }
    };
}
impl_py_temporal_type_name!(RyDate, "Date");
impl_py_temporal_type_name!(RyDateTime, "DateTime");
impl_py_temporal_type_name!(RyTime, "Time");
impl_py_temporal_type_name!(RyZoned, "ZonedDateTime");
impl_py_temporal_type_name!(RyTimestamp, "Timestamp");

impl<'a, 'py> FromPyObject<'a, 'py> for PyTemporalTypes<'a, 'py> {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        macro_rules! try_extract_type {
            ($ty:ty, $variant:ident) => {
                if let Some(val) = obj.cast_exact_opt::<$ty>() {
                    return Ok(Self::$variant(val));
                }
            };
        }

        try_extract_type!(RyZoned, Zoned);
        try_extract_type!(RyTimestamp, Timestamp);
        try_extract_type!(RyDateTime, DateTime);
        try_extract_type!(RyDate, Date);
        try_extract_type!(RyTime, Time);
        py_type_err!("Expected a Date, DateTime, Time, Timestamp, or ZonedDateTime type")
    }
}

/// Input to temporal types subtraction ops
pub(crate) enum TemporalSubInput<'a, 'py, T> {
    Temporal(Borrowed<'a, 'py, T>),
    Spanish(Spanish<'a, 'py>),
}

impl<'a, 'py, T> FromPyObject<'a, 'py> for TemporalSubInput<'a, 'py, T>
where
    T: PyTemporalTypeName + PyTypeInfo,
{
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(ts) = ob.cast_exact::<T>() {
            Ok(Self::Temporal(ts))
        } else if let Ok(spanish) = ob.extract::<Spanish>() {
            Ok(Self::Spanish(spanish))
        } else {
            py_type_err!("Expected a {} or span-like object", T::PY_NAME)
        }
    }
}

pub(crate) enum TemporalSubOutput<T> {
    ValueError(jiff::Error),
    Overflow(jiff::Error),
    Temporal(T),
    Span(RySpan),
}

impl<T> From<jiff::Span> for TemporalSubOutput<T> {
    fn from(span: jiff::Span) -> Self {
        Self::Span(RySpan(span))
    }
}

impl<T> From<RySpan> for TemporalSubOutput<T> {
    fn from(span: RySpan) -> Self {
        Self::Span(span)
    }
}

impl<T> From<Result<T, jiff::Error>> for TemporalSubOutput<T> {
    fn from(result: Result<T, jiff::Error>) -> Self {
        match result {
            Ok(value) => Self::Temporal(value),
            Err(e) => {
                if e.is_range() {
                    Self::Overflow(e)
                } else {
                    Self::ValueError(e)
                }
            }
        }
    }
}

impl<'py, T> IntoPyObject<'py> for TemporalSubOutput<T>
where
    T: IntoPyObject<'py>,
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Temporal(ts) => ts.into_bound_py_any(py),
            Self::Span(span) => span.into_pyobject(py).map(Bound::into_any),
            Self::Overflow(err) => ryo3_core::py_overflow_err!("{err}"),
            Self::ValueError(err) => py_value_err!("{err}"),
        }
    }
}
