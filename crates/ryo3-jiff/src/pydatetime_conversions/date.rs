use jiff::civil::Date;
use pyo3::prelude::*;

use crate::JiffDate;

#[cfg(not(Py_LIMITED_API))]
pub fn pydate_to_date(py_date: &impl pyo3::types::PyDateAccess) -> PyResult<Date> {
    use ryo3_core::map_py_value_err;

    let y = py_date.get_year();
    let y_i16 = i16::try_from(y).map_err(map_py_value_err)?;
    let m = py_date.get_month();
    let m_i8 = i8::try_from(m).map_err(map_py_value_err)?;
    let d = py_date.get_day();
    let d_i8 = i8::try_from(d).map_err(map_py_value_err)?;
    let date = Date::new(y_i16, m_i8, d_i8).map_err(map_py_value_err)?;
    Ok(date)
}

#[cfg(Py_LIMITED_API)]
pub fn pydate_to_date<'py>(py_date: &impl AsRef<Bound<'py, PyAny>>) -> PyResult<Date> {
    use crate::interns;
    let py_date = py_date.as_ref();
    let py = py_date.py();
    Ok(Date::new(
        py_date.getattr(interns::year(py))?.extract()?,
        py_date.getattr(interns::month(py))?.extract()?,
        py_date.getattr(interns::day(py))?.extract()?,
    )?)
}

pub fn date_to_pydate<'py>(
    py: Python<'py>,
    date: &Date,
) -> PyResult<Bound<'py, pyo3::types::PyDate>> {
    pyo3::types::PyDate::new(
        py,
        date.year().into(),
        date.month().cast_unsigned(),
        date.day().cast_unsigned(),
    )
}

impl<'py> IntoPyObject<'py> for JiffDate {
    type Target = pyo3::types::PyDate;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffDate {
    type Target = pyo3::types::PyDate;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        date_to_pydate(py, &self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffDate {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py_date = ob.cast::<pyo3::types::PyDate>()?;
        #[cfg(not(Py_LIMITED_API))]
        {
            use pyo3::types::PyDateAccess;
            use ryo3_core::map_py_value_err;

            Date::new(
                py_date.get_year().try_into()?,
                py_date.get_month().try_into()?,
                py_date.get_day().try_into()?,
            )
            .map(Self::from)
            .map_err(map_py_value_err)
        }

        #[cfg(Py_LIMITED_API)]
        {
            pydate_to_date(&*py_date).map(Self::from)
        }
    }
}
