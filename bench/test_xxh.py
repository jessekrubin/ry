from __future__ import annotations

import typing as t
from typing import TYPE_CHECKING

import pytest

from ry import xxhash as ry_xxh

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

from dataclasses import dataclass


@dataclass(kw_only=True)
class _XxHashImpl:
    name: str
    xxhasher: type[ry_xxh.xxh32 | ry_xxh.xxh64 | ry_xxh.xxh3_64 | ry_xxh.xxh3_128]
    int_fn: t.Callable[[bytes], int]
    bytes_fn: t.Callable[[bytes], bytes]
    hex_fn: t.Callable[[bytes], str]


_XXHASH_IMPLS = [
    _XxHashImpl(
        name="xxh32",
        xxhasher=ry_xxh.xxh32,
        int_fn=ry_xxh.xxh32_intdigest,
        bytes_fn=ry_xxh.xxh32_digest,
        hex_fn=ry_xxh.xxh32_hexdigest,
    ),
    _XxHashImpl(
        name="xxh64",
        xxhasher=ry_xxh.xxh64,
        int_fn=ry_xxh.xxh64_intdigest,
        bytes_fn=ry_xxh.xxh64_digest,
        hex_fn=ry_xxh.xxh64_hexdigest,
    ),
    _XxHashImpl(
        name="xxh3_64",
        xxhasher=ry_xxh.xxh3_64,
        int_fn=ry_xxh.xxh3_64_intdigest,
        bytes_fn=ry_xxh.xxh3_64_digest,
        hex_fn=ry_xxh.xxh3_64_hexdigest,
    ),
    _XxHashImpl(
        name="xxh3_128",
        xxhasher=ry_xxh.xxh3_128,
        int_fn=ry_xxh.xxh3_128_intdigest,
        bytes_fn=ry_xxh.xxh3_128_digest,
        hex_fn=ry_xxh.xxh3_128_hexdigest,
    ),
]

_DATA = bytes(range(256)) * 10


@pytest.mark.benchmark(group="xxhash-intdigest")
@pytest.mark.parametrize("xxhasher", _XXHASH_IMPLS)
def test_intdigest(
    benchmark: BenchmarkFixture,
    xxhasher: _XxHashImpl,
) -> None:
    benchmark.group = xxhasher.name
    benchmark(lambda: xxhasher.int_fn(_DATA))


@pytest.mark.benchmark(group="xxhash-bytesdigest")
@pytest.mark.parametrize("xxhasher", _XXHASH_IMPLS)
def test_bytesdigest(
    benchmark: BenchmarkFixture,
    xxhasher: _XxHashImpl,
) -> None:
    benchmark.group = xxhasher.name
    benchmark(lambda: xxhasher.bytes_fn(_DATA))


@pytest.mark.benchmark(group="xxhash-hexdigest")
@pytest.mark.parametrize("xxhasher", _XXHASH_IMPLS)
def test_hexdigest(
    benchmark: BenchmarkFixture,
    xxhasher: _XxHashImpl,
) -> None:
    benchmark.group = xxhasher.name
    benchmark(lambda: xxhasher.hex_fn(_DATA))
