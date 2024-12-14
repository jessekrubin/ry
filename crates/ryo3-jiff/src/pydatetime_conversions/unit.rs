use crate::JiffUnit;
use jiff::Unit;
use pyo3::prelude::*;
use pyo3::types::PyString;

impl<'py> IntoPyObject<'py> for JiffUnit {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffUnit {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            Unit::Year => "year",
            Unit::Month => "month",
            Unit::Week => "week",
            Unit::Day => "day",
            Unit::Hour => "hour",
            Unit::Minute => "minute",
            Unit::Second => "second",
            Unit::Millisecond => "millisecond",
            Unit::Microsecond => "microsecond",
            Unit::Nanosecond => "nanosecond",
        };

        s.into_pyobject(py)
    }
}

const JIFF_UNIT_STRINGS: &str =
    "'year', 'month', 'week', 'day', 'hour', 'minute', 'second', 'millisecond', 'microsecond', 'nanosecond'";
const JIFF_UNIT_OPTIONS: &str =  "0='year', 1='month', 2='week', 3='day', 4='hour', 5='minute', 6='second', 7='millisecond', 8='microsecond', 9='nanosecond'";

impl FromPyObject<'_> for JiffUnit {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffUnit> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "year" => Ok(JiffUnit(Unit::Year)),
                "month" => Ok(JiffUnit(Unit::Month)),
                "week" => Ok(JiffUnit(Unit::Week)),
                "day" => Ok(JiffUnit(Unit::Day)),
                "hour" => Ok(JiffUnit(Unit::Hour)),
                "minute" => Ok(JiffUnit(Unit::Minute)),
                "second" => Ok(JiffUnit(Unit::Second)),
                "millisecond" => Ok(JiffUnit(Unit::Millisecond)),
                "microsecond" => Ok(JiffUnit(Unit::Microsecond)),
                "nanosecond" => Ok(JiffUnit(Unit::Nanosecond)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid unit: {s} (options: {JIFF_UNIT_STRINGS})"
                ))),
            }
        } else {
            let i = ob.extract::<i64>()?;
            match i {
                0 => Ok(JiffUnit(Unit::Year)),
                1 => Ok(JiffUnit(Unit::Month)),
                2 => Ok(JiffUnit(Unit::Week)),
                3 => Ok(JiffUnit(Unit::Day)),
                4 => Ok(JiffUnit(Unit::Hour)),
                5 => Ok(JiffUnit(Unit::Minute)),
                6 => Ok(JiffUnit(Unit::Second)),
                7 => Ok(JiffUnit(Unit::Millisecond)),
                8 => Ok(JiffUnit(Unit::Microsecond)),
                9 => Ok(JiffUnit(Unit::Nanosecond)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid unit: {i} (options: {JIFF_UNIT_OPTIONS})"
                ))),
            }
        }
    }
}
//
