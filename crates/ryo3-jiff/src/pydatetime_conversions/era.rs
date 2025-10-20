use crate::jiff_types::JiffEra;
use jiff::civil::Era;
use pyo3::prelude::*;
use pyo3::types::PyString;

const JIFF_ERA_STRINGS: &str = "'BCE'/'BC', 'CE'/'AD' (case insensitive)";

impl<'py> IntoPyObject<'py> for &JiffEra {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            Era::BCE => crate::interns::bce(py),
            Era::CE => crate::interns::ce(py),
        };
        Ok(s.as_borrowed())
    }
}

impl<'py> IntoPyObject<'py> for JiffEra {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for JiffEra {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = obj.extract::<&str>() {
            match s {
                "bce" | "bc" | "BCE" | "BC" => Ok(Self(Era::BCE)),
                "ce" | "ad" | "CE" | "AD" => Ok(Self(Era::CE)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid era: {s} (options: {JIFF_ERA_STRINGS})"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Expected a string with one of the options: {JIFF_ERA_STRINGS}"
            )))
        }
    }
}
