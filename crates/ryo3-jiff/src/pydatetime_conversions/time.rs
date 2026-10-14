use jiff::civil::Time;
use pyo3::prelude::*;
use pyo3::types::PyTime;

use crate::JiffTime;

pub fn time_to_pytime<'py>(
    py: Python<'py>,
    time: &jiff::civil::Time,
) -> PyResult<Bound<'py, PyTime>> {
    pyo3::types::PyTime::new(
        py,
        time.hour().try_into()?,
        time.minute().try_into()?,
        time.second().try_into()?,
        (time.subsec_nanosecond() / 1000).try_into()?,
        None,
    )
}

#[cfg(not(Py_LIMITED_API))]
#[expect(clippy::arithmetic_side_effects)]
pub fn pytime_to_time(time: &impl pyo3::types::PyTimeAccess) -> PyResult<Time> {
    Ok(Time::new(
        time.get_hour().try_into()?,
        time.get_minute().try_into()?,
        time.get_second().try_into()?,
        (time.get_microsecond() * 1000).try_into()?,
    )?)
}

#[cfg(Py_LIMITED_API)]
#[expect(clippy::arithmetic_side_effects)]
pub fn pytime_to_time<'py>(time: &impl AsRef<Bound<'py, PyAny>>) -> PyResult<Time> {
    use crate::interns;
    let time = time.as_ref();
    let py = time.py();
    Ok(Time::new(
        time.getattr(interns::hour(py))?.extract()?,
        time.getattr(interns::minute(py))?.extract()?,
        time.getattr(interns::second(py))?.extract()?,
        time.getattr(interns::microsecond(py))?.extract::<i32>()? * 1000,
    )?)
}

impl<'py> IntoPyObject<'py> for JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pytime(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pytime(py, &self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let time = ob.cast::<PyTime>()?;
        pytime_to_time(&*time).map(Self::from)
    }
}
