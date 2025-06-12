//! Serializer for `PyAny`
//!
//! Based on a combination of `orjson`, `pythonize` and `rtoml`.
use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, SerializeMap, SerializeSeq, Serializer};
use std::fmt;

use pyo3::sync::GILOnceCell;
use pyo3::types::{
    PyBool, PyByteArray, PyBytes, PyDate, PyDateTime, PyDict, PyFloat, PyInt, PyList, PyNone,
    PyString, PyTime, PyTuple,
};
use pyo3::PyTypeInfo;
use ryo3_uuid::{uuid, PyUuid as RyUuid};

pub enum PyObType {
    None,
    Int,
    Bool,
    Float,
    String,
    Bytes,
    ByteArray,
    List,
    Tuple,
    Dict,
    DateTime,
    Date,
    Time,
    PyUuid, // not used yet
    // ry-types
    RyUuid,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PyTypeLookup {
    pub none: usize,
    // numeric types
    pub int: usize,
    pub bool: usize,
    pub float: usize,
    // string types
    pub string: usize,
    pub bytes: usize,
    pub bytearray: usize,
    // sequence types
    pub list: usize,
    pub tuple: usize,
    // mapping types
    pub dict: usize,
    // datetime types
    pub datetime: usize,
    pub date: usize,
    pub time: usize,
    // uuid
    pub py_uuid: usize,
    pub ry_uuid: usize, // not used yet
}

static TYPE_LOOKUP: GILOnceCell<PyTypeLookup> = GILOnceCell::new();

fn get_uuid_ob_pointer(py: Python) -> usize {
    let uuid_mod = py.import("uuid").expect("uuid to be importable");
    // get a uuid how orjson does it...
    let uuid_ob = uuid_mod
        .getattr("NAMESPACE_DNS")
        .expect("uuid.NAMESPACE_DNS to be available");
    let uuid_type = uuid_ob.get_type();

    uuid_type.as_type_ptr() as usize
}

impl PyTypeLookup {
    fn new(py: Python) -> Self {
        Self {
            none: PyNone::type_object_raw(py) as usize,
            // numeric types
            int: PyInt::type_object_raw(py) as usize,
            bool: PyBool::type_object_raw(py) as usize,
            float: PyFloat::type_object_raw(py) as usize,
            // string types
            string: PyString::type_object_raw(py) as usize,
            bytes: PyBytes::type_object_raw(py) as usize,
            bytearray: PyByteArray::type_object_raw(py) as usize,
            // sequence types
            list: PyList::type_object_raw(py) as usize,
            tuple: PyTuple::type_object_raw(py) as usize,
            // mapping types
            dict: PyDict::type_object_raw(py) as usize,
            // datetime types
            datetime: PyDateTime::type_object_raw(py) as usize,
            date: PyDate::type_object_raw(py) as usize,
            time: PyTime::type_object_raw(py) as usize,
            // uuid
            py_uuid: get_uuid_ob_pointer(py), // use uuid.NAMESPACE_DNS as a proxy for the uuid type
            ry_uuid: RyUuid::type_object_raw(py) as usize,
        }
    }

    pub fn cached(py: Python<'_>) -> &PyTypeLookup {
        TYPE_LOOKUP.get_or_init(py, || PyTypeLookup::new(py))
    }

    #[must_use]
    pub fn obtype(&self, ob: &Bound<'_, PyAny>) -> Option<PyObType> {
        let ob_type = ob.get_type_ptr() as usize;
        if ob_type == self.none {
            Some(PyObType::None)
        } else if ob_type == self.int {
            Some(PyObType::Int)
        } else if ob_type == self.bool {
            Some(PyObType::Bool)
        } else if ob_type == self.float {
            Some(PyObType::Float)
        } else if ob_type == self.string {
            Some(PyObType::String)
        } else if ob_type == self.bytes {
            Some(PyObType::Bytes)
        } else if ob_type == self.bytearray {
            Some(PyObType::ByteArray)
        } else if ob_type == self.list {
            Some(PyObType::List)
        } else if ob_type == self.tuple {
            Some(PyObType::Tuple)
        } else if ob_type == self.dict {
            Some(PyObType::Dict)
        } else if ob_type == self.datetime {
            Some(PyObType::DateTime)
        } else if ob_type == self.date {
            Some(PyObType::Date)
        } else if ob_type == self.time {
            Some(PyObType::Time)
        } else if ob_type == self.py_uuid {
            Some(PyObType::PyUuid)
        } else {
            None
        }
    }
}
pub struct SerializePyAny<'py> {
    obj: Bound<'py, PyAny>,
    none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeLookup,
}

impl<'py> SerializePyAny<'py> {
    #[must_use]
    pub fn new(py: Python<'py>, obj: Bound<'py, PyAny>, none_value: Option<&'py str>) -> Self {
        Self {
            obj,
            none_value,
            ob_type_lookup: PyTypeLookup::cached(py),
        }
    }

    fn with_obj(&self, obj: Bound<'py, PyAny>) -> Self {
        Self {
            obj,
            none_value: self.none_value,
            ob_type_lookup: self.ob_type_lookup,
        }
    }

    fn ser_dict<S: Serializer>(
        &self,
        map: &mut S::SerializeMap,
        dict_items: Vec<(Bound<'_, PyAny>, Bound<'_, PyAny>)>,
    ) -> Result<(), S::Error> {
        for (k, v) in dict_items {
            let key = mapping_key(&k)?;
            let value = self.with_obj(v);
            map.serialize_entry(key, &value)?;
        }
        Ok(())
    }
}

