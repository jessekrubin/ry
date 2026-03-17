from __future__ import annotations

import random
from functools import cache
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_RANDOM_FLOAT = 0.12345678901234567890
_SIZES = [1, 10, 100, 1_000, 10_000]


def _random_float(min_value: float = 0.0, max_value: float = 1.0) -> float:
    """Generate a random float between min_value and max_value."""
    return random.uniform(min_value, max_value)  # noqa: S311


@cache
def _py_float_list(size: int) -> list[float]:
    return [_random_float(min_value=-1e10, max_value=1e10) for _ in range(size)]


@pytest.mark.benchmark(group="float-ser-single")
def test_bench_stringify_single_py_float(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.stringify, _RANDOM_FLOAT)


@pytest.mark.benchmark(group="float-ser-arr-py")
@pytest.mark.parametrize("size", _SIZES)
def test_bench_stringify_py_float_arr(benchmark: BenchmarkFixture, size: int) -> None:
    data = _py_float_list(size)
    benchmark(ry.stringify, data)
