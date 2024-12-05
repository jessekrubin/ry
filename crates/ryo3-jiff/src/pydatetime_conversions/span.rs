use crate::{JiffSignedDuration, JiffSpan};
use pyo3::types::{PyAnyMethods, PyDelta, PyDeltaAccess};
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};

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

impl<'py> IntoPyObject<'py> for JiffSpan {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, _py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // // Total number of days
        // let days = self.num_days();
        // // Remainder of seconds
        // let secs_dur = self - Duration::days(days);
        // let secs = secs_dur.num_seconds();
        // // Fractional part of the microseconds
        // let micros = (secs_dur - Duration::seconds(secs_dur.num_seconds()))
        //     .num_microseconds()
        //     // This should never panic since we are just getting the fractional
        //     // part of the total microseconds, which should never overflow.
        //     .unwrap();
        //
        // #[cfg(not(Py_LIMITED_API))]
        // {
        //     // We do not need to check the days i64 to i32 cast from rust because
        //     // python will panic with OverflowError.
        //     // We pass true as the `normalize` parameter since we'd need to do several checks here to
        //     // avoid that, and it shouldn't have a big performance impact.
        //     // The seconds and microseconds cast should never overflow since it's at most the number of seconds per day
        //     PyDelta::new(
        //         py,
        //         days.try_into().unwrap_or(i32::MAX),
        //         secs.try_into()?,
        //         micros.try_into()?,
        //         true,
        //     )
        // }
        //
        // #[cfg(Py_LIMITED_API)]
        // {
        //     DatetimeTypes::try_get(py)
        //         .and_then(|dt| dt.timedelta.bind(py).call1((days, secs, micros)))
        // }
        (&self).into_pyobject(_py)
        // Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
        //     "jiff_span_to_py_time_detla not implemented",
        // ))
    }
}

impl<'py> IntoPyObject<'py> for &JiffSpan {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, _py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let signed_duration = jiff::SignedDuration::try_from(self.0)
            .map(JiffSignedDuration::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        signed_duration.into_pyobject(_py)
    }
}

impl FromPyObject<'_> for JiffSpan {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffSpan> {
        #[cfg(not(Py_LIMITED_API))]
        let (days, seconds, microseconds): (i64, i64, i64) = {
            let delta = ob.downcast::<PyDelta>()?;
            (
                delta.get_days().into(),
                delta.get_seconds().into(),
                delta.get_microseconds().into(),
            )
        };
        #[cfg(Py_LIMITED_API)]
        let (days, seconds, microseconds) = {
            check_type(ob, &DatetimeTypes::get(ob.py()).timedelta, "PyDelta")?;
            (
                ob.getattr(intern!(ob.py(), "days"))?.extract()?,
                ob.getattr(intern!(ob.py(), "seconds"))?.extract()?,
                ob.getattr(intern!(ob.py(), "microseconds"))?.extract()?,
            )
        };

        // i128 to not overflow
        const MICROS_PER_SECOND: i128 = 1_000_000;
        const SECONDS_PER_DAY: i128 = 86_400;
        const MICROS_PER_DAY: i128 = MICROS_PER_SECOND * SECONDS_PER_DAY; // 86_400 * 1_000_000

        let days_i128 = days as i128;
        let seconds_i128 = seconds as i128;
        let microseconds_i128 = microseconds as i128;

        let total_us: i128 = days_i128
            .checked_mul(MICROS_PER_DAY) // boom
            .and_then(|v| v.checked_add(seconds_i128.checked_mul(MICROS_PER_SECOND)?)) // even more boom
            .and_then(|v| v.checked_add(microseconds_i128)) // further boom
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyOverflowError, _>(
                    "Overflow in total_us calculation",
                )
            })?; // final boom

        let is_negative = total_us < 0;
        let total_us_abs = total_us.abs();
        let abs_days = total_us_abs / MICROS_PER_DAY;
        let remainder = total_us_abs % MICROS_PER_DAY;
        let abs_seconds = remainder / MICROS_PER_SECOND;
        let abs_microseconds = remainder % MICROS_PER_SECOND;

        // Convert back to i64 safely
        let abs_days_i64: i64 = abs_days
            .try_into()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyOverflowError, _>("days overflow"))?;
        let abs_seconds_i64: i64 = abs_seconds
            .try_into()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyOverflowError, _>("seconds overflow"))?;
        let abs_microseconds_i64: i64 = abs_microseconds.try_into().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyOverflowError, _>("microseconds overflow")
        })?;

        let mut span = jiff::Span::new();

        if abs_days_i64 != 0 {
            span = span.try_days(abs_days_i64).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("days: {e}"))
            })?;
        }
        if abs_seconds_i64 != 0 {
            span = span.try_seconds(abs_seconds_i64).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("seconds: {e}"))
            })?;
        }
        if abs_microseconds_i64 != 0 {
            span = span.try_microseconds(abs_microseconds_i64).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microseconds: {e}"))
            })?;
        }

        if is_negative {
            span = -span;
        }

        Ok(JiffSpan::from(span))
    }
    // fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffSpan> {
    //     // what is the max value of i64?
    //
    //     #[cfg(not(Py_LIMITED_API))]
    //     let (days, seconds, microseconds): (i64, i64, i64) = {
    //         let delta = ob.downcast::<PyDelta>()?;
    //         (
    //             delta.get_days().into(),
    //             delta.get_seconds().into(),
    //             delta.get_microseconds().into(),
    //         )
    //     };
    //     #[cfg(Py_LIMITED_API)]
    //     let (days, seconds, microseconds) = {
    //         check_type(ob, &DatetimeTypes::get(ob.py()).timedelta, "PyDelta")?;
    //         (
    //             ob.getattr(intern!(ob.py(), "days"))?.extract()?,
    //             ob.getattr(intern!(ob.py(), "seconds"))?.extract()?,
    //             ob.getattr(intern!(ob.py(), "microseconds"))?.extract()?,
    //         )
    //     };
    //     let mut span = jiff::Span::new();
    //     let tdelta_is_negative = days <0;
    //
    //
    //
    //     if days != 0 {
    //         span = span.try_days(days.abs()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("days: {e}")))?;
    //     }
    //     if seconds != 0 {
    //         span = span.try_seconds(seconds).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("seconds: {e}")))?;
    //     }
    //     if microseconds != 0 {
    //         span = span.try_microseconds(microseconds).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microseconds: {e}")))?;
    //     }
    //
    //     if  tdelta_is_negative {
    //         span = -span;
    //     }
    //     Ok(JiffSpan::from(span))
    //         // .try_days(days)
    //         // .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("days: {e}")))?
    //         // .try_seconds(seconds)
    //         // .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("seconds: {e}")))?
    //         // .try_microseconds(microseconds)
    //         // .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("microseconds: {e}")))?;
    //     // Ok(JiffSpan::from(span))
    //
    //     // Ok(
    //     //     Duration::days(days)
    //     //         + Duration::seconds(seconds)
    //     //         + Duration::microseconds(microseconds),
    //     // )
    //     // Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //     //     "jiff_span_to_py_time_detla not implemented",
    //     // ))
    // }
}
