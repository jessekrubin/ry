//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.

use crate::bytes::PyBytes;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyType};
use pyo3::IntoPyObjectExt;
use std::hash::Hash;

#[pymethods]
impl PyBytes {
    /// Hash bytes
    fn __hash__(&self) -> u64 {
        // STD-HASHER VERSION
        // let mut hasher = std::collections::hash_map::DefaultHasher::new();
        // let bref: &[u8] = self.as_ref();
        // bref.hash(&mut hasher);
        // hasher.finish()
        use ahash::AHasher;
        use std::hash::Hasher;
        let mut hasher = AHasher::default();
        let bref: &[u8] = self.as_ref();
        bref.hash(&mut hasher);
        hasher.finish()
    }

    /// Decode the bytes using the codec registered for encoding.
    ///
    ///   encoding
    ///     The encoding with which to decode the bytes.
    ///   errors
    ///     The error handling scheme to use for the handling of decoding errors.
    ///     The default is 'strict' meaning that decoding errors raise a
    ///     UnicodeDecodeError. Other possible values are 'ignore' and 'replace'
    ///     as well as any other name registered with codecs.register_error that
    ///     can handle UnicodeDecodeErrors.
    ///
    /// ## python-signature
    /// ```python
    /// (encoding='utf-8', errors='strict')
    /// ```
    #[pyo3(signature = (encoding="utf-8", errors="strict"))]
    fn decode<'py>(
        slf: PyRef<'py, Self>,
        py: Python<'py>,
        encoding: &str,
        errors: &str,
    ) -> PyResult<Bound<'py, PyString>> {
        let a = slf.into_bound_py_any(py)?;
        // ensure str is null-term
        let encoding = {
            let mut enc = encoding.to_owned();
            if !enc.ends_with('\0') {
                enc.push('\0');
            }
            enc
        };
        let errors = {
            let mut err = errors.to_owned();
            if !err.ends_with('\0') {
                err.push('\0');
            }
            err
        };
        // this is screwy?
        PyString::from_object(&a, &encoding, &errors)
    }

    /// Create a string of hexadecimal numbers from a bytes object.
    ///
    ///   sep
    ///     An optional single character or byte to separate hex bytes.
    ///   bytes_per_sep
    ///     How many bytes between separators.  Positive values count from the
    ///     right, negative values count from the left.
    ///
    /// Example:
    /// >>> value = b'\xb9\x01\xef'
    /// >>> value.hex()
    /// 'b901ef'
    /// >>> value.hex(':')
    /// 'b9:01:ef'
    /// >>> value.hex(':', 2)
    /// 'b9:01ef'
    /// >>> value.hex(':', -2)
    /// 'b901:ef'
    #[pyo3(signature = (sep=None, bytes_per_sep=None))]
    fn hex(&self, sep: Option<&str>, bytes_per_sep: Option<usize>) -> PyResult<String> {
        // TODO handle sep and bytes_per_sep
        if sep.is_some() || bytes_per_sep.is_some() {
            Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Not implemented (yet)",
            ))
        } else {
            let bslice: &[u8] = self.as_ref();
            let mut s = String::with_capacity(bslice.len() * 2);
            for b in bslice {
                s.push_str(&format!("{b:02x}"));
            }
            Ok(s)
        }
    }

    /// Create a bytes object from a string of hexadecimal numbers.
    ///
    /// Spaces between two numbers are accepted.
    /// Example: bytes.fromhex('B9 01EF') -> b'\\xb9\\x01\\xef'.
    ///
    /// ## python-signature
    /// ```python
    /// (string, /)
    /// ```
    #[classmethod]
    fn fromhex(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        let s = s.replace(' ', "");
        let mut bytes = Vec::new();
        for i in 0..s.len() {
            if i % 2 == 0 {
                let byte = u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!("Invalid hex string: {e}"))
                })?;
                bytes.push(byte);
            }
        }
        Ok(Self::from(bytes))
    }
}
