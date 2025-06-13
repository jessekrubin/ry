use crate::errors::map_py_err;
use crate::ser::SerializePyAny;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};
use pyo3::Bound;
use serde::ser::SerializeSeq;

pub fn list<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_list: &Bound<'_, PyList> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_list.len();
    if len == 0 {
        serializer.serialize_seq(Some(0))?.end()
    } else {
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_list {
            seq.serialize_element(&ser.with_obj(element))?;
        }
        seq.end()
    }
}

pub fn tuple<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let py_tuple: &Bound<'_, PyTuple> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_tuple.len();
    let mut seq = serializer.serialize_seq(Some(len))?;
    for element in py_tuple {
        // if self.none_value.is_some() || !element.is_none() {
        seq.serialize_element(&ser.with_obj(element))?;
        // }
    }
    seq.end()
}
