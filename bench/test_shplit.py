from __future__ import annotations

from shlex import split as py_shplit
from typing import TYPE_CHECKING

import pytest

from ry import shplit as ry_shplit
from tests._shlex.test_shplit import SHPLITESTS

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

SHPLIT_STRINGS = [e.string for e in SHPLITESTS]


@pytest.mark.benchmark(group="shplit")
def test_benchmark_py_shplit(benchmark: BenchmarkFixture):
    benchmark(lambda: [py_shplit(e) for e in SHPLIT_STRINGS])


@pytest.mark.benchmark(group="shplit")
def test_benchmark_ry_shplit(benchmark: BenchmarkFixture):
    benchmark(lambda: [ry_shplit(e) for e in SHPLIT_STRINGS])
