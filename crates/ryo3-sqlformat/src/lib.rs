#![doc = include_str!("../README.md")]
use pyo3::types::{PyDict, PyInt, PyString, PyTuple};
use pyo3::{IntoPyObjectExt, intern, prelude::*};
use sqlformat::{self, QueryParams};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

#[pyclass(name = "SqlfmtQueryParams", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct PySqlfmtQueryParams(QueryParams);

#[pymethods]
impl PySqlfmtQueryParams {
    #[new]
    #[pyo3(signature = (params=None))]
    fn py_new(params: Option<PyQueryParamsLike>) -> PyResult<Self> {
        sqlfmt_params(params)
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __len__(&self) -> usize {
        match &self.0 {
            QueryParams::Named(p) => p.len(),
            QueryParams::Indexed(p) => p.len(),
            QueryParams::None => 0,
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        let params: Bound<'py, PyAny> = match &self.0 {
            QueryParams::Named(p) => {
                let dict = pyo3::types::PyDict::new(py);
                for (k, v) in p {
                    dict.set_item(k, v)?;
                }
                dict.into_bound_py_any(py)?
            }
            QueryParams::Indexed(p) => {
                let mut arr = Vec::with_capacity(p.len());
                for v in p {
                    arr.push(v.clone());
                }
                arr.into_bound_py_any(py)?
            }
            QueryParams::None => py.None().into_bound_py_any(py)?,
        };
        pyo3::types::PyTuple::new(py, &[params])
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for PySqlfmtQueryParams {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (QueryParams::Named(p1), QueryParams::Named(p2)) => {
                // make 2 vecccccs o refs...
                let mut p1: Vec<(&str, &str)> =
                    p1.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                p1.sort_by(|a, b| a.0.cmp(b.0));

                let mut p2: Vec<(&str, &str)> =
                    p2.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                p2.sort_by(|a, b| a.0.cmp(b.0));
                p1 == p2
            }
            (QueryParams::Indexed(p1), QueryParams::Indexed(p2)) => p1 == p2,
            (QueryParams::None, QueryParams::None) => true,
            _ => false,
        }
    }
}

impl Hash for PySqlfmtQueryParams {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            QueryParams::Named(p) => {
                let mut p: Vec<(&str, &str)> =
                    p.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                p.sort_by(|a, b| a.0.cmp(b.0));
                p.hash(state);
            }
            QueryParams::Indexed(p) => {
                p.hash(state);
            }
            QueryParams::None => {}
        }
    }
}

impl Display for PySqlfmtQueryParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("SqlfmtQueryParams(")?;
        QueryParamsFormatter(&self.0).fmt(f)?;
        f.write_str(")")
    }
}

struct QueryParamsFormatter<'p>(pub &'p QueryParams);

impl QueryParamsFormatter<'_> {
    fn is_empty(&self) -> bool {
        match &self.0 {
            QueryParams::Named(p) => p.is_empty(),
            QueryParams::Indexed(p) => p.is_empty(),
            QueryParams::None => true,
        }
    }
}

impl Display for QueryParamsFormatter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_empty() {
            return f.write_str("None");
        }
        match &self.0 {
            // ================================================================
            // { "key": "value", ... }
            // ================================================================
            QueryParams::Named(map) => {
                f.write_str("{")?;

                let mut iter = map.iter();
                if let Some((k, v)) = iter.next() {
                    write!(f, r#""{k}": "{v}""#)?;
                    for (k, v) in iter {
                        f.write_str(", ")?; // two bytes only
                        write!(f, r#""{k}": "{v}""#)?;
                    }
                }

                f.write_str("}")
            }

            // ================================================================
            // [ "value", ... ]
            // ================================================================
            QueryParams::Indexed(list) => {
                f.write_str("[")?;

                let mut iter = list.iter();
                if let Some(v) = iter.next() {
                    write!(f, r#""{v}""#)?;
                    for v in iter {
                        f.write_str(", ")?;
                        write!(f, r#""{v}""#)?;
                    }
                }

                f.write_str("]")
            }

            // ─────────────────────────────────────────────────────────────
            QueryParams::None => f.write_str("None"),
        }
    }
}

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
    type Error = pyo3::PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            sqlformat::Indent::Tabs => Ok((-1i8).into_pyobject(py)?),
            sqlformat::Indent::Spaces(n) => Ok(n.into_pyobject(py)?),
        }
    }
}

