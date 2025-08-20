use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::sync::GILOnceCell;
use pyo3::types::{
    PyBool, PyByteArray, PyBytes, PyDate, PyDateTime, PyDelta, PyDict, PyEllipsis, PyFloat,
    PyFrozenSet, PyInt, PyList, PyMemoryView, PyNone, PySet, PyString, PyTime, PyTuple,
};
use pyo3::{Bound, PyAny, PyTypeInfo, Python};

pub(crate) enum PyObType {
    Unknown,
    // ========================================================================
    // PY-TYPES
    // ========================================================================
    // ------------------------------------------------------------------------
    // SINGLETONS
    // ------------------------------------------------------------------------
    None,
    Ellipsis,
    // ------------------------------------------------------------------------
    // BUILTINS
    // ------------------------------------------------------------------------
    Int,
    Bool,
    Float,
    String,
    Bytes,
    ByteArray,
    MemoryView,
    List,
    Tuple,
    Dict,
    Set,
    FrozenSet,
    // ------------------------------------------------------------------------
    // PY-DATETIME
    // ------------------------------------------------------------------------
    DateTime,
    Date,
    Time,
    Timedelta,

    // ------------------------------------------------------------------------
    // UUID
    // ------------------------------------------------------------------------
    PyUuid,
    // ========================================================================
    // RY-TYPES
    // ========================================================================
    // -----------------------------------------------------------------------
    // UUID
    // -----------------------------------------------------------------------
    #[cfg(feature = "ryo3-uuid")]
    RyUuid,
    // -----------------------------------------------------------------------
    // ULID
    // -----------------------------------------------------------------------
    #[cfg(feature = "ryo3-ulid")]
    RyUlid, // ulid is treated as a uuid for now
    // -----------------------------------------------------------------------
    // URL
    // -----------------------------------------------------------------------
    #[cfg(feature = "ryo3-url")]
    RyUrl,
    // -----------------------------------------------------------------------
    // HTTP
    // -----------------------------------------------------------------------
    #[cfg(feature = "ryo3-http")]
    RyHttpStatus,
    #[cfg(feature = "ryo3-http")]
    RyHeaders,
    // -----------------------------------------------------------------------
    // JIFF
    // -----------------------------------------------------------------------
    #[cfg(feature = "ryo3-jiff")]
    RyDate,
    #[cfg(feature = "ryo3-jiff")]
    RyDateTime,
    #[cfg(feature = "ryo3-jiff")]
    RySignedDuration,
    #[cfg(feature = "ryo3-jiff")]
    RyTime,
    #[cfg(feature = "ryo3-jiff")]
    RyTimeSpan,
    #[cfg(feature = "ryo3-jiff")]
    RyTimeZone,
    #[cfg(feature = "ryo3-jiff")]
    RyTimestamp,
    #[cfg(feature = "ryo3-jiff")]
    RyZoned,
}

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PyTypeCache {
    pub none: usize,
    pub ellipsis: usize,
    // numeric types
    pub int: usize,
    pub bool: usize,
    pub float: usize,
    // string types
    pub string: usize,
    // bytes types
    pub bytes: usize,
    pub bytearray: usize,
    pub memoryview: usize,
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
    pub timedelta: usize,
    // uuid
    pub py_uuid: usize,
    // ------------------------------------------------------------------------
    // RY-TYPES
    // ------------------------------------------------------------------------
    // __ryo3_uuid__
    #[cfg(feature = "ryo3-uuid")]
    pub ry_uuid: usize,
    // __ryo3_ulid__
    #[cfg(feature = "ryo3-ulid")]
    pub ry_ulid: usize,
    // __ryo3_url__
    #[cfg(feature = "ryo3-url")]
    pub ry_url: usize,
    // __ryo3_http__
    #[cfg(feature = "ryo3-http")]
    pub ry_http_status: usize,
    #[cfg(feature = "ryo3-http")]
    pub ry_headers: usize,
    // __ryo3_jiff__
    #[cfg(feature = "ryo3-jiff")]
    pub ry_date: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_datetime: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_signed_duration: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_time: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_timespan: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_timestamp: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_timezone: usize,
    #[cfg(feature = "ryo3-jiff")]
    pub ry_zoned: usize,
}

static TYPE_LOOKUP: GILOnceCell<PyTypeCache> = GILOnceCell::new();

