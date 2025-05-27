from __future__ import annotations

import json

import pytest
from pytest_benchmark.fixture import BenchmarkFixture

import ry as ry

ORJSON_AVAILABLE = False
try:
    import orjson

    ORJSON_AVAILABLE = True
except ImportError:
    ...


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json_bytes, b"[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes_orjson(benchmark: BenchmarkFixture):
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, b"[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json, "[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str_orjson(benchmark: BenchmarkFixture):
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, "[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str_or_bytes")
def test_benchmark_parse_str_or_bytes(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json, "[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, "[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, b"[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str_or_bytes")
def test_benchmark_parse_str_or_bytes_stdlib(benchmark: BenchmarkFixture):
    benchmark(json.loads, "[true, false, null, 123, 456.7]")
