use crate::JiffRoundMode;
use jiff::RoundMode;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyString;
use pyo3::{intern, prelude::*};

impl<'py> IntoPyObject<'py> for JiffRoundMode {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffRoundMode {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            RoundMode::Ceil => intern!(py, "ceil"),
            RoundMode::Floor => intern!(py, "floor"),
            RoundMode::Expand => intern!(py, "expand"),
            RoundMode::Trunc => intern!(py, "trunc"),
            RoundMode::HalfCeil => intern!(py, "half-ceil"),
            RoundMode::HalfFloor => intern!(py, "half-floor"),
            RoundMode::HalfExpand => intern!(py, "half-expand"),
            RoundMode::HalfTrunc => intern!(py, "half-trunc"),
            RoundMode::HalfEven => intern!(py, "half-even"),
            _ => intern!(py, "unknown"),
        };
        Ok(s.as_borrowed())
    }
}

const JIFF_ROUND_MODE_ACCEPTED: &str = "'ceil', 'floor', 'expand', 'trunc', 'half_ceil', 'half_floor', 'half_expand', 'half_trunc', 'half_even' (case-insensitive; underscores and hyphens are interchangeable)";

impl<'py> FromPyObject<'py> for JiffRoundMode {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(str_mode) = ob.extract::<&str>() {
            match str_mode.to_ascii_lowercase().replace('_', "-").as_str() {
                "ceil" => Ok(Self(RoundMode::Ceil)),
                "floor" => Ok(Self(RoundMode::Floor)),
                "expand" => Ok(Self(RoundMode::Expand)),
                "trunc" => Ok(Self(RoundMode::Trunc)),
                "half-ceil" => Ok(Self(RoundMode::HalfCeil)),
                "half-floor" => Ok(Self(RoundMode::HalfFloor)),
                "half-expand" => Ok(Self(RoundMode::HalfExpand)),
                "half-trunc" => Ok(Self(RoundMode::HalfTrunc)),
                "half-even" => Ok(Self(RoundMode::HalfEven)),
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
