use crate::ry_datetime::RyDateTime;
use crate::ry_time::RyTime;
use crate::ry_timezone::RyTimeZone;
use crate::ry_zoned::RyZoned;
use jiff::civil::Date;
use jiff::Zoned;
use pyo3::basic::CompareOp;
use pyo3::types::{PyDate, PyDateAccess, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{pyclass, pymethods, Bound, IntoPy, PyErr, PyObject, PyResult, Python};
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

    fn to_datetime(&self, time: RyTime) -> RyDateTime {
        RyDateTime::from(self.0.to_datetime(time.0))
    }

    fn to_zoned(&self, tz: RyTimeZone) -> PyResult<RyZoned> {
        self.0
            .to_zoned(tz.0)
            .map(RyZoned::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
    }

    fn __richcmp__(&self, other: &RyDate, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self.0 == other.0).into_py(py),
            CompareOp::Ne => (self.0 != other.0).into_py(py),
            CompareOp::Lt => (self.0 < other.0).into_py(py),
            CompareOp::Le => (self.0 <= other.0).into_py(py),
            CompareOp::Gt => (self.0 > other.0).into_py(py),
            CompareOp::Ge => (self.0 >= other.0).into_py(py),
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
        let a = PyDate::new_bound(
            py,
            i32::from(self.0.year()),
            self.0.month() as u8,
            self.0.day() as u8,
        );
        a
    }

    fn astuple<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let tup = PyTuple::new_bound(
            py,
            vec![
                self.0.year().into_py(py),
                self.0.month().into_py(py),
                self.0.day().into_py(py),
            ],
        );
        Ok(tup)
    }

    fn asdict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("year", self.0.year())?;
        dict.set_item("month", self.0.month())?;
        dict.set_item("day", self.0.day())?;
        Ok(dict.into())
    }
}

fn pydate2rydate(py_date: &impl PyDateAccess) -> PyResult<RyDate> {
    let y = py_date.get_year();
    let y_i16 = i16::try_from(y)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let m = py_date.get_month();
    let m_i8 = i8::try_from(m)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let d = py_date.get_day();
    let d_i8 = i8::try_from(d)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    RyDate::new(y_i16, m_i8, d_i8)
}

// #[derive(Debug, FromPyObject)]
// enum RyDateComparable<'py> {
//     RyDate(RyDate),
//     PyDate(&'py Bound<'py, PyDate>),
// }

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
