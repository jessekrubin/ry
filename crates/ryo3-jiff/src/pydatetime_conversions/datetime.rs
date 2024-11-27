use pyo3::types::PyDateTime;
use pyo3::{Bound, PyErr, PyResult, Python};

pub fn datetime_to_pyobject<'a>(
    py: Python<'a>,
    datetime: &jiff::civil::DateTime,
) -> PyResult<Bound<'a, PyDateTime>> {
    let year = i32::from(datetime.year());
    let m_u8 = u8::try_from(datetime.month())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let d_u8 = u8::try_from(datetime.day())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let hour_u8 = u8::try_from(datetime.hour())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("hour: {e}")))?;
    let minute_u8 = u8::try_from(datetime.minute())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("minute: {e}")))?;
    let second_u8 = u8::try_from(datetime.second())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("second: {e}")))?;
    let microsecond_u32 = u32::try_from(datetime.microsecond()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microsecond: {e}"))
    })?;
    PyDateTime::new(
        py,
        year,
        m_u8,
        d_u8,
        hour_u8,
        minute_u8,
        second_u8,
        microsecond_u32,
        None,
    )
}
