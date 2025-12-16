use crate::JiffTimeZone;
use crate::errors::map_py_value_err;
use crate::ry_datetime::RyDateTime;
use crate::ry_offset::RyOffset;
use crate::ry_timestamp::RyTimestamp;
use crate::ry_zoned::RyZoned;
use jiff::Timestamp;
use jiff::tz::{Offset, TimeZone};
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyTzInfo;
use pyo3::types::{PyDict, PyString, PyTuple};
use ryo3_macro_rules::{py_type_err, pytodo};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
#[pyclass(name = "TimeZone", frozen, immutable_type, from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimeZone(pub(crate) std::sync::Arc<TimeZone>);

#[pymethods]
impl RyTimeZone {
    #[new]
    fn py_new(time_zone_name: &str) -> PyResult<Self> {
        if time_zone_name.is_empty() || time_zone_name.eq_ignore_ascii_case("unknown") {
            return Ok(Self::from(TimeZone::unknown()));
        }
        if time_zone_name.eq_ignore_ascii_case("utc") {
            return Ok(Self::from(TimeZone::fixed(Offset::UTC)));
        }
        TimeZone::get(time_zone_name)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[classattr]
    #[expect(non_snake_case)]
    fn UTC() -> Self {
        Self::from(TimeZone::UTC)
    }

    // =====================================================================
    // DUNDERS
    // =====================================================================

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(
            py,
            vec![self.iana_name().unwrap_or("").into_bound_py_any(py)?],
        )
    }

    fn __call__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __str__(&self) -> String {
        if let Some(name) = self.iana_name() {
            name.to_string()
        } else {
            // REALLY NOT SURE IF THIS IS CORRECT
            let offset = self.0.to_offset(Timestamp::now());
            format!("{offset}")
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        match self.0.to_fixed_offset() {
            Ok(offset) => offset.hash(&mut hasher),
            Err(_) => {
                if let Some(name) = self.iana_name() {
                    name.hash(&mut hasher);
                }
            }
        }
        hasher.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn equiv<'py>(&self, other: &'py Bound<'py, PyAny>) -> PyResult<bool> {
        if let Ok(other) = other.cast::<Self>() {
            Ok(self.0.eq(&other.get().0))
        } else if let Ok(other) = other.cast::<PyTzInfo>() {
            let tz: jiff::tz::TimeZone = other.extract()?;
            Ok((*self.0).eq(&tz))
        } else if let Ok(other) = other.cast::<PyString>() {
            let other_str = other.extract::<&str>()?;
            Ok(self.0.iana_name() == Some(other_str))
        } else {
            py_type_err!("Expected TimeZone, datetime.tzinfo or string")
        }
    }

    // =====================================================================
    // PY-CONVERSIONS
    // =====================================================================

    fn to_py(&self) -> &TimeZone {
        &self.0
    }

    fn to_pytzinfo(&self) -> &TimeZone {
        &self.0
    }

    #[staticmethod]
    fn from_pytzinfo(tz: JiffTimeZone) -> Self {
        Self::from(tz.0)
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        use crate::interns;
        let dict = PyDict::new(py);
        dict.set_item(interns::tz(py), self.iana_name().unwrap_or("unknown"))?;
        Ok(dict)
    }

    // =====================================================================
    // CLASS METHODS
    // =====================================================================
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        use ryo3_core::PyFromStr;
        Self::py_from_str(s)
    }

    #[staticmethod]
    fn parse(s: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyParse;
        Self::py_parse(s)
    }

    #[staticmethod]
    fn fixed(offset: &RyOffset) -> Self {
        Self::from(TimeZone::fixed(offset.0))
    }

    #[staticmethod]
    fn posix(tz_name: &str) -> PyResult<Self> {
        TimeZone::posix(tz_name)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn get(tz_name: &str) -> PyResult<Self> {
        TimeZone::get(tz_name)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn tzif(name: &str, data: &[u8]) -> PyResult<Self> {
        TimeZone::tzif(name, data)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn utc() -> Self {
        Self::UTC()
    }

    #[staticmethod]
    fn try_system() -> PyResult<Self> {
        TimeZone::try_system()
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn system() -> PyResult<Self> {
        TimeZone::try_system()
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    // =====================================================================
    // INSTANCE METHODS
    // =====================================================================

    /// Return dictionary with `offset`, `dst` and `abbreviation` from `TimeZone`
    /// given a `Timestamp`
    fn to_offset_info<'py>(
        &self,
        py: Python<'py>,
        timestamp: &RyTimestamp,
    ) -> PyResult<Bound<'py, PyDict>> {
        let offset_info = self.0.to_offset_info(timestamp.0);
        let dict = PyDict::new(py);
        let ryoff = RyOffset::from(offset_info.offset());
        dict.set_item(crate::interns::offset(py), ryoff.to_dict(py)?)?;
        dict.set_item(crate::interns::dst(py), offset_info.dst().is_dst())?;
        dict.set_item(crate::interns::abbreviation(py), offset_info.abbreviation())?;
        Ok(dict)
    }

    fn to_datetime(&self, timestamp: &RyTimestamp) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(timestamp.0))
    }

    /// Return `Offset` from `TimeZone`
    fn to_offset(&self, timestamp: &RyTimestamp) -> RyOffset {
        RyOffset::from(self.0.to_offset(timestamp.0))
    }

    /// Return `Timestamp` from `TimeZone` given a `DateTime`
    fn to_timestamp(&self, datetime: &RyDateTime) -> Result<RyTimestamp, PyErr> {
        self.0
            .to_timestamp(datetime.0)
            .map(RyTimestamp::from)
            .map_err(map_py_value_err)
    }

    /// Return `Zoned` from `TimeZone` given a `DateTime`
    fn to_zoned(&self, datetime: &RyDateTime) -> PyResult<RyZoned> {
        self.0
            .to_zoned(datetime.0)
            .map(RyZoned::from)
            .map_err(map_py_value_err)
    }

    fn iana_name(&self) -> Option<&str> {
        self.0.iana_name()
    }

    fn to_fixed_offset(&self) -> PyResult<RyOffset> {
        self.0
            .to_fixed_offset()
            .map(RyOffset::from)
            .map_err(map_py_value_err)
    }

    // =====================================================================
    // PROPERTIES
    // =====================================================================

    #[getter]
    fn name(&self) -> Option<&str> {
        self.iana_name()
    }

    #[getter]
    fn is_unknown(&self) -> bool {
        self.0.is_unknown()
    }

    // ===============
    // NOT IMPLEMENTED
    // ===============
    #[expect(clippy::unused_self)]
    fn to_ambiguous_timestamp(&self) -> PyResult<()> {
        pytodo!()
    }

    #[expect(clippy::unused_self)]
    fn to_ambiguous_zoned(&self) -> PyResult<()> {
        pytodo!()
    }
}

impl std::fmt::Display for RyTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeZone(")?;

        if self.is_unknown() {
            write!(f, "\"unknown\"")?;
        } else if let Some(name) = self.iana_name() {
            write!(f, "\"{name}\"")?;
        } else {
            // REALLY NOT SURE IF THIS IS CORRECT
            let offset = self.0.to_offset(Timestamp::now());
            write!(f, "'{offset}'")?;
        }
        write!(f, ")")
    }
}
