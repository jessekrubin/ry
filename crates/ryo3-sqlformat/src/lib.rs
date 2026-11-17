#![doc = include_str!("../README.md")]
mod py_types;
mod ry_sql_formatter;
mod ry_sqlfmt_query_params;

use pyo3::prelude::*;
use sqlformat::QueryParams;

pub use crate::py_types::{PyDialect, PyIndent, PyQueryParamValue, PyQueryParamsLike};
pub use crate::ry_sql_formatter::RySqlFormatter;
pub use crate::ry_sqlfmt_query_params::RySqlfmtQueryParams;

#[pyfunction]
#[pyo3(signature = (params = None))]
pub fn sqlfmt_params(params: Option<PyQueryParamsLike>) -> PyResult<RySqlfmtQueryParams> {
    match params {
        Some(params) => match params {
            PyQueryParamsLike::NamedMap(p) => {
                if p.is_empty() {
                    return Ok(RySqlfmtQueryParams(QueryParams::None));
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
                    return Ok(RySqlfmtQueryParams(QueryParams::None));
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
                    return Ok(RySqlfmtQueryParams(QueryParams::None));
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
        None => Ok(RySqlfmtQueryParams(QueryParams::None)),
    }
}

/// Format SQL queries
///
/// Based on [sqlformat-crate](https://crates.io/crates/sqlformat)
#[pyfunction]
#[pyo3(
    signature = (
        sql,
        params = None,
        *,
        indent = PyIndent::default(),
        uppercase = None,
        lines_between_queries = 1,
        ignore_case_convert = None,
        inline = false,
        max_inline_block = 50,
        max_inline_arguments = None,
        max_inline_top_level = None,
        joins_as_top_level = false,
        dialect = PyDialect::default()
    ),
    text_signature = "(sql, params=None, *, indent=2, uppercase=None, lines_between_queries=1, ignore_case_convert=None, inline=False, max_inline_block=50, max_inline_arguments=None, max_inline_top_level=None, joins_as_top_level=False, dialect='generic')"
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
            let py_params = RySqlfmtQueryParams::py_new(Some(p))?;
            Ok(sqlformat::format(sql, &py_params.0, &options))
        }
    } else {
        let nada = QueryParams::None;
        Ok(sqlformat::format(sql, &nada, &options))
    }
}

// ----------------------------------------------------------------------------

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RySqlfmtQueryParams>()?;
    m.add_class::<RySqlFormatter>()?;
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
            Some(true),
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
