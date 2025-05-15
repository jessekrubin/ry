use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use serde::Serialize;

#[derive(Serialize, FromPyObject)]
#[serde(untagged)] // I think this is the way????????? but not sure....
pub(crate) enum QueryValue {
    Bool(bool),
    I64(i64),
    Float(f64),
    String(String),
}
#[derive(Serialize)]
pub(crate) struct QueryLike(Vec<(String, QueryValue)>);

impl FromPyObject<'_> for QueryLike {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pairs) = ob.downcast::<PyDict>() {
            let mut vec = Vec::new();
            for (k, v) in pairs.iter() {
                if k.is_none() || v.is_none() {
                    continue;
                }
                let k = k.extract::<String>()?;
                let v = v.extract::<QueryValue>()?;
                vec.push((k, v));
            }
            Ok(QueryLike(vec))
        } else if let Ok(pairs) = ob.downcast::<PyTuple>() {
            let mut vec = Vec::new();
            for item in pairs.iter() {
                if item.is_none() {
                    continue;
                }
                let item = item.extract::<(String, QueryValue)>()?;
                vec.push(item);
            }
            return Ok(QueryLike(vec));
        } else if let Ok(pairs) = ob.downcast::<PyList>() {
            let mut vec = Vec::new();
            for item in pairs.iter() {
                if item.is_none() {
                    continue;
                }
                let item = item.extract::<(String, QueryValue)>()?;
                vec.push(item);
            }
            return Ok(QueryLike(vec));
        } else {
            Err(PyValueError::new_err("Invalid query"))
        }
    }
}
