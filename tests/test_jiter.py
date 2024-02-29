from __future__ import annotations
import pytest
from pytest_benchmark.fixture import BenchmarkFixture
import ry as ry
import json


def test_parse_bytes():
    assert ry.parse_json(b"[true, false, null, 123, 456.7]") == [
        True,
        False,
        None,
        123,
        456.7,
    ]
    assert ry.parse_json(b'{"foo": "bar"}') == {"foo": "bar"}


def test_parse_str():
    assert ry.parse_json("[true, false, null, 123, 456.7]") == [
        True,
        False,
        None,
        123,
        456.7,
    ]
    assert ry.parse_json('{"foo": "bar"}') == {"foo": "bar"}


# benchmarks


@pytest.mark.benchmark(group="parse_bytes")
def test_benchmark_parse_bytes(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json_bytes, b"[true, false, null, 123, 456.7]")


@pytest.mark.benchmark(group="parse_str")
def test_benchmark_parse_str(benchmark: BenchmarkFixture):
    benchmark(ry.parse_json_str, "[true, false, null, 123, 456.7]")


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
