from __future__ import annotations

import random
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    import datetime as pydt

    from pytest_benchmark.fixture import BenchmarkFixture


class _TestData:
    @staticmethod
    def bool_array(size: int = 100) -> list[bool]:
        return [i % 2 == 0 for i in range(size)]

    @staticmethod
    def int_array(size: int = 100) -> list[int]:
        ints = list(range(size))
        random.shuffle(ints)
        return ints

    @staticmethod
    def float_array(size: int = 100) -> list[float]:
        return [float(i) + 0.1 for i in range(size)]

    @staticmethod
    def date_array(size: int = 100) -> list[pydt.date]:
        start_date = ry.date(2024, 1, 1)
        return [e.to_py() for e in start_date.series(ry.timespan(days=1)).take(size)]

    @staticmethod
    def time_array(size: int = 100) -> list[pydt.time]:
        start_time = ry.time(0, 0, 0)
        return [e.to_py() for e in start_time.series(ry.timespan(seconds=1)).take(size)]


@pytest.mark.benchmark(group="stringify")
@pytest.mark.parametrize(
    "data_fn",
    [
        _TestData.bool_array,
        _TestData.date_array,
        _TestData.time_array,
        _TestData.int_array,
        _TestData.float_array,
    ],
)
def test_bench_serialize(
    benchmark: BenchmarkFixture, data_fn: t.Callable[[], t.Any]
) -> None:
    data = data_fn()
    benchmark(ry.stringify, data)
