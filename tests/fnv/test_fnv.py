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
    assert ry.fnv1a(b"").intdigest() == 0xCBF29CE484222325


@pytest.mark.parametrize("input,expected", FNV_TEST_DATA)
def test_fnv1a(input: bytes, expected: int) -> None:
    fnvhash = ry.fnv1a(input)
    int_digest = fnvhash.intdigest()
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
    assert thingy.intdigest() == expected
    thingy_with_init = ry.FnvHasher(input)
    assert thingy_with_init.intdigest() == expected


def test_copy_hasher() -> None:
    thingy = ry.FnvHasher()
    thingy.update(b"abc")
    thingy_copy = thingy.copy()
    thingy_copy.update(b"def")
    assert thingy.intdigest() != thingy_copy.intdigest()
    assert thingy_copy.intdigest() == ry.fnv1a(b"abcdef").intdigest()
    r = thingy_copy.intdigest()
    assert r is not None
    fnhashing = ry.fnv1a(b"abc")
    fnhashing.update(b"def")
    assert fnhashing.digest() == ry.fnv1a(b"abcdef").digest()
