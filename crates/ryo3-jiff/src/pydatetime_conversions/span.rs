// 24 * 60 * 60

use pyo3::types::PyDelta;
use pyo3::{Bound, PyErr, PyResult, Python};

// TODO: THIS IS NOT RIGHT PROLLY
pub fn span_to_pyobject<'py>(
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
