from __future__ import annotations

import fnmatch
import pathlib
import random
import re
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

_PATTERN = "*.py"
_FILENAMES_RAW = [
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
_FILENAMES = _FILENAMES_RAW * 10  # repeat to simulate load
random.shuffle(_FILENAMES)


ry_glob_match = ry.Glob(_PATTERN).is_match_str
ry_pattern_match = ry.GlobPattern(_PATTERN).matches
ry_regex_match = ry.Regex(r".*\.py$").is_match


def fnmatch_std(name: str) -> bool:
    return fnmatch.fnmatch(name, _PATTERN)


def fnmatch_case(name: str) -> bool:
    return fnmatch.fnmatchcase(name, _PATTERN)


def path_match(name: str) -> bool:
    return pathlib.Path(name).match(_PATTERN)


_py_regex = re.compile(fnmatch.translate(_PATTERN))


def regex_match(name: str) -> bool:
    return _py_regex.match(name) is not None


_FUNCTIONS = [
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
    for name in _FILENAMES:
        results = [fn(name) for fn in _FUNCTIONS]
        assert all(result == results[0] for result in results[1:]), f"Failed for {name}"


_benchmark = pytest.mark.benchmark(group="glob-fnmatch", warmup=True, min_rounds=10000)
# ===================
# == RY-BENCHMARKS ==
# ===================


@_benchmark
def test_ry_pattern(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            ry_pattern_match(name)

    benchmark(_fn)


@_benchmark
def test_ry_glob(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            ry_glob_match(name)

    benchmark(_fn)


@_benchmark
def test_ry_regex(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            ry_regex_match(name)

    benchmark(_fn)


# ===================
# == PY-BENCHMARKS ==
# ===================
@_benchmark
def test_fnmatch(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            fnmatch_std(name)

    benchmark(_fn)


@_benchmark
def test_fnmatchcase(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            fnmatch_case(name)

    benchmark(_fn)


@_benchmark
def test_regex_match(benchmark: BenchmarkFixture) -> None:
    regex = re.compile(fnmatch.translate(_PATTERN))

    def _fn() -> None:
        for name in _FILENAMES:
            _ = regex.match(name) is not None

    benchmark(_fn)


@_benchmark
@pytest.mark.skip(reason="pathlib is slow af")
def test_pathlib_match(benchmark: BenchmarkFixture) -> None:
    def _fn() -> None:
        for name in _FILENAMES:
            pathlib.Path(name).match(_PATTERN)

    benchmark(_fn)
