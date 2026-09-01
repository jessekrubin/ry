from __future__ import annotations

import hashlib
import typing as t
import zlib
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

    from ry.ryo3._bytes import ReadableBuffer


# class _HasherInfo(t.TypedDict):
#     name: str
#     py_hasher: t.Any
#     ry_hasher: t.Any
#     block_size: int
#     digest_size: int


class _PyCrc32:
    """A ry-like python impl of crc32 using zlib"""

    __slots__ = ("_hash",)

    def __init__(self, data: ReadableBuffer | None = None, *, seed: int = 0) -> None:
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
        return _PyCrc32(seed=self._hash)

    @staticmethod
    def oneshot(data: ReadableBuffer) -> bytes:
        return zlib.crc32(bytes(data)).to_bytes(4, "big")

    @staticmethod
    def oneshot_int(data: ReadableBuffer) -> int:
        return zlib.crc32(bytes(data))

    @staticmethod
    def oneshot_hex(data: ReadableBuffer) -> str:
        return zlib.crc32(bytes(data)).to_bytes(4, "big").hex()


def random_bytes(size: int) -> bytes:
    # make random bytes
    return bytes([i % 256 for i in range(size)])


_BYTES = [
    ("1kib", random_bytes(1024)),
    ("10kib", random_bytes(1024 * 10)),
    ("100kib", random_bytes(1024 * 100)),
    ("1mib", random_bytes(1024 * 1024)),
    ("10mib", random_bytes(1024 * 1024 * 10)),
]
_HASHERS = [
    ("crc32", ry.crc32, _PyCrc32),
]
_RY_HASHERS = [(name, "ry", cls) for name, cls, _ in _HASHERS]
_PY_HASHERS = [(name, "py", cls) for name, _, cls in _HASHERS]


@pytest.mark.benchmark(group="crc32fast")
@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize(
    "impl", [*_RY_HASHERS, *_PY_HASHERS], ids=lambda impl: f"{impl[0]}-{impl[1]}"
)
def test_bench_crc32fast_hasher(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, str, type]
) -> None:
    name, data = id_data
    benchmark.group = impl[0] + "-" + name

    def _fn() -> None:
        h = impl[2]()
        h.update(data)
        h.digest()

    benchmark(_fn)


@pytest.mark.benchmark(group="crc32fast")
@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize(
    "impl", [*_RY_HASHERS, *_PY_HASHERS], ids=lambda impl: f"{impl[0]}-{impl[1]}"
)
def test_bench_crc32fast_oneshot_bytes(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, str, type]
) -> None:
    name, data = id_data
    benchmark.group = impl[0] + "-" + name

    def _fn() -> None:
        impl[2].oneshot(data)

    benchmark(_fn)


@pytest.mark.benchmark(group="crc32fast")
@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize(
    "impl", [*_RY_HASHERS, *_PY_HASHERS], ids=lambda impl: f"{impl[0]}-{impl[1]}"
)
def test_bench_crc32fast_oneshot_int(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, str, type]
) -> None:
    name, data = id_data
    benchmark.group = impl[0] + "-" + name

    def _fn() -> None:
        impl[2].oneshot_int(data)

    benchmark(_fn)
