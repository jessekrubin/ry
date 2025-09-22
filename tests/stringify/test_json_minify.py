from __future__ import annotations

import json as pyjson
import typing as t
from dataclasses import dataclass

import pytest

from ry import JSON


@dataclass
class JsonDataTestCase:
    tid: str
    value: t.Any
    expected: bytes


def py_stringify(data: t.Any) -> str:
    """
    Convert data to a JSON string using Python's built-in json module.
    """
    return pyjson.dumps(data, indent=2, ensure_ascii=True, separators=(",", ":"))


JSON_DATA = [
    JsonDataTestCase(
        tid="simple-object",
        value={"key": "value", "number": 123, "bool": True, "null": None},
        expected=b'{"key":"value","number":123,"bool":true,"null":null}',
    ),
    JsonDataTestCase(
        tid="simple-object",
        value={"outer": {"inner": {"key": "value"}}},
        expected=b'{"outer":{"inner":{"key":"value"}}}',
    ),
    JsonDataTestCase(
        tid="simple-object",
        value=[1, 2, 3, {"key": "value"}],
        expected=b'[1,2,3,{"key":"value"}]',
    ),
    JsonDataTestCase(tid="simple-object", value={}, expected=b"{}"),
    JsonDataTestCase(tid="simple-object", value=[], expected=b"[]"),
    JsonDataTestCase(tid="simple-object", value=True, expected=b"true"),
    JsonDataTestCase(tid="simple-object", value=False, expected=b"false"),
    JsonDataTestCase(tid="simple-object", value=None, expected=b"null"),
    JsonDataTestCase(
        tid="simple-object",
        value="Hello, world! \n\t\r",
        expected=b'"Hello, world! \\n\\t\\r"',
    ),
    JsonDataTestCase(
        tid="simple-object",
        value="Unicode: ☂ ♥",
        expected=b'"Unicode: \xe2\x98\x82 \xe2\x99\xa5"',
    ),
    JsonDataTestCase(
        tid="simple-object",
        value={
            "name": "Test",
            "details": {
                "age": 30,
                "is_active": True,
                "tags": ["python", "json", "test"],
                "address": {"street": "123 Main St", "city": "Anytown", "zip": "12345"},
            },
            "scores": [95, 85, 75],
            "metadata": None,
        },
        expected=b'{"name":"Test","details":{"age":30,"is_active":true,"tags":["python","json","test"],"address":{"street":"123 Main St","city":"Anytown","zip":"12345"}},"scores":[95,85,75],"metadata":null}',
    ),
    JsonDataTestCase(
        tid="simple-object",
        value=[1, "two", 3.0, {"key": "value"}, None],
        expected=b'[1,"two",3.0,{"key":"value"},null]',
    ),
    JsonDataTestCase(tid="simple-object", value="", expected=b'""'),
    JsonDataTestCase(
        tid="simple-object", value="Hello 🌍!", expected=b'"Hello \xf0\x9f\x8c\x8d!"'
    ),
]


@pytest.mark.parametrize("tdata", JSON_DATA, ids=lambda x: x.tid)
def test_json_minify_format(tdata: JsonDataTestCase) -> None:
    """
    Test the JSON minification functionality.
    """
    json_string_indented = JSON.stringify(tdata.value, fmt=True)

    for el in (
        json_string_indented,
        bytes(json_string_indented),
        memoryview(json_string_indented),
        json_string_indented.decode("utf-8"),
    ):
        minified_json = JSON.minify(el)
        assert minified_json == tdata.expected

        # go backwards...
        for min_el in (
            minified_json,
            bytes(minified_json),
            memoryview(minified_json),
            minified_json.decode("utf-8"),
        ):
            unminified = JSON.fmt(min_el)
            assert unminified == json_string_indented
