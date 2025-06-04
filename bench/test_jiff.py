from __future__ import annotations

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import ry


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_datetime(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.datetime(2020, 2, 29, 12, 30, 45))


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_no_tz(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.zoned(2020, 2, 29, 12, 30, 45))


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_la(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.zoned(2020, 2, 29, 12, 30, 45, tz="America/Los_Angeles"))
