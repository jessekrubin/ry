use crate::JiffUnit;
use jiff::Unit;
use pyo3::types::PyString;
use pyo3::{intern, prelude::*};

impl<'py> IntoPyObject<'py> for JiffUnit {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffUnit {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            Unit::Year => intern!(py, "year"),
            Unit::Month => intern!(py, "month"),
            Unit::Week => intern!(py, "week"),
            Unit::Day => intern!(py, "day"),
            Unit::Hour => intern!(py, "hour"),
            Unit::Minute => intern!(py, "minute"),
            Unit::Second => intern!(py, "second"),
            Unit::Millisecond => intern!(py, "millisecond"),
            Unit::Microsecond => intern!(py, "microsecond"),
            Unit::Nanosecond => intern!(py, "nanosecond"),
        };
        let b = s.as_borrowed();
        #[cfg(Py_LIMITED_API)]
        {
            Ok(b.into_any())
        }
        #[cfg(not(Py_LIMITED_API))]
        {
            Ok(b)
        }
    }
}

const JIFF_UNIT_STRINGS: &str = "'year', 'month', 'week', 'day', 'hour', 'minute', 'second', 'millisecond', 'microsecond', 'nanosecond'";
const JIFF_UNIT_OPTIONS: &str = "0='year', 1='month', 2='week', 3='day', 4='hour', 5='minute', 6='second', 7='millisecond', 8='microsecond', 9='nanosecond'";

impl FromPyObject<'_> for JiffUnit {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffUnit> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "year" | "y" => Ok(JiffUnit(Unit::Year)),
                "month" | "mo" => Ok(JiffUnit(Unit::Month)),
                "week" | "w" => Ok(JiffUnit(Unit::Week)),
                "day" | "d" => Ok(JiffUnit(Unit::Day)),
                "hour" | "h" => Ok(JiffUnit(Unit::Hour)),
                "minute" | "m" => Ok(JiffUnit(Unit::Minute)),
                "second" | "s" => Ok(JiffUnit(Unit::Second)),
                "millisecond" | "ms" => Ok(JiffUnit(Unit::Millisecond)),
                "microsecond" | "µs" => Ok(JiffUnit(Unit::Microsecond)),
                "nanosecond" | "ns" => Ok(JiffUnit(Unit::Nanosecond)),
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
