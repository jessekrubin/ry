use std::fmt;

use pyo3::prelude::*;
use serde::ser::{Error as SerError, Serialize, SerializeMap, SerializeSeq, Serializer};

use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::{
    PyBool, PyByteArray, PyBytes, PyDate, PyDateTime, PyDict, PyFloat, PyInt, PyList, PyNone,
    PyString, PyTime, PyTuple,
};
use pyo3::PyTypeInfo;

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
}

static TYPE_LOOKUP: GILOnceCell<PyTypeLookup> = GILOnceCell::new();

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
        }
    }

    pub fn cached(py: Python<'_>) -> &PyTypeLookup {
        TYPE_LOOKUP.get_or_init(py, || PyTypeLookup::new(py))
    }
}
pub struct SerializePyObject<'py> {
    obj: Bound<'py, PyAny>,
    none_value: Option<&'py str>,
    ob_type_lookup: &'py PyTypeLookup,
}

impl<'py> SerializePyObject<'py> {
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
            let key = table_key(&k, self.none_value)?;
            let value = self.with_obj(v);
            map.serialize_entry(key, &value)?;
        }
        Ok(())
    }
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

impl Serialize for SerializePyObject<'_> {
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
            serializer.serialize_str(self.none_value.unwrap_or("null"))
        } else if ob_type == lookup.bool {
            serialize!(bool)
        } else if ob_type == lookup.int {
            serialize!(i64)
        } else if ob_type == lookup.float {
            serialize!(f64)
        } else if ob_type == lookup.list {
            let py_list: &Bound<'_, PyList> = self.obj.downcast().map_err(map_py_err)?;
            let len = py_list.len();
            let mut seq = serializer.serialize_seq(
                Some(len)
            )?;
            for element in py_list {
                if self.none_value.is_some() || !element.is_none() {
                    seq.serialize_element(&self.with_obj(element))?
                }
            }
            seq.end()
        } else if ob_type == lookup.tuple {
            let py_tuple: &Bound<'_, PyTuple> = self.obj.downcast().map_err(map_py_err)?;
            let len = py_tuple.len();
            let mut seq = serializer.serialize_seq(
                Some(len)
            )?;
            for element in py_tuple {
                if self.none_value.is_some() || !element.is_none() {
                    seq.serialize_element(&self.with_obj(element))?
                }
            }
            seq.end()
        } else if ob_type == lookup.string {
            let py_str: &Bound<'_, PyString> = self.obj.downcast().map_err(map_py_err)?;
            let s = py_str.to_str().map_err(map_py_err)?;
            serializer.serialize_str(s)

        } else if ob_type == lookup.dict {
            let py_dict: &Bound<'_, PyDict> = self.obj.downcast().map_err(map_py_err)?;
            let mut m = serializer.serialize_map(Some(py_dict.len()))?;
            for (k, v) in py_dict {
                m.serialize_entry (
                    table_key(&k, self.none_value)?,
                    &self.with_obj(v),
                )?;
            }
            m.end()
        }
        // else if ob_type == lookup.datetime {
        //     let py_dt: &Bound<'_, PyDateTime> = self.obj.downcast().map_err(map_py_err)?;
        //     let dt_pystr = py_dt.str().map_err(map_py_err)?;
        //     let dt_str = dt_pystr.to_str().map_err(map_py_err)?;
        //     let iso_str = dt_str.replacen("+00:00", "Z", 1);
        //     match Datetime::from_str(&iso_str) {
        //         Ok(dt) => dt.serialize(serializer),
        //         Err(e) => serde_err!(
        //             "unable to convert datetime string to TOML datetime object {:?}",
        //             e
        //         ),
        //     }
        // } else if ob_type == lookup.date {
        //     let py_date: &Bound<'_, PyDate> = self.obj.downcast().map_err(map_py_err)?;
        //     let date_pystr = py_date.str().map_err(map_py_err)?;
        //     let date_str = date_pystr.to_str().map_err(map_py_err)?;
        //     match Datetime::from_str(date_str) {
        //         Ok(dt) => dt.serialize(serializer),
        //         Err(e) => serde_err!("unable to convert date string to TOML date object {:?}", e),
        //     }
        // } else if ob_type == lookup.time {
        //     let py_time: &Bound<'_, PyTime> = self.obj.downcast().map_err(map_py_err)?;
        //     let time_pystr = py_time.str().map_err(map_py_err)?;
        //     let time_str = time_pystr.to_str().map_err(map_py_err)?;
        //     match Datetime::from_str(time_str) {
        //         Ok(dt) => dt.serialize(serializer),
        //         Err(e) => serde_err!("unable to convert time string to TOML time object {:?}", e),
        //     }
        // }
        else if ob_type == lookup.bytes || ob_type == lookup.bytearray {
            serialize!(&[u8])
        } else {
            serde_err!("{} is not serializable to TOML", any_repr(&self.obj))
        }
    }
}

fn map_py_err<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}

fn table_key<'py, E: SerError>(
    key: &'py Bound<'py, PyAny>,
    none_value: Option<&'py str>,
) -> Result<&'py str, E> {
    if let Ok(py_string) = key.downcast::<PyString>() {
        py_string.to_str().map_err(map_py_err)
    } else if key.is_none() {
        Ok(none_value.unwrap_or("null"))
    } else if let Ok(key) = key.extract::<bool>() {
        Ok(if key { "true" } else { "false" })
    } else {
        let key_repr = any_repr(key);
        serde_err!("{} is not JSON-serializable as map-key", key_repr)
    }
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
