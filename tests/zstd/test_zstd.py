from __future__ import annotations

import ry as ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.zstd_encode(input_data)
    assert output_data is not None
    decoded = ry.zstd_decode(output_data)
    assert decoded == input_data


def test_zstd_compression_level() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.zstd_encode(input_data, 1)
    assert output_data is not None
    decoded = ry.zstd_decode(output_data)
    assert decoded == input_data

    output_data_c9 = ry.zstd_encode(input_data, 9)
    assert output_data_c9 is not None
    decoded = ry.zstd_decode(output_data_c9)
    assert decoded == input_data
    assert output_data_c9 != output_data


def test_decompress() -> None:
    _10x_10y_compressed = b"(\xb5/\xfd\x00X]\x00\x00\x18XXY\x02\x00\xa0\x08\x02`\x01"
    decoded = ry.zstd_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"
