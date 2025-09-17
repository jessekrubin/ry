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

impl FromPyObject<'_> for JiffEra {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // downcast to string...
        if let Ok(s) = ob.cast::<PyString>() {
            let s = s.to_string().to_ascii_lowercase();
            match s.as_str() {
                "bce" | "bc" => Ok(Self(Era::BCE)),
                "ce" | "ad" => Ok(Self(Era::CE)),
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
