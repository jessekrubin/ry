from __future__ import annotations

import ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_encode(input_data)
    assert output_data is not None
    decoded = ry.brotli_decode(output_data)
    assert decoded == input_data


def test_10x10y_round_trip_magic_number() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data_magic_true = ry.brotli_encode(input_data, magic_number=True)
    output_data_magic_false = ry.brotli_encode(input_data, magic_number=False)
    assert output_data_magic_false is not None and output_data_magic_true is not None
    assert output_data_magic_false != output_data_magic_true
    assert output_data_magic_true is not None
    decoded = ry.brotli_decode(output_data_magic_true)
    assert decoded == input_data
    # ends with


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02\x8a"
    decoded = ry.brotli_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"
