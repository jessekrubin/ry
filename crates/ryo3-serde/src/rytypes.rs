#![cfg(feature = "ry")]
use crate::errors::pyerr2sererr;

use crate::py_serialize::SerializePyAny;
use pyo3::prelude::*;
use serde::ser::Serialize;

// #[cfg(feature = "ryo3-uuid")]
// #[inline]
// pub(crate) fn ry_uuid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let ry_uu = ser
//         .obj
//         .downcast::<ryo3_uuid::PyUuid>()
//         .map_err(pyerr2sererr)?;
//     ry_uu.borrow().serialize(serializer)
// }
//
// #[cfg(feature = "ryo3-ulid")]
// #[inline]
// pub(crate) fn ry_ulid<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let ryob = ser
//         .obj
//         .downcast::<ryo3_ulid::PyUlid>()
//         .map_err(pyerr2sererr)?;
//     ryob.borrow().serialize(serializer)
// }
//
// // jiff types...
// #[cfg(feature = "ryo3-jiff")]
// #[inline]
// pub(crate) fn ry_time<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let jiff = ser
//         .obj
//         .downcast::<ryo3_jiff::RyTime>()
//         .map_err(pyerr2sererr)?;
//     jiff.borrow().serialize(serializer)
// }
// #[cfg(feature = "ryo3-jiff")]
// #[inline]
// pub(crate) fn ry_span<S>(ser: &SerializePyAny<'_>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let jiff = ser
//         .obj
//         .downcast::<ryo3_jiff::RySpan>()
//         .map_err(pyerr2sererr)?;
//     jiff.borrow().serialize(serializer)
// }
macro_rules! ry_helpers {
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
ry_helpers! {
    #[cfg(feature = "ryo3-uuid")]  ry_uuid => ryo3_uuid::PyUuid;
    #[cfg(feature = "ryo3-ulid")]  ry_ulid => ryo3_ulid::PyUlid;

    // jiff types
    #[cfg(feature = "ryo3-jiff")]  ry_time => ryo3_jiff::RyTime;
    #[cfg(feature = "ryo3-jiff")]  ry_span => ryo3_jiff::RySpan;
    #[cfg(feature = "ryo3-jiff")]  ry_date => ryo3_jiff::RyDate;
    #[cfg(feature = "ryo3-jiff")]  ry_datetime => ryo3_jiff::RyDateTime;
    #[cfg(feature = "ryo3-jiff")]  ry_signed_duration => ryo3_jiff::RySignedDuration;
    #[cfg(feature = "ryo3-jiff")]  ry_timestamp => ryo3_jiff::RyTimestamp;
    #[cfg(feature = "ryo3-jiff")]  ry_zoned => ryo3_jiff::RyZoned;
}
