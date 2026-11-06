from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_ALL_BYTES = bytes(list(range(256)))
_ALL_BYTES_10X = _ALL_BYTES * 10


class TestBytes2String:
    @pytest.mark.benchmark(group="bytes-repr")
    def test_python_bytes_string(self, benchmark: BenchmarkFixture) -> None:
        def bytes2str(b: bytes) -> int:
            return len(repr(b))

        benchmark(bytes2str, _ALL_BYTES_10X)

    @pytest.mark.benchmark(group="bytes-repr")
    def test_ry_bytes_string(self, benchmark: BenchmarkFixture) -> None:
        ry_b = ry.Bytes(_ALL_BYTES_10X)

        def bytes2str(b: ry.Bytes) -> int:
            return len(repr(b))

        benchmark(bytes2str, ry_b)
