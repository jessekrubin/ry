//! Serializer(s) implementation(s) for ry-types
//!
//! As noted below this module was originally written by fugue-state-jesse,
//! and I (non-fugue-state-jesse who is writing this) find the macro impossible
//! to understand. Fugue-state-jesse doesn't use LLMs, as he has a wildly
//! more advanced understanding of almost all things programming than I do.
//!
//! Summoning fugue-state-jesse is not something that can be done at will; if
//! it were, I would do it all the time.
//!
//! TODO (fugue-state-jesse):
//!     if/when you revisit this file, idk what is going on here, chad-gpt also
//!     doesn't know, can you please refactor this to be saner?
//!     --yourself
//!
//!     Response:
//!       Hello normal-jesse,
//!
//!       Perhaps. What I can do is make it way more complicated/unreadable..
//!
//!       Regards,
//!       fugue-state-jesse
//!
#![cfg(feature = "ry")]
use crate::errors::pyerr2sererr;
use pyo3::prelude::*;
use serde::ser::Serialize;

// THE FOLLOWING MACRO WAS WRITTEN BY FUGUE-STATE-JESSE (NOT AN LLM).
// I (NORMAL JESSE) HAVE NO CLUE HOW IT WORKS :(
// UNFORTUNATELY, FUGUE-STATE-JESSE ONLY APPEARS AT RANDOM, SO YOU HAVE TO ASK
// HIM IF/WHEN HE RE-APPEARS...
// macro_rules! ry_type_serializers {
//     ( $(
//         $( #[$meta:meta] )*          // feature gate(s)
//         $fn_name:ident => $ty:path   // helper name  and  PyO3 type
//     ; )+ $(;)?) => {
//         $(
//             $( #[$meta] )*
//             #[inline]
//             pub(crate) fn $fn_name<S>(
//                 ser: &SerializePyAny<'_>,
//                 serializer: S,
//             ) -> Result<S::Ok, S::Error>
//             where
//                 S: serde::Serializer,
//             {
//                 let ob = ser
//                     .obj
//                     .cast::<$ty>()
//                     .map_err(pyerr2sererr)?;
//                 ob.get().serialize(serializer)
//             }
//         )+
//     }
// }

// ry_type_serializers! {
//     // http
//     #[cfg(feature = "ryo3-http")]  ry_headers => ryo3_http::PyHeaders;
//     #[cfg(feature = "ryo3-http")]  ry_http_status => ryo3_http::PyHttpStatus;
//     // jiff
//     #[cfg(feature = "ryo3-jiff")]  ry_date => ryo3_jiff::RyDate;
//     #[cfg(feature = "ryo3-jiff")]  ry_datetime => ryo3_jiff::RyDateTime;
//     #[cfg(feature = "ryo3-jiff")]  ry_signed_duration => ryo3_jiff::RySignedDuration;
//     #[cfg(feature = "ryo3-jiff")]  ry_span => ryo3_jiff::RySpan;
//     #[cfg(feature = "ryo3-jiff")]  ry_time => ryo3_jiff::RyTime;
//     #[cfg(feature = "ryo3-jiff")]  ry_timestamp => ryo3_jiff::RyTimestamp;
//     #[cfg(feature = "ryo3-jiff")]  ry_timezone => ryo3_jiff::RyTimeZone;
//     #[cfg(feature = "ryo3-jiff")]  ry_zoned => ryo3_jiff::RyZoned;
//     // ulid
//     #[cfg(feature = "ryo3-ulid")]  ry_ulid => ryo3_ulid::PyUlid;
//     // url
//     #[cfg(feature = "ryo3-url")]   ry_url => ryo3_url::PyUrl;
//     // uuid
//     #[cfg(feature = "ryo3-uuid")]  ry_uuid => ryo3_uuid::PyUuid;
// }

macro_rules! ry_type_serializer_struct {
    (
        $( #[$meta:meta] )*
        $name:ident => $ty:path
    ) => (
        $(#[$meta])*
        pub(crate) struct $name<'py>{
            ob:  &'py Bound<'py, PyAny>
        }

        $(#[$meta])*
        impl<'py> $name<'py> {
            #[inline]
            pub(crate) fn new(obj: &'py Bound<'py, PyAny>) -> Self {
                Self {
                    ob: obj,
                }
            }
        }

        $(#[$meta])*
        impl<'py> Serialize for $name<'py> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let ob = self
                    .ob
                    .cast_exact::<$ty>()
                    .map_err(pyerr2sererr)?;
                ob.get().serialize(serializer)
            }
        }
    );
}

// ===========================================================================
// HTTP
// ===========================================================================
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-http")]
    PyHeadersSerializer => ryo3_http::PyHeaders
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-http")]
    PyHttpStatusSerializer => ryo3_http::PyHttpStatus
);

// ===========================================================================
// JIFF
// ===========================================================================
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyDateSerializer => ryo3_jiff::RyDate
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyDateTimeSerializer => ryo3_jiff::RyDateTime
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RySignedDurationSerializer => ryo3_jiff::RySignedDuration
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RySpanSerializer => ryo3_jiff::RySpan
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyTimeSerializer => ryo3_jiff::RyTime
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyTimestampSerializer => ryo3_jiff::RyTimestamp
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyTimeZoneSerializer => ryo3_jiff::RyTimeZone
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-jiff")]
    RyZonedSerializer => ryo3_jiff::RyZoned
);
// ===========================================================================
// ULID
// ===========================================================================
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-ulid")]
    PyUlidSerializer => ryo3_ulid::PyUlid
);
// ===========================================================================
// URL
// ===========================================================================
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-url")]
    PyUrlSerializer => ryo3_url::PyUrl
);
// ===========================================================================
// UUID
// ===========================================================================
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-uuid")]
    PyUuidSerializer => ryo3_uuid::PyUuid
);

// ===========================================================================
// STD
// ===========================================================================
#[cfg(feature = "ryo3-std")]
pub(crate) struct PyDurationSerializer<'py> {
    ob: &'py Bound<'py, PyAny>,
}

#[cfg(feature = "ryo3-std")]
impl<'py> PyDurationSerializer<'py> {
    pub(crate) fn new(obj: &'py Bound<'py, PyAny>) -> Self {
        Self { ob: obj }
    }
}

#[cfg(feature = "ryo3-std")]
impl Serialize for PyDurationSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let ob = self
            .ob
            .cast_exact::<ryo3_std::time::PyDuration>()
            .map_err(pyerr2sererr)?;
        let pydur = ob.get();

        let dur = *pydur.inner();
        if let Ok(signed_duration) = jiff::SignedDuration::try_from(dur) {
            signed_duration.serialize(serializer)
        } else {
            // TODO: Figure out what to do in this case... I dont love this...
            dur.serialize(serializer)
        }
    }
}

// socket types
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PySocketAddrSerializer => ryo3_std::net::PySocketAddr
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PySocketAddrV4Serializer => ryo3_std::net::PySocketAddrV4
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PySocketAddrV6Serializer => ryo3_std::net::PySocketAddrV6
);

//  ip
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PyIpAddrSerializer => ryo3_std::net::PyIpAddr
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PyIpv4AddrSerializer => ryo3_std::net::PyIpv4Addr
);
ry_type_serializer_struct! (
    #[cfg(feature = "ryo3-std")]
    PyIpv6AddrSerializer => ryo3_std::net::PyIpv6Addr
);
