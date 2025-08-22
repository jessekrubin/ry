from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_datetime(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.datetime(2020, 2, 29, 12, 30, 45))


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_no_tz(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.zoned(2020, 2, 29, 12, 30, 45))


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_la(benchmark: BenchmarkFixture):
    benchmark(lambda: ry.zoned(2020, 2, 29, 12, 30, 45, tz="America/Los_Angeles"))
