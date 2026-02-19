use pyo3::FromPyObject;
use pyo3::prelude::*;
use ryo3_core::py_value_err;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PyEncodingName(&'static str);

// const encodings
impl PyEncodingName {
    pub(crate) const UTF_8: Self = Self("utf-8");
}

impl AsRef<str> for PyEncodingName {
    fn as_ref(&self) -> &str {
        self.0
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

impl<'py> FromPyObject<'_, 'py> for PyEncodingName {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            return Ok(Self::UTF_8);
        }
        if let Ok(s) = obj.extract::<&str>() {
            if let Some(encoding) = encoding_rs::Encoding::for_label(s.as_bytes()) {
                Ok(Self(encoding.name()))
            } else {
                py_value_err!("Unknown encoding: {s}")
            }
        } else if let Ok(s) = obj.extract::<&[u8]>() {
            if let Some(encoding) = encoding_rs::Encoding::for_label(s) {
                Ok(Self(encoding.name()))
            } else {
                py_value_err!("Unknown encoding: {s:?}")
            }
        } else {
            py_value_err!("Expected str/bytes for encoding")
        }
    }
}
