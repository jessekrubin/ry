from __future__ import annotations

import json
from typing import TYPE_CHECKING, Any

import pytest

import ry

if TYPE_CHECKING:
    from pathlib import Path


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
    "obj",
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
def test_parse_json_raises_type_err(obj: Any) -> None:
    with pytest.raises(TypeError):
        ry.parse_json(obj)


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


def test_parse_bytes_in_numpy_u8_arr() -> None:
    """Test that reading bytes from a buffer-protocol obj (eg numpy uint8 array works)"""
    try:
        import numpy as np

    except ImportError:
        pytest.skip("Numpy is not available")

    json_bytes = b"[1, 2, 3]"
    arr = np.frombuffer(json_bytes, dtype=np.uint8)
    parsed_arr = ry.parse_json(arr)  # type: ignore[arg-type,unused-ignore]
    assert isinstance(parsed_arr, list), "Parsed result should be a list"
    assert parsed_arr == [1, 2, 3], "Parsed array does not match original"


def test_parse_massive_ints() -> None:
    expected = [
        1134_771_341_237_886_746_928_399_281_379_818_916_646_777_378_528_823_008_040_502,
        1075_006_011_535_502_328_635_476_185_316_520_074_006_917_254_747_885_836_283_199,
    ]
    b = b"[1134771341237886746928399281379818916646777378528823008040502, 1075006011535502328635476185316520074006917254747885836283199]"
    result = ry.parse_json(b)
    assert result == expected


class TestJSONDecodeError:
    def test_is_value_error(self) -> None:
        assert issubclass(ry.JSONDecodeError, ValueError)
        assert ry.JSON.JSONDecodeError is ry.JSONDecodeError

    def test_not_constructable(self) -> None:
        with pytest.raises(NotImplementedError):
            ry.JSONDecodeError("msg", None, 0)

    @pytest.mark.parametrize("data", ['{"a": 1,\n "b": 2,}', b'{"a": 1,\n "b": 2,}'])
    def test_attrs(self, data: str | bytes) -> None:
        with pytest.raises(ry.JSONDecodeError) as exc_info:
            ry.parse_json(data)
        e = exc_info.value
        assert str(e) == "trailing comma: line 2 column 9 (char 17)"
        assert e.msg == "trailing comma"
        assert e.kind == "trailing-comma"
        assert e.pos == 17
        assert e.lineno == 2
        assert e.colno == 9
        assert e.doc == ry.Bytes(b'{"a": 1,\n "b": 2,}')

    def test_pos_is_byte_index(self) -> None:
        with pytest.raises(ry.JSONDecodeError) as exc_info:
            ry.parse_json('{"\u00e9": 1,}')
        # 'é' is 2 bytes in utf-8
        assert exc_info.value.pos == 9
        assert exc_info.value.colno == 10

    def test_parse_jsonl_position_is_relative_to_document(self) -> None:
        data = '{"a": 1}\n\n{"b": 2}\n{"c": 3,}\n'
        with pytest.raises(ry.JSONDecodeError) as exc_info:
            ry.parse_jsonl(data)
        e = exc_info.value
        assert e.doc == ry.Bytes(data.encode())
        assert e.lineno == 4
        assert e.colno == 9
        assert e.pos == 26
