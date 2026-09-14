import typing as t

import ry
from ry import JSON

json_str_min = '{"key":"value","number":123,"bool":true}'
json_str_fmt = '{\n    "key": "value",\n    "number": 123,\n    "bool": true\n}'


def test_json_fmt_minify() -> None:
    py_str = JSON.fmt(json_str_min)
    t.assert_type(JSON.fmt(json_str_min), str)
    assert isinstance(py_str, str)
    py_bytes = JSON.fmt(json_str_min.encode("utf-8"))
    t.assert_type(JSON.fmt(json_str_min.encode("utf-8")), bytes)
    assert isinstance(py_bytes, bytes)
    py_bytes_buf = JSON.fmt(ry.Bytes(json_str_min.encode("utf-8")))
    assert isinstance(py_bytes_buf, ry.Bytes)

    t.assert_type(JSON.fmt(ry.Bytes(json_str_min.encode("utf-8"))), ry.Bytes)

    # bytearray should fall back to buffer?
    py_bytes_barray = JSON.fmt(bytearray(json_str_min.encode("utf-8")))
    assert isinstance(py_bytes_barray, ry.Bytes)

    t.assert_type(JSON.fmt(bytearray(json_str_min.encode("utf-8"))), ry.Bytes)
