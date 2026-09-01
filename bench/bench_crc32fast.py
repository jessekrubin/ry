from __future__ import annotations

import zlib
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

    from ry.ryo3._bytes import ReadableBuffer


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
    ("crc32fast", ry.crc32),
    ("python-zlib", _PyCrc32),
]


@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize("impl", _HASHERS, ids=lambda impl: impl[0])
def test_bench_crc32fast_hasher(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, type]
) -> None:
    name, data = id_data
    benchmark.group = f"crc32-hasher-{name}"

    def _fn() -> None:
        h = impl[1]()
        h.update(data)
        h.digest()

    benchmark(_fn)


@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize("impl", _HASHERS, ids=lambda impl: impl[0])
def test_bench_crc32fast_oneshot_bytes(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, type]
) -> None:
    name, data = id_data
    benchmark.group = f"crc32-oneshot-bytes-{name}"

    def _fn() -> None:
        impl[1].oneshot(data)  # ty: ignore[unresolved-attribute]

    benchmark(_fn)


@pytest.mark.parametrize("id_data", _BYTES, ids=lambda id_data: id_data[0])
@pytest.mark.parametrize("impl", _HASHERS, ids=lambda impl: impl[0])
def test_bench_crc32fast_oneshot_int(
    benchmark: BenchmarkFixture, id_data: tuple[str, bytes], impl: tuple[str, type]
) -> None:
    name, data = id_data
    benchmark.group = f"crc32-oneshot-int-{name}"

    def _fn() -> None:
        impl[1].oneshot_int(data)  # ty: ignore[unresolved-attribute]

    benchmark(_fn)
