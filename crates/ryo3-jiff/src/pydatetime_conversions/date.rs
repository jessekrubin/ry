use crate::JiffDate;
use jiff::civil::Date;
use pyo3::prelude::*;
use pyo3::types::PyDate;

impl<'py> IntoPyObject<'py> for JiffDate {
    type Target = PyDate;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffDate {
    type Target = PyDate;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        PyDate::new(
            py,
            self.0.year().into(),
            self.0.month().try_into()?,
            self.0.day().try_into()?,
        )
    }
}

impl FromPyObject<'_> for JiffDate {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let py_date = ob.downcast::<PyDate>()?;
        let date = py_date_to_date(py_date)?;
        Ok(Self::from(date))
    }
}
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
pub fn py_date_to_date(py_date: &impl pyo3::types::PyDateAccess) -> PyResult<Date> {
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
pub fn py_date_to_date(py_date: &Bound<'_, PyAny>) -> PyResult<Date> {
    let py = py_date.py();
    Ok(Date::new(
        py_date.getattr(pyo3::intern!(py, "year"))?.extract()?,
        py_date.getattr(pyo3::intern!(py, "month"))?.extract()?,
        py_date.getattr(pyo3::intern!(py, "day"))?.extract()?,
    )?)
}

#[cfg(not(Py_LIMITED_API))]
pub fn py_date_to_jiff_date(py_date: &impl pyo3::types::PyDateAccess) -> PyResult<JiffDate> {
    let d = py_date_to_date(py_date)?;
    Ok(JiffDate::from(d))
}

#[cfg(Py_LIMITED_API)]
pub fn py_date_to_jiff_date(py_date: &Bound<'_, PyAny>) -> PyResult<JiffDate> {
    Ok(JiffDate::from(py_date_to_date(py_date)?))
}
