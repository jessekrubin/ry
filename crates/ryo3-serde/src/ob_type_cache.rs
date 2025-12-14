use crate::ob_type::PyObType;
use crate::ser::dataclass::is_dataclass;
use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
use pyo3::sync::PyOnceLock;
use pyo3::types::{
    PyBool, PyByteArray, PyBytes, PyDate, PyDateTime, PyDelta, PyDict, PyEllipsis, PyFloat,
    PyFrozenSet, PyInt, PyList, PyMemoryView, PyNone, PySet, PyString, PyTime, PyTuple,
};
use pyo3::{Borrowed, Bound, PyAny, PyTypeInfo, Python};

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
    // __ryo3_std__
    #[cfg(feature = "ryo3-std")]
    pub ry_duration: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_ip_addr: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_ipv4_addr: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_ipv6_addr: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_socket_addr: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_socket_addr_v4: usize,
    #[cfg(feature = "ryo3-std")]
    pub ry_socket_addr_v6: usize,
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

static TYPE_LOOKUP: PyOnceLock<PyTypeCache> = PyOnceLock::new();

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
            #[cfg(feature = "ryo3-std")]
            ry_duration: ryo3_std::time::PyDuration::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_ip_addr: ryo3_std::net::PyIpAddr::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_ipv4_addr: ryo3_std::net::PyIpv4Addr::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_ipv6_addr: ryo3_std::net::PyIpv6Addr::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_socket_addr: ryo3_std::net::PySocketAddr::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_socket_addr_v4: ryo3_std::net::PySocketAddrV4::type_object_raw(py) as usize,
            #[cfg(feature = "ryo3-std")]
            ry_socket_addr_v6: ryo3_std::net::PySocketAddrV6::type_object_raw(py) as usize,
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

    #[inline]
    pub(crate) fn cached(py: Python<'_>) -> &Self {
        TYPE_LOOKUP.get_or_init(py, || Self::new(py))
    }

    #[must_use]
    #[inline]
    pub(crate) fn obtype(&self, ob: Borrowed<'_, '_, PyAny>) -> PyObType {
        self.ptr2type(ob.get_type_ptr() as usize, ob)
    }

    #[must_use]
    #[inline]
    pub(crate) fn obtype_key(&self, ob: Borrowed<'_, '_, PyAny>) -> PyObType {
        self.ptr2type_key(ob.get_type_ptr() as usize)
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

macro_rules! py_obj_ptr {
    ($self:ident, $ptr:ident, $field:ident, $Variant:ident) => {
        if $ptr == $self.$field {
            return PyObType::$Variant;
        }
    };
}

macro_rules! py_obj_ptr_feat {
    ($self:ident, $ptr:ident, $feat:literal, $field:ident, $Variant:ident) => {
        #[cfg(feature = $feat)]
        {
            py_obj_ptr!($self, $ptr, $field, $Variant);
        }
    };
}

impl PyTypeCache {
    #[inline]
    pub(crate) fn ptr2type(&self, ptr: usize, ob: Borrowed<'_, '_, PyAny>) -> PyObType {
        // --- das builtins ---
        py_obj_ptr!(self, ptr, string, String);
        py_obj_ptr!(self, ptr, int, Int);
        py_obj_ptr!(self, ptr, bool, Bool);
        py_obj_ptr!(self, ptr, none, None);
        py_obj_ptr!(self, ptr, float, Float);
        py_obj_ptr!(self, ptr, list, List);
        py_obj_ptr!(self, ptr, dict, Dict);
        py_obj_ptr!(self, ptr, datetime, DateTime);
        py_obj_ptr!(self, ptr, tuple, Tuple);
        py_obj_ptr!(self, ptr, date, Date);
        py_obj_ptr!(self, ptr, time, Time);
        py_obj_ptr!(self, ptr, timedelta, Timedelta);
        py_obj_ptr!(self, ptr, bytes, Bytes);
        py_obj_ptr!(self, ptr, py_uuid, PyUuid);
        py_obj_ptr!(self, ptr, bytearray, ByteArray);
        py_obj_ptr!(self, ptr, memoryview, MemoryView);
        py_obj_ptr!(self, ptr, ellipsis, Ellipsis);
        py_obj_ptr!(self, ptr, frozenset, FrozenSet);
        py_obj_ptr!(self, ptr, set, Set);

        // --- ryo3-* features ---
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_duration, PyDuration);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_ip_addr, PyIpAddr);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_ipv4_addr, PyIpv4Addr);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_ipv6_addr, PyIpv6Addr);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_socket_addr, PySocketAddr);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_socket_addr_v4, PySocketAddrV4);
        py_obj_ptr_feat!(self, ptr, "ryo3-std", ry_socket_addr_v6, PySocketAddrV6);

        py_obj_ptr_feat!(self, ptr, "ryo3-uuid", ry_uuid, RyUuid);

        py_obj_ptr_feat!(self, ptr, "ryo3-ulid", ry_ulid, RyUlid);

        py_obj_ptr_feat!(self, ptr, "ryo3-url", ry_url, RyUrl);

        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_date, RyDate);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_datetime, RyDateTime);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_signed_duration, RySignedDuration);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_time, RyTime);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_timespan, RyTimeSpan);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_timestamp, RyTimestamp);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_timezone, RyTimeZone);
        py_obj_ptr_feat!(self, ptr, "ryo3-jiff", ry_zoned, RyZoned);

        py_obj_ptr_feat!(self, ptr, "ryo3-http", ry_http_status, RyHttpStatus);
        py_obj_ptr_feat!(self, ptr, "ryo3-http", ry_headers, RyHeaders);

        // structural check last
        if is_dataclass(ob) {
            return PyObType::Dataclass;
        }
        PyObType::Unknown
    }

    #[inline]
    pub(crate) fn ptr2type_key(&self, ptr: usize) -> PyObType {
        // --- das builtins ---
        py_obj_ptr!(self, ptr, string, String);
        py_obj_ptr!(self, ptr, bool, Bool);
        PyObType::Unknown
    }
}
