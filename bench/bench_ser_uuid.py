from __future__ import annotations

import uuid as pyuuid
from functools import cache
from typing import TYPE_CHECKING

import pytest

import ry
import ry.uuid as ryuuid

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_UUID_STR = "550e8400-e29b-41d4-a716-446655440000"
_SIZES = [1, 10, 100, 1_000]


@cache
def _py_uuid_list(size: int) -> list[pyuuid.UUID]:
    return [pyuuid.uuid4() for _ in range(size)]


@cache
def _ry_uuid_list(size: int) -> list[ryuuid.UUID]:
    return [ryuuid.uuid4() for _ in range(size)]


@pytest.mark.benchmark(group="uuid-ser-single")
def test_bench_stringify_single_py_uuid(benchmark: BenchmarkFixture) -> None:
    u = pyuuid.UUID(_UUID_STR)
    benchmark(ry.stringify, u)


@pytest.mark.benchmark(group="uuid-ser-single")
def test_bench_stringify_single_ry_uuid(benchmark: BenchmarkFixture) -> None:
    u = ryuuid.UUID(_UUID_STR)
    benchmark(ry.stringify, u)


@pytest.mark.benchmark(group="uuid-ser-arr-py")
@pytest.mark.parametrize("size", _SIZES)
def test_bench_stringify_py_uuid_arr(benchmark: BenchmarkFixture, size: int) -> None:
    data = _py_uuid_list(size)
    benchmark(ry.stringify, data)


@pytest.mark.benchmark(group="uuid-ser-arr-ry")
@pytest.mark.parametrize("size", _SIZES)
def test_bench_stringify_ry_uuid_arr(benchmark: BenchmarkFixture, size: int) -> None:
    data = _ry_uuid_list(size)
    benchmark(ry.stringify, data)
