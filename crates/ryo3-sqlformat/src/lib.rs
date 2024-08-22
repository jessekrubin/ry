use std::collections::HashMap;

use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use sqlformat::{self, QueryParams};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyQueryParams {
    pub params: QueryParams,
}

#[pymethods]
impl PyQueryParams {
    #[new]
    fn new(params: PyQueryParamsLike) -> Self {
        match params {
            PyQueryParamsLike::NamedMap(p) => {
                let named_params = p
                    .iter()
                    .map(|(k, v)| match v {
                        PyStringOrInt::PyString(s) => (k.clone(), s.clone()),
                        PyStringOrInt::PyInt(i) => (k.clone(), i.to_string()),
                        PyStringOrInt::PyFloat(f) => (k.clone(), f.to_string()),
                    })
                    .collect();
                let p = QueryParams::Named(named_params);
                PyQueryParams { params: p }
            }
            PyQueryParamsLike::NamedVec(p) => {
                let named_params = p
                    .iter()
                    .map(|(k, v)| match v {
                        PyStringOrInt::PyString(s) => (k.clone(), s.clone()),
                        PyStringOrInt::PyInt(i) => (k.clone(), i.to_string()),
                        PyStringOrInt::PyFloat(f) => (k.clone(), f.to_string()),
                    })
                    .collect();
                let p = QueryParams::Named(named_params);
                PyQueryParams { params: p }
            }
            PyQueryParamsLike::Indexed(p) => {
                let p = QueryParams::Indexed(p);
                PyQueryParams { params: p }
            }
            PyQueryParamsLike::PyQueryParams(p) => PyQueryParams { params: p.params },
        }
    }

    fn __str__(&self) -> String {
        match &self.params {
            QueryParams::Named(p) => {
                // collect into string for display
                let s = p
                    .iter()
                    .map(|(k, v)| format!("(\"{}, \"{}\")", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("QueryParams({})", s)
            }
            QueryParams::Indexed(p) => {
                let s = p
                    .iter()
                    .map(|v| format!("\"{}\"", v))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("QueryParams({})", s)
            }
            QueryParams::None => {
                format!("QueryParams(None)")
            }
        }
    }
}

impl From<Vec<(String, String)>> for PyQueryParams {
    fn from(p: Vec<(String, String)>) -> Self {
        let named_params = p.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let p = QueryParams::Named(named_params);
        PyQueryParams { params: p }
    }
}

#[derive(FromPyObject)]
pub enum PyStringOrInt {
    PyString(String),
    PyInt(i64),
    PyFloat(f64),
}

#[derive(FromPyObject)]
pub enum PyQueryParamsLike {
    PyQueryParams(PyQueryParams),
    NamedMap(HashMap<String, PyStringOrInt>),
    NamedVec(Vec<(String, PyStringOrInt)>),
    Indexed(Vec<String>),
}

impl From<QueryParams> for PyQueryParamsLike {
    fn from(p: QueryParams) -> Self {
        match p {
            QueryParams::Named(p) => {
                let named_params = p
                    .iter()
                    .map(|(k, v)| {
                        if let Ok(i) = v.parse::<i64>() {
                            (k.clone(), PyStringOrInt::PyInt(i))
                        } else if let Ok(f) = v.parse::<f64>() {
                            (k.clone(), PyStringOrInt::PyFloat(f))
                        } else {
                            (k.clone(), PyStringOrInt::PyString(v.clone()))
                        }
                    })
                    .collect();
                PyQueryParamsLike::NamedMap(named_params)
            }
            QueryParams::Indexed(p) => PyQueryParamsLike::Indexed(p),
            QueryParams::None => PyQueryParamsLike::NamedMap(HashMap::new()),
        }
    }
}

#[pyfunction]
#[pyo3(signature = (params=None))]
pub fn params(params: Option<PyQueryParamsLike>) -> PyResult<PyQueryParams> {
    match params {
        Some(p) => Ok(PyQueryParams::new(p)),
        None => Ok(PyQueryParams::new(PyQueryParamsLike::NamedMap(
            HashMap::new(),
        ))),
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
        uppercase: uppercase.unwrap_or(true),
        lines_between_queries: lines_between_queries.unwrap_or(1),
    };
    if let Some(p) = params {
        match p {
            PyQueryParamsLike::PyQueryParams(p) => Ok(sqlformat::format(sql, &p.params, options)),
            _ => {
                let py_params = PyQueryParams::new(p);
                Ok(sqlformat::format(sql, &py_params.params, options))
            }
        }
    } else {
        let nada = sqlformat::QueryParams::None;
        Ok(sqlformat::format(sql, &nada, options))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sqlfmt, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_params() {
        let sql = "SELECT * FROM poopy     WHERE column = :value";
        let formatted = sqlfmt(
            sql,
            Some(PyQueryParamsLike::NamedVec(vec![(
                "value".to_string(),
                PyStringOrInt::PyString("1".to_string()),
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
