use crate::errors::map_py_value_err;
use crate::ry_offset::RyOffset;
use crate::ry_timestamp::RyTimestamp;
use crate::JiffTimeZone;
use jiff::tz::{Dst, Offset, TimeZone};
use jiff::Timestamp;
use pyo3::types::{PyAnyMethods, PyTuple, PyType, PyTzInfo};
use pyo3::{
    pyclass, pymethods, Bound, FromPyObject, IntoPyObject, IntoPyObjectExt, PyAny, PyErr, PyResult,
    Python,
};
use ryo3_macros::err_py_not_impl;
use std::any::Any;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", module = "ryo3")]
pub struct RyTimeZone(pub(crate) TimeZone);

impl From<TimeZone> for RyTimeZone {
    fn from(value: TimeZone) -> Self {
        RyTimeZone(value)
    }
}

impl From<&TimeZone> for RyTimeZone {
    fn from(value: &TimeZone) -> Self {
        RyTimeZone(value.clone())
    }
}

#[pymethods]
impl RyTimeZone {
    #[new]
    pub fn new(time_zone_name: &str) -> PyResult<Self> {
        TimeZone::get(time_zone_name)
            .map(RyTimeZone::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn utc(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(TimeZone::fixed(Offset::UTC))
    }

    #[classmethod]
    fn system(_cls: &Bound<'_, PyType>) -> Self {
        Self::from(TimeZone::system())
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    #[getter]
    fn name(&self) -> Option<&str> {
        self.iana_name()
    }

    fn __repr__(&self) -> String {
        // let iana_name = self.0.iana_name();
        let iana_name = self.0.iana_name();
        match iana_name {
            Some(name) => format!("TimeZone(\"{name}\")"),
            None => {
                // REALLY NOT SURE IF THIS IS CORRECT
                let (offset, _dst, _s) = self.0.to_offset(Timestamp::now());
                format!("TimeZone('{offset}')")
            }
        }
    }

    #[classmethod]
    fn from_offset(_cls: &Bound<'_, PyType>, offset: &RyOffset) -> Self {
        Self::from(TimeZone::fixed(offset.0))
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.__str__().hash(&mut hasher);
        hasher.finish()
    }

    fn __str__(&self) -> String {
        self.iana_name()
            .unwrap_or(&self.0.to_offset(Timestamp::now()).0.to_string())
            .to_string()
    }

    fn __eq__(&self, other: TimeZoneEquality) -> bool {
        match other {
            TimeZoneEquality::TimeZone(other) => {
                self.0.eq(&other.0) || self.0.iana_name() == other.0.iana_name()
            }
            TimeZoneEquality::Str(other) => self.0.iana_name() == Some(other.as_str()),
        }
    }

    fn to_pytzinfo<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let jiff_tz = JiffTimeZone::from(self.0.clone()); // TODO: figure out no clone
        jiff_tz.into_pyobject(py)
    }

    #[classmethod]
    fn from_pytzinfo(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyTzInfo>) -> PyResult<Self> {
        let jiff_tz: JiffTimeZone = d.extract()?;
        Ok(Self::from(jiff_tz.0))
    }

    #[classmethod]
    fn fixed(_cls: &Bound<'_, PyType>, offset: &RyOffset) -> Self {
        Self::from(TimeZone::fixed(offset.0))
    }

    // ===============
    // NOT IMPLEMENTED
    // ===============

    fn get(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    // fn into_ambiguous_zoned(self) -> PyResult<()> {
    //     err_py_not_impl!()
    // }

    #[classmethod]
    fn posix(_cls: &Bound<'_, PyType>, string: &str) -> PyResult<Self> {
        TimeZone::posix(string)
            .map(RyTimeZone::from)
            .map_err(map_py_value_err)
    }

    fn to_ambiguous_timestamp(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn to_ambiguous_zoned(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn to_datetime(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn to_offset<'py>(
        &self,
        py: Python<'py>,
        timestamp: RyTimestamp,
    ) -> PyResult<Bound<'py, PyTuple>> {
        let (offset, dst, s) = self.0.to_offset(timestamp.0);
        let offset = RyOffset::from(offset).into_py_any(py)?;
        let dst_bool = matches!(dst, Dst::Yes);
        let dst_py = dst_bool.into_py_any(py)?;
        let s = s.into_py_any(py)?;
        PyTuple::new(py, vec![offset, dst_py, s])
    }

    fn to_timestamp(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn to_zoned(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn try_system(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
    fn tzif(&self) -> PyResult<()> {
        err_py_not_impl!()
    }
}

#[derive(Debug, Clone, FromPyObject)]
enum TimeZoneEquality {
    TimeZone(RyTimeZone),
    Str(String),
}
