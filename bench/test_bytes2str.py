from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import ry

ALL_BYTES = bytes([i for i in range(256)])
ALL_BYTES_10X = ALL_BYTES * 10


def test_benchmark_not_debug() -> None:
    assert not ry.__build_profile__ == "release"


class TestBytes2String:
    @pytest.mark.benchmark(group="bytes2str")
    def test_python_bytes_string(self, benchmark: BenchmarkFixture) -> None:
        def bytes2str(b):
            return len(b.__repr__())

        benchmark(bytes2str, ALL_BYTES_10X)

    @pytest.mark.benchmark(group="bytes2str")
    def test_ry_bytes_string(self, benchmark: BenchmarkFixture) -> None:
        ry_b = ry.Bytes(ALL_BYTES_10X)

        def bytes2str(b):
            return len(b.__repr__())

        benchmark(bytes2str, ry_b)
