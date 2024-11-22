#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unused_self)]

use std::collections::HashMap;

use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use sqlformat::{self, QueryParams};

#[pyclass(name = "SqlfmtQueryParams", module = "ryo3")]
#[derive(Debug, Clone)]
pub struct PySqlfmtQueryParams {
    pub params: QueryParams,
}

#[pymethods]
impl PySqlfmtQueryParams {
    #[new]
    fn new(params: PyQueryParamsLike) -> PyResult<Self> {
        sqlfmt_params(Some(params))
    }

    fn __str__(&self) -> String {
        match &self.params {
            QueryParams::Named(p) => {
                // collect into string for display
                let s = p
                    .iter()
                    .map(|(k, v)| format!("(\"{k}, \"{v}\")"))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("SqlfmtQueryParams({s})")
            }
            QueryParams::Indexed(p) => {
                let s = p
                    .iter()
                    .map(|v| format!("\"{v}\""))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("SqlfmtQueryParams([{s}])")
            }
            QueryParams::None => String::from("SqlfmtQueryParams(None)"),
        }
    }
}

impl From<Vec<(String, String)>> for PySqlfmtQueryParams {
    fn from(p: Vec<(String, String)>) -> Self {
        let named_params = p.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let p = QueryParams::Named(named_params);
        PySqlfmtQueryParams { params: p }
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
        Some(params) => {
            let py_params = {
                match params {
                    PyQueryParamsLike::NamedMap(p) => {
                        let named_params = p
                            .iter()
                            .map(|(k, v)| match v {
                                PyQueryParamValue::PyString(s) => (k.clone(), s.clone()),
                                PyQueryParamValue::PyInt(i) => (k.clone(), i.to_string()),
                                PyQueryParamValue::PyFloat(f) => (k.clone(), f.to_string()),
                            })
                            .collect();
                        let p = QueryParams::Named(named_params);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::NamedVec(p) => {
                        let named_params = p
                            .iter()
                            .map(|(k, v)| match v {
                                PyQueryParamValue::PyString(s) => (k.clone(), s.clone()),
                                PyQueryParamValue::PyInt(i) => (k.clone(), i.to_string()),
                                PyQueryParamValue::PyFloat(f) => (k.clone(), f.to_string()),
                            })
                            .collect();
                        let p = QueryParams::Named(named_params);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::Indexed(p) => {
                        let strings: Vec<String> = p
                            .iter()
                            .map(|v| match v {
                                PyQueryParamValue::PyString(s) => s.clone(),
                                PyQueryParamValue::PyInt(i) => i.to_string(),
                                PyQueryParamValue::PyFloat(f) => f.to_string(),
                            })
                            .collect();
                        let p = QueryParams::Indexed(strings);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::PyQueryParams(p) => PySqlfmtQueryParams { params: p.params },
                }
            };
            Ok(py_params)
        }
        None => Ok(PySqlfmtQueryParams {
            params: QueryParams::None,
        }),
    }
}

#[pyfunction]
#[pyo3(signature = (sql, params=None, *, indent=None, uppercase=None, lines_between_queries=None))]
/// Format SQL queries
///
/// Based on the sqlformat crate (https://crates.io/crates/sqlformat)
pub fn sqlfmt(
    sql: &str,
    params: Option<PyQueryParamsLike>,
    indent: Option<i16>,
    uppercase: Option<bool>,
    lines_between_queries: Option<u8>,
) -> PyResult<String> {
    // if indent is negative, use tabs
    let indent = match indent {
        Some(i) if i < 0 => sqlformat::Indent::Tabs,
        Some(i) => sqlformat::Indent::Spaces(i.try_into().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Indent must be a positive integer")
        })?),
        None => sqlformat::Indent::Spaces(2),
    };
    let options = sqlformat::FormatOptions {
        indent,
        uppercase: Option::from(uppercase.unwrap_or(true)),
        lines_between_queries: lines_between_queries.unwrap_or(1),
        ignore_case_convert: None,
    };
    if let Some(p) = params {
        if let PyQueryParamsLike::PyQueryParams(p) = p {
            Ok(sqlformat::format(sql, &p.params, &options))
        } else {
            let py_params = PySqlfmtQueryParams::new(p)?;
            Ok(sqlformat::format(sql, &py_params.params, &options))
        }
    } else {
        let nada = QueryParams::None;
        Ok(sqlformat::format(sql, &nada, &options))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sqlfmt, m)?)?;
    m.add_function(wrap_pyfunction!(sqlfmt_params, m)?)?;
    m.add_class::<PySqlfmtQueryParams>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
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
            None,
            None,
            None,
        )
        .unwrap();
        let expected = "SELECT\n  *\nFROM\n  poopy\nWHERE\n  COLUMN = 1";
        assert_eq!(formatted, expected);
    }
}
