use crate::JiffUnit;
use jiff::Unit;
use pyo3::prelude::*;
use pyo3::types::PyString;

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
        let s = match self.0 {
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

impl FromPyObject<'_> for JiffUnit {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "year" => Ok(Self(Unit::Year)),
                "month" => Ok(Self(Unit::Month)),
                "week" => Ok(Self(Unit::Week)),
                "day" => Ok(Self(Unit::Day)),
                "hour" => Ok(Self(Unit::Hour)),
                "minute" => Ok(Self(Unit::Minute)),
                "second" => Ok(Self(Unit::Second)),
                "millisecond" => Ok(Self(Unit::Millisecond)),
                "microsecond" => Ok(Self(Unit::Microsecond)),
                "nanosecond" => Ok(Self(Unit::Nanosecond)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid unit: {s} (options: {JIFF_UNIT_STRINGS})"
                ))),
            }
        } else {
            let i = ob.extract::<i64>()?;
            match i {
                0 => Ok(Self(Unit::Year)),
                1 => Ok(Self(Unit::Month)),
                2 => Ok(Self(Unit::Week)),
                3 => Ok(Self(Unit::Day)),
                4 => Ok(Self(Unit::Hour)),
                5 => Ok(Self(Unit::Minute)),
                6 => Ok(Self(Unit::Second)),
                7 => Ok(Self(Unit::Millisecond)),
                8 => Ok(Self(Unit::Microsecond)),
                9 => Ok(Self(Unit::Nanosecond)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid unit: {i} (options: {JIFF_UNIT_OPTIONS})"
                ))),
            }
        }
    }
}
//
