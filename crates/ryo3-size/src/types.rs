use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Base(pub size::fmt::Base);

impl PartialEq for Base {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self.0, other.0),
            (size::fmt::Base::Base2, size::fmt::Base::Base2)
                | (size::fmt::Base::Base10, size::fmt::Base::Base10)
        )
    }
}

impl Default for Base {
    fn default() -> Self {
        Self(size::fmt::Base::Base10)
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            size::fmt::Base::Base2 => write!(f, "2"),
            size::fmt::Base::Base10 => write!(f, "10"),
            _ => write!(f, "unknown"),
        }
    }
}

const BASE_ERR_MSG: &str = "base must be be int(2)/int(10)";
impl FromPyObject<'_> for Base {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // if is int...
        if ob.is_none() {
            Ok(Self::default())
        } else if let Ok(i) = ob.cast::<PyInt>() {
            let base = i.extract::<u8>()?;
            match base {
                2 => Ok(Self(size::fmt::Base::Base2)),
                10 => Ok(Self(size::fmt::Base::Base10)),
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "{BASE_ERR_MSG} ~ given: {base}"
                ))),
            }
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(BASE_ERR_MSG))
        }
    }
}

#[derive(Clone, Copy)]
pub struct Style(pub size::fmt::Style);

impl PartialEq for Style {
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

impl Default for Style {
    fn default() -> Self {
        Self(size::fmt::Style::Default)
    }
}

const STYLE_ERR_MSG: &str =
    "style must be None/'default'/'abbreviated'/'abbreviated_lowercase'/'full'/'full_lowercase'";

impl FromPyObject<'_> for Style {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if ob.is_none() {
            Ok(Self::default())
        } else if let Ok(s) = ob.cast::<PyString>() {
            let s_ref = s.to_str()?;
            match s_ref.to_ascii_lowercase().as_str() {
                "default" => Ok(Self(size::fmt::Style::Default)),
                "abbreviated" => Ok(Self(size::fmt::Style::Abbreviated)),
                "abbreviated_lowercase" | "abbreviated-lowercase" => {
                    Ok(Self(size::fmt::Style::AbbreviatedLowercase))
                }
                "full" => Ok(Self(size::fmt::Style::Full)),
                "full_lowercase" | "full-lowercase" => Ok(Self(size::fmt::Style::FullLowercase)),
                _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "{STYLE_ERR_MSG} ~ given: {s_ref}"
                ))),
            }
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(STYLE_ERR_MSG))
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            size::fmt::Style::Default => write!(f, "default"),
            size::fmt::Style::Abbreviated => write!(f, "abbreviated"),
            size::fmt::Style::AbbreviatedLowercase => write!(f, "abbreviated_lowercase"),
            size::fmt::Style::Full => write!(f, "full"),
            size::fmt::Style::FullLowercase => write!(f, "full_lowercase"),
            _ => write!(f, "unknown"),
        }
    }
}
