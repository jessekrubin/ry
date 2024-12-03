from __future__ import annotations

import pytest

import ry

from ._fnv_test_data import FNV_TEST_DATA


def test_fnv_hasher_name() -> None:
    assert ry.FnvHasher().__class__.__name__ == "FnvHasher"
    instance = ry.fnv1a(b"")
    assert instance.name == "fnv1a"
    assert ry.FnvHasher.name == "fnv1a"
    assert instance.name == "fnv1a"


def test_fnv1a_empty() -> None:
    assert ry.fnv1a(b"").digest() == 0xCBF29CE484222325


@pytest.mark.parametrize("input,expected", FNV_TEST_DATA)
def test_fnv1a(input: bytes, expected: int) -> None:
    fnvhash = ry.fnv1a(input)
    int_digest = fnvhash.digest()
    assert int_digest == expected
    hex_str_expected = hex(expected)[2:]

    hex_digest_str_og_hasher = fnvhash.hexdigest()
    assert hex_digest_str_og_hasher == hex_str_expected

    hex_digest_str = ry.fnv1a(input).hexdigest()
    assert hex_digest_str == hex_str_expected
    assert hex_digest_str == hex_digest_str.lower()


@pytest.mark.parametrize("input,expected", FNV_TEST_DATA)
def test_fnv1a_hasher(input: bytes, expected: int) -> None:
    thingy = ry.FnvHasher()
    thingy.update(input)
    assert thingy.digest() == expected
    thingy_with_init = ry.FnvHasher(input)
    assert thingy_with_init.digest() == expected


def test_copy_hasher() -> None:
    thingy = ry.FnvHasher()
    thingy.update(b"abc")
    thingy_copy = thingy.copy()
    thingy_copy.update(b"def")
    assert thingy.digest() != thingy_copy.digest()
    assert thingy_copy.digest() == ry.fnv1a(b"abcdef").digest()
    r = thingy_copy.digest()
    assert r is not None
    fnhashing = ry.fnv1a(b"abc")
    fnhashing.update(b"def")
    assert fnhashing.digest() == ry.fnv1a(b"abcdef").digest()
