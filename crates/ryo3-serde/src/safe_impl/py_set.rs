use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use crate::errors::pyerr2sererr;

use crate::SerializePyAny;
use crate::constants::Depth;
use crate::safe_impl::with_obj::ObjTypeRef;
use crate::type_cache::PyTypeCache;
use pyo3::Bound;
use pyo3::types::{PyFrozenSet, PyIterator, PySet};

pub(crate) struct SerializePySet<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
    ob_type_lookup: &'py PyTypeCache,
}

impl<'a, 'py> ObjTypeRef<'py> for SerializePySet<'a, 'py> {
    fn type_ref(&self) -> &'py PyTypeCache {
        self.ob_type_lookup
    }
}

impl<'a, 'py> SerializePySet<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ob_type_lookup: &'py PyTypeCache,
        default: Option<&'py Bound<'py, PyAny>>,
    ) -> Self {
        Self {
            obj,
            ob_type_lookup,
            depth: Depth::default(),
            default,
        }
    }
}

impl Serialize for SerializePySet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_set: &Bound<'_, PyAny> = self.obj.downcast::<PySet>().map_err(pyerr2sererr)?;
        let len = py_set.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_set).expect("set is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ser_pyany = SerializePyAny::new_with_depth(
                &pyany,
                self.default,
                self.depth + 1,
                self.ob_type_lookup,
            );
            seq.serialize_element(&ser_pyany).map_err(pyerr2sererr)?;
        }
        seq.end()
    }
}

// ----------------------------------------------------------------------------
// frozenset
// ----------------------------------------------------------------------------
pub(crate) struct SerializePyFrozenSet<'a, 'py> {
    pub(crate) obj: &'a Bound<'py, PyAny>,
    pub(crate) depth: Depth,
    default: Option<&'py Bound<'py, PyAny>>,
    ob_type_lookup: &'py PyTypeCache,
}

impl<'a, 'py> SerializePyFrozenSet<'a, 'py> {
    pub(crate) fn new(
        obj: &'a Bound<'py, PyAny>,
        ob_type_lookup: &'py PyTypeCache,
        default: Option<&'py Bound<'py, PyAny>>,
    ) -> Self {
        Self {
            obj,
            ob_type_lookup,
            depth: Depth::default(),
            default,
        }
    }
}

impl Serialize for SerializePyFrozenSet<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_frozenset: &Bound<'_, PyAny> =
            self.obj.downcast::<PyFrozenSet>().map_err(pyerr2sererr)?;
        let len = py_frozenset.len().map_err(pyerr2sererr)?;
        if len == 0 {
            return serializer.serialize_seq(Some(0))?.end();
        }
        let py_iter = PyIterator::from_object(py_frozenset).expect("frozenset is always iterable");
        let mut seq = serializer.serialize_seq(Some(len))?;
        for element in py_iter {
            let pyany = element.map_err(pyerr2sererr)?;
            let ser_pyany = SerializePyAny::new_with_depth(
                &pyany,
                self.default,
                self.depth + 1,
                self.ob_type_lookup,
            );
            seq.serialize_element(&ser_pyany).map_err(pyerr2sererr)?;
        }
        seq.end()
    }
}
