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

enum IntOrFloat {
    Int(i64),
    Float(f64),
}

impl Serialize for IntOrFloat {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Int(i) => serializer.serialize_i64(*i),
            Self::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

impl From<f64> for IntOrFloat {
    #[expect(clippy::cast_possible_truncation)]
    fn from(value: f64) -> Self {
        if value.fract() == 0.0 {
            Self::Int(value as i64)
        } else {
            Self::Float(value)
        }
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

        let r = IntOrFloat::from(c.real());
        struct_ser.serialize_field(&r)?;

        let i = IntOrFloat::from(c.imag());
        struct_ser.serialize_field(&i)?;
        struct_ser.end()
    }
}
