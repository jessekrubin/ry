from __future__ import annotations

import typing as t
from shlex import split as py_shplit

import pytest

from ry import shplit as ry_shplit
from tests._shlex.test_shplit import SHPLITESTS

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

SHPLIT_STRINGS = [e.string for e in SHPLITESTS]


@pytest.mark.benchmark(group="shplit")
def test_benchmark_py_shplit(benchmark: BenchmarkFixture) -> None:
    benchmark(lambda: [py_shplit(e) for e in SHPLIT_STRINGS])


@pytest.mark.benchmark(group="shplit")
def test_benchmark_ry_shplit(benchmark: BenchmarkFixture) -> None:
    benchmark(lambda: [ry_shplit(e) for e in SHPLIT_STRINGS])
