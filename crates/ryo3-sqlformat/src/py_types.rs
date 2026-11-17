use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};

use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};

use crate::RySqlfmtQueryParams;

#[derive(Clone, Copy)]
pub struct PyIndent(sqlformat::Indent);

impl Default for PyIndent {
    fn default() -> Self {
        Self(sqlformat::Indent::Spaces(2))
    }
}

impl PartialEq for PyIndent {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (sqlformat::Indent::Tabs, sqlformat::Indent::Tabs) => true,
            (sqlformat::Indent::Spaces(a), sqlformat::Indent::Spaces(b)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Debug for PyIndent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
            sqlformat::Indent::Tabs => f.write_str("-1"),
            sqlformat::Indent::Spaces(n) => write!(f, "{n}"),
        }
    }
}

impl<'py> IntoPyObject<'py> for &PyIndent {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            sqlformat::Indent::Tabs => (-1i8).into_pyobject(py),
            sqlformat::Indent::Spaces(n) => n.into_pyobject(py),
        }
    }
}

impl<'py> IntoPyObject<'py> for PyIndent {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const PY_INDENT_ERR_MSG: &str = "Indent must be an integer (0 <= indent < 256 | -1 for tabs), 'tabs'/'\\t', or 'spaces' (default 2 spaces)";
impl<'py> FromPyObject<'_, 'py> for PyIndent {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        // none go to default (2 spaces)
        if ob.is_none() {
            Ok(Self(sqlformat::Indent::Spaces(2)))
        } else if let Ok(i) = ob.extract::<u8>() {
            Ok(Self(sqlformat::Indent::Spaces(i)))
        } else if let Ok(i) = ob.extract::<i8>() {
            if i == -1 {
                Ok(Self(sqlformat::Indent::Tabs))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    PY_INDENT_ERR_MSG,
                ))
            }
        } else if let Ok(s) = ob.extract::<&str>() {
            match s {
                "tabs" | "\t" => Ok(Self(sqlformat::Indent::Tabs)),
                "spaces" => Ok(Self(sqlformat::Indent::Spaces(2))),
                _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    PY_INDENT_ERR_MSG,
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                PY_INDENT_ERR_MSG,
            ))
        }
    }
}

impl From<PyIndent> for sqlformat::Indent {
    fn from(i: PyIndent) -> Self {
        i.0
    }
}

#[derive(FromPyObject)]
pub enum PyQueryParamValue {
    PyString(String),
    PyInt(i64),
    PyFloat(f64),
}

#[derive(FromPyObject)]
pub enum PyQueryParamsLike {
    PyQueryParams(RySqlfmtQueryParams),
    NamedMap(HashMap<String, PyQueryParamValue>),
    NamedVec(Vec<(String, PyQueryParamValue)>),
    Indexed(Vec<PyQueryParamValue>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyDialect(pub(crate) sqlformat::Dialect);

impl Default for PyDialect {
    fn default() -> Self {
        Self(sqlformat::Dialect::Generic)
    }
}

impl From<PyDialect> for sqlformat::Dialect {
    fn from(d: PyDialect) -> Self {
        d.0
    }
}

impl Display for PyDialect {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
            sqlformat::Dialect::Generic => f.write_str("generic"),
            sqlformat::Dialect::PostgreSql => f.write_str("postgresql"),
            sqlformat::Dialect::SQLServer => f.write_str("sqlserver"),
        }
    }
}

impl<'py> IntoPyObject<'py> for &PyDialect {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s = match self.0 {
            sqlformat::Dialect::Generic => pyo3::intern!(py, "generic"),
            sqlformat::Dialect::PostgreSql => pyo3::intern!(py, "postgresql"),
            sqlformat::Dialect::SQLServer => pyo3::intern!(py, "sqlserver"),
        };
        Ok(s.as_borrowed())
    }
}

impl<'py> IntoPyObject<'py> for PyDialect {
    type Target = PyString;
    type Output = Borrowed<'py, 'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const SQLFORMAT_DIALECT_STRINGS: &str = "'generic', 'postgresql', 'sqlserver'";
impl<'py> FromPyObject<'_, 'py> for PyDialect {
    type Error = pyo3::PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "generic" => Ok(Self(sqlformat::Dialect::Generic)),
                "postgresql" => Ok(Self(sqlformat::Dialect::PostgreSql)),
                "sqlserver" => Ok(Self(sqlformat::Dialect::SQLServer)),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid dialect; valid options: {SQLFORMAT_DIALECT_STRINGS}"
                ))),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Invalid type for dialect expected string (options: {SQLFORMAT_DIALECT_STRINGS})"
            )))
        }
    }
}
