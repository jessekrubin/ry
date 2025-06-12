import datetime as pydt
import json
import typing as t
import uuid as pyuuid

import pytest
from hypothesis import given

import ry

from .strategies import st_json_js

_ORJSON_INSTALLED: bool = False
try:
    import orjson

    _ORJSON_INSTALLED = True
except ImportError:
    orjson = None

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
    oj_res = oj_stringify(data)
    ry_res = ry.stringify(data)
    combos = [
        (ry_res, oj_res),
        (ry_res, py_res),
    ]

    try:
        json.loads(json_str)
    except json.JSONDecodeError as e:
        emsg = f"Stringified JSON is not valid: {e}"
        raise AssertionError(emsg) from e

    for ry_res, other_res in combos:
        assert ry.parse_json(ry_res) == ry.parse_json(other_res)


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


@given(st_json_js(datetimes=True))
def test_stringify_json_orjson_compatible(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""
    _test_stringify_json_orjson_compatible(data)


@given(st_json_js(datetimes=True, finite_only=False))
def test_stringify_json_orjson_compatible_inf_nan(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings compatible with orjson."""
    _test_stringify_json_orjson_compatible(data)


def test_inf_nan_neginf():
    """Test that stringify_json handles inf, nan, and -inf correctly."""
    data = {
        "inf": float("inf"),
        "nan": float("nan"),
        "neg_inf": float("-inf"),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert parsed == dict.fromkeys(data)


def test_uuid():
    """Test that stringify_json handles inf, nan, and -inf correctly."""
    data = {
        "py_uuid": pyuuid.UUID("88475448-f091-42ef-b574-2452952931c1"),
        "ry_uuid": ry.uuid.UUID("88475448-f091-42ef-b574-2452952931c1"),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert parsed == {
        "py_uuid": "88475448-f091-42ef-b574-2452952931c1",
        "ry_uuid": "88475448-f091-42ef-b574-2452952931c1",
    }


data2test = [
    "",
    1,
    1.0,
    False,
    None,
    True,
    [1, 2, 3, 4, 5],
    [1, 2, 3, {"a": 1, "b": 2}],
    [],
    (),
    {"a": [1, 2, 3], "b": {"c": 4}},
    {"name": "ry", "version": "0.1.0", "features": ["fast", "reliable"]},
    {"name": "ry", "version": "0.1.0"},
    {},
    {
        "inf": float("inf"),
        "nan": float("nan"),
        "neg_inf": float("-inf"),
        "datetime": pydt.datetime(2023, 10, 1, 12, 0, 0),
        "date": pydt.date(2023, 10, 1),
        "time": pydt.time(12, 0, 0),
        "list": [1, 2, 3],
    },
]


@pytest.mark.parametrize("data", data2test)
def test_stringify_json_data(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings for various data types."""
    _test_stringify_json_orjson_compatible(data)
