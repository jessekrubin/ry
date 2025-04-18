//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.
use crate::bytes::PyBytes;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple, PyType};
use pyo3::IntoPyObjectExt;
use std::fmt::Write;
use std::hash::Hash;

#[pymethods]
impl PyBytes {
    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let pybytes = pyo3::types::PyBytes::new(py, self.as_ref()).into_bound_py_any(py)?;
        PyTuple::new(py, vec![pybytes])
    }

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
        let py_any = slf.into_bound_py_any(py)?;
        PyString::from_object(&py_any, encoding, errors)
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
                let _ = write!(s, "{b:02x}");
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
        // filter out whitespace
        let mut it = s.chars().filter(|c| !c.is_ascii_whitespace());
        let mut bytes = Vec::with_capacity(s.len() / 2);
        while let Some(char_a) = it.next() {
            // second char
            let char_b = it.next().ok_or_else(|| {
                PyValueError::new_err("Odd-length hex string; missing final digit")
            })?;
            // convert and err if not hex
            let a = hex_val(char_a)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid hex digit `{char_a}`")))?;
            let b = hex_val(char_b)
                .ok_or_else(|| PyValueError::new_err(format!("Invalid hex digit `{char_b}`")))?;
            bytes.push(a << 4 | b);
        }
        Ok(Self::from(bytes))
    }

    // #[pyo3(signature = (keepends=false, filter_empty=true))]
    // fn splitlines(&self, py: Python, keepends: bool, filter_empty: bool) -> PyResult<Vec<Self>> {
    //     let bytes: &[u8] = self.as_ref();
    //     if bytes.is_empty() {
    //         return Ok(vec![]);
    //     }
    //     let is_break = |&b: &u8| matches!(b, b'\n' | b'\r' | b'\x0b' | b'\x0c');
    //     // inclusive if keepends
    //     // terminator if not keepends
    //     // as impl iterator
    //     let it: Box<dyn Iterator<Item=&[u8]>> = if keepends {
    //         Box::new(bytes.split_inclusive(is_break))
    //     } else {
    //         Box::new(bytes.split(is_break))
    //     };

    //     if filter_empty {
    //         let lines = it.filter(|line| !line.is_empty()).map(
    //             |line | {
    //                 PyBytes::from(line.to_vec())
    //             }
    //         )
    //         .collect::<Vec<_>>();
    //         Ok(lines)
    //     } else {
    //         let lines = it.map(
    //             |line | {
    //                 PyBytes::from(line.to_vec())
    //             }
    //         )
    //         .collect::<Vec<_>>();
    //         Ok(lines)
    //     }
    // }
}

#[inline]
fn hex_val(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'a'..='f' => Some((c as u8) - b'a' + 10),
        'A'..='F' => Some((c as u8) - b'A' + 10),
        _ => None,
    }
}
