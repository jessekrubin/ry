use crate::ry_date::RyDate;
use pyo3::types::{PyDate, PyDateAccess};
use pyo3::{Bound, PyErr, PyResult, Python};

pub fn jiff_date2pydate<'a>(py: Python<'a>, d: &jiff::civil::Date) -> PyResult<Bound<'a, PyDate>> {
    let y = i32::from(d.year());
    let m = d.month();

    let m_u8 = u8::try_from(m)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let d = d.day();
    let d_u8 = u8::try_from(d)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    PyDate::new(py, y, m_u8, d_u8)
}

pub fn pydate2rydate(py_date: &impl PyDateAccess) -> PyResult<RyDate> {
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
