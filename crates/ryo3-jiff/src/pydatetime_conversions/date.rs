use jiff::civil::Date;
use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateAccess};

pub fn date_to_pyobject<'a>(py: Python<'a>, d: &Date) -> PyResult<Bound<'a, PyDate>> {
    let y = i32::from(d.year());
    let m = d.month();

    let m_u8 = u8::try_from(m)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let d = d.day();
    let d_u8 = u8::try_from(d)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    PyDate::new(py, y, m_u8, d_u8)
}

#[cfg(not(Py_LIMITED_API))]
pub fn py_date_to_jiff_date(py_date: &impl PyDateAccess) -> PyResult<jiff::civil::Date> {
    let y = py_date.get_year();
    let y_i16 = i16::try_from(y)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let m = py_date.get_month();
    let m_i8 = i8::try_from(m)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let d = py_date.get_day();
    let d_i8 = i8::try_from(d)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let date = Date::new(y_i16, m_i8, d_i8)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("date: {e}")))?;
    Ok(date)
}

#[cfg(Py_LIMITED_API)]
pub fn py_date_to_naive_date(py_date: &Bound<'_, PyAny>) -> PyResult<jiff::civil::Date> {
    Date::new(
        py_date
            .getattr(pyo3::intern!(py_date.py(), "year"))?
            .extract()?,
        py_date
            .getattr(pyo3::intern!(py_date.py(), "month"))?
            .extract()?,
        py_date
            .getattr(pyo3::intern!(py_date.py(), "day"))?
            .extract()?,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))
}
pub fn date_from_pyobject(py_date: &impl PyDateAccess) -> PyResult<Date> {
    py_date_to_jiff_date(py_date)
}
