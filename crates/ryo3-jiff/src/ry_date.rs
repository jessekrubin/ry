use crate::pydatetime_conversions::{jiff_date2pydate, pydate2rydate};
use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::civil::Date;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::{PyDate, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{intern, pyclass, pymethods, Bound, IntoPyObject, PyErr, PyResult, Python};
use std::fmt::Display;

#[derive(Debug, Clone)]
#[pyclass(name = "Date")]
pub struct RyDate(pub(crate) Date);

#[pymethods]
impl RyDate {
    #[new]
    pub fn new(year: i16, month: i8, day: i8) -> PyResult<Self> {
        Date::new(year, month, day)
            .map(RyDate::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    #[classmethod]
    fn today(_cls: &Bound<'_, PyType>) -> Self {
        let z = jiff::civil::Date::from(Zoned::now());
        Self::from(z)
    }
    fn at(&self, hour: i8, minute: i8, second: i8, subsec_nanosecond: i32) -> RyDateTime {
        RyDateTime::from(self.0.at(hour, minute, second, subsec_nanosecond))
    }

    fn year(&self) -> i16 {
        self.0.year()
    }

    fn month(&self) -> i8 {
        self.0.month()
    }

    fn day(&self) -> i8 {
        self.0.day()
    }

    fn to_datetime(&self, time: &RyTime) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(time.0))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __richcmp__(&self, other: &RyDate, op: CompareOp) -> PyResult<bool> {
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
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Date<{self}>")
    }

    #[classmethod]
    fn from_pydate(_cls: &Bound<'_, PyType>, d: &Bound<'_, PyDate>) -> PyResult<Self> {
        pydate2rydate(d)
    }

    fn to_pydate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDate>> {
        jiff_date2pydate(py, &self.0)
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let year_any = self.0.year().into_pyobject(py)?.into_any();
        let month_any = self.0.month().into_pyobject(py)?.into_any();
        let day_any = self.0.day().into_pyobject(py)?.into_any();
        let parts = vec![year_any, month_any, day_any];

        PyTuple::new(py, parts)
    }
    fn asdict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "year"), self.0.year())?;
        dict.set_item(intern!(py, "month"), self.0.month())?;
        dict.set_item(intern!(py, "day"), self.0.day())?;
        Ok(dict)
    }
}

impl Display for RyDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date<{}>", self.0)
    }
}

impl From<Date> for RyDate {
    fn from(value: Date) -> Self {
        RyDate(value)
    }
}
