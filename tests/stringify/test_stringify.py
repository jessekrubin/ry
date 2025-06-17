import datetime as pydt
import json
import typing as t
import uuid as pyuuid

import pytest

import ry

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


def test_inf_nan_neginf() -> None:
    """Test that stringify_json handles inf, nan, and -inf correctly."""
    data = {
        "inf": float("inf"),
        "nan": float("nan"),
        "neg_inf": float("-inf"),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert parsed == dict.fromkeys(data)


def test_typed_dict() -> None:
    """Test that `ry.stringify` handles TypedDicts correctly."""

    class Point(t.TypedDict):
        x: int
        y: int

    data = {
        "point": Point(x=1, y=2),
        "point2": Point(x=3, y=4),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert parsed == {
        "point": {"x": 1, "y": 2},
        "point2": {"x": 3, "y": 4},
    }


def test_namedtuples() -> None:
    """Test that `ry.stringify` handles namedtuples correctly."""

    class Point(t.NamedTuple):
        x: int
        y: int

    data = {
        "point": Point(1, 2),
        "point2": Point(3, 4),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert parsed == {
        "point": [1, 2],
        "point2": [3, 4],
    }


def test_set_and_frozenset() -> None:
    """Test that `ry.stringify` handles sets correctly."""
    data = {
        "set": {1, 2, 3},
        "frozenset": frozenset({"a", "b", "c"}),
    }
    json_bytes = ry.stringify(data)
    parsed = ry.parse_json(json_bytes)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert isinstance(parsed["set"], list), "Set should be converted to a list"
    assert isinstance(parsed["frozenset"], list), (
        "Frozenset should be converted to a list"
    )
    assert set(parsed["set"]) == {1, 2, 3}, "Should be eq to original set"
    assert set(parsed["frozenset"]) == {"a", "b", "c"}, (
        "Should be eq to original frozenset"
    )


def test_uuid() -> None:
    """Test that `ry.stringify` handles UUIDs correctly as values"""
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


def test_uuid_keys() -> None:
    data = {
        # as keys - different namespaces bc diff keys
        pyuuid.NAMESPACE_DNS: "py",
        ry.uuid.NAMESPACE_URL: "ry",
    }
    with pytest.raises(TypeError):
        _json_bytes = ry.stringify(data)
        return
    # parsed = ry.parse_json(json_bytes)
    # assert parsed == {
    #     str(pyuuid.NAMESPACE_DNS): "py",
    #     str(ry.uuid.NAMESPACE_URL): "ry",
    # }


PYTYPES_JSON_SER = [
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


@pytest.mark.parametrize("data", PYTYPES_JSON_SER)
def test_stringify_json_data(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings for various data types."""
    _test_stringify_json_orjson_compatible(data)


RYTYPES_JSON_SER = {
    # uuid ~ ryo3-uuid
    "uuid": ry.uuid.UUID("88475448-f091-42ef-b574-2452952931c1"),
    # ulid ~ ryo3-ulid
    "ulid": ry.ulid.ULID("01H7Z5F8Y3V9G4J6K8D5E6F7G8"),
    # url ~ ryo3-url
    "url": ry.URL("https://example.com"),
    # jiff ~ ryo3-jiff
    "date": ry.date(2020, 8, 26),
    "datetime": ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    "+signed_duration": ry.SignedDuration(3),
    "-signed_duration": -ry.SignedDuration(3),
    "time": ry.time(6, 27, 0, 0),
    "timespan": ry.timespan(weeks=1),
    "timestamp": ry.Timestamp.from_millisecond(1598438400000),
    "zoned": ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
    # "offset": ry.Offset(1),
    # "iso_week_date": ry.date(2020, 8, 26).iso_week_date(),
}
EXPECTED = {
    "uuid": "88475448-f091-42ef-b574-2452952931c1",
    "ulid": "01H7Z5F8Y3V9G4J6K8D5E6F7G8",
    "url": "https://example.com/",
    "date": "2020-08-26",
    "datetime": "2020-08-26T06:27:00",
    "+signed_duration": "PT3S",
    "-signed_duration": "-PT3S",
    "time": "06:27:00",
    "timespan": "P1W",
    "timestamp": "2020-08-26T10:40:00Z",
    "zoned": "2020-08-26T06:27:00-04:00[America/New_York]",
}


def test_stringify_ry_types() -> None:
    """Test that `stringify` handles ry types correctly."""
    res = ry.stringify(RYTYPES_JSON_SER, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"

    def _format_different() -> str:
        different_vals = {
            k: {"expected": EXPECTED[k], "actual": v}
            for k, v in parsed.items()
            if EXPECTED.get(k) != v
        }
        return "\n".join(
            f"{k}: expected `{v['expected']}`, got `{v['actual']}`"
            for k, v in different_vals.items()
        )

    assert parsed == EXPECTED, (
        f"Parsed JSON does not match expected result: \n{_format_different()}\n"
    )
