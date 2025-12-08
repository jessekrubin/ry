use pyo3::prelude::*;
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

use crate::errors::pyerr2sererr;
use pyo3::Bound;
use pyo3::types::{PyComplex, PyComplexMethods};

pub(crate) struct SerializePyComplex<'a, 'py> {
    obj: &'a Bound<'py, PyAny>,
}

impl<'a, 'py> SerializePyComplex<'a, 'py> {
    #[inline]
    pub(crate) fn new(obj: &'a Bound<'py, PyAny>) -> Self {
        Self { obj }
    }
}

impl Serialize for SerializePyComplex<'_, '_> {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let c = self.obj.cast_exact::<PyComplex>().map_err(pyerr2sererr)?;
        let mut struct_ser = serializer.serialize_tuple_struct("complex", 2)?;

        let r = c.real();
        // how to tell if it's float or int?
        if r.fract() == 0.0 {
            let integer = r as i64;
            struct_ser.serialize_field(&integer)?; // integer
        } else {
            struct_ser.serialize_field(&r)?; // float
        }
        let i = c.imag();
        if i.fract() == 0.0 {
            let integer = i as i64;
            struct_ser.serialize_field(&integer)?; // integer
        } else {
            struct_ser.serialize_field(&i)?; // float
        }
        struct_ser.end()
    }
}
