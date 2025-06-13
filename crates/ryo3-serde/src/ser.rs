//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.
use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, Serializer};
use std::fmt;

use crate::any_repr::any_repr;
use crate::pytypes::{
    bool_, byteslike, date, datetime, dict, float, int, list, none, py_uuid, str, time, tuple,
};
use crate::rytypes::ry_uuid;
use crate::type_cache::PyTypeCache;
use pyo3::types::PyString;

pub struct SerializePyAny<'py> {
    pub(crate) obj: Bound<'py, PyAny>,
    pub(crate) none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeCache,
}

impl<'py> SerializePyAny<'py> {
    #[must_use]
    pub fn new(py: Python<'py>, obj: Bound<'py, PyAny>, none_value: Option<&'py str>) -> Self {
        Self {
            obj,
            none_value,
            ob_type_lookup: PyTypeCache::cached(py),
        }
    }

    pub(crate) fn with_obj(&self, obj: Bound<'py, PyAny>) -> Self {
        Self {
            obj,
            none_value: self.none_value,
            ob_type_lookup: self.ob_type_lookup,
        }
    }
}

pub struct SerializePyString<'py> {
    obj: &'py Bound<'py, PyAny>,
    none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeCache,
}

// macro_rules! serde_err {
//     ($msg:expr, $( $msg_args:expr ),+ ) => {
//         Err(SerError::custom(format!($msg, $( $msg_args ),+ )))
//     };
// }
macro_rules! serde_err {
    ($($arg:tt)*) => {
        Err(SerError::custom(format_args!($($arg)*)))
    }
}

// impl Serialize for SerializePyString<'_> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let ptr = self.obj.as_ptr();
//         if unsafe { PyUnicode_IS_ASCII(ptr) } != 0 {
//             unsafe {
//                 let bytes = {
//                     std::slice::from_raw_parts(
//                         pyo3::ffi::PyUnicode_1BYTE_DATA(ptr),
//                         pyo3::ffi::PyUnicode_GET_LENGTH(ptr) as usize,
//                     )
//                 };
//                 return serializer.serialize_str(std::str::from_utf8_unchecked(bytes));
//             }
//         } else {
//             let py_str: &Bound<'_, PyString> = self.obj.downcast().map_err(map_py_err)?;
//             let s = py_str.to_str().map_err(map_py_err)?;
//             return serializer.serialize_str(s);
//         }
//     }
// }
impl Serialize for SerializePyString<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let py_str: &Bound<'_, PyString> = self.obj.downcast().map_err(map_py_err)?;
        let s = py_str.to_str().map_err(map_py_err)?;
        serializer.serialize_str(s)
    }
}

impl Serialize for SerializePyAny<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        macro_rules! serialize {
            ($t:ty) => {
                match self.obj.extract::<$t>() {
                    Ok(v) => v.serialize(serializer),
                    Err(e) => Err(map_py_err(e)),
                }
            };
        }

        let lookup = self.ob_type_lookup;
        let ob_type_ptr = self.obj.get_type_ptr() as usize;
        // if let Some(ob_type) = lookup.obtype(&self.obj) {
        //     match ob_type {
        //         PyObType::None => none(self, serializer),
        //
        //         PyObType::Bool => bool(self, serializer),
        //         PyObType::Int => int(self, serializer),
        //         PyObType::Float => float(self, serializer),
        //         PyObType::String => str(self, serializer),
        //         PyObType::List =>  list(self, serializer),
        //         PyObType::Tuple =>  tuple(self, serializer),
        //         PyObType::Dict =>  dict(self, serializer),
        //         PyObType::DateTime =>  datetime(self, serializer),
        //         PyObType::Date =>  date(self, serializer),
        //         PyObType::Time =>  time(self, serializer),
        //         PyObType::Bytes | PyObType::ByteArray => byteslike(self, serializer),
        //         PyObType::PyUuid => py_uuid(self, serializer),
        //         PyObType::RyUuid => {
        //             let ry_uu = self.obj.downcast::<RyUuid>().map_err(map_py_err)?;
        //             return ry_uu.borrow().serialize(serializer);
        //         }
        //     }
        // } else {
        //     serde_err!("{} is not JSON-serializable", any_repr(&self.obj))
        // }
        //
        // // ugly but this seems to be just marginally faster than a guarded match, also allows for custom cases
        // // if we wanted to add them
        if ob_type_ptr == lookup.none {
            none(self, serializer)
        } else if ob_type_ptr == lookup.bool {
            bool_(self, serializer)
        } else if ob_type_ptr == lookup.int {
            int(self, serializer)
        } else if ob_type_ptr == lookup.float {
            float(self, serializer)
        } else if ob_type_ptr == lookup.string {
            str(self, serializer)
        } else if ob_type_ptr == lookup.list {
            list(self, serializer)
        } else if ob_type_ptr == lookup.tuple {
            tuple(self, serializer)
        } else if ob_type_ptr == lookup.dict {
            dict(self, serializer)
        } else if ob_type_ptr == lookup.datetime {
            datetime(self, serializer)
        } else if ob_type_ptr == lookup.date {
            date(self, serializer)
        } else if ob_type_ptr == lookup.time {
            time(self, serializer)
        } else if ob_type_ptr == lookup.bytes || ob_type_ptr == lookup.bytearray {
            byteslike(self, serializer)
        } else if ob_type_ptr == lookup.py_uuid {
            py_uuid(self, serializer)
        } else if ob_type_ptr == lookup.ry_uuid {
            ry_uuid(self, serializer)
        } else {
            serde_err!("{} is not JSON-serializable", any_repr(&self.obj))
        }
    }
}

fn map_py_err<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}
