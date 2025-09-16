from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, TypeAlias

import pytest

import ry

if TYPE_CHECKING:
    from collections.abc import Generator

    from ry._types import Buffer

# =============================================================================
# memchr1
# =============================================================================


def test_memchr_int() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr(b"k"[0], haystack) == 8


def test_memchr_byte() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr(b"k", haystack) == 8


thing = (b"k", b"the quick brown fox", 8)


Needle: TypeAlias = bytes | int


@dataclass()
class Memchr1TestCase:
    needle: Needle
    haystack: Buffer
    forward: int | None
    reverse: int | None


def _memch1_test_cases(
    needle: bytes,
    haystack: bytes,
) -> Generator[Memchr1TestCase, None, None]:
    assert isinstance(needle, bytes) and len(needle) == 1
    assert isinstance(haystack, bytes) and len(haystack) > 0

    _forward_ix = haystack.find(needle)
    forward = _forward_ix if _forward_ix != -1 else None
    _reverse_ix = haystack.rfind(needle)
    reverse = _reverse_ix if _reverse_ix != -1 else None
    needles = (needle, needle[0])
    haystacks = (
        haystack,
        ry.Bytes(haystack),
        memoryview(haystack),
        bytearray(haystack),
    )
    return (
        Memchr1TestCase(
            needle=needle, haystack=haystack, forward=forward, reverse=reverse
        )
        for needle in needles
        for haystack in haystacks
    )


_MEMCHR1_TEST_CASES = [
    *_memch1_test_cases(needle=b"k", haystack=b"the quick brown fox"),
    # none
    *_memch1_test_cases(needle=b"a", haystack=b"the quick brown fox"),
]


@pytest.mark.parametrize("data", _MEMCHR1_TEST_CASES)
def test_memchr_forward(data: Memchr1TestCase) -> None:
    assert ry.memchr(data.needle, data.haystack) == data.forward


@pytest.mark.parametrize("data", _MEMCHR1_TEST_CASES)
def test_memchr_reverse(data: Memchr1TestCase) -> None:
    assert ry.memrchr(data.needle, data.haystack) == data.reverse


# =============================================================================
# memchr2
# =============================================================================
def test_memchr2_int() -> None:
    haystack = b"the quick brown fox"

    assert ry.memchr2(b"k"[0], b"q"[0], haystack) == 4


def test_memchr2_bytes() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr2(b"k", b"q", haystack) == 4


def test_memchr2_reverse_bytes() -> None:
    haystack = b"the quick brown fox"
    assert ry.memrchr2(b"k", b"o", haystack) == 17


def test_memchr2_reverse_int() -> None:
    haystack = b"the quick brown fox"
    assert ry.memrchr2(b"k"[0], b"o"[0], haystack) == 17


# =============================================================================
# memchr3
# =============================================================================
def test_memchr3_int() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr3(b"k"[0], b"q"[0], b"u"[0], haystack) == 4


def test_memchr3_bytes() -> None:
    haystack = b"the quick brown fox"
    assert ry.memchr3(b"k", b"q", b"u", haystack) == 4


def test_memchr3_reverse_bytes() -> None:
    haystack = b"the quick brown fox"
    assert ry.memrchr3(b"k", b"o", b"n", haystack) == 17


def test_memchr3_reverse_int() -> None:
    haystack = b"the quick brown fox"
    assert ry.memrchr3(b"k"[0], b"o"[0], b"n"[0], haystack) == 17
