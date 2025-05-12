use crate::{JiffSignedDuration, JiffSpan};
use jiff::SpanRelativeTo;
use pyo3::types::{PyAnyMethods, PyDelta, PyDeltaAccess};
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::ops::Neg;

impl<'py> IntoPyObject<'py> for JiffSpan {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyDelta;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
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
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let signed_duration = self
            .0
            .to_duration(SpanRelativeTo::days_are_24_hours())
            .map(JiffSignedDuration::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        signed_duration.into_pyobject(py)
    }
}

impl FromPyObject<'_> for JiffSpan {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffSpan> {
        // i128 to not overflow
        const MICROS_PER_SECOND: i128 = 1_000_000;
        const SECONDS_PER_DAY: i128 = 86_400;
        const MICROS_PER_DAY: i128 = MICROS_PER_SECOND * SECONDS_PER_DAY; // 86_400 * 1_000_000
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

        let days_i128 = i128::from(days);
        let seconds_i128 = i128::from(seconds);
        let microseconds_i128 = i128::from(microseconds);

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
            span = span.neg();
        }

        Ok(JiffSpan::from(span))
    }
}
