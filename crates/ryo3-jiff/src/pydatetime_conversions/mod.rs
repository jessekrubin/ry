use crate::ry_date::RyDate;
use pyo3::types::{PyAnyMethods, PyDate, PyDateAccess, PyDateTime, PyDelta, PyDeltaAccess, PyTime};
use pyo3::{Bound, PyErr, PyResult, Python};

mod signed_duration;
pub use signed_duration::{signed_duration_from_pyobject, signed_duration_to_pyobject};

pub fn jiff_time2pytime<'py>(
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

pub fn jiff_datetime2pydatetime<'a>(
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

pub fn jiff_zoned2pydatetime<'a>(
    py: Python<'a>,
    datetime: &jiff::Zoned,
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
    // let tz = datetime.time_zone();
    // let pytzinfo  =  TODO: implement pytzinfo
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

// 24 * 60 * 60

// TODO: THIS IS NOT RIGHT PROLLY
pub fn jiff_span_to_py_time_detla<'py>(
    _py: Python<'py>,
    _span: &jiff::Span,
) -> PyResult<Bound<'py, PyDelta>> {
    //
    // // total number o days
    // let days_f = span
    //     .total(jiff::Unit::Day)
    //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("days: {e}")))?;
    // // total number of seconds
    // let seconds_f = span
    //     .total(jiff::Unit::Second)
    //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("seconds: {e}")))?;
    //
    // // total number of microseconds
    // let micros_f = span.total(jiff::Unit::Microsecond).map_err(|e| {
    //     PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microseconds: {e}"))
    // })?;
    // // sub the days from the total seconds
    // let seconds_i32 = (seconds_f - (days_f * 86400.0)) as i32;
    //
    // // round down to the nearest whole number
    // let days_i32 = days_f as i32;
    //
    // // round down to the nearest whole number
    // let micros_i32 = micros_f as i32;
    //
    // PyDelta::new(
    //     py,
    //     // days
    //     days_i32,
    //     // seconds
    //     seconds_i32,
    //     // microseconds
    //     micros_i32,
    //     false,
    // )

    //not implemented
    Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
        "jiff_span_to_py_time_detla not implemented",
    ))
}
