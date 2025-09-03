//! Extension(s) to the `pyo3-bytes` which will be hopefully be upstreamed.
use crate::bytes::PyBytes;
use crate::python_bytes_methods::PythonBytesMethods;
use bytes::BytesMut;
use pyo3::prelude::*;
use pyo3::types::PyString;

impl PythonBytesMethods for PyBytes {}

#[pymethods]
impl PyBytes {
    /// Return python-hash of bytes
    fn __hash__(&self) -> u64 {
        self.py_hash()
    }

    fn __rmul__(&self, value: usize) -> PyBytes {
        let buf = self.as_slice();
        let mut out_buf = BytesMut::with_capacity(buf.len() * value);
        (0..value).for_each(|_| out_buf.extend_from_slice(buf));
        out_buf.into()
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
        PythonBytesMethods::py_decode(slf, py, encoding, errors)
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
        self.py_hex(sep, bytes_per_sep)
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
    #[staticmethod]
    fn fromhex(s: &str) -> PyResult<Self> {
        Self::py_fromhex(s)
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
        self.py_istitle()
    }

    fn title(&self) -> Self {
        self.py_title()
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
        self.py_capitalize()
    }

    fn swapcase(&self) -> Self {
        self.py_swapcase()
    }

    #[pyo3(signature = (tabsize = 8))]
    fn expandtabs(&self, tabsize: usize) -> Self {
        self.py_expandtabs(tabsize)
    }

    #[pyo3(signature = (bin=None))]
    fn strip(&self, bin: Option<PyBytes>) -> Self {
        if let Some(bin) = bin {
            self.py_strip(Some(bin.as_ref()))
        } else {
            self.py_strip(None)
        }
    }

    #[pyo3(signature = (bin=None))]
    fn lstrip(&self, bin: Option<Self>) -> Self {
        if let Some(bin) = bin {
            self.py_lstrip(Some(bin.as_ref()))
        } else {
            self.py_lstrip(None)
        }
    }

    #[pyo3(signature = (bin=None))]
    fn rstrip(&self, bin: Option<Self>) -> Self {
        if let Some(bin) = bin {
            self.py_rstrip(Some(bin.as_ref()))
        } else {
            self.py_rstrip(None)
        }
    }
}