impl<'py> IntoPyObject<'py> for PyIndent {
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

const PY_INDENT_ERR_MSG: &str = "Indent must be an integer (0 <= indent < 256 | -1 for tabs), 'tabs'/'\\t', or 'spaces' (default 2 spaces)";
impl FromPyObject<'_> for PyIndent {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
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
    PyQueryParams(PySqlfmtQueryParams),
    NamedMap(HashMap<String, PyQueryParamValue>),
    NamedVec(Vec<(String, PyQueryParamValue)>),
    Indexed(Vec<PyQueryParamValue>),
}

#[pyfunction]
#[pyo3(signature = (params=None))]
pub fn sqlfmt_params(params: Option<PyQueryParamsLike>) -> PyResult<PySqlfmtQueryParams> {
    match params {
        Some(params) => match params {
            PyQueryParamsLike::NamedMap(p) => {
                if p.is_empty() {
                    return Ok(PySqlfmtQueryParams(QueryParams::None));
                }
                let named_params = p
                    .into_iter()
                    .map(|(k, v)| match v {
                        PyQueryParamValue::PyString(s) => (k, s),
                        PyQueryParamValue::PyInt(i) => (k, i.to_string()),
                        PyQueryParamValue::PyFloat(f) => (k, f.to_string()),
                    })
                    .collect();
                Ok(QueryParams::Named(named_params).into())
            }
            PyQueryParamsLike::NamedVec(p) => {
                if p.is_empty() {
                    return Ok(PySqlfmtQueryParams(QueryParams::None));
                }
                let named_params = p
                    .into_iter()
                    .map(|(k, v)| match v {
                        PyQueryParamValue::PyString(s) => (k, s),
                        PyQueryParamValue::PyInt(i) => (k, i.to_string()),
                        PyQueryParamValue::PyFloat(f) => (k, f.to_string()),
                    })
                    .collect();
                let p = QueryParams::Named(named_params);
                Ok(p.into())
            }
            PyQueryParamsLike::Indexed(p) => {
                if p.is_empty() {
                    return Ok(PySqlfmtQueryParams(QueryParams::None));
                }
                let strings: Vec<String> = p
                    .into_iter()
                    .map(|v| match v {
                        PyQueryParamValue::PyString(s) => s,
                        PyQueryParamValue::PyInt(i) => i.to_string(),
                        PyQueryParamValue::PyFloat(f) => f.to_string(),
                    })
                    .collect();
                let p = QueryParams::Indexed(strings);
                Ok(p.into())
            }
            PyQueryParamsLike::PyQueryParams(p) => Ok(p),
        },
        None => Ok(PySqlfmtQueryParams(QueryParams::None)),
    }
}

