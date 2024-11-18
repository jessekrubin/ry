use crate::internal::RySpanRelativeTo;
use crate::ry_signed_duration::RySignedDuration;
use jiff::Span;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
#[pyclass(name = "Span", module = "ryo3")]
pub struct RySpan(pub(crate) Span);

#[pymethods]
impl RySpan {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(Span::new()))
    }
    fn __str__(&self) -> String {
        self.string()
    }

    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }

    fn __repr__(&self) -> String {
        format!("Span<{}>", self.0)
    }

    fn __neg__(&self) -> PyResult<Self> {
        Ok(Self(self.0.negate()))
    }

    fn __invert__(&self) -> PyResult<Self> {
        Ok(Self(self.0.negate()))
    }
    #[classmethod]
    fn parse(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        Span::from_str(s)
            .map(RySpan::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn years(&self, n: i64) -> PyResult<Self> {
        let s = self.0.years(n);
        Ok(RySpan::from(s))
    }

    fn months(&self, n: i64) -> PyResult<Self> {
        let s = self.0.months(n);
        Ok(RySpan::from(s))
    }

    fn weeks(&self, n: i64) -> PyResult<Self> {
        let s = self.0.weeks(n);
        Ok(RySpan::from(s))
    }

    fn days(&self, n: i64) -> PyResult<Self> {
        let s = self.0.days(n);
        Ok(RySpan::from(s))
    }

    fn hours(&self, n: i64) -> PyResult<Self> {
        let s = self.0.hours(n);
        Ok(RySpan::from(s))
    }

    fn minutes(&self, n: i64) -> PyResult<Self> {
        let s = self.0.minutes(n);
        Ok(RySpan::from(s))
    }

    fn seconds(&self, n: i64) -> PyResult<Self> {
        let s = self.0.seconds(n);
        Ok(RySpan::from(s))
    }

    fn milliseconds(&self, n: i64) -> PyResult<Self> {
        let s = self.0.milliseconds(n);
        Ok(RySpan::from(s))
    }

    fn microseconds(&self, n: i64) -> PyResult<Self> {
        let s = self.0.microseconds(n);
        Ok(RySpan::from(s))
    }

    fn to_jiff_duration(&self, relative: RySpanRelativeTo) -> PyResult<RySignedDuration> {
        match relative {
            RySpanRelativeTo::Zoned(z) => self
                .0
                .to_jiff_duration(&z.0)
                .map(RySignedDuration)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            RySpanRelativeTo::Date(d) => self
                .0
                .to_jiff_duration(d.0)
                .map(RySignedDuration)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
            RySpanRelativeTo::DateTime(dt) => self
                .0
                .to_jiff_duration(dt.0)
                .map(RySignedDuration)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
        }
    }
}
impl Display for RySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Span> for RySpan {
    fn from(span: Span) -> Self {
        Self(span)
    }
}
