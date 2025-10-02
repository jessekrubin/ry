use crate::JiffTime;
#[cfg(not(Py_LIMITED_API))]
use jiff::civil::Time;
use pyo3::prelude::*;
use pyo3::types::PyTime;

pub fn time_to_pyobject<'py>(
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
pub(crate) fn pytime_to_time(time: &impl pyo3::types::PyTimeAccess) -> PyResult<Time> {
    Ok(Time::new(
        time.get_hour().try_into()?,
        time.get_minute().try_into()?,
        time.get_second().try_into()?,
        (time.get_microsecond() * 1000).try_into()?,
    )?)
}

#[cfg(Py_LIMITED_API)]
pub(crate) fn pytime_to_time(time: &Bound<'_, PyAny>) -> PyResult<Time> {
    let py = time.py();
    Ok(Time::new(
        time.getattr(intern!(py, "hour"))?.extract()?,
        time.getattr(intern!(py, "minute"))?.extract()?,
        time.getattr(intern!(py, "second"))?.extract()?,
        time.getattr(intern!(py, "microsecond"))?.extract::<i32>()? * 1000,
    )?)
}
// #[cfg(not(Py_LIMITED_API))]
// #[expect(clippy::arithmetic_side_effects)]
// pub fn pytime_to_time(py_time: &impl pyo3::types::PyTimeAccess) -> PyResult<jiff::civil::Time> {
//     let hour = py_time.get_hour();
//     let hour_i8 = i8::try_from(hour)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("hour: {e}")))?;

//     let minute = py_time.get_minute();

//     let minute_i8 = i8::try_from(minute)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("minute: {e}")))?;

//     let second = py_time.get_second();
//     let second_i8 = i8::try_from(second)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("second: {e}")))?;

//     let microsecond = py_time.get_microsecond();
//     let subsec_nanosecond = microsecond * 1000;
//     let subsec_nanosecond_i32 = i32::try_from(subsec_nanosecond).map_err(|e| {
//         PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("subsec_nanosecond: {e}"))
//     })?;
//     let t = jiff::civil::Time::new(hour_i8, minute_i8, second_i8, subsec_nanosecond_i32)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     Ok(t)
// }

// #[cfg(Py_LIMITED_API)]
// pub fn pytime_to_time(py_time: &Bound<'_, PyAny>) -> PyResult<jiff::civil::Time> {
//     use pyo3::intern;
//     let hour_i8 = py_time
//         .getattr(intern!(py_time.py(), "hour"))?
//         .extract::<i8>()?;
//     let minute_i8 = py_time
//         .getattr(intern!(py_time.py(), "minute"))?
//         .extract::<i8>()?;
//     let second_i8 = py_time
//         .getattr(intern!(py_time.py(), "second"))?
//         .extract::<i8>()?;
//     let microsecond = py_time
//         .getattr(intern!(py_time.py(), "microsecond"))?
//         .extract::<i32>()?;
//     let subsec_nanosecond = microsecond * 1000;
//     Ok(
//         jiff::civil::Time::new(hour_i8, minute_i8, second_i8, subsec_nanosecond)
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?,
//     )
// }

impl<'py> IntoPyObject<'py> for JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pyobject(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTime {
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pyobject(py, &self.0)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        #[cfg(not(Py_LIMITED_API))]
        {
            let time = ob.cast::<PyTime>()?;
            pytime_to_time(&*time).map(Self::from)
        }
        #[cfg(Py_LIMITED_API)]
        {
            pytime_to_time(ob).map(Self::from)
        }
    }
}
