use pyo3::ffi::setentry;
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::sync::GILOnceCell;
use pyo3::types::{
    PyBool, PyByteArray, PyBytes, PyDate, PyDateTime, PyDict, PyFloat, PyFrozenSet, PyInt, PyList,
    PyNone, PySet, PyString, PyTime, PyTuple,
};
use pyo3::{Bound, PyAny, PyTypeInfo, Python};
use ryo3_uuid::PyUuid as RyUuid;

pub(crate) enum PyObType {
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
    Set,
    Frozenset,
    DateTime,
    Date,
    Time,
    PyUuid, // not used yet
    // ry-types
    RyUuid,
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PyTypeCache {
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
    // set & frozenset
    pub set: usize,
    pub frozenset: usize,
    // datetime types
    pub datetime: usize,
    pub date: usize,
    pub time: usize,
    // uuid
    pub py_uuid: usize,
    pub ry_uuid: usize, // not used yet
}

static TYPE_LOOKUP: GILOnceCell<PyTypeCache> = GILOnceCell::new();

impl PyTypeCache {
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
            // set & frozenset\
            set: PySet::type_object_raw(py) as usize,
            frozenset: PyFrozenSet::type_object_raw(py) as usize,
            // datetime types
            datetime: PyDateTime::type_object_raw(py) as usize,
            date: PyDate::type_object_raw(py) as usize,
            time: PyTime::type_object_raw(py) as usize,
            // uuid
            py_uuid: get_uuid_ob_pointer(py), // use uuid.NAMESPACE_DNS as a proxy for the uuid type
            ry_uuid: RyUuid::type_object_raw(py) as usize,
        }
    }

    pub(crate) fn cached(py: Python<'_>) -> &PyTypeCache {
        TYPE_LOOKUP.get_or_init(py, || PyTypeCache::new(py))
    }

    pub(crate) fn ptr2type(&self, ptr: usize) -> Option<PyObType> {
        if ptr == self.none {
            Some(PyObType::None)
        } else if ptr == self.int {
            Some(PyObType::Int)
        } else if ptr == self.bool {
            Some(PyObType::Bool)
        } else if ptr == self.float {
            Some(PyObType::Float)
        } else if ptr == self.string {
            Some(PyObType::String)
        } else if ptr == self.bytes {
            Some(PyObType::Bytes)
        } else if ptr == self.bytearray {
            Some(PyObType::ByteArray)
        } else if ptr == self.list {
            Some(PyObType::List)
        } else if ptr == self.tuple {
            Some(PyObType::Tuple)
        } else if ptr == self.dict {
            Some(PyObType::Dict)
        } else if ptr == self.datetime {
            Some(PyObType::DateTime)
        } else if ptr == self.date {
            Some(PyObType::Date)
        } else if ptr == self.time {
            Some(PyObType::Time)
        } else if ptr == self.py_uuid {
            Some(PyObType::PyUuid)
        } else if ptr == self.ry_uuid {
            Some(PyObType::RyUuid)
        } else if ptr == self.set {
            Some(PyObType::Set)
        } else if ptr == self.frozenset {
            Some(PyObType::Frozenset)
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn obtype(&self, ob: &Bound<'_, PyAny>) -> Option<PyObType> {
        self.ptr2type(ob.get_type_ptr() as usize)
    }
}

fn get_uuid_ob_pointer(py: Python) -> usize {
    let uuid_mod = py.import("uuid").expect("uuid to be importable");
    // get a uuid how orjson does it...
    let uuid_ob = uuid_mod
        .getattr("NAMESPACE_DNS")
        .expect("uuid.NAMESPACE_DNS to be available");
    let uuid_type = uuid_ob.get_type();

    uuid_type.as_type_ptr() as usize
}
