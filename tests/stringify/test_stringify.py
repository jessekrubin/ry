from __future__ import annotations

import datetime as pydt
import json
import sys
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
    if orjson is None:
        msg = "orjson is not installed, cannot use oj_stringify"
        raise ImportError(msg)
    return orjson.dumps(data)


class TestStringifyRecursion:
    def _depth(self, d: dict[str, t.Any] | tuple[t.Any, ...] | list[t.Any]) -> int:
        if isinstance(d, dict) and d:
            return 1 + self._depth(next(iter(d.values())))
        elif isinstance(d, list) and d:
            return 1 + self._depth(d[0])
        elif isinstance(d, tuple) and d:
            return 1 + self._depth(d[0])
        return 0

    def test_stringify_recursive_dict_infinite(self) -> None:
        """Test that stringify raises `RecursionError` for recursive data structures."""
        a = {
            "k": "v",
        }
        b = {
            "a": a,
        }
        a["b"] = b  # type: ignore[assignment]
        with pytest.raises(RecursionError, match="Recursion limit reached"):
            _r = ry.stringify(a)

    def test_ry_recursion_dictionary(self) -> None:
        data: dict[str, t.Any] = {"a": 123}
        for _i in range(254):
            data = {"a": data}
        assert self._depth(data) == 255
        a = ry.stringify(data).decode()
        assert isinstance(a, str)

        with pytest.raises(RecursionError):
            _a = ry.stringify({"a": data})

    def test_recursion_tuple(self) -> None:
        data: tuple[t.Any, ...] = (123,)
        for _i in range(254):
            data = (data,)

        assert self._depth(data) == 255
        a = ry.stringify(data).decode()
        assert isinstance(a, str)

        with pytest.raises(RecursionError):
            _a = ry.stringify((data,))

    def test_recursion_list(self) -> None:
        data: list[t.Any] = [123]
        for _i in range(254):
            data = [data]
        assert self._depth(data) == 255
        a = ry.stringify(data).decode()
        assert isinstance(a, str)

        with pytest.raises(RecursionError):
            _a = ry.stringify([data])


def test_stringify_pybytes_output() -> None:
    data = {
        "key": "val",
        "list": [1, 2, 3],
        "dict": {"a": 1, "b": 2},
    }
    rs_bytes = ry.stringify(data)
    assert isinstance(rs_bytes, ry.Bytes), "Result should be a `ry.Bytes`"
    py_bytes = ry.stringify(data, pybytes=True)
    assert isinstance(py_bytes, bytes), "Result should be a `bytes`"
    parsed_py = ry.parse_json(py_bytes)
    parsed_rs = ry.parse_json(rs_bytes)
    assert parsed_py == parsed_rs


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
    try:
        oj_res = oj_stringify(data)
    except TypeError as _e:
        return  # orjson does not support this data type, skip the test

    assert isinstance(json_bytes, ry.Bytes), "Result should be a `ry.Bytes`"

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


def test_ellipsis() -> None:
    """Test that stringify_json raises TypeError for Ellipsis."""
    data = {"key": Ellipsis}
    res = ry.stringify(data)
    assert isinstance(res, ry.Bytes), "Result should be a `ry.Bytes`"
    assert res == b'{"key":null}', "Ellipsis should be serialized as null"


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


PYTYPES_JSON_SER = [
    "",
    1,
    1.0,
    False,
    None,
    True,
    [1, 2, 3, 4, 5],
    (1, 2, 3, 4, 5),
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
        "list": [1, 2, 3],
        "date": pydt.date(2023, 10, 1),
        "datetime": pydt.datetime(2023, 10, 1, 12, 0, 0),
        "time": pydt.time(12, 0, 0),
        "timedelta": pydt.timedelta(days=1, seconds=3600),
        # TODO: add tzinfo/timezone? "tzinfo": pydt.datetime.now(pydt.timezone.utc).tzinfo,
    },
]


