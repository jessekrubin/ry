from __future__ import annotations

import json
import typing as t
from pathlib import Path

import pytest

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture

ORJSON_AVAILABLE = False
try:
    import orjson

    ORJSON_AVAILABLE = True
except ImportError:
    ...

_REPO_ROOT = Path(__file__).parent.parent
_JSON_STRING = (_REPO_ROOT / "package.json").read_text(encoding="utf-8")
_JSON_BYTES = _JSON_STRING.encode()


def test_parse_bytes_orjson(benchmark: BenchmarkFixture) -> None:
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, _JSON_BYTES)


def test_parse_str(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.parse_json, _JSON_STRING)


def test_parse_bytes(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.parse_json, _JSON_BYTES)


def test_parse_str_orjson(benchmark: BenchmarkFixture) -> None:
    if not ORJSON_AVAILABLE:
        pytest.skip("orjson is not available")
    benchmark(orjson.loads, _JSON_STRING)


def test_parse_str_or_bytes(benchmark: BenchmarkFixture) -> None:
    benchmark(ry.parse_json, _JSON_STRING)


def test_parse_str_stdlib(benchmark: BenchmarkFixture) -> None:
    benchmark(json.loads, _JSON_STRING)


def test_parse_bytes_stdlib(benchmark: BenchmarkFixture) -> None:
    benchmark(json.loads, _JSON_BYTES)


def test_parse_str_or_bytes_stdlib(benchmark: BenchmarkFixture) -> None:
    benchmark(json.loads, _JSON_STRING)
