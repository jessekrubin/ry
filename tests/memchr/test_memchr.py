from __future__ import annotations

import ry

# use memchr::memchr;

# let haystack = b"the quick brown fox";
# assert_eq!(memchr(b'k', haystack), Some(8));


def test_memchr_int() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr(b"k"[0], haystack) == 8


def test_memchr_byte() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr(b"k", haystack) == 8
