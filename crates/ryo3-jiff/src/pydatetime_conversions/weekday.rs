use crate::JiffWeekday;
use pyo3::prelude::*;
use pyo3::types::PyInt;
use pyo3::types::PyString;

impl<'py> IntoPyObject<'py> for JiffWeekday {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffWeekday {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let num: u8 = match self.0 {
            jiff::civil::Weekday::Monday => 1,
            jiff::civil::Weekday::Tuesday => 2,
            jiff::civil::Weekday::Wednesday => 3,
            jiff::civil::Weekday::Thursday => 4,
            jiff::civil::Weekday::Friday => 5,
            jiff::civil::Weekday::Saturday => 6,
            jiff::civil::Weekday::Sunday => 7,
        };
        num.into_pyobject(py).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (weekday={num})"))
        })
    }
}

const JIFF_WEEKDAY_STRING: &str =
    "1='monday', 2='tuesday', 3='wednesday', 4='thursday', 5='friday', 6='saturday', 7='sunday'";

impl FromPyObject<'_> for JiffWeekday {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.downcast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "monday" => Ok(Self(jiff::civil::Weekday::Monday)),
                "tuesday" => Ok(Self(jiff::civil::Weekday::Tuesday)),
                "wednesday" => Ok(Self(jiff::civil::Weekday::Wednesday)),
                "thursday" => Ok(Self(jiff::civil::Weekday::Thursday)),
                "friday" => Ok(Self(jiff::civil::Weekday::Friday)),
                "saturday" => Ok(Self(jiff::civil::Weekday::Saturday)),
                "sunday" => Ok(Self(jiff::civil::Weekday::Sunday)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid weekday: {s} (options: {JIFF_WEEKDAY_STRING})"
                ))),
            }
        } else {
            let i = ob.extract::<u8>()?;
            match i {
                1 => Ok(Self(jiff::civil::Weekday::Monday)),
                2 => Ok(Self(jiff::civil::Weekday::Tuesday)),
                3 => Ok(Self(jiff::civil::Weekday::Wednesday)),
                4 => Ok(Self(jiff::civil::Weekday::Thursday)),
                5 => Ok(Self(jiff::civil::Weekday::Friday)),
                6 => Ok(Self(jiff::civil::Weekday::Saturday)),
                7 => Ok(Self(jiff::civil::Weekday::Sunday)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid weekday: {i} (options: {JIFF_WEEKDAY_STRING})"
                ))),
            }
        }
    }
}
