from __future__ import annotations

import random
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    import datetime as pydt

    from pytest_benchmark.fixture import BenchmarkFixture

_T = t.TypeVar("_T")


class _TestData:
    @staticmethod
    def _arr2dict_fn(
        arr_fn: t.Callable[[int], list[_T]],
    ) -> t.Callable[[int], dict[str, _T]]:
        def fn(size: int = 1000) -> dict[str, _T]:
            return {f"key-{i}": el for i, el in enumerate(arr_fn(size))}

        return fn

    @staticmethod
    def bool_array(size: int = 1000) -> list[bool]:
        return [i % 2 == 0 for i in range(size)]

    @staticmethod
    def bool_dict(size: int = 1000) -> dict[str, bool]:
        return _TestData._arr2dict_fn(_TestData.bool_array)(size)

    @staticmethod
    def str_array(size: int = 1000) -> list[str]:
        return [f"string_{i}" for i in range(size)]

    @staticmethod
    def str_dict(size: int = 1000) -> dict[str, str]:
        return _TestData._arr2dict_fn(_TestData.str_array)(size)

    @staticmethod
    def none_array(size: int = 1000) -> list[None]:
        return [None] * size

    @staticmethod
    def none_dict(size: int = 1000) -> dict[str, None]:
        return _TestData._arr2dict_fn(_TestData.none_array)(size)

    @staticmethod
    def int_array(size: int = 1000) -> list[int]:
        ints = list(range(size))
        random.shuffle(ints)
        return ints

    @staticmethod
    def int_dict(size: int = 1000) -> dict[str, int]:
        return _TestData._arr2dict_fn(_TestData.int_array)(size)

    @staticmethod
    def float_array(size: int = 1000) -> list[float]:
        return [float(i) + 0.1 for i in range(size)]

    @staticmethod
    def float_dict(size: int = 1000) -> dict[str, float]:
        return _TestData._arr2dict_fn(_TestData.float_array)(size)

    @staticmethod
    def date_array(size: int = 1000) -> list[pydt.date]:
        start_date = ry.date(2024, 1, 1)
        return [e.to_py() for e in start_date.series(ry.timespan(days=1)).take(size)]

    @staticmethod
    def date_dict(size: int = 1000) -> dict[str, pydt.date]:
        return _TestData._arr2dict_fn(_TestData.date_array)(size)

    @staticmethod
    def time_array(size: int = 1000) -> list[pydt.time]:
        start_time = ry.time(0, 0, 0)
        return [e.to_py() for e in start_time.series(ry.timespan(seconds=1)).take(size)]

    @staticmethod
    def time_dict(size: int = 1000) -> dict[str, pydt.time]:
        return _TestData._arr2dict_fn(_TestData.time_array)(size)

    @staticmethod
    def mixed_array(size: int = 1000) -> list[t.Any]:
        arr = []
        for i in range(size):
            if i % 4 == 0:
                arr.append(f"string_{i}")
            elif i % 4 == 1:
                arr.append(i)
            elif i % 4 == 2:
                arr.append(float(i) + 0.1)
            else:
                arr.append(None)
        random.shuffle(arr)
        return arr

    @staticmethod
    def mixed_dict(size: int = 1000) -> dict[str, t.Any]:
        return _TestData._arr2dict_fn(_TestData.mixed_array)(size)


@pytest.mark.benchmark(group="stringify")
@pytest.mark.parametrize(
    "data_fn",
    [
        getattr(_TestData, fn)
        for fn in dir(_TestData)
        if callable(getattr(_TestData, fn)) and not fn.startswith("_")
    ],
)
def test_bench_serialize(
    benchmark: BenchmarkFixture, data_fn: t.Callable[[], t.Any]
) -> None:
    # set the group
    benchmark.group = data_fn.__name__
    data = data_fn()
    benchmark(ry.stringify, data)
