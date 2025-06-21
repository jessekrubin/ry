#![cfg(feature = "ry")]
use crate::errors::pyerr2sererr;

use crate::py_serialize::SerializePyAny;
use pyo3::prelude::*;
use serde::ser::Serialize;

macro_rules! ry_type_serializers {
    ( $(
        $( #[$meta:meta] )*          // feature gate(s)
        $fn_name:ident => $ty:path   // helper name  and  PyO3 type
    ; )+ $(;)?) => {
        $(
            $( #[$meta] )*
            #[inline]
            pub(crate) fn $fn_name<S>(
                ser: &SerializePyAny<'_>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let ob = ser
                    .obj
                    .downcast::<$ty>()
                    .map_err(pyerr2sererr)?;
                ob.borrow().serialize(serializer)
            }
        )+
    }
}

ry_type_serializers! {
    // ulid
    #[cfg(feature = "ryo3-ulid")]  ry_ulid => ryo3_ulid::PyUlid;
    // url
    #[cfg(feature = "ryo3-url")]   ry_url => ryo3_url::PyUrl;
    // uuid
    #[cfg(feature = "ryo3-uuid")]  ry_uuid => ryo3_uuid::PyUuid;
    // jiff
    #[cfg(feature = "ryo3-jiff")]  ry_time => ryo3_jiff::RyTime;
    #[cfg(feature = "ryo3-jiff")]  ry_span => ryo3_jiff::RySpan;
    #[cfg(feature = "ryo3-jiff")]  ry_date => ryo3_jiff::RyDate;
    #[cfg(feature = "ryo3-jiff")]  ry_datetime => ryo3_jiff::RyDateTime;
    #[cfg(feature = "ryo3-jiff")]  ry_signed_duration => ryo3_jiff::RySignedDuration;
    #[cfg(feature = "ryo3-jiff")]  ry_timestamp => ryo3_jiff::RyTimestamp;
    #[cfg(feature = "ryo3-jiff")]  ry_timezone => ryo3_jiff::RyTimeZone;
    #[cfg(feature = "ryo3-jiff")]  ry_zoned => ryo3_jiff::RyZoned;
    // http
    #[cfg(feature = "ryo3-http")]  ry_headers => ryo3_http::PyHeaders;
    #[cfg(feature = "ryo3-http")]  ry_http_status => ryo3_http::PyHttpStatus;
}