/// Format SQL queries
///
/// Based on [sqlformat-crate](https://crates.io/crates/sqlformat)
#[pyfunction]
#[pyo3(
    signature = (
        sql,
        params=None,
        *,
        indent=PyIndent::default(),
        uppercase=None,
        lines_between_queries=1,
        ignore_case_convert=None,
        inline=false,
        max_inline_block=50,
        max_inline_arguments=None,
        max_inline_top_level=None,
        joins_as_top_level=false,
        dialect=PyDialect::default()
    )
)]
#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::too_many_arguments)]
pub fn sqlfmt(
    sql: &str,
    params: Option<PyQueryParamsLike>,
    indent: PyIndent,
    uppercase: Option<bool>,
    lines_between_queries: u8,
    ignore_case_convert: Option<Vec<String>>,
    inline: bool,
    max_inline_block: usize,
    max_inline_arguments: Option<usize>,
    max_inline_top_level: Option<usize>,
    joins_as_top_level: bool,
    dialect: PyDialect,
) -> PyResult<String> {
    let ignore_case_convert: Option<Vec<&str>> = ignore_case_convert
        .as_ref()
        .map(|v| v.iter().map(String::as_str).collect());
    let options = sqlformat::FormatOptions {
        indent: indent.into(),
        uppercase,
        lines_between_queries,
        ignore_case_convert,
        inline,
        max_inline_block,
        max_inline_arguments,
        max_inline_top_level,
        joins_as_top_level,
        dialect: dialect.into(),
    };
    if let Some(p) = params {
        if let PyQueryParamsLike::PyQueryParams(p) = p {
            Ok(sqlformat::format(sql, &p.0, &options))
        } else {
            let py_params = PySqlfmtQueryParams::py_new(Some(p))?;
            Ok(sqlformat::format(sql, &py_params.0, &options))
        }
    } else {
        let nada = QueryParams::None;
        Ok(sqlformat::format(sql, &nada, &options))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PyDialect(sqlformat::Dialect);

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
            sqlformat::Dialect::Generic => intern!(py, "generic"),
            sqlformat::Dialect::PostgreSql => intern!(py, "postgresql"),
            sqlformat::Dialect::SQLServer => intern!(py, "sqlserver"),
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
impl FromPyObject<'_> for PyDialect {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
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

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct PySqlFormatterOptions {
    indent: PyIndent,
    uppercase: Option<bool>,
    lines_between_queries: u8,
    ignore_case_convert: Option<Vec<String>>,
    inline: bool,
    max_inline_block: usize,
    max_inline_arguments: Option<usize>,
    max_inline_top_level: Option<usize>,
    joins_as_top_level: bool,
    dialect: PyDialect,
}

#[derive(Clone)]
#[pyclass(name = "SqlFormatter", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct PySqlFormatter(PySqlFormatterOptions);

impl PySqlFormatter {
    fn ignore_case_convert(&self) -> Option<Vec<&str>> {
        self.0
            .ignore_case_convert
            .as_ref()
            .map(|v| v.iter().map(String::as_str).collect())
    }
}

#[pymethods]
impl PySqlFormatter {
    #[new]
    #[pyo3(
        signature = (
            *,
            indent=PyIndent::default(),
            uppercase=None,
            lines_between_queries=1,
            ignore_case_convert=None,
            inline=false,
            max_inline_block=50,
            max_inline_arguments=None,
            max_inline_top_level=None,
            joins_as_top_level=false,
            dialect=PyDialect::default()
        )
    )]
    #[expect(clippy::too_many_arguments)]
    fn py_new(
        indent: PyIndent,
        uppercase: Option<bool>,
        lines_between_queries: u8,
        ignore_case_convert: Option<Vec<String>>,
        inline: bool,
        max_inline_block: usize,
        max_inline_arguments: Option<usize>,
        max_inline_top_level: Option<usize>,
        joins_as_top_level: bool,
        dialect: PyDialect,
    ) -> Self {
        Self(PySqlFormatterOptions {
            indent,
            uppercase,
            lines_between_queries,
            ignore_case_convert: ignore_case_convert.filter(|v| !v.is_empty()),
            inline,
            max_inline_block,
            max_inline_arguments,
            max_inline_top_level,
            joins_as_top_level,
            dialect,
        })
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    fn __getnewargs_ex__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let args = PyTuple::empty(py).into_bound_py_any(py)?;
        let kwargs = self.to_dict(py)?.into_bound_py_any(py)?;
        PyTuple::new(py, vec![args, kwargs])
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item(intern!(py, "indent"), self.0.indent)?;
        dict.set_item(intern!(py, "uppercase"), self.0.uppercase)?;
        dict.set_item(
            intern!(py, "lines_between_queries"),
            self.0.lines_between_queries,
        )?;
        let ignore_case_convert_py_str = intern!(py, "ignore_case_convert");
        if let Some(v) = &self.0.ignore_case_convert {
            let pylist = pyo3::types::PyList::new(py, v)?;
            dict.set_item(ignore_case_convert_py_str, pylist)?;
        } else {
            dict.set_item(ignore_case_convert_py_str, py.None())?;
        }
        dict.set_item(intern!(py, "inline"), self.0.inline)?;
        dict.set_item(intern!(py, "max_inline_block"), self.0.max_inline_block)?;
        dict.set_item(
            intern!(py, "max_inline_arguments"),
            self.0.max_inline_arguments,
        )?;
        dict.set_item(
            intern!(py, "max_inline_top_level"),
            self.0.max_inline_top_level,
        )?;
        dict.set_item(intern!(py, "joins_as_top_level"), self.0.joins_as_top_level)?;
        dict.set_item(intern!(py, "dialect"), self.0.dialect)?;
        Ok(dict)
    }

    #[pyo3(signature = (sql, params=None))]
    fn fmt(&self, sql: &str, params: Option<PyQueryParamsLike>) -> PyResult<String> {
        let opts = sqlformat::FormatOptions {
            indent: self.0.indent.into(),
            uppercase: self.0.uppercase,
            lines_between_queries: self.0.lines_between_queries,
            ignore_case_convert: self.ignore_case_convert(),
            inline: self.0.inline,
            max_inline_block: self.0.max_inline_block,
            max_inline_arguments: self.0.max_inline_arguments,
            max_inline_top_level: self.0.max_inline_top_level,
            joins_as_top_level: self.0.joins_as_top_level,
            dialect: self.0.dialect.into(),
        };
        if let Some(p) = params {
            if let PyQueryParamsLike::PyQueryParams(p) = p {
                Ok(sqlformat::format(sql, &p.0, &opts))
            } else {
                let py_params = PySqlfmtQueryParams::py_new(Some(p))?;
                Ok(sqlformat::format(sql, &py_params.0, &opts))
            }
        } else {
            let nada = QueryParams::None;
            Ok(sqlformat::format(sql, &nada, &opts))
        }
    }

    #[pyo3(signature = (sql, params=None))]
    fn __call__(&self, sql: &str, params: Option<PyQueryParamsLike>) -> PyResult<String> {
        self.fmt(sql, params)
    }
}

