from __future__ import annotations

import fnmatch
import pathlib
import random
import re
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

PATTERN = "*.py"
FILENAMES_RAW = [
    "src/main.py",
    "src/utils/helpers.py",
    "src/lib/math_ops.py",
    "src/data/input.csv",
    "docs/readme.md",
    "setup.py",
    "tests/test_main.py",
    "tests/test_utils.py",
    "something_else.txt",
    "README.md",
    "READYOU.md",
    "READTHEM.md",
    "READHIM.md",
    "READHER.md",
    "WOOFME.md",
]
FILENAMES = FILENAMES_RAW * 10  # repeat to simulate load
random.shuffle(FILENAMES)


ry_glob_match = ry.Glob(PATTERN).is_match_str
ry_pattern_match = ry.Pattern(PATTERN).matches
ry_regex_match = ry.Regex(r".*\.py$").is_match


def fnmatch_std(name: str) -> bool:
    return fnmatch.fnmatch(name, PATTERN)


def fnmatch_case(name: str) -> bool:
    return fnmatch.fnmatchcase(name, PATTERN)


def path_match(name: str) -> bool:
    return pathlib.Path(name).match(PATTERN)


_py_regex = re.compile(fnmatch.translate(PATTERN))


def regex_match(name: str) -> bool:
    return _py_regex.match(name) is not None


FUNCTIONS = [
    fnmatch_std,
    fnmatch_case,
    path_match,
    regex_match,
    ry_glob_match,
    ry_pattern_match,
    ry_regex_match,
]


def test_all_equiv() -> None:
    """Test that all functions return the same result."""
    for name in FILENAMES:
        results = [fn(name) for fn in FUNCTIONS]
        assert all(result == results[0] for result in results[1:]), f"Failed for {name}"


_benchmark = pytest.mark.benchmark(group="fnmatch", warmup=True, min_rounds=10000)
# ===================
# == RY-BENCHMARKS ==
# ===================


@_benchmark
def test_ry_pattern(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            ry_pattern_match(name)


@_benchmark
def test_ry_glob(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            ry_glob_match(name)


@_benchmark
def test_ry_regex(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            ry_regex_match(name)


# ===================
# == PY-BENCHMARKS ==
# ===================
@_benchmark
def test_fnmatch(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            fnmatch_std(name)


@_benchmark
def test_fnmatchcase(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            fnmatch_case(name)


@_benchmark
def test_regex_match(benchmark: BenchmarkFixture) -> None:
    regex = re.compile(fnmatch.translate(PATTERN))

    @benchmark
    def _fn():
        for name in FILENAMES:
            _ = regex.match(name) is not None


@_benchmark
@pytest.mark.skip(reason="pathlib is slow af")
def test_pathlib_match(benchmark: BenchmarkFixture) -> None:
    @benchmark
    def _fn():
        for name in FILENAMES:
            pathlib.Path(name).match(PATTERN)
