from __future__ import annotations

import json
import typing as t

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

from ..strategies import st_json_js

_ORJSON_INSTALLED: bool = False
try:
    import orjson

    _ORJSON_INSTALLED = True
except ImportError:
    orjson = None  # type: ignore[assignment]

pytest_mark_skip_orjson = pytest.mark.skipif(
    not _ORJSON_INSTALLED,
    reason="orjson is not installed, skipping tests that require it",
)


def py_stringify(data: t.Any) -> bytes:
    """Convert data to a JSON string using Python's built-in json module."""
    return json.dumps(data, separators=(",", ":")).encode()


def oj_stringify(data: t.Any) -> bytes:
    """Convert data to a JSON string using orjson."""
    return orjson.dumps(data)


def _test_stringify_json(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings."""

    json_bytes = ry.stringify(data)
    assert isinstance(json_bytes, ry.Bytes), "Result should be a `ry.Bytes`"

    json_str = json_bytes.decode("utf-8")

    py_res = py_stringify(data)
    ry_res = ry.stringify(data)

    try:
        json.loads(json_str)
    except json.JSONDecodeError as e:
        emsg = f"Stringified JSON is not valid: {e}"
        raise AssertionError(emsg) from e

    assert ry.parse_json(ry_res) == ry.parse_json(py_res)


@given(st_json_js())
def test_stringify_json(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings."""
    _test_stringify_json(data)


def _test_stringify_json_orjson_compatible(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""

    json_bytes = ry.stringify(data)
    assert isinstance(json_bytes, ry.Bytes), "Result should be a `ry.Bytes`"
    oj_res = oj_stringify(data)

    json_str = json_bytes.decode("utf-8")

    try:
        orjson.loads(json_str)
    except orjson.JSONDecodeError as e:
        emsg = f"Stringified JSON is not valid for orjson: {e}"
        raise AssertionError(emsg) from e

    oj_parsed = orjson.loads(oj_res)
    ry_parsed = ry.parse_json(json_bytes)
    assert ry_parsed == oj_parsed, (
        "Parsed JSON from ry.stringify does not match orjson parsed result"
    )


@given(st_json_js(datetimes=False))
@pytest_mark_skip_orjson
def test_stringify_json_orjson_compatible(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""
    _test_stringify_json_orjson_compatible(data)


@given(st_json_js(datetimes=False, finite_only=False))
@pytest_mark_skip_orjson
def test_stringify_json_orjson_compatible_inf_nan(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""
    _test_stringify_json_orjson_compatible(data)


@given(st.datetimes())
@pytest_mark_skip_orjson
def test_stringify_datetimes(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""
    # strip the quotes
    ry_json = ry.stringify(data, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data).decode().strip('"')
    assert ry.DateTime.parse(ry_json) == ry.DateTime.parse(oj_json)


@given(st.dates())
@pytest_mark_skip_orjson
def test_stringify_dates(data: t.Any) -> None:
    """Test orjson/ry.stringify for dates."""
    ry_json = ry.stringify(data, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data).decode().strip('"')
    assert ry.Date.parse(ry_json) == ry.Date.parse(oj_json)


@given(st.times())
@pytest_mark_skip_orjson
def test_stringify_times(data: t.Any) -> None:
    """Test orjson/ry.stringify for dates."""
    ry_json = ry.stringify(data, pybytes=True).decode().strip('"')
    oj_json = oj_stringify(data).decode().strip('"')
    assert ry.Time.parse(ry_json) == ry.Time.parse(oj_json)