impl std::fmt::Debug for PySqlFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: exclude defaults from output

        // indent: PyIndent,
        // uppercase: Option<bool>,
        // lines_between_queries: u8,
        // ignore_case_convert: Option<Vec<String>>,
        // inline: bool,
        // max_inline_block: usize,
        // max_inline_arguments: Option<usize>,
        // max_inline_top_level: Option<usize>,
        // joins_as_top_level: bool,
        // dialect: PyDialect,
        write!(f, "SqlFormatter(")?;
        write!(f, "indent={:?}, ", self.0.indent)?;
        if let Some(uc) = self.0.uppercase {
            if uc {
                write!(f, "uppercase=True, ")?;
            } else {
                write!(f, "uppercase=False, ")?;
            }
        }
        write!(
            f,
            "lines_between_queries={}, ",
            self.0.lines_between_queries
        )?;
        if let Some(v) = &self.0.ignore_case_convert
            && !v.is_empty()
        {
            write!(f, "ignore_case_convert={v:?}, ")?;
        }
        if self.0.inline {
            write!(f, "inline=True, ")?;
        } else {
            write!(f, "inline=False, ")?;
        }
        write!(f, "max_inline_block={}, ", self.0.max_inline_block)?;
        if let Some(v) = self.0.max_inline_arguments {
            write!(f, "max_inline_arguments={v}, ")?;
        }
        if let Some(v) = self.0.max_inline_top_level {
            write!(f, "max_inline_top_level={v}, ")?;
        }
        if self.0.joins_as_top_level {
            write!(f, "joins_as_top_level=True, ")?;
        } else {
            write!(f, "joins_as_top_level=False, ")?;
        }

        write!(f, "dialect=\"{}\"", self.0.dialect)?;
        f.write_str(")")
    }
}

// ----------------------------------------------------------------------------
// FROM
// ----------------------------------------------------------------------------

impl From<QueryParams> for PySqlfmtQueryParams {
    fn from(p: QueryParams) -> Self {
        Self(p)
    }
}

// ----------------------------------------------------------------------------

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySqlfmtQueryParams>()?;
    m.add_class::<PySqlFormatter>()?;
    m.add_function(wrap_pyfunction!(sqlfmt, m)?)?;
    m.add_function(wrap_pyfunction!(sqlfmt_params, m)?)?;
    Ok(())
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![expect(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_named_params() {
        let sql = "SELECT * FROM poopy     WHERE column = :value";
        let formatted = sqlfmt(
            sql,
            Some(PyQueryParamsLike::NamedVec(vec![(
                "value".to_string(),
                PyQueryParamValue::PyString("1".to_string()),
            )])),
            PyIndent::default(),
            None,
            1,
            None,
            false,
            50,
            None,
            None,
            false,
            PyDialect(sqlformat::Dialect::Generic),
        )
        .unwrap();
        let expected = "SELECT\n  *\nFROM\n  poopy\nWHERE\n  COLUMN = 1";
        assert_eq!(formatted, expected);
    }
}
