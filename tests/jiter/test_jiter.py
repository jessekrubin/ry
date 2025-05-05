from __future__ import annotations

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


# def test_parse_json_lines():
#     data = [
#         {
#             "a": ix,
#             "b": ix * 2,
#             "c": ix * 3,
#             "d": {
#                 "deee": "d" * ix,
#             },
#         }
#         for ix in range(10)
#     ]
#     print(data)
#     json_lines_str = "\n".join(
#         [
#             json.dumps(
#                 item,
#                 separators=(",", ":"),
#             )
#             for item in data
#         ]
#     )
#     print(json_lines_str)
#     parse_json_l = ry.parse_jsonl(json_lines_str)
#     print(parse_json_l)
#     assert False
