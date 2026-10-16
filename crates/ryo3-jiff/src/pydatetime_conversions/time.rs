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

#[expect(clippy::arithmetic_side_effects)]
pub fn pytime_to_time(time: &impl pyo3::types::PyTimeAccess) -> PyResult<Time> {
    use ryo3_core::map_py_value_err;

    Time::new(
        time.get_hour().try_into()?,
        time.get_minute().try_into()?,
        time.get_second().try_into()?,
        (time.get_microsecond() * 1000).try_into()?,
    )
    .map_err(map_py_value_err)
}

impl<'py> IntoPyObject<'py> for JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pytime(py, self.inner())
    }
}

impl<'py> IntoPyObject<'py> for &JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pytime(py, self.inner())
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let time = ob.cast::<PyTime>()?;
        pytime_to_time(&*time).map(Self::from)
    }
}
