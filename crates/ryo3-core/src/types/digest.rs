use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyDigest<T>(T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyHexDigest<T>(T);

impl<T> PyDigest<T> {
    #[inline]
    #[must_use]
    pub fn new(t: T) -> Self {
        Self(t)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &T {
        &self.0
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> PyHexDigest<T> {
    #[inline]
    #[must_use]
    pub fn new(t: T) -> Self {
        Self(t)
    }

    #[inline]
    #[must_use]
    pub fn inner(&self) -> &T {
        &self.0
    }

    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for PyDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> From<T> for PyHexDigest<T> {
    #[inline]
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

/// digest bytes (big-endian for ints)
trait DigestBytes {
    type Bytes: AsRef<[u8]>;
    fn digest_bytes(self) -> Self::Bytes;
}

impl DigestBytes for u32 {
    type Bytes = [u8; 4];
    #[inline]
    fn digest_bytes(self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl DigestBytes for u64 {
    type Bytes = [u8; 8];
    #[inline]
    fn digest_bytes(self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl DigestBytes for u128 {
    type Bytes = [u8; 16];
    #[inline]
    fn digest_bytes(self) -> Self::Bytes {
        self.to_be_bytes()
    }
}

impl<const N: usize> DigestBytes for [u8; N] {
    type Bytes = Self;
    #[inline]
    fn digest_bytes(self) -> Self::Bytes {
        self
    }
}

impl<const N: usize> DigestBytes for &[u8; N] {
    type Bytes = Self;
    #[inline]
    fn digest_bytes(self) -> Self::Bytes {
        self
    }
}

impl<'py, T: DigestBytes> IntoPyObject<'py> for PyDigest<T> {
    type Target = PyBytes;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(PyBytes::new(py, self.0.digest_bytes().as_ref()))
    }
}

impl<'py, T: DigestBytes> IntoPyObject<'py> for PyHexDigest<T> {
    type Target = PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(pystring_hex(py, self.0.digest_bytes().as_ref()))
    }
}

// 1byte -> 2 lowercase hex chars
//
// inner part generated via this dope python thang:
// ```
// >>> s = "\n".join(" ".join(row) for HEX in ["0123456789abcdef"] for row in ((f'*b"{hi}{lo}",' for lo in HEX) for hi in HEX))
// >>> print(_)
// ```
#[rustfmt::skip]
const HEX_LUT: [[u8; 2]; 256] = [
    *b"00", *b"01", *b"02", *b"03", *b"04", *b"05", *b"06", *b"07", *b"08", *b"09", *b"0a", *b"0b", *b"0c", *b"0d", *b"0e", *b"0f",
    *b"10", *b"11", *b"12", *b"13", *b"14", *b"15", *b"16", *b"17", *b"18", *b"19", *b"1a", *b"1b", *b"1c", *b"1d", *b"1e", *b"1f",
    *b"20", *b"21", *b"22", *b"23", *b"24", *b"25", *b"26", *b"27", *b"28", *b"29", *b"2a", *b"2b", *b"2c", *b"2d", *b"2e", *b"2f",
    *b"30", *b"31", *b"32", *b"33", *b"34", *b"35", *b"36", *b"37", *b"38", *b"39", *b"3a", *b"3b", *b"3c", *b"3d", *b"3e", *b"3f",
    *b"40", *b"41", *b"42", *b"43", *b"44", *b"45", *b"46", *b"47", *b"48", *b"49", *b"4a", *b"4b", *b"4c", *b"4d", *b"4e", *b"4f",
    *b"50", *b"51", *b"52", *b"53", *b"54", *b"55", *b"56", *b"57", *b"58", *b"59", *b"5a", *b"5b", *b"5c", *b"5d", *b"5e", *b"5f",
    *b"60", *b"61", *b"62", *b"63", *b"64", *b"65", *b"66", *b"67", *b"68", *b"69", *b"6a", *b"6b", *b"6c", *b"6d", *b"6e", *b"6f",
    *b"70", *b"71", *b"72", *b"73", *b"74", *b"75", *b"76", *b"77", *b"78", *b"79", *b"7a", *b"7b", *b"7c", *b"7d", *b"7e", *b"7f",
    *b"80", *b"81", *b"82", *b"83", *b"84", *b"85", *b"86", *b"87", *b"88", *b"89", *b"8a", *b"8b", *b"8c", *b"8d", *b"8e", *b"8f",
    *b"90", *b"91", *b"92", *b"93", *b"94", *b"95", *b"96", *b"97", *b"98", *b"99", *b"9a", *b"9b", *b"9c", *b"9d", *b"9e", *b"9f",
    *b"a0", *b"a1", *b"a2", *b"a3", *b"a4", *b"a5", *b"a6", *b"a7", *b"a8", *b"a9", *b"aa", *b"ab", *b"ac", *b"ad", *b"ae", *b"af",
    *b"b0", *b"b1", *b"b2", *b"b3", *b"b4", *b"b5", *b"b6", *b"b7", *b"b8", *b"b9", *b"ba", *b"bb", *b"bc", *b"bd", *b"be", *b"bf",
    *b"c0", *b"c1", *b"c2", *b"c3", *b"c4", *b"c5", *b"c6", *b"c7", *b"c8", *b"c9", *b"ca", *b"cb", *b"cc", *b"cd", *b"ce", *b"cf",
    *b"d0", *b"d1", *b"d2", *b"d3", *b"d4", *b"d5", *b"d6", *b"d7", *b"d8", *b"d9", *b"da", *b"db", *b"dc", *b"dd", *b"de", *b"df",
    *b"e0", *b"e1", *b"e2", *b"e3", *b"e4", *b"e5", *b"e6", *b"e7", *b"e8", *b"e9", *b"ea", *b"eb", *b"ec", *b"ed", *b"ee", *b"ef",
    *b"f0", *b"f1", *b"f2", *b"f3", *b"f4", *b"f5", *b"f6", *b"f7", *b"f8", *b"f9", *b"fa", *b"fb", *b"fc", *b"fd", *b"fe", *b"ff",
];

#[inline]
fn hex_into(out: &mut [u8], bytes: &[u8]) {
    debug_assert_eq!(out.len(), bytes.len() * 2);
    let (pairs, _) = out.as_chunks_mut::<2>();
    for (pair, &b) in pairs.iter_mut().zip(bytes) {
        *pair = HEX_LUT[b as usize];
    }
}

/// lowercase hex `str`, written straight into the new python str
#[inline]
fn pystring_hex<'py>(py: Python<'py>, bytes: &[u8]) -> Bound<'py, PyString> {
    let len = bytes.len() * 2;
    cfg_select! {
        any(PyPy, GraalPy, Py_LIMITED_API) => {{
            let mut buf = vec![0u8; len];
            hex_into(&mut buf, bytes);
            PyString::new(py, std::str::from_utf8(&buf).expect("hex is ascii"))
        }}
        _ => {{
            #[expect(unsafe_code)]
            unsafe {
                // `PyUnicode_New` nul-terminates ~ convert pre touching data
                // b/c NULL (oom) should panic like `PyString::new` (akin to
                // `pystring_ascii_new` in this here crate (`ryo3-core`))
                let ptr = pyo3::ffi::PyUnicode_New(len.cast_signed(), 127);
                let s: Bound<'py, PyString> = Bound::from_owned_ptr(py, ptr).cast_into_unchecked();
                let data = std::slice::from_raw_parts_mut(
                    pyo3::ffi::PyUnicode_DATA(s.as_ptr()).cast(),
                    len,
                );
                hex_into(data, bytes);
                s
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    #[test]
    fn hex_into_matches_format() {
        let bytes: Vec<u8> = (0..=255).collect();
        let mut out = vec![0u8; 512];
        hex_into(&mut out, &bytes);
        let mut expected = String::new();
        for b in &bytes {
            let _ = write!(expected, "{b:02x}");
        }
        assert_eq!(out, expected.as_bytes());
    }
}
