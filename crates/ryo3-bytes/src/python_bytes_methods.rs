//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyString;
use pyo3::{PyClass, prelude::*};
use std::fmt::Write;
use std::hash::Hash;

pub(crate) trait PythonBytesMethods: AsRef<[u8]> + From<Vec<u8>> + Sized + PyClass {
    /// Hash bytes
    fn py_hash(&self) -> u64 {
        // STD-HASHER VERSION
        // let mut hasher = std::hash::DefaultHasher::new();
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

    fn py_decode<'py>(
        slf: PyRef<'py, Self>,
        py: Python<'py>,
        encoding: &str,
        errors: &str,
    ) -> PyResult<Bound<'py, PyString>> {
        let py_any = slf.into_bound_py_any(py)?;
        let encoding = std::ffi::CString::new(encoding)?;
        let errors = std::ffi::CString::new(errors)?;
        PyString::from_encoded_object(&py_any, Some(&encoding), Some(&errors))
    }

    fn py_capitalize(&self) -> Self {
        let b = self.as_ref();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        let mut bytes = self.as_ref().to_vec();
        if let Some(first) = bytes.first_mut() {
            *first = first.to_ascii_uppercase();
        }
        for byte in &mut bytes[1..] {
            *byte = byte.to_ascii_lowercase();
        }
        Self::from(bytes)
    }

    fn py_strip(&self, bin: Option<&[u8]>) -> Self {
        let b = self.as_ref();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        if let Some(bin) = bin {
            if bin.is_empty() {
                return Self::from(b.to_vec());
            }
            let table = &mut [false; 256];
            for &b in bin {
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

    fn py_lstrip(&self, bin: Option<&[u8]>) -> Self {
        let b = self.as_ref();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        if let Some(bin) = bin {
            if bin.is_empty() {
                return Self::from(b.to_vec());
            }
            let table = &mut [false; 256];
            for &b in bin {
                table[b as usize] = true;
            }
            let start = b
                .iter()
                .position(|&b| !table[b as usize])
                .unwrap_or(b.len());
            Self::from(b[start..].to_vec())
        } else {
            // must do manually to match python behavior
            let is_ascii_whitespace =
                |&x: &u8| matches!(x, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c');
            let starting_ix_opt = b.iter().position(|x| !is_ascii_whitespace(x));
            let Some(starting_ix) = starting_ix_opt else {
                return Self::from(Vec::new());
            };
            Self::from(b[starting_ix..].to_vec())
        }
    }

    fn py_rstrip(&self, bin: Option<&[u8]>) -> Self {
        let b = self.as_ref();
        if b.is_empty() {
            return Self::from(vec![]);
        }
        if let Some(bin) = bin {
            if bin.is_empty() {
                return Self::from(b.to_vec());
            }
            let table = &mut [false; 256];
            for &b in bin {
                table[b as usize] = true;
            }
            let end = b
                .iter()
                .rposition(|&b| !table[b as usize])
                .map_or(0, |ix| ix + 1);
            Self::from(b[..end].to_vec())
        } else {
            // must do manually to match python behavior
            let is_ascii_whitespace =
                |&x: &u8| matches!(x, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c');
            let ending_ix = b
                .iter()
                .rposition(|x| !is_ascii_whitespace(x))
                .map_or(0, |ix| ix + 1);
            Self::from(b[..ending_ix].to_vec())
        }
    }

    // Return True if B is a titlecased string and there is at least one
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
    fn py_istitle(&self) -> bool {
        let bytes = self.as_ref();
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

    /// Return a copy of the bytes with only its first character capitalized
    fn py_title(&self) -> Self {
        let bytes = self.as_ref();
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

    /// Create a bytes object from a string of hexadecimal numbers.
    ///
    /// Spaces between two numbers are accepted.
    /// Example: bytes.fromhex('B9 01EF') -> b'\\xb9\\x01\\xef'.
    ///
    /// ## python-signature
    /// ```python
    /// (string, /)
    /// ```
    fn py_fromhex(s: &str) -> PyResult<Self> {
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

    fn py_hex(&self, sep: Option<&str>, bytes_per_sep: Option<usize>) -> PyResult<String> {
        if sep.is_some() || bytes_per_sep.is_some() {
            Err(pyo3::exceptions::PyNotImplementedError::new_err(
                "Not implemented (yet)",
            ))
        } else {
            let s = hex_encode(self.as_ref());
            Ok(s)
        }
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
    fn py_expandtabs(&self, tabsize: usize) -> Self {
        let b = self.as_ref();
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
                b'\n' | b'\r' => {
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

    fn py_swapcase(&self) -> Self {
        let b = self.as_ref();
        if b.is_empty() {
            return Self::from(vec![]);
        }

        let mut out = Vec::with_capacity(b.len()); // meh -- guess len
        for &byte in b {
            if byte.is_ascii_uppercase() {
                out.push(byte.to_ascii_lowercase());
            } else if byte.is_ascii_lowercase() {
                out.push(byte.to_ascii_uppercase());
            } else {
                out.push(byte);
            }
        }
        Self::from(out)
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

#[inline]
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}
