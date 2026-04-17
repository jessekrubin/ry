use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};
use ryo3_core::{py_type_err, py_value_err};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PyBase {
    #[default]
    Base2,
    Base10,
}

impl std::fmt::Display for PyBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base2 => write!(f, "2"),
            Self::Base10 => write!(f, "10"),
        }
    }
}

impl From<PyBase> for u8 {
    fn from(base: PyBase) -> Self {
        match base {
            PyBase::Base2 => 2,
            PyBase::Base10 => 10,
        }
    }
}

impl From<PyBase> for size::fmt::Base {
    fn from(base: PyBase) -> Self {
        match base {
            PyBase::Base2 => Self::Base2,
            PyBase::Base10 => Self::Base10,
        }
    }
}

const BASE_ERR_MSG: &str = "size-fmt-base must be 2 or 10 (default: 2)";

impl<'py> FromPyObject<'_, 'py> for PyBase {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(base) = obj.extract::<u8>() {
            match base {
                2 => Ok(Self::Base2),
                10 => Ok(Self::Base10),
                _ => py_value_err!("{BASE_ERR_MSG}"),
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
        Ok(PyInt::new(py, u8::from(self)))
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

impl PyStyle {
    const DEFAULT: Self = Self(size::fmt::Style::Default);
    const ABBREVIATED: Self = Self(size::fmt::Style::Abbreviated);
    const ABBREVIATED_LOWERCASE: Self = Self(size::fmt::Style::AbbreviatedLowercase);
    const FULL: Self = Self(size::fmt::Style::Full);
    const FULL_LOWERCASE: Self = Self(size::fmt::Style::FullLowercase);
}

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
        Self::DEFAULT
    }
}

const STYLE_ERR_MSG: &str =
    "style must be None/'default'/'abbreviated'/'abbreviated-lowercase'/'full'/'full-lowercase'";

impl<'py> FromPyObject<'_, 'py> for PyStyle {
    type Error = pyo3::PyErr;
    fn extract(ob: pyo3::Borrowed<'_, 'py, pyo3::PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s.to_ascii_lowercase().as_str() {
                "default" => Ok(Self::DEFAULT),
                "abbreviated" => Ok(Self::ABBREVIATED),
                "abbreviated-lowercase" | "abbreviated_lowercase" => {
                    Ok(Self::ABBREVIATED_LOWERCASE)
                }
                "full" => Ok(Self::FULL),
                "full-lowercase" | "full_lowercase" => Ok(Self::FULL_LOWERCASE),
                _ => py_value_err!("{STYLE_ERR_MSG} ~ given: {s}"),
            }
        } else {
            py_type_err!("{STYLE_ERR_MSG}")
        }
    }
}

impl std::fmt::Display for PyStyle {
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
