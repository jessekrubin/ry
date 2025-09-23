from __future__ import annotations

import pickle

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


def test_fnv1a_repr() -> None:
    assert repr(ry.fnv1a(b"")) == "fnv1a<cbf29ce484222325>"


def test_fnv1a_pickling() -> None:
    hasher = ry.fnv1a(b"abc")
    pickled = pickle.dumps(hasher)
    unpickled = pickle.loads(pickled)
    assert unpickled.intdigest() == hasher.intdigest()
    assert ry.fnv1a(key=hasher.intdigest()).intdigest() == hasher.intdigest()
    assert ry.fnv1a(b"", key=hasher.intdigest()).intdigest() == hasher.intdigest()
    hasher.update(b"def")
    pickled2 = pickle.dumps(hasher)
    unpickled2 = pickle.loads(pickled2)
    assert unpickled2.intdigest() == hasher.intdigest()
    assert unpickled2.intdigest() == ry.fnv1a(b"abcdef").intdigest()


@pytest.mark.parametrize("data,expected", FNV_TEST_DATA)
def test_fnv1a(data: bytes, expected: int) -> None:
    fnvhash = ry.fnv1a(data)
    int_digest = fnvhash.intdigest()
    assert int_digest == expected
    hex_str_expected = hex(expected)[2:]

    hex_digest_str_og_hasher = fnvhash.hexdigest()
    assert hex_digest_str_og_hasher == hex_str_expected

    hex_digest_str = ry.fnv1a(data).hexdigest()
    assert hex_digest_str == hex_str_expected
    assert hex_digest_str == hex_digest_str.lower()


@pytest.mark.parametrize("data,expected", FNV_TEST_DATA)
def test_fnv1a_hasher(data: bytes, expected: int) -> None:
    thingy = ry.FnvHasher()
    thingy.update(data)
    assert thingy.intdigest() == expected
    thingy_with_init = ry.FnvHasher(data)
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
