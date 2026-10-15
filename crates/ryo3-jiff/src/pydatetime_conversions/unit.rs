use jiff::Unit;
use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_macro_rules::py_value_err;

use crate::JiffUnit;

impl<'py> IntoPyObject<'py> for &JiffUnit {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for JiffUnit {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.into_inner() {
            Unit::Year => crate::interns::year(py),
            Unit::Month => crate::interns::month(py),
            Unit::Week => crate::interns::week(py),
            Unit::Day => crate::interns::day(py),
            Unit::Hour => crate::interns::hour(py),
            Unit::Minute => crate::interns::minute(py),
            Unit::Second => crate::interns::second(py),
            Unit::Millisecond => crate::interns::millisecond(py),
            Unit::Microsecond => crate::interns::microsecond(py),
            Unit::Nanosecond => crate::interns::nanosecond(py),
        };
        Ok(s.as_borrowed())
    }
}

const JIFF_UNIT_STRINGS: &str = "'year', 'month', 'week', 'day', 'hour', 'minute', 'second', 'millisecond', 'microsecond', 'nanosecond'";
const JIFF_UNIT_OPTIONS: &str = "0='year', 1='month', 2='week', 3='day', 4='hour', 5='minute', 6='second', 7='millisecond', 8='microsecond', 9='nanosecond'";

impl<'py> FromPyObject<'_, 'py> for JiffUnit {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "year" => Ok(Self::YEAR),
                "month" => Ok(Self::MONTH),
                "week" => Ok(Self::WEEK),
                "day" => Ok(Self::DAY),
                "hour" => Ok(Self::HOUR),
                "minute" => Ok(Self::MINUTE),
                "second" => Ok(Self::SECOND),
                "millisecond" => Ok(Self::MILLISECOND),
                "microsecond" => Ok(Self::MICROSECOND),
                "nanosecond" => Ok(Self::NANOSECOND),
                _ => py_value_err!("Invalid unit: {s} (options: {JIFF_UNIT_STRINGS})"),
            }
        } else if let Ok(i) = ob.extract::<i64>() {
            match i {
                0 => Ok(Self::YEAR),
                1 => Ok(Self::MONTH),
                2 => Ok(Self::WEEK),
                3 => Ok(Self::DAY),
                4 => Ok(Self::HOUR),
                5 => Ok(Self::MINUTE),
                6 => Ok(Self::SECOND),
                7 => Ok(Self::MILLISECOND),
                8 => Ok(Self::MICROSECOND),
                9 => Ok(Self::NANOSECOND),
                _ => py_value_err!("Invalid unit: {i} (options: {JIFF_UNIT_OPTIONS})"),
            }
        } else {
            py_value_err!(
                "Invalid type for unit, expected a string or integer (options: {JIFF_UNIT_STRINGS})"
            )
        }
    }
}
//
