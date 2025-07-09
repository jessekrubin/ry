//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.
use crate::bytes::PyBytes;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple, PyType};
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
            bytes.push((a << 4) | b);
        }
        Ok(Self::from(bytes))
    }

    /// Return True if B is a titlecased string and there is at least one
    /// character in B, i.e. uppercase characters may only follow uncased
    /// characters and lowercase characters only cased ones. Return False
    /// otherwise.
    ///
    /// Impl based on cpython's implementation ([permalink](https://github.com/python/cpython/blob/main/Objects/bytes_methods.c#L201) / [maybe-outdated](https://github.com/python/cpython/blob/main/Objects/bytes_methods.c#L201))
    ///
    /// ```c
    /// PyObject*
    /// _Py_bytes_istitle(const char *cptr, Py_ssize_t len)
    /// {
    ///     const unsigned char *p
    ///         = (const unsigned char *) cptr;
    ///     const unsigned char *e;
    ///     int cased, previous_is_cased;
    ///
    ///     if (len == 1) {
    ///         if (Py_ISUPPER(*p)) {
    ///             Py_RETURN_TRUE;
    ///         }
    ///         Py_RETURN_FALSE;
    ///     }
    ///
    ///     /* Special case for empty strings */
    ///     if (len == 0)
    ///         Py_RETURN_FALSE;
    ///
    ///     e = p + len;
    ///     cased = 0;
    ///     previous_is_cased = 0;
    ///     for (; p < e; p++) {
    ///         const unsigned char ch = *p;
    ///
    ///         if (Py_ISUPPER(ch)) {
    ///             if (previous_is_cased)
    ///                 Py_RETURN_FALSE;
    ///             previous_is_cased = 1;
    ///             cased = 1;
    ///         }
    ///         else if (Py_ISLOWER(ch)) {
    ///             if (!previous_is_cased)
    ///                 Py_RETURN_FALSE;
    ///             previous_is_cased = 1;
    ///             cased = 1;
    ///         }
    ///         else
    ///             previous_is_cased = 0;
    ///     }
    ///     return PyBool_FromLong(cased);
    /// }
    /// ```
    fn istitle(&self) -> bool {
        let bytes = self.as_slice();
        if bytes.is_empty() {
            return false;
        }
        if bytes.len() == 1 {
            return bytes[0].is_ascii_uppercase();
        }
        let mut cased = false;
        let mut previous_is_cased = false;
        for &byte in bytes {
            if byte.is_ascii_uppercase() {
                if previous_is_cased {
                    return false;
                }
                previous_is_cased = true;
                cased = true;
            } else if byte.is_ascii_lowercase() {
                if !previous_is_cased {
                    return false;
                }
                previous_is_cased = true;
                cased = true;
            } else {
                previous_is_cased = false;
            }
        }
        cased
    }

    fn title(&self) -> Self {
        let bytes = self.as_slice();
        if bytes.is_empty() {
            return Self::from(vec![]);
        }
        let mut result = Vec::with_capacity(bytes.len());
        let mut previous_is_cased = false;

        for &byte in bytes {
            if byte.is_ascii_uppercase() {
                if previous_is_cased {
                    result.push(byte.to_ascii_lowercase());
                } else {
                    result.push(byte);
                }
                previous_is_cased = true;
            } else if byte.is_ascii_lowercase() {
                if previous_is_cased {
                    result.push(byte);
                } else {
                    result.push(byte.to_ascii_uppercase());
                }

                previous_is_cased = true;
            } else {
                result.push(byte);
                previous_is_cased = false;
            }
        }
        Self::from(result)
    }

    #[pyo3(signature = (prefix, /))]
    fn startswith(&self, prefix: PyBytes) -> bool {
        self.as_slice().starts_with(prefix.as_ref())
    }

    #[pyo3(signature = (suffix, /))]
    fn endswith(&self, suffix: PyBytes) -> bool {
        self.as_slice().ends_with(suffix.as_ref())
    }

    fn capitalize(&self) -> Self {
        let b = self.as_slice();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        let mut bytes = self.as_slice().to_vec();
        if let Some(first) = bytes.first_mut() {
            *first = first.to_ascii_uppercase();
        }
        for byte in &mut bytes[1..] {
            *byte = byte.to_ascii_lowercase();
        }
        Self::from(bytes)
    }

    fn swapcase(&self) -> Self {
        let b = self.as_slice();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        let b = self
            .as_slice()
            .iter()
            .map(|byte| {
                if byte.is_ascii_uppercase() {
                    byte.to_ascii_lowercase()
                } else if byte.is_ascii_lowercase() {
                    byte.to_ascii_uppercase()
                } else {
                    *byte
                }
            })
            .collect::<Vec<u8>>();
        Self::from(b)
    }

    // ======================
    // CPYTHON IMPLEMENTATION
    // ======================
    // static PyObject *
    // stringlib_expandtabs_impl(PyObject *self, int tabsize)
    // /*[clinic end generated code: output=069cb7fae72e4c2b input=3c6d3b12aa3ccbea]*/
    // {
    //     const char *e, *p;
    //     char *q;
    //     Py_ssize_t i, j;
    //     PyObject *u;

    //     /* First pass: determine size of output string */
    //     i = j = 0;
    //     e = STRINGLIB_STR(self) + STRINGLIB_LEN(self);
    //     for (p = STRINGLIB_STR(self); p < e; p++) {
    //         if (*p == '\t') {
    //             if (tabsize > 0) {
    //                 Py_ssize_t incr = tabsize - (j % tabsize);
    //                 if (j > PY_SSIZE_T_MAX - incr)
    //                     goto overflow;
    //                 j += incr;
    //             }
    //         }
    //         else {
    //             if (j > PY_SSIZE_T_MAX - 1)
    //                 goto overflow;
    //             j++;
    //             if (*p == '\n' || *p == '\r') {
    //                 if (i > PY_SSIZE_T_MAX - j)
    //                     goto overflow;
    //                 i += j;
    //                 j = 0;
    //             }
    //         }
    //     }

    //     if (i > PY_SSIZE_T_MAX - j)
    //         goto overflow;

    //     /* Second pass: create output string and fill it */
    //     u = STRINGLIB_NEW(NULL, i + j);
    //     if (!u)
    //         return NULL;

    //     j = 0;
    //     q = STRINGLIB_STR(u);

    //     for (p = STRINGLIB_STR(self); p < e; p++) {
    //         if (*p == '\t') {
    //             if (tabsize > 0) {
    //                 i = tabsize - (j % tabsize);
    //                 j += i;
    //                 while (i--)
    //                     *q++ = ' ';
    //             }
    //         }
    //         else {
    //             j++;
    //             *q++ = *p;
    //             if (*p == '\n' || *p == '\r')
    //                 j = 0;
    //         }
    //     }

    //     return u;
    //   overflow:
    //     PyErr_SetString(PyExc_OverflowError, "result too long");
    //     return NULL;
    // }
    #[pyo3(signature = (tabsize = 8))]
    fn expandtabs(&self, tabsize: usize) -> Self {
        let b = self.as_slice();
        if b.is_empty() || tabsize == 0 {
            return Self::from(b.to_vec());
        }

        let mut col = 0usize;
        let mut out = Vec::with_capacity(b.len()); // meh -- guess len

        for &byte in b {
            match byte {
                b'\t' => {
                    let pad = tabsize - (col % tabsize);
                    out.extend(std::iter::repeat_n(b' ', pad));
                    col += pad;
                }
                b'\n' | b'\r' | 0x0C => {
                    out.push(byte);
                    col = 0;
                }
                _ => {
                    out.push(byte);
                    col += 1;
                }
            }
        }
        Self::from(out)
    }

    #[pyo3(signature = (bin=None))]
    fn strip(&self, bin: Option<PyBytes>) -> Self {
        let b = self.as_slice();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        if let Some(bin) = bin {
            let strip_bytes = bin.as_slice();
            if strip_bytes.is_empty() {
                return Self::from(b.to_vec());
            }
            let table = &mut [false; 256];
            for &b in strip_bytes {
                table[b as usize] = true;
            }
            let Some(start) = b.iter().position(|&b| !table[b as usize]) else {
                return Self::from(Vec::new());
            };
            if start == b.len() {
                return Self::from(Vec::new());
            }
            let end = b
                .iter()
                .rposition(|&b| !table[b as usize])
                .map_or(b.len(), |ix| ix + 1);
            Self::from(b[start..end].to_vec())
        } else {
            // must do manually to match python behavior
            let is_ascii_whitespace =
                |&x: &u8| matches!(x, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c');
            let starting_ix_opt = b.iter().position(|x| !is_ascii_whitespace(x));
            let Some(starting_ix) = starting_ix_opt else {
                return Self::from(Vec::new());
            };

            let ending_ix = b.iter().rposition(|x| !is_ascii_whitespace(x)).unwrap() + 1;

            Self::from(b[starting_ix..ending_ix].to_vec())
        }
    }
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
