use std::fmt::Display;

use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};
use ryo3_core::{py_type_err, py_value_err};
#[derive(Clone, Copy)]
pub struct PyBase(pub size::fmt::Base);

impl PartialEq for PyBase {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self.0, other.0),
            (size::fmt::Base::Base2, size::fmt::Base::Base2)
                | (size::fmt::Base::Base10, size::fmt::Base::Base10)
        )
    }
}

impl Default for PyBase {
    fn default() -> Self {
        Self(size::fmt::Base::Base10)
    }
}

impl Display for PyBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            size::fmt::Base::Base2 => write!(f, "2"),
            size::fmt::Base::Base10 => write!(f, "10"),
            _ => unreachable!(),
        }
    }
}

const BASE_ERR_MSG: &str = "base must be be int(2)/int(10)";

impl<'py> FromPyObject<'_, 'py> for PyBase {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if obj.is_none() {
            Ok(Self::default())
        } else if let Ok(i) = obj.cast::<PyInt>() {
            let base = i.extract::<u8>()?;
            match base {
                2 => Ok(Self(size::fmt::Base::Base2)),
                10 => Ok(Self(size::fmt::Base::Base10)),
                _ => py_value_err!("{BASE_ERR_MSG} ~ given: {base}"),
            }
        } else {
            py_type_err!("{BASE_ERR_MSG}")
        }
    }
}

impl<'py> IntoPyObject<'py> for PyBase {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let i: u8 = match self.0 {
            size::fmt::Base::Base2 => 2,
            size::fmt::Base::Base10 => 10,
            _ => unreachable!(),
        };
        Ok(PyInt::new(py, i))
    }
}

impl<'py> IntoPyObject<'py> for &PyBase {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py)
    }
}

#[derive(Clone, Copy)]
pub struct PyStyle(pub size::fmt::Style);

impl PartialEq for PyStyle {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self.0, other.0),
            (size::fmt::Style::Default, size::fmt::Style::Default)
                | (size::fmt::Style::Abbreviated, size::fmt::Style::Abbreviated)
                | (
                    size::fmt::Style::AbbreviatedLowercase,
                    size::fmt::Style::AbbreviatedLowercase
                )
                | (size::fmt::Style::Full, size::fmt::Style::Full)
                | (
                    size::fmt::Style::FullLowercase,
                    size::fmt::Style::FullLowercase
                )
        )
    }
}

impl Default for PyStyle {
    fn default() -> Self {
        Self(size::fmt::Style::Default)
    }
}

const STYLE_ERR_MSG: &str =
    "style must be None/'default'/'abbreviated'/'abbreviated-lowercase'/'full'/'full-lowercase'";

impl<'py> FromPyObject<'_, 'py> for PyStyle {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'_, 'py, pyo3::PyAny>) -> PyResult<Self> {
        if ob.is_none() {
            Ok(Self::default())
        } else if let Ok(s) = ob.cast::<PyString>() {
            let s_ref = s.to_str()?;
            match s_ref.to_ascii_lowercase().as_str() {
                "default" => Ok(Self(size::fmt::Style::Default)),
                "abbreviated" => Ok(Self(size::fmt::Style::Abbreviated)),
                "abbreviated-lowercase" | "abbreviated_lowercase" => {
                    Ok(Self(size::fmt::Style::AbbreviatedLowercase))
                }
                "full" => Ok(Self(size::fmt::Style::Full)),
                "full-lowercase" | "full_lowercase" => Ok(Self(size::fmt::Style::FullLowercase)),
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "{STYLE_ERR_MSG} ~ given: {s_ref}"
                ))),
            }
        } else {
            py_type_err!("{STYLE_ERR_MSG}")
        }
    }
}

impl Display for PyStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            size::fmt::Style::Default => write!(f, "default"),
            size::fmt::Style::Abbreviated => write!(f, "abbreviated"),
            size::fmt::Style::AbbreviatedLowercase => write!(f, "abbreviated-lowercase"),
            size::fmt::Style::Full => write!(f, "full"),
            size::fmt::Style::FullLowercase => write!(f, "full-lowercase"),
            _ => unreachable!(),
        }
    }
}

impl<'py> IntoPyObject<'py> for PyStyle {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            size::fmt::Style::Default => pyo3::intern!(py, "default"),
            size::fmt::Style::Abbreviated => pyo3::intern!(py, "abbreviated"),
            size::fmt::Style::AbbreviatedLowercase => pyo3::intern!(py, "abbreviated-lowercase"),
            size::fmt::Style::Full => pyo3::intern!(py, "full"),
            size::fmt::Style::FullLowercase => pyo3::intern!(py, "full-lowercase"),
            _ => unreachable!(),
        };
        Ok(s.as_borrowed())
    }
}

impl<'py> IntoPyObject<'py> for &PyStyle {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py)
    }
}
