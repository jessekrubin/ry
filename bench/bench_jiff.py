from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_datetime(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.datetime, 2020, 2, 29, 12, 30, 45)


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_no_tz(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.zoned, 2020, 2, 29, 12, 30, 45)


@pytest.mark.benchmark(group="jiff-constructors")
def test_fn_zoned_la(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.zoned, 2020, 2, 29, 12, 30, 45, tz="America/Los_Angeles")
