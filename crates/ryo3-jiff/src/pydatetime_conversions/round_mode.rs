use crate::JiffRoundMode;
use jiff::RoundMode;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;

impl<'py> IntoPyObject<'py> for JiffRoundMode {
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

impl<'py> IntoPyObject<'py> for &JiffRoundMode {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible; // the conversion error type, has to be convertible to `PyErr`
    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            RoundMode::Ceil => "ceil",
            RoundMode::Floor => "floor",
            RoundMode::Expand => "expand",
            RoundMode::Trunc => "trunc",
            RoundMode::HalfCeil => "half-ceil",
            RoundMode::HalfFloor => "half-floor",
            RoundMode::HalfExpand => "half-expand",
            RoundMode::HalfTrunc => "half-trunc",
            RoundMode::HalfEven => "half-even",
            _ => "unknown",
        };
        Ok(PyString::new(py, s))
    }
}
const JIFF_ROUND_MODE_ERROR: &str = "Invalid round mode, should be `'ceil'`, `'floor'`, `'expand'`, `'trunc'`, `'half_ceil'`, `'half_floor'`, `'half_expand'`, `'half_trunc'` or `'half_even'`";
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
                _ => Err(PyValueError::new_err(JIFF_ROUND_MODE_ERROR)),
            }
        } else {
            Err(PyValueError::new_err(JIFF_ROUND_MODE_ERROR))
        }
    }
}
