use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::errors::pyerr2sererr;

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::ser::PySerializeContext;
use pyo3::Bound;
use pyo3::types::{PyFrozenSet, PyIterator, PySet};

pub(crate) struct SerializePySet<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    // default: Option<&'py Bound<'py, PyAny>>,
    // ob_type_lookup: &'py PyTypeCache,
}

impl<'a, 'py> SerializePySet<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePySet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_set: &Bound<'_, PyAny> = self.obj.cast::<PySet>().map_err(pyerr2sererr)?;
        let len = py_set.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_set).expect("set is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ser_pyany = SerializePyAny::new_with_depth(&pyany, self.ctx, self.depth + 1);
            seq.serialize_element(&ser_pyany).map_err(pyerr2sererr)?;
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// frozenset
// ----------------------------------------------------------------------------
pub(crate) struct SerializePyFrozenSet<'a, 'py> {
    pub(crate) ctx: PySerializeContext<'py>,
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
}

impl<'a, 'py> SerializePyFrozenSet<'a, 'py> {
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>, ctx: PySerializeContext<'py>) -> Self {
        Self {
            obj,
            ctx,
            depth: Depth::default(),
        }
    }
}

impl Serialize for SerializePyFrozenSet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_frozenset: &Bound<'_, PyAny> =
            self.obj.cast::<PyFrozenSet>().map_err(pyerr2sererr)?;
        let len = py_frozenset.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_frozenset).expect("frozenset is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ser_pyany = SerializePyAny::new_with_depth(&pyany, self.ctx, self.depth + 1);
            seq.serialize_element(&ser_pyany).map_err(pyerr2sererr)?;
        }
        seq.end()
    }
}
