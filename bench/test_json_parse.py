from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

import ry as ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

ORJSON_AVAILABLE = False
try:
    import orjson

    ORJSON_AVAILABLE = True
except ImportError:
    ...

REPO_ROOT = Path(__file__).parent.parent
JSON_STRING = (REPO_ROOT / "package.json").read_text(encoding="utf-8")
JSON_BYTES = JSON_STRING.encode()


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes_orjson(benchmark: BenchmarkFixture):
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, JSON_BYTES)


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json, JSON_STRING)


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str_orjson(benchmark: BenchmarkFixture):
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, JSON_STRING)


@pytest.mark.benchmark(group="parse_str_or_bytes")
def test_benchmark_parse_str_or_bytes(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json, JSON_STRING)


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, JSON_STRING)


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, JSON_BYTES)


@pytest.mark.benchmark(group="parse_str_or_bytes")
def test_benchmark_parse_str_or_bytes_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, JSON_STRING)
