use crate::ry_datetime::RyDateTime;
use crate::ry_signed_duration::RySignedDuration;
use crate::ry_span::RySpan;
use crate::ry_timestamp::RyTimestamp;
use jiff::tz::Offset;
use pyo3::types::PyType;
use pyo3::{pyclass, pyfunction, pymethods, Bound, PyErr, PyResult};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "Offset", module = "ryo3")]
pub struct RyOffset(pub(crate) Offset);

#[pymethods]
impl RyOffset {
    #[new]
    pub fn new(hours: i8) -> Self {
        RyOffset::from(jiff::tz::offset(hours))
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MIN() -> Self {
        RyOffset::from(Offset::MIN)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn MAX() -> Self {
        RyOffset::from(Offset::MAX)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn ZERO() -> Self {
        RyOffset::from(Offset::ZERO)
    }

    #[allow(non_snake_case)]
    #[classattr]
    fn UTC() -> Self {
        RyOffset::from(Offset::UTC)
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        RyOffset::from(Offset::UTC)
    }

    #[classmethod]
    pub fn constant(_cls: &Bound<'_, PyType>, hours: i8) -> Self {
        Self::new(hours)
    }

    #[classmethod]
    pub fn from_hours(_cls: &Bound<'_, PyType>, hours: i8) -> Self {
        Self::new(hours)
    }

    #[classmethod]
    pub fn from_seconds(_cls: &Bound<'_, PyType>, seconds: i32) -> PyResult<Self> {
        Offset::from_seconds(seconds)
            .map(RyOffset::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    pub fn seconds(&self) -> i32 {
        self.0.seconds()
    }

    pub fn negate(&self) -> Self {
        RyOffset::from(self.0.negate())
    }

    fn __neg__(&self) -> Self {
        self.negate()
    }

    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    pub fn to_datetime(&self, timestamp: &RyTimestamp) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(timestamp.0))
    }

    pub fn to_timestamp(&self, datetime: &RyDateTime) -> PyResult<RyTimestamp> {
        self.0
            .to_timestamp(datetime.0)
            .map(RyTimestamp::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    pub fn until(&self, other: &RyOffset) -> RySpan {
        let s = self.0.until(other.0);
        RySpan::from(s)
    }

    pub fn since(&self, other: &RyOffset) -> RySpan {
        let s = self.0.since(other.0);
        RySpan::from(s)
    }

    pub fn duration_until(&self, other: &RyOffset) -> RySignedDuration {
        let s = self.0.duration_until(other.0);
        RySignedDuration::from(s)
    }

    pub fn duration_since(&self, other: &RyOffset) -> RySignedDuration {
        let s = self.0.duration_since(other.0);
        RySignedDuration::from(s)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

impl From<Offset> for RyOffset {
    fn from(value: Offset) -> Self {
        RyOffset(value)
    }
}

#[pyfunction]
pub(crate) fn offset(hours: i8) -> RyOffset {
    RyOffset::from(jiff::tz::offset(hours))
}
