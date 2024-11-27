use pyo3::types::PyTime;
use pyo3::{Bound, PyErr, PyResult, Python};

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
    PyTime::new(py, hour_u8, minute_u8, second_u8, microsecond_u32, None)
}
