use std::borrow::Cow;

use pyo3::FromPyObject;
use pyo3::prelude::*;
use ryo3_core::py_value_err;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PyEncoding(&'static encoding_rs::Encoding);

// const encodings
impl PyEncoding {
    pub(crate) const UTF_8: Self = Self(encoding_rs::UTF_8);

    pub(crate) fn from_encoding(encoding: &'static encoding_rs::Encoding) -> Self {
        Self(encoding)
    }

    pub(crate) fn as_static_str(self) -> &'static str {
        self.0.name()
    }

    pub(crate) fn decode(
        self,
        bytes: &[u8],
    ) -> (Cow<'_, str>, &'static encoding_rs::Encoding, bool) {
        self.0.decode(bytes)
    }
}

impl AsRef<str> for PyEncoding {
    fn as_ref(&self) -> &str {
        self.0.name()
    }
}

// --- FUTURE CHARSET FEATURE FLAG ---
// #[cfg(not(feature = "charset"))]
// impl<'py> FromPyObject<'_, 'py> for PyEncodingName {
//     type Error = PyErr;
//     fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
//         if obj.is_none() {
//             return Ok(Self::UTF_8);
//         }
//         if let Ok(s) = obj.extract::<&str>() {
//             if s.eq_ignore_ascii_case("utf-8") || s.eq_ignore_ascii_case("utf8") {
//                 Ok(Self::UTF_8)
//             } else {
//                 py_value_err!("Only utf-8 encoding is supported without the charset feature")
//             }
//         } else if let Ok(s) = obj.extract::<&[u8]>() {
//             if s.eq_ignore_ascii_case(b"utf-8") || s.eq_ignore_ascii_case(b"utf8") {
//                 Ok(Self::UTF_8)
//             } else {
//                 py_value_err!("Only utf-8 encoding is supported without the charset feature")
//             }
//         } else {
//             py_value_err!("Expected str/bytes for encoding")
//         }
//     }
// }

impl<'py> FromPyObject<'_, 'py> for PyEncoding {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            return Ok(Self::UTF_8);
        }
        if let Ok(s) = obj.extract::<&str>() {
            if let Some(encoding) = encoding_rs::Encoding::for_label(s.as_bytes()) {
                Ok(Self(encoding))
            } else {
                py_value_err!("Unknown encoding: {s}")
            }
        } else if let Ok(s) = obj.extract::<&[u8]>() {
            if let Some(encoding) = encoding_rs::Encoding::for_label(s) {
                Ok(Self(encoding))
            } else {
                py_value_err!("Unknown encoding: {s:?}")
            }
        } else {
            py_value_err!("Expected str/bytes for encoding")
        }
    }
}
