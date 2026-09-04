from __future__ import annotations

import hashlib
import typing as t
import zlib

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

if t.TYPE_CHECKING:
    from ry.ryo3._bytes import ReadableBuffer


class _HasherInfo(t.TypedDict):
    name: str
    py_hasher: t.Any
    ry_hasher: t.Any
    block_size: int
    digest_size: int


class _PyCrc32:
    """A wrapper around hashlib's sha1 to make it look like a hasher with the same interface as ry's hashers"""

    def __init__(self, data: ReadableBuffer = b"", *, seed: int = 0) -> None:
        self._hash = seed
        if data:
            self.update(data)

    @property
    def block_size(self) -> int:
        return 1

    @property
    def digest_size(self) -> int:
        return 4

    @property
    def name(self) -> str:
        return "crc32"

    def update(self, data: ReadableBuffer) -> None:
        self._hash = zlib.crc32(data, self._hash)

    def digest(self) -> bytes:
        return self._hash.to_bytes(4, "big")

    def hexdigest(self) -> str:
        return self._hash.to_bytes(4, "big").hex()

    def copy(self) -> _PyCrc32:
        new_hasher = _PyCrc32()
        new_hasher._hash = self._hash
        return new_hasher


_HASHERS: list[_HasherInfo] = [
    {
        "name": "crc32",
        "py_hasher": _PyCrc32,
        "ry_hasher": ry.crc32,
        "block_size": 1,
        "digest_size": 4,
    },
]


@pytest.mark.parametrize("info", _HASHERS)
def test_hashers_info(info: _HasherInfo) -> None:
    assert info["py_hasher"]().block_size == info["block_size"]
    assert info["py_hasher"]().digest_size == info["digest_size"]
    assert info["ry_hasher"]().block_size == info["block_size"]
    assert info["ry_hasher"]().digest_size == info["digest_size"]


def test_crc32_repr() -> None:
    hasher = ry.crc32()
    assert repr(hasher) == "crc32<00000000>"
    some_bytes = b"random"
    hasher.update(some_bytes)
    expected_crc = zlib.crc32(some_bytes)
    expected_repr = f"crc32<{expected_crc:08x}>"
    assert repr(hasher) == expected_repr
    assert repr(hasher) == repr(hasher).lower()


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
