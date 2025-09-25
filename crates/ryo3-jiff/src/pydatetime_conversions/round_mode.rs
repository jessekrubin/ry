use crate::JiffRoundMode;
use jiff::RoundMode;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

impl<'py> IntoPyObject<'py> for JiffRoundMode {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffRoundMode {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            RoundMode::Ceil => crate::interns::ceil(py),
            RoundMode::Floor => crate::interns::floor(py),
            RoundMode::Expand => crate::interns::expand(py),
            RoundMode::Trunc => crate::interns::trunc(py),
            RoundMode::HalfCeil => crate::interns::half_ceil(py),
            RoundMode::HalfFloor => crate::interns::half_floor(py),
            RoundMode::HalfExpand => crate::interns::half_expand(py),
            RoundMode::HalfTrunc => crate::interns::half_trunc(py),
            RoundMode::HalfEven => crate::interns::half_even(py),
            _ => crate::interns::unknown(py),
        };
        Ok(s.as_borrowed())
    }
}

const JIFF_ROUND_MODE_ACCEPTED: &str = "'ceil', 'floor', 'expand', 'trunc', 'half-ceil', 'half-floor', 'half-expand', 'half-trunc', 'half-even' (underscores and hyphens are interchangeable)";

impl<'py> FromPyObject<'py> for JiffRoundMode {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode.to_ascii_lowercase().as_str() {
                "ceil" => Ok(Self(RoundMode::Ceil)),
                "floor" => Ok(Self(RoundMode::Floor)),
                "expand" => Ok(Self(RoundMode::Expand)),
                "trunc" => Ok(Self(RoundMode::Trunc)),
                "half-ceil" | "half_ceil" => Ok(Self(RoundMode::HalfCeil)),
                "half-floor" | "half_floor" => Ok(Self(RoundMode::HalfFloor)),
                "half-expand" | "half_expand" => Ok(Self(RoundMode::HalfExpand)),
                "half-trunc" | "half_trunc" => Ok(Self(RoundMode::HalfTrunc)),
                "half-even" | "half_even" => Ok(Self(RoundMode::HalfEven)),
                _ => Err(PyValueError::new_err(format!(
                    "Invalid round mode: {str_mode} (options: {JIFF_ROUND_MODE_ACCEPTED})"
                ))),
            }
        } else {
            Err(PyValueError::new_err(format!(
                "Invalid type for round mode, expected a string (options: {JIFF_ROUND_MODE_ACCEPTED})"
            )))
        }
    }
}
