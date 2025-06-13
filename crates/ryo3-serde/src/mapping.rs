use crate::any_repr::any_repr;
use crate::errors::map_py_err;
use crate::ser::SerializePyAny;
use crate::serde_err;
use pyo3::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pyo3::Bound;
use serde::ser::{Error as SerError, SerializeMap, SerializeSeq, Serializer};

fn mapping_key<'py, E: SerError>(key: &'py Bound<'py, PyAny>) -> Result<&'py str, E> {
    if let Ok(py_string) = key.downcast::<PyString>() {
        py_string.to_str().map_err(map_py_err)
    } else if let Ok(key) = key.extract::<bool>() {
        Ok(if key { "true" } else { "false" })
    } else {
        let key_repr = any_repr(key);
        serde_err!("{} is not JSON-serializable as map-key", key_repr)
    }
}

pub fn dict<'py, S>(ser: &SerializePyAny<'py>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let py_dict: &Bound<'_, PyDict> = ser.obj.downcast().map_err(map_py_err)?;
    let len = py_dict.len();
    if len == 0 {
        return serializer.serialize_map(Some(0))?.end();
    }
    let mut m = serializer.serialize_map(Some(len))?;
    for (k, v) in py_dict {
        m.serialize_entry(mapping_key(&k)?, &ser.with_obj(v))?;
    }
    m.end()
}