@pytest.mark.parametrize("data", PYTYPES_JSON_SER)
@pytest_mark_skip_orjson
def test_stringify_json_data(data: t.Any) -> None:
    """Test that stringify_json produces valid JSON strings for various data types."""
    _test_stringify_json_orjson_compatible(data)


RYTYPES_JSON_SER = {
    # std-time
    "duration": ry.Duration(secs=1),
    # uuid ~ ryo3-uuid
    "uuid": ry.uuid.UUID("88475448-f091-42ef-b574-2452952931c1"),
    # ulid ~ ryo3-ulid
    "ulid": ry.ulid.ULID("01H7Z5F8Y3V9G4J6K8D5E6F7G8"),
    # url ~ ryo3-url
    "url": ry.URL("https://example.com"),
    # http
    "headers": ry.Headers({
        "Content-Type": "application/json",
        "Accept": "application/json",
        "X-Content-Type-Options": "nosniff",
    }),
    "http-status": ry.HttpStatus(200),
    # jiff ~ ryo3-jiff
    "date": ry.date(2020, 8, 26),
    "datetime": ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    "+signed_duration": ry.SignedDuration(3),
    "-signed_duration": -ry.SignedDuration(3),
    "time": ry.time(6, 27, 0, 0),
    "timespan": ry.timespan(weeks=1),
    "timestamp": ry.Timestamp.from_millisecond(1598438400000),
    "timezone": ry.TimeZone("America/New_York"),
    "zoned": ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
}
EXPECTED = {
    "duration": "PT1S",
    "uuid": "88475448-f091-42ef-b574-2452952931c1",
    "ulid": "01H7Z5F8Y3V9G4J6K8D5E6F7G8",
    "url": "https://example.com/",
    "headers": {
        "accept": "application/json",
        "content-type": "application/json",
        "x-content-type-options": "nosniff",
    },
    "http-status": 200,
    "date": "2020-08-26",
    "datetime": "2020-08-26T06:27:00",
    "+signed_duration": "PT3S",
    "-signed_duration": "-PT3S",
    "time": "06:27:00",
    "timespan": "P1W",
    "timestamp": "2020-08-26T10:40:00Z",
    "timezone": "America/New_York",
    "zoned": "2020-08-26T06:27:00-04:00[America/New_York]",
}


