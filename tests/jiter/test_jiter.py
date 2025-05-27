from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

import ry as ry


def test_parse_bytes_function() -> None:
    assert ry.parse_json_bytes(b"[true, false, null, 123, 456.7]") == [
        True,
        False,
        None,
        123,
        456.7,
    ]
    assert ry.parse_json_bytes(b'{"foo": "bar"}') == {"foo": "bar"}


def test_parse_bytes() -> None:
    assert ry.parse_json(b"[true, false, null, 123, 456.7]") == [
        True,
        False,
        None,
        123,
        456.7,
    ]
    assert ry.parse_json(b'{"foo": "bar"}') == {"foo": "bar"}


def test_parse_str() -> None:
    assert ry.parse_json("[true, false, null, 123, 456.7]") == [
        True,
        False,
        None,
        123,
        456.7,
    ]
    assert ry.parse_json('{"foo": "bar"}') == {"foo": "bar"}


@pytest.mark.parametrize(
    "input",
    [
        123,
        456.7,
        True,
        False,
        None,
        [123, 123],
        {"foo": "bar"},
    ],
)
def test_parse_json_raises_type_err(input: Any) -> None:
    with pytest.raises(TypeError):
        ry.parse_json(input)


def test_parse_from_bytesio() -> None:
    import io

    data = b"[true, false, null, 123, 456.7]"
    assert ry.parse_json(io.BytesIO(data).getbuffer()) == [
        True,
        False,
        None,
        123,
        456.7,
    ]


def _stringify(data: Any) -> str:
    return json.dumps(
        data,
        separators=(",", ":"),
    )


def _json_lines_data() -> tuple[list[dict[str, Any]], str]:
    data = [
        {
            "a": ix,
            "b": ix * 2,
            "c": ix * 3,
            "d": {
                "deee": "d" * ix,
            },
        }
        for ix in range(10)
    ]
    lines_str = "\n".join(
        map(
            _stringify,
            data,
        )
    )
    return data, lines_str


def test_read_jsonl(tmp_path: Path) -> None:
    data, json_lines_str = _json_lines_data()
    with open(tmp_path / "test.jsonl", "w") as f:
        f.write(json_lines_str)
    parse_json_l = ry.read_json(tmp_path / "test.jsonl", lines=True)
    assert parse_json_l == data


def test_parse_json_lines() -> None:
    data, json_lines_str = _json_lines_data()
    parse_json_l = ry.parse_jsonl(json_lines_str)
    assert parse_json_l == data
