use pyo3::prelude::*;

use ryo3_bytes::PyBytes;
use ryo3_core::types::Byte;

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memchr(needle: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memchr(*needle, haystack.as_slice())
}

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memchr2(needle1: Byte, needle2: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memchr2(*needle1, *needle2, haystack.as_slice())
}

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memchr3(needle1: Byte, needle2: Byte, needle3: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memchr3(*needle1, *needle2, *needle3, haystack.as_slice())
}

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memrchr(needle: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memrchr(*needle, haystack.as_slice())
}

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memrchr2(needle1: Byte, needle2: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memrchr2(*needle1, *needle2, haystack.as_slice())
}

#[expect(clippy::needless_pass_by_value)]
#[must_use]
#[pyfunction]
pub fn memrchr3(needle1: Byte, needle2: Byte, needle3: Byte, haystack: PyBytes) -> Option<usize> {
    ::memchr::memrchr3(*needle1, *needle2, *needle3, haystack.as_slice())
}