def test_stringify_ry_types() -> None:
    """Test that `stringify` handles ry types correctly."""
    res = ry.stringify(RYTYPES_JSON_SER, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    parsed_dict: dict[str, t.Any] = t.cast("dict[str, t.Any]", parsed)

    def _format_different() -> str:
        different_vals = {
            k: {
                "expected": EXPECTED.get(k, f"Expected value for {k} not found"),
                "actual": v,
            }
            for k, v in parsed_dict.items()
            if EXPECTED.get(k) != v
        }
        return "\n".join(
            f"{k}: expected `{v['expected']}`, got `{v['actual']}`"
            for k, v in different_vals.items()
        )

    assert parsed_dict == EXPECTED, (
        f"Parsed JSON does not match expected result: \n{_format_different()}\n"
    )


def test_stringify_some_mapping() -> None:
    """Test that `stringify` handles some mapping types correctly."""
    data = {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
    }

    class SomeMapping(t.Mapping[str, str]):
        def __init__(self, data: dict[str, str]) -> None:
            self._data = data

        def __getitem__(self, key: str) -> str:
            return self._data[key]

        def __iter__(self) -> t.Iterator[str]:
            return iter(self._data)

        def __len__(self) -> int:
            return len(self._data)

    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed == data, (
        f"Parsed JSON does not match original data: {parsed} != {data}"
    )


def test_stringify_deque() -> None:
    """Test that `stringify` handles deque correctly."""
    from collections import deque

    data = {
        "key1": "value1",
        "key2": "value2",
        "key3": deque(["a", "b", "c"]),
    }
    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed["key3"] == ["a", "b", "c"], (
        f"Parsed JSON does not match original deque: {parsed['key3']} != ['a', 'b', 'c']"
    )


class TestStringifyDefault:
    class SomeSTupidCustomType:
        value: str

        def __init__(self, value: str) -> None:
            self.value = value

        def __repr__(self) -> str:
            return f"{self.__class__.__name__}({self.value})"

    def test_stringify_custom_type_no_default_throws_err(self) -> None:
        """Test that stringify raises an error for custom types without a default."""

        with pytest.raises(TypeError, match="Failed to serialize"):
            ry.stringify(self.SomeSTupidCustomType("test"))

    def test_stringify_custom_type_with_default(self) -> None:
        """Test that stringify works for custom types with a default."""

        def _default_fn(obj: t.Any) -> t.Any:
            if isinstance(obj, self.SomeSTupidCustomType):
                return obj.value
            msg = f"Cannot serialize {obj}"
            raise TypeError(msg)

        data = {
            "key1": "value1",
            "key2": self.SomeSTupidCustomType("test"),
        }
        res = ry.stringify(data, default=_default_fn, fmt=True)
        parsed = ry.parse_json(res)
        assert isinstance(parsed, dict), "Parsed result should be a dictionary"
        assert parsed["key2"] == "test", (
            f"Parsed JSON does not match original custom type: {parsed['key2']} != 'test'"
        )

    def test_stringify_default_is_not_callable(self) -> None:
        """Test that stringify raises an error if default is not callable."""
        data = {
            "key1": "value1",
            "key2": self.SomeSTupidCustomType("test"),
        }
        with pytest.raises(TypeError, match="'str' is not callable"):
            ry.stringify(data, default="poopy::not-a-callable", fmt=True)  # type: ignore[call-overload]


def test_stringify_dataclass() -> None:
    """Test that `stringify` handles dataclasses correctly."""
    from dataclasses import dataclass

    @dataclass
    class Point:
        x: int
        y: int

    data = {
        "point1": Point(1, 2),
        "point2": Point(3, 4),
    }
    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed == {
        "point1": {"x": 1, "y": 2},
        "point2": {"x": 3, "y": 4},
    }, f"Parsed JSON does not match original data: {parsed} != {data}"


@pytest.mark.skipif(
    sys.version_info < (3, 10),
    reason="dataclass(slots=True) is python3.10+ (IIRC -jesse)",
)
def test_stringify_dataclass_with_slots_kwarg() -> None:
    """Test that `stringify` handles dataclasses with slots correctly."""
    from dataclasses import dataclass

    @dataclass(slots=True)
    class Point:
        x: int
        y: int

    data = {
        "point1": Point(1, 2),
        "point2": Point(3, 4),
    }
    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed == {
        "point1": {"x": 1, "y": 2},
        "point2": {"x": 3, "y": 4},
    }, f"Parsed JSON does not match original data: {parsed} != {data}"


def test_stringify_dataclass_with_slots_manually_added() -> None:
    """Test that `stringify` handles dataclasses with slots manually added correctly."""
    from dataclasses import dataclass

    @dataclass
    class Point:
        x: int
        y: int

        __slots__ = ("x", "y")

    data = {
        "point1": Point(1, 2),
        "point2": Point(3, 4),
    }
    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed == {
        "point1": {"x": 1, "y": 2},
        "point2": {"x": 3, "y": 4},
    }, f"Parsed JSON does not match original data: {parsed} != {data}"


def test_stringify_dataclass_nested() -> None:
    """Test that `stringify` handles nested dataclasses correctly."""
    from dataclasses import dataclass

    @dataclass
    class Point:
        x: int
        y: int

    @dataclass
    class Shape:
        name: str
        point: Point

    data = {
        "shape1": Shape("circle", Point(1, 2)),
        "shape2": Shape("square", Point(3, 4)),
    }
    res = ry.stringify(data, fmt=True)
    parsed = ry.parse_json(res)
    assert isinstance(parsed, dict), "Parsed result should be a dictionary"
    assert parsed == {
        "shape1": {"name": "circle", "point": {"x": 1, "y": 2}},
        "shape2": {"name": "square", "point": {"x": 3, "y": 4}},
    }
