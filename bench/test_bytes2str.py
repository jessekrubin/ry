from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

ALL_BYTES = bytes(list(range(256)))
ALL_BYTES_10X = ALL_BYTES * 10


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
