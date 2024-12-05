use crate::JiffTime;
use pyo3::prelude::*;
use pyo3::types::{PyTime, PyTimeAccess};

pub fn time_to_pyobject<'py>(
    py: Python<'py>,
    time: &jiff::civil::Time,
) -> PyResult<Bound<'py, PyTime>> {
    let hour_u8 = u8::try_from(time.hour())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("hour: {e}")))?;
    let minute_u8 = u8::try_from(time.minute())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("minute: {e}")))?;
    let second_u8 = u8::try_from(time.second())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("second: {e}")))?;
    let microsecond_u32 = u32::try_from(time.microsecond()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microsecond: {e}"))
    })?;
    #[cfg(not(Py_LIMITED_API))]
    let time = PyTime::new(py, hour_u8, minute_u8, second_u8, microsecond_u32, None)?;
    #[cfg(Py_LIMITED_API)]
    let time = DatetimeTypes::try_get(py).and_then(|dt| {
        dt.time
            .bind(py)
            .call1((hour_u8, minute_u8, second_u8, microsecond_u32))
    })?;
    Ok(time)
}

#[cfg(not(Py_LIMITED_API))]
pub fn py_time_to_jiff_time(py_time: &impl PyTimeAccess) -> PyResult<jiff::civil::Time> {
    let hour = py_time.get_hour();
    let hour_i8 = i8::try_from(hour)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("hour: {e}")))?;

    let minute = py_time.get_minute();

    let minute_i8 = i8::try_from(minute)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("minute: {e}")))?;

    let second = py_time.get_second();
    let second_i8 = i8::try_from(second)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("second: {e}")))?;

    let microsecond = py_time.get_microsecond();
    let subsec_nanosecond = microsecond * 1000;
    let subsec_nanosecond_i32 = i32::try_from(subsec_nanosecond).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("subsec_nanosecond: {e}"))
    })?;
    let t = jiff::civil::Time::new(hour_i8, minute_i8, second_i8, subsec_nanosecond_i32)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    Ok(t)
}

#[cfg(Py_LIMITED_API)]
pub fn py_time_to_jiff_time(py_time: &Bound<'_, PyAny>) -> PyResult<jiff::civil::Time> {
    let hour_i8 = py_time
        .getattr(intern!(py_time.py(), "hour"))?
        .extract::<i8>()?;
    let minute_i8 = py_time
        .getattr(intern!(py_time.py(), "minute"))?
        .extract::<i8>()?;
    let second_i8 = py_time
        .getattr(intern!(py_time.py(), "second"))?
        .extract::<i8>()?;
    let microsecond = py_time
        .getattr(intern!(py_time.py(), "microsecond"))?
        .extract::<i32>()?;
    let subsec_nanosecond = microsecond * 1000;
    Ok(
        jiff::civil::Time::new(hour_i8, minute_i8, second_i8, subsec_nanosecond)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?,
    )
}

pub fn time_from_pyobject(py_date: &impl PyTimeAccess) -> PyResult<jiff::civil::Time> {
    py_time_to_jiff_time(py_date)
}

impl<'py> IntoPyObject<'py> for JiffTime {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pyobject(py, &self.0)
    }
}

impl<'py> IntoPyObject<'py> for &JiffTime {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyTime;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        time_to_pyobject(py, &self.0)
    }
}
impl FromPyObject<'_> for JiffTime {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffTime> {
        #[cfg(not(Py_LIMITED_API))]
        {
            let time = ob.downcast::<PyTime>()?;
            py_time_to_jiff_time(time).map(JiffTime::from)
        }
        #[cfg(Py_LIMITED_API)]
        {
            check_type(ob, &DatetimeTypes::get(ob.py()).time, "PyTime")?;
            py_time_to_jiff_time(ob).map(JiffTime::from)
        }
    }
}
