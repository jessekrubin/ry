//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.
use std::fmt::Write;
use std::hash::Hash;
use std::ops::Range;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::{IntoPyObjectExt, PyClass};

use crate::ReadableBuffer;

pub(crate) trait PythonBytesMethods: AsRef<[u8]> + From<Vec<u8>> + Sized + PyClass {
    /// Hash bytes
    fn py_hash(&self) -> u64 {
        // STD-HASHER VERSION
        // let mut hasher = std::hash::DefaultHasher::new();
        // let bref: &[u8] = self.as_ref();
        // bref.hash(&mut hasher);
        // hasher.finish()
        use std::hash::Hasher;

        use ahash::AHasher;
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

    fn py_hex(&self, sep: Option<char>, bytes_per_sep: usize) -> String {
        let formatter = PyHexFormatter::new(sep, bytes_per_sep);
        formatter.format(self.as_ref())
    }

    // ======================
    // CPYTHON IMPLEMENTATION
    // ======================
    // ```c
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
    // ```
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

macro_rules! normalized_strip_range {
    ($start:expr, $end:expr) => {{
        let start = $start;
        let end = $end;
        if start >= end { 0..0 } else { start..end }
    }};
}

#[derive(Clone, Debug, Default)]
pub(crate) enum PythonBytesStrip {
    #[default]
    AsciiWhitespace,
    None,
    One(u8),
    Table([bool; 256]),
}

impl PythonBytesStrip {
    const ASCII_WHITESPACE_TABLE: [bool; 256] = {
        let mut table = [false; 256];
        table[b' ' as usize] = true;
        table[b'\t' as usize] = true;
        table[b'\n' as usize] = true;
        table[b'\r' as usize] = true;
        table[b'\x0b' as usize] = true;
        table[b'\x0c' as usize] = true;
        table
    };

    pub(crate) fn strip_range(&self, buf: &[u8]) -> Range<usize> {
        match self {
            Self::AsciiWhitespace => Self::strip_range_ascii_whitespace(buf),
            Self::None => 0..buf.len(),
            Self::One(byte) => Self::strip_range_one(buf, *byte),
            Self::Table(table) => Self::strip_range_table(buf, table),
        }
    }

    pub(crate) fn lstrip_range(&self, buf: &[u8]) -> usize {
        match self {
            Self::AsciiWhitespace => Self::lstrip_range_ascii_whitespace(buf),
            Self::None => 0,
            Self::One(byte) => Self::lstrip_range_one(buf, *byte),
            Self::Table(table) => Self::lstrip_range_table(buf, table),
        }
    }

    pub(crate) fn rstrip_range(&self, buf: &[u8]) -> usize {
        match self {
            Self::AsciiWhitespace => Self::rstrip_range_ascii_whitespace(buf),
            Self::None => buf.len(),
            Self::One(byte) => Self::rstrip_range_one(buf, *byte),
            Self::Table(table) => Self::rstrip_range_table(buf, table),
        }
    }

    fn from_bytes(chars: &[u8]) -> Self {
        match chars {
            [] => Self::None,
            [byte] => Self::One(*byte),
            _ => {
                let t = Self::byte_lookup_table(chars);
                if t == Self::ASCII_WHITESPACE_TABLE {
                    Self::AsciiWhitespace
                } else {
                    Self::Table(t)
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // STRIP FNS
    // ------------------------------------------------------------------------

    fn byte_lookup_table(bytes: &[u8]) -> [bool; 256] {
        let mut table = [false; 256];
        for &byte in bytes {
            table[byte as usize] = true;
        }
        table
    }

    fn strip_range_ascii_whitespace(buf: &[u8]) -> Range<usize> {
        let l = Self::lstrip_range_ascii_whitespace(buf);
        let r = Self::rstrip_range_ascii_whitespace(buf);
        normalized_strip_range!(l, r)
    }

    #[inline]
    fn lstrip_range_ascii_whitespace(buf: &[u8]) -> usize {
        buf.iter()
            .position(|&byte| !is_python_ascii_whitespace(byte))
            .unwrap_or(buf.len())
    }

    #[inline]
    fn rstrip_range_ascii_whitespace(buf: &[u8]) -> usize {
        buf.iter()
            .rposition(|&byte| !is_python_ascii_whitespace(byte))
            .map_or(0, |ix| ix + 1)
    }

    fn strip_range_one(buf: &[u8], byte: u8) -> Range<usize> {
        let l = Self::lstrip_range_one(buf, byte);
        let r = Self::rstrip_range_one(buf, byte);
        normalized_strip_range!(l, r)
    }

    fn lstrip_range_one(buf: &[u8], byte: u8) -> usize {
        buf.iter()
            .position(|&candidate| candidate != byte)
            .unwrap_or(buf.len())
    }

    fn rstrip_range_one(buf: &[u8], byte: u8) -> usize {
        buf.iter()
            .rposition(|&candidate| candidate != byte)
            .map_or(0, |ix| ix + 1)
    }

    fn strip_range_table(buf: &[u8], table: &[bool; 256]) -> Range<usize> {
        let l = Self::lstrip_range_table(buf, table);
        let r = Self::rstrip_range_table(buf, table);
        normalized_strip_range!(l, r)
    }

    fn lstrip_range_table(buf: &[u8], table: &[bool; 256]) -> usize {
        buf.iter()
            .position(|&byte| !table[byte as usize])
            .unwrap_or(buf.len())
    }

    fn rstrip_range_table(buf: &[u8], table: &[bool; 256]) -> usize {
        buf.iter()
            .rposition(|&byte| !table[byte as usize])
            .map_or(0, |ix| ix + 1)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PythonBytesStrip {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if ob.is_none() {
            return Ok(Self::AsciiWhitespace);
        }

        let buf = ob.extract::<ReadableBuffer<'a, 'py>>()?;
        Ok(Self::from_bytes(buf.as_slice()))
    }
}

#[inline]
const fn hex_val(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'a'..='f' => Some((c as u8) - b'a' + 10),
        'A'..='F' => Some((c as u8) - b'A' + 10),
        _ => None,
    }
}

#[inline]
const fn is_python_ascii_whitespace(x: u8) -> bool {
    matches!(x, b' ' | b'\t' | b'\n' | b'\r' | b'\x0b' | b'\x0c')
}

// single char as a byte/str
pub(crate) struct PyHexSep(char);

impl From<PyHexSep> for char {
    fn from(value: PyHexSep) -> Self {
        value.0
    }
}

impl FromPyObject<'_, '_> for PyHexSep {
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(pystr) = ob.cast_exact::<PyString>() {
            let len = pystr.len()?;
            if len != 1 {
                return Err(PyValueError::new_err(
                    "Separator must be a single ASCII character",
                ));
            }
            let s = pystr.to_str()?;
            if !s.is_ascii() {
                return Err(PyValueError::new_err(
                    "Separator must be a single ASCII character",
                ));
            }
            let char = s.chars().next().expect("wenodis");
            Ok(Self(char))
        } else {
            Err(PyValueError::new_err(
                "Separator must be a single ASCII character string",
            ))
        }
    }
}

struct PyHexFormatter {
    sep: Option<char>,
    bytes_per_sep: usize,
}

impl PyHexFormatter {
    fn new(sep: Option<char>, bytes_per_sep: usize) -> Self {
        Self { sep, bytes_per_sep }
    }

    fn format_default(&self, bytes: &[u8]) -> String {
        bytes.iter().fold(
            String::with_capacity(self.capacity(bytes.len())),
            |mut output, b| {
                let _ = write!(output, "{b:02x}");
                output
            },
        )
    }

    fn capacity(&self, num_bytes: usize) -> usize {
        if self.sep.is_none() {
            return num_bytes * 2;
        }

        let group_size = self.bytes_per_sep;
        let sep_count = if num_bytes == 0 {
            0
        } else if group_size == 0 {
            0
        } else {
            (num_bytes - 1) / group_size
        };
        num_bytes * 2 + sep_count
    }

    fn format(&self, bytes: &[u8]) -> String {
        if self.sep.is_none() {
            return self.format_default(bytes);
        }

        if self.bytes_per_sep == 0 {
            return self.format_default(bytes);
        }

        let mut output = String::with_capacity(self.capacity(bytes.len()));
        let sep = self.sep.expect("checked above");
        let group_size = self.bytes_per_sep;
        for (i, b) in bytes.iter().enumerate() {
            if i > 0 && (bytes.len() - i).is_multiple_of(group_size) {
                output.push(sep);
            }
            let _ = write!(output, "{b:02x}");
        }
        output
    }
}