impl PyTypeCache {
    fn new(py: Python) -> Self {
        Self {
            none: PyNone::type_object_raw(py) as usize,
            ellipsis: PyEllipsis::type_object_raw(py) as usize,
            // numeric types
            int: PyInt::type_object_raw(py) as usize,
            bool: PyBool::type_object_raw(py) as usize,
            float: PyFloat::type_object_raw(py) as usize,
            // string types
            string: PyString::type_object_raw(py) as usize,
            // bytes types
            bytes: PyBytes::type_object_raw(py) as usize,
            bytearray: PyByteArray::type_object_raw(py) as usize,
            memoryview: PyMemoryView::type_object_raw(py) as usize, // memoryview is a generic type, not a specific one
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
            timedelta: PyDelta::type_object_raw(py) as usize,
            // uuid
            py_uuid: get_uuid_ob_pointer(py), // use uuid.NAMESPACE_DNS as a proxy for the uuid type

            // ----------------------------------------------------------------
            // RY-TYPES
            // ----------------------------------------------------------------
            // ----------------------------------------------------------------
            // UUID
            // ----------------------------------------------------------------
            #[cfg(feature = "ryo3-uuid")]
            ry_uuid: ryo3_uuid::PyUuid::type_object_raw(py) as usize,
            // ----------------------------------------------------------------
            // ULID
            // ----------------------------------------------------------------
            #[cfg(feature = "ryo3-ulid")]
            ry_ulid: ryo3_ulid::PyUlid::type_object_raw(py) as usize,
            // ----------------------------------------------------------------
            // URL
            // ----------------------------------------------------------------
            #[cfg(feature = "ryo3-url")]
            ry_url: ryo3_url::PyUrl::type_object_raw(py) as usize,
            // ----------------------------------------------------------------
            // HTTP
            // ----------------------------------------------------------------
            #[cfg(feature = "ryo3-http")]
            ry_http_status: ryo3_http::PyHttpStatus::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-http")]
            ry_headers: ryo3_http::PyHeaders::type_object_raw(py) as usize,
            // ----------------------------------------------------------------
            // JIFF
            // ----------------------------------------------------------------
            #[cfg(feature = "ryo3-jiff")]
            ry_date: ryo3_jiff::RyDate::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_datetime: ryo3_jiff::RyDateTime::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_signed_duration: ryo3_jiff::RySignedDuration::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_time: ryo3_jiff::RyTime::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_timespan: ryo3_jiff::RySpan::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_timestamp: ryo3_jiff::RyTimestamp::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_timezone: ryo3_jiff::RyTimeZone::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-jiff")]
            ry_zoned: ryo3_jiff::RyZoned::type_object_raw(py) as usize,
        }
    }

    pub(crate) fn cached(py: Python<'_>) -> &Self {
        TYPE_LOOKUP.get_or_init(py, || Self::new(py))
    }

    pub(crate) fn ptr2type(&self, ptr: usize) -> PyObType {
        match ptr {
            x if x == self.none => PyObType::None,
            x if x == self.ellipsis => PyObType::Ellipsis,
            x if x == self.int => PyObType::Int,
            x if x == self.bool => PyObType::Bool,
            x if x == self.float => PyObType::Float,
            x if x == self.string => PyObType::String,
            x if x == self.bytes => PyObType::Bytes,
            x if x == self.bytearray => PyObType::ByteArray,
            x if x == self.memoryview => PyObType::MemoryView,
            x if x == self.list => PyObType::List,
            x if x == self.tuple => PyObType::Tuple,
            x if x == self.dict => PyObType::Dict,
            x if x == self.set => PyObType::Set,
            x if x == self.frozenset => PyObType::FrozenSet,
            // py-datetime
            x if x == self.datetime => PyObType::DateTime,
            x if x == self.date => PyObType::Date,
            x if x == self.time => PyObType::Time,
            x if x == self.timedelta => PyObType::Timedelta,
            // py-uuid
            x if x == self.py_uuid => PyObType::PyUuid,
            // =================================================================
            // RY-TYPES
            // =================================================================
            // -----------------------------------------------------------------
            // UUID
            // -----------------------------------------------------------------
            #[cfg(feature = "ryo3-uuid")]
            x if x == self.ry_uuid => PyObType::RyUuid,
            // -----------------------------------------------------------------
            // ULID:wq
            // -----------------------------------------------------------------
            #[cfg(feature = "ryo3-ulid")]
            x if x == self.ry_ulid => PyObType::RyUlid,
            // -----------------------------------------------------------------
            // URL
            // -----------------------------------------------------------------
            #[cfg(feature = "ryo3-url")]
            x if x == self.ry_url => PyObType::RyUrl,

            // -----------------------------------------------------------------
            // HTTP
            // -----------------------------------------------------------------
            #[cfg(feature = "ryo3-http")]
            x if x == self.ry_http_status => PyObType::RyHttpStatus,
            #[cfg(feature = "ryo3-http")]
            x if x == self.ry_headers => PyObType::RyHeaders,

            // -----------------------------------------------------------------
            // JIFF
            // -----------------------------------------------------------------
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_date => PyObType::RyDate,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_datetime => PyObType::RyDateTime,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_signed_duration => PyObType::RySignedDuration,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_time => PyObType::RyTime,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_timespan => PyObType::RyTimeSpan,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_timestamp => PyObType::RyTimestamp,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_timezone => PyObType::RyTimeZone,
            #[cfg(feature = "ryo3-jiff")]
            x if x == self.ry_zoned => PyObType::RyZoned,

            _ => PyObType::Unknown,
        }
    }

    #[must_use]
    pub(crate) fn obtype(&self, ob: &Bound<'_, PyAny>) -> PyObType {
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
