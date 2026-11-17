use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use sqlformat::QueryParams;

use crate::py_types::PyQueryParamsLike;
use crate::sqlfmt_params;

#[pyclass(name = "SqlfmtQueryParams", frozen, immutable_type, eq)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct RySqlfmtQueryParams(pub(crate) QueryParams);

#[pymethods]
impl RySqlfmtQueryParams {
    #[new]
    #[pyo3(signature = (params = None))]
    pub(crate) fn py_new(params: Option<PyQueryParamsLike>) -> PyResult<Self> {
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

    fn __hash__(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for RySqlfmtQueryParams {
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

impl Hash for RySqlfmtQueryParams {
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

impl Display for RySqlfmtQueryParams {
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

            QueryParams::None => f.write_str("None"),
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for RySqlfmtQueryParams {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            return Ok(Self(QueryParams::None));
        } else if let Ok(p) = obj.cast_exact::<Self>() {
            return Ok(p.get().clone());
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid type for SqlfmtQueryParams; expected None, dict[str, str], or list[str]",
        ))
    }
}

// ----------------------------------------------------------------------------
// FROM
// ----------------------------------------------------------------------------

impl From<QueryParams> for RySqlfmtQueryParams {
    fn from(p: QueryParams) -> Self {
        Self(p)
    }
}
