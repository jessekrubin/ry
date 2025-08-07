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
#![cfg(feature = "ry")]
use crate::errors::pyerr2sererr;

use crate::ser::py_serialize::SerializePyAny;
use pyo3::prelude::*;
use serde::ser::Serialize;

// THE FOLLOWING MACRO WAS WRITTEN BY FUGUE-STATE-JESSE (NOT AN LLM).
// I (NORMAL JESSE) HAVE NO CLUE HOW IT WORKS :(
// UNFORTUNATELY, FUGUE-STATE-JESSE ONLY APPEARS AT RANDOM, SO YOU HAVE TO ASK
// HIM IF/WHEN HE RE-APPEARS...
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
    // http
    #[cfg(feature = "ryo3-http")]  ry_headers => ryo3_http::PyHeaders;
    #[cfg(feature = "ryo3-http")]  ry_http_status => ryo3_http::PyHttpStatus;
    // jiff
    #[cfg(feature = "ryo3-jiff")]  ry_date => ryo3_jiff::RyDate;
    #[cfg(feature = "ryo3-jiff")]  ry_datetime => ryo3_jiff::RyDateTime;
    #[cfg(feature = "ryo3-jiff")]  ry_signed_duration => ryo3_jiff::RySignedDuration;
    #[cfg(feature = "ryo3-jiff")]  ry_span => ryo3_jiff::RySpan;
    #[cfg(feature = "ryo3-jiff")]  ry_time => ryo3_jiff::RyTime;
    #[cfg(feature = "ryo3-jiff")]  ry_timestamp => ryo3_jiff::RyTimestamp;
    #[cfg(feature = "ryo3-jiff")]  ry_timezone => ryo3_jiff::RyTimeZone;
    #[cfg(feature = "ryo3-jiff")]  ry_zoned => ryo3_jiff::RyZoned;
    // ulid
    #[cfg(feature = "ryo3-ulid")]  ry_ulid => ryo3_ulid::PyUlid;
    // url
    #[cfg(feature = "ryo3-url")]   ry_url => ryo3_url::PyUrl;
    // uuid
    #[cfg(feature = "ryo3-uuid")]  ry_uuid => ryo3_uuid::PyUuid;
}
