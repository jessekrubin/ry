from __future__ import annotations

import ry as ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_encode(input_data)
    assert output_data is not None
    decoded = ry.brotli_decode(output_data)
    assert decoded == input_data


def test_decompress():
    _10x_10y_compressed = b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02\x8a"
    decoded = ry.brotli_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"