pub struct SerializePyString<'py> {
    obj: &'py Bound<'py, PyAny>,
    none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeLookup,
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
        let ob_type = self.obj.get_type_ptr() as usize;
        // ugly but this seems to be just marginally faster than a guarded match, also allows for custom cases
        // if we wanted to add them
        if ob_type == lookup.none {
            if let Some(none_value) = self.none_value {
                serializer.serialize_str(none_value)
            } else {
                // if no none_value is set, serialize as None
                serializer.serialize_none()
            }
        } else if ob_type == lookup.bool {
            let py_bool = self.obj.downcast::<PyBool>().map_err(map_py_err)?;
            let v: bool = py_bool.is_true();
            return serializer.serialize_bool(v);
        } else if ob_type == lookup.int {
            let py_int = self.obj.downcast::<PyInt>().map_err(map_py_err)?;
            let v: i64 = py_int.extract().map_err(map_py_err)?;
            return serializer.serialize_i64(v);
        } else if ob_type == lookup.float {
            // serialize!(f64)
            let py_float = self.obj.downcast::<PyFloat>().map_err(map_py_err)?;
            let v: f64 = py_float.extract().map_err(map_py_err)?;
            let a = serializer.serialize_f64(v);
            return a;
        } else if ob_type == lookup.string {
            let pystr_ser = SerializePyString {
                obj: &self.obj,
                none_value: self.none_value,
                ob_type_lookup: self.ob_type_lookup,
            };
            return pystr_ser.serialize(serializer);
            // let py_str: &Bound<'_, PyString> = self.obj.downcast().map_err(map_py_err)?;
            // let s = py_str.to_str().map_err(map_py_err)?;
            // return serializer.serialize_str(s);
        } else if ob_type == lookup.list {
            let py_list: &Bound<'_, PyList> = self.obj.downcast().map_err(map_py_err)?;
            let len = py_list.len();
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_list {
                // if self.none_value.is_some() || !element.is_none() {
                seq.serialize_element(&self.with_obj(element))?;
                // }
            }
            seq.end()
        } else if ob_type == lookup.tuple {
            let py_tuple: &Bound<'_, PyTuple> = self.obj.downcast().map_err(map_py_err)?;
            let len = py_tuple.len();
            let mut seq = serializer.serialize_seq(Some(len))?;
            for element in py_tuple {
                // if self.none_value.is_some() || !element.is_none() {
                seq.serialize_element(&self.with_obj(element))?;
                // }
            }
            seq.end()
        } else if ob_type == lookup.dict {
            let py_dict: &Bound<'_, PyDict> = self.obj.downcast().map_err(map_py_err)?;
            let mut m = serializer.serialize_map(Some(py_dict.len()))?;
            for (k, v) in py_dict {
                m.serialize_entry(mapping_key(&k)?, &self.with_obj(v))?;
            }
            m.end()
        } else if ob_type == lookup.datetime {
            let py_dt: &Bound<'_, PyDateTime> = self.obj.downcast().map_err(map_py_err)?;
            let dt_pystr = py_dt.str().map_err(map_py_err)?;
            let dt_str = dt_pystr.to_str().map_err(map_py_err)?;
            // TODO: use jiff to do all the date-time formatting
            let iso_str = dt_str.replacen("+00:00", "Z", 1).replace(' ', "T");
            serializer.serialize_str(iso_str.as_ref())
        } else if ob_type == lookup.date {
            let py_date: &Bound<'_, PyDate> = self.obj.downcast().map_err(map_py_err)?;
            let date_pystr = py_date.str().map_err(map_py_err)?;
            let date_str = date_pystr.to_str().map_err(map_py_err)?;
            serializer.serialize_str(date_str)
        } else if ob_type == lookup.time {
            let py_time: &Bound<'_, PyTime> = self.obj.downcast().map_err(map_py_err)?;
            let time_pystr = py_time.str().map_err(map_py_err)?;
            let time_str = time_pystr.to_str().map_err(map_py_err)?;
            serializer.serialize_str(time_str)
        } else if ob_type == lookup.bytes || ob_type == lookup.bytearray {
            serialize!(&[u8])
        } else if ob_type == lookup.ry_uuid {
            let ry_uu = self.obj.downcast::<RyUuid>().map_err(map_py_err)?;
            let uu = ry_uu.borrow().0;
            serializer.serialize_str(&uu.hyphenated().to_string())
        } else if ob_type == lookup.py_uuid {
            let uu = ryo3_uuid::CPythonUuid::extract_bound(&self.obj)
                // .map_err(|e| serde_err!("Failed to extract CPythonUuid: {}", e))
                .map(|u| uuid::Uuid::from(u))
                .map_err(|e| map_py_err(e))?;
            serializer.serialize_str(&uu.hyphenated().to_string())
        } else {
            serde_err!("{} is not JSON-serializable", any_repr(&self.obj))
        }
    }
}

fn map_py_err<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}

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

fn pystr(obj: &Bound<'_, PyAny>) -> PyResult<String> {
    // call the `__str__` fn on the object
    // call '__str__' on the object and convert it to a string

    // let a=
    obj.str().map(|s| s.extract())?
}

fn any_repr(obj: &Bound<'_, PyAny>) -> String {
    let typ = obj.get_type();
    let name = typ
        .name()
        .unwrap_or_else(|_| PyString::new(obj.py(), "unknown"));
    match obj.repr() {
        Ok(repr) => format!("{repr} ({name})"),
        Err(_) => name.to_string(),
    }
}
