use pyo3::prelude::*;
use pyo3::types::PyInt;
use ryo3_macro_rules::{py_type_err, py_value_err};

use crate::JiffWeekday;

impl<'py> IntoPyObject<'py> for JiffWeekday {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffWeekday {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let num: u8 = match self.into_inner() {
            jiff::civil::Weekday::Monday => 1,
            jiff::civil::Weekday::Tuesday => 2,
            jiff::civil::Weekday::Wednesday => 3,
            jiff::civil::Weekday::Thursday => 4,
            jiff::civil::Weekday::Friday => 5,
            jiff::civil::Weekday::Saturday => 6,
            jiff::civil::Weekday::Sunday => 7,
        };
        num.into_pyobject(py)
    }
}

const JIFF_WEEKDAY_STRING: &str =
    "1='monday', 2='tuesday', 3='wednesday', 4='thursday', 5='friday', 6='saturday', 7='sunday'";

impl<'py> FromPyObject<'_, 'py> for JiffWeekday {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "monday" | "MONDAY" => Ok(Self::MONDAY),
                "tuesday" | "TUESDAY" => Ok(Self::TUESDAY),
                "wednesday" | "WEDNESDAY" => Ok(Self::WEDNESDAY),
                "thursday" | "THURSDAY" => Ok(Self::THURSDAY),
                "friday" | "FRIDAY" => Ok(Self::FRIDAY),
                "saturday" | "SATURDAY" => Ok(Self::SATURDAY),
                "sunday" | "SUNDAY" => Ok(Self::SUNDAY),
                _ => py_value_err!("Invalid weekday: {s} (options: {JIFF_WEEKDAY_STRING})"),
            }
        } else if let Ok(i) = ob.extract::<u8>() {
            match i {
                1 => Ok(Self::MONDAY),
                2 => Ok(Self::TUESDAY),
                3 => Ok(Self::WEDNESDAY),
                4 => Ok(Self::THURSDAY),
                5 => Ok(Self::FRIDAY),
                6 => Ok(Self::SATURDAY),
                7 => Ok(Self::SUNDAY),
                _ => py_value_err!("Invalid weekday: {i} (options: {JIFF_WEEKDAY_STRING})"),
            }
        } else {
            py_type_err!(
                "Invalid type for weekday, expected a string or integer (options: {JIFF_WEEKDAY_STRING})"
            )
        }
    }
}
