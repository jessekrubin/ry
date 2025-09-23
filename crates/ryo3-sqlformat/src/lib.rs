#![doc = include_str!("../README.md")]
use pyo3::prelude::PyModule;
use pyo3::{IntoPyObjectExt, prelude::*};
use sqlformat::{self, QueryParams};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

#[pyclass(name = "SqlfmtQueryParams", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct PySqlfmtQueryParams {
    pub params: QueryParams,
}

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

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __len__(&self) -> usize {
        match &self.params {
            QueryParams::Named(p) => p.len(),
            QueryParams::Indexed(p) => p.len(),
            QueryParams::None => 0,
        }
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        let params: Bound<'py, PyAny> = match &self.params {
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
        match (&self.params, &other.params) {
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

    fn __ne__(&self, other: &Self) -> bool {
        !self.__eq__(other)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        match &self.params {
            QueryParams::Named(p) => {
                let mut p: Vec<(&str, &str)> =
                    p.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                p.sort_by(|a, b| a.0.cmp(b.0));
                for (k, v) in p {
                    k.hash(&mut hasher);
                    v.hash(&mut hasher);
                }
            }
            QueryParams::Indexed(p) => {
                for v in p {
                    v.hash(&mut hasher);
                }
            }
            QueryParams::None => {}
        }
        hasher.finish()
    }
}

impl Display for PySqlfmtQueryParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("SqlfmtQueryParams(")?;
        QueryParamsFormatter(&self.params).fmt(f)?;
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

#[derive(Debug, Clone, Copy)]
pub struct PyIndent(sqlformat::Indent);

const PY_INDENT_ERR_MSG: &str =
    "Indent must be an integer, 'tabs'/'\\t', or 'spaces' (default 2 spaces)";
impl FromPyObject<'_> for PyIndent {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        // none go to default (2 spaces)
        if ob.is_none() {
            return Ok(Self(sqlformat::Indent::Spaces(2)));
        }

        if let Ok(i) = ob.extract::<i16>() {
            if i < 0 {
                return Ok(Self(sqlformat::Indent::Tabs));
            }
            return Ok(Self(sqlformat::Indent::Spaces(
                i.try_into().expect("i16 to u8 should not fail here"),
            )));
        }

        if let Ok(s) = ob.extract::<&str>() {
            match s.to_lowercase().as_str() {
                "tabs" | "\t" => return Ok(Self(sqlformat::Indent::Tabs)),
                "spaces" => return Ok(Self(sqlformat::Indent::Spaces(2))),
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        PY_INDENT_ERR_MSG,
                    ));
                }
            }
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            PY_INDENT_ERR_MSG,
        ))
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
                            .into_iter()
                            .map(|(k, v)| match v {
                                PyQueryParamValue::PyString(s) => (k, s),
                                PyQueryParamValue::PyInt(i) => (k, i.to_string()),
                                PyQueryParamValue::PyFloat(f) => (k, f.to_string()),
                            })
                            .collect();
                        let p = QueryParams::Named(named_params);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::NamedVec(p) => {
                        let named_params = p
                            .into_iter()
                            .map(|(k, v)| match v {
                                PyQueryParamValue::PyString(s) => (k, s),
                                PyQueryParamValue::PyInt(i) => (k, i.to_string()),
                                PyQueryParamValue::PyFloat(f) => (k, f.to_string()),
                            })
                            .collect();
                        let p = QueryParams::Named(named_params);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::Indexed(p) => {
                        let strings: Vec<String> = p
                            .into_iter()
                            .map(|v| match v {
                                PyQueryParamValue::PyString(s) => s,
                                PyQueryParamValue::PyInt(i) => i.to_string(),
                                PyQueryParamValue::PyFloat(f) => f.to_string(),
                            })
                            .collect();
                        let p = QueryParams::Indexed(strings);
                        PySqlfmtQueryParams { params: p }
                    }
                    PyQueryParamsLike::PyQueryParams(p) => p,
                }
            };
            Ok(py_params)
        }
        None => Ok(PySqlfmtQueryParams {
            params: QueryParams::None,
        }),
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
        indent=None,
        uppercase=None,
        lines_between_queries=1,
        ignore_case_convert=None,
        inline=false,
        max_inline_block=50,
        max_inline_arguments=None,
        max_inline_top_level=None,
        joins_as_top_level=false,
    )
)]
#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::too_many_arguments)]
pub fn sqlfmt(
    sql: &str,
    params: Option<PyQueryParamsLike>,
    indent: Option<PyIndent>,
    uppercase: Option<bool>,
    lines_between_queries: u8,
    ignore_case_convert: Option<Vec<String>>,
    inline: bool,
    max_inline_block: usize,
    max_inline_arguments: Option<usize>,
    max_inline_top_level: Option<usize>,
    joins_as_top_level: bool,
) -> PyResult<String> {
    let indent = indent.map_or(sqlformat::Indent::Spaces(2), |i| i.0);
    let ignore_case_convert: Option<Vec<&str>> = ignore_case_convert
        .as_ref()
        .map(|v| v.iter().map(String::as_str).collect());
    let options = sqlformat::FormatOptions {
        indent,
        uppercase,
        lines_between_queries,
        ignore_case_convert,
        inline,
        max_inline_block,
        max_inline_arguments,
        max_inline_top_level,
        joins_as_top_level,
    };
    if let Some(p) = params {
        if let PyQueryParamsLike::PyQueryParams(p) = p {
            Ok(sqlformat::format(sql, &p.params, &options))
        } else {
            let py_params = PySqlfmtQueryParams::py_new(Some(p))?;
            Ok(sqlformat::format(sql, &py_params.params, &options))
        }
    } else {
        let nada = QueryParams::None;
        Ok(sqlformat::format(sql, &nada, &options))
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySqlfmtQueryParams>()?;
    m.add_function(wrap_pyfunction!(sqlfmt, m)?)?;
    m.add_function(wrap_pyfunction!(sqlfmt_params, m)?)?;
    Ok(())
}

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
            None,
            None,
            1,
            None,
            false,
            50,
            None,
            None,
            false,
        )
        .unwrap();
        let expected = "SELECT\n  *\nFROM\n  poopy\nWHERE\n  COLUMN = 1";
        assert_eq!(formatted, expected);
    }
}
