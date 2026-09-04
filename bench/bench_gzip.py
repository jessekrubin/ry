from __future__ import annotations

import gzip
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from collections.abc import Callable

    from pytest_benchmark.fixture import BenchmarkFixture


def _payload(size: int) -> bytes:
    chunk = (
        b'{"id":123,"name":"gzip bench","active":true,"tags":["ry","flate2"],'
        b'"message":"the quick brown fox jumps over the lazy dog"}\n'
    )
    return (chunk * ((size // len(chunk)) + 1))[:size]


_DATA = [
    ("1kib", _payload(1024)),
    ("100kib", _payload(100 * 1024)),
    ("1mib", _payload(1024 * 1024)),
]

_COMPRESSORS: list[tuple[str, Callable[[bytes], bytes]]] = [
    ("ry", lambda data: bytes(ry.gzip_encode(data, quality=6))),
    ("stdlib", lambda data: gzip.compress(data, compresslevel=6)),
]


@pytest.mark.parametrize("name_data", _DATA, ids=lambda name_data: name_data[0])
@pytest.mark.parametrize("impl", _COMPRESSORS, ids=lambda impl: impl[0])
def test_bench_gzip_compress(
    benchmark: BenchmarkFixture,
    name_data: tuple[str, bytes],
    impl: tuple[str, Callable[[bytes], bytes]],
) -> None:
    name, data = name_data
    impl_name, compress = impl
    benchmark.group = f"gzip-compress-{name}"

    def _fn() -> bytes:
        return compress(data)

    compressed = benchmark(_fn)
    assert gzip.decompress(compressed) == data, impl_name


@pytest.mark.parametrize("name_data", _DATA, ids=lambda name_data: name_data[0])
@pytest.mark.parametrize("impl", _COMPRESSORS, ids=lambda impl: impl[0])
def test_bench_gzip_decompress(
    benchmark: BenchmarkFixture,
    name_data: tuple[str, bytes],
    impl: tuple[str, Callable[[bytes], bytes]],
) -> None:
    name, data = name_data
    impl_name, compress = impl
    compressed = compress(data)
    benchmark.group = f"gzip-decompress-{name}"

    def _fn() -> bytes:
        if impl_name == "ry":
            return bytes(ry.gzip_decode(compressed))
        return gzip.decompress(compressed)

    decompressed = benchmark(_fn)
    assert decompressed == data
