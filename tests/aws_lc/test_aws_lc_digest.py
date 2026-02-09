from __future__ import annotations

import hashlib
import sys
import typing as t

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry


class _HasherInfo(t.TypedDict):
    name: str
    py_hasher: t.Any
    ry_hasher: t.Any
    block_size: int
    digest_size: int


_HASHERS: list[_HasherInfo] = [
    {
        "name": "sha1",
        "py_hasher": hashlib.sha1,
        "ry_hasher": ry.sha1,
        "block_size": 64,
        "digest_size": 20,
    },
    {
        "name": "sha224",
        "py_hasher": hashlib.sha224,
        "ry_hasher": ry.sha224,
        "block_size": 64,
        "digest_size": 28,
    },
    {
        "name": "sha256",
        "py_hasher": hashlib.sha256,
        "ry_hasher": ry.sha256,
        "block_size": 64,
        "digest_size": 32,
    },
    {
        "name": "sha384",
        "py_hasher": hashlib.sha384,
        "ry_hasher": ry.sha384,
        "block_size": 128,
        "digest_size": 48,
    },
    {
        "name": "sha3_256",
        "py_hasher": hashlib.sha3_256,
        "ry_hasher": ry.sha3_256,
        "block_size": 136,
        "digest_size": 32,
    },
    {
        "name": "sha3_384",
        "py_hasher": hashlib.sha3_384,
        "ry_hasher": ry.sha3_384,
        "block_size": 104,
        "digest_size": 48,
    },
    {
        "name": "sha3_512",
        "py_hasher": hashlib.sha3_512,
        "ry_hasher": ry.sha3_512,
        "block_size": 72,
        "digest_size": 64,
    },
    {
        "name": "sha512",
        "py_hasher": hashlib.sha512,
        "ry_hasher": ry.sha512,
        "block_size": 128,
        "digest_size": 64,
    },
    {
        "name": "sha512_256",
        "py_hasher": lambda *a, **kw: hashlib.new("sha512_256", *a, **kw),
        "ry_hasher": ry.sha512_256,
        "block_size": 128,
        "digest_size": 32,
    },
]


@pytest.mark.parametrize("info", _HASHERS)
def test_hashers_info(info: _HasherInfo) -> None:
    assert info["py_hasher"]().block_size == info["block_size"]
    assert info["py_hasher"]().digest_size == info["digest_size"]
    assert info["ry_hasher"]().block_size == info["block_size"]
    assert info["ry_hasher"]().digest_size == info["digest_size"]


@pytest.mark.parametrize("info", _HASHERS)
def test_sha_hasher_repr(info: _HasherInfo) -> None:
    hasher = info["ry_hasher"]()
    repr_str = repr(hasher)
    id_ptr = hex(id(hasher))
    assert repr_str.startswith(f"{info['name']}<")
    assert repr_str.endswith(f"{id_ptr}>")
    # pypy prt don't work good?
    if sys.implementation.name == "cpython":
        assert repr_str[len(info["name"]) + 1 : -1] == id_ptr


@pytest.mark.parametrize("info", _HASHERS)
def test_hashers_oneshot(info: _HasherInfo) -> None:
    data = b"abcdefghijklmnopqrstuvwxyz0123456789"
    py_digest = info["py_hasher"](data).digest()
    ry_digest = info["ry_hasher"].oneshot(data)
    assert py_digest == ry_digest


@pytest.mark.parametrize("info", _HASHERS)
@given(data=st.binary())
def test_sha_hashers(info: _HasherInfo, data: bytes) -> None:
    py_hasher = info["py_hasher"]()
    py_hasher.update(data)
    py_digest = py_hasher.digest()
    ry_hasher = info["ry_hasher"]()
    ry_hasher.update(data)
    ry_digest = ry_hasher.digest()
    assert py_digest == ry_digest
    py_hexdigest = py_hasher.hexdigest()
    ry_hexdigest = ry_hasher.hexdigest()
    assert py_hexdigest == ry_hexdigest

    # see that we can continue to update the hasher after calling digest/hexdigest
    py_hasher.update(data)
    py_digest2 = py_hasher.digest()
    ry_hasher.update(data)
    ry_digest2 = ry_hasher.digest()
    assert py_digest2 == ry_digest2
    py_hexdigest2 = py_hasher.hexdigest()
    ry_hexdigest2 = ry_hasher.hexdigest()
    assert py_hexdigest2 == ry_hexdigest2


@pytest.mark.parametrize("info", _HASHERS)
@given(data=st.binary(min_size=10))
def test_copy_hasher(info: _HasherInfo, data: bytes) -> None:
    py_hasher = info["py_hasher"]()
    py_hasher.update(data)
    py_digest = py_hasher.digest()

    ry_hasher = info["ry_hasher"](data)
    ry_digest = ry_hasher.digest()
    assert py_digest == ry_digest

    py_copy = py_hasher.copy()
    py_copy.update(data)
    py_copy_digest = py_copy.digest()
    assert py_copy_digest != py_digest

    ry_copy = ry_hasher.copy()
    ry_copy.update(data)
    ry_copy_digest = ry_copy.digest()
    assert ry_copy_digest != ry_digest


@pytest.mark.parametrize("info", _HASHERS)
@given(data=st.binary())
def test_initial_data(info: _HasherInfo, data: bytes) -> None:
    py_hasher = info["py_hasher"]()
    py_hasher.update(data)
    py_digest = py_hasher.digest()

    ry_hasher = info["ry_hasher"](data)
    ry_digest = ry_hasher.digest()
    assert py_digest == ry_digest
    py_hexdigest = py_hasher.hexdigest()
    ry_hexdigest = ry_hasher.hexdigest()
    assert py_hexdigest == ry_hexdigest

    # see that we can continue to update the hasher after calling digest/hexdigest
    py_hasher.update(data)
    py_digest2 = py_hasher.digest()
    ry_hasher.update(data)
    ry_digest2 = ry_hasher.digest()
    assert py_digest2 == ry_digest2
    py_hexdigest2 = py_hasher.hexdigest()
    ry_hexdigest2 = ry_hasher.hexdigest()
    assert py_hexdigest2 == ry_hexdigest2


@pytest.mark.parametrize("info", _HASHERS)
@given(data=st.binary())
def test_oneshot_methods(info: _HasherInfo, data: bytes) -> None:
    py_hasher = info["py_hasher"]()
    py_hasher.update(data)
    py_digest = py_hasher.digest()

    ry_digest = info["ry_hasher"].oneshot(data)
    assert py_digest == ry_digest


@given(data=st.binary())
def test_sha256(data: bytes) -> None:
    py_hasher = hashlib.sha256()
    py_hasher.update(data)
    py_digest = py_hasher.digest()
    ry_hasher = ry.sha256()
    ry_hasher.update(data)
    ry_digest = ry_hasher.digest()
    assert py_digest == ry_digest
    py_hexdigest = py_hasher.hexdigest()
    ry_hexdigest = ry_hasher.hexdigest()
    assert py_hexdigest == ry_hexdigest

    # see that we can continue to update the hasher after calling digest/hexdigest
    py_hasher.update(data)
    py_digest2 = py_hasher.digest()
    ry_hasher.update(data)
    ry_digest2 = ry_hasher.digest()
    assert py_digest2 == ry_digest2
    py_hexdigest2 = py_hasher.hexdigest()
    ry_hexdigest2 = ry_hasher.hexdigest()
    assert py_hexdigest2 == ry_hexdigest2
