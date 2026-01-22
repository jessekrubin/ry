from __future__ import annotations

import pytest

import ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_encode(input_data)
    output_data_alias = ry.brotli(input_data)
    assert output_data == output_data_alias
    assert output_data is not None
    assert isinstance(output_data, bytes)
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


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02\x8a"
    decoded = ry.brotli_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


def test_invalid_decompress() -> None:
    invalid_data = b"\x00\x01\x02\x03\x04\x05"
    with pytest.raises(ValueError, match="Invalid Data"):
        ry.brotli_decode(invalid_data)


@pytest.mark.parametrize("quality", list(range(0, 12)))
def test_quality(quality: int) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_encode(input_data, quality=quality)  # type: ignore[arg-type]
    assert output_data is not None
    decoded = ry.brotli_decode(output_data)
    assert decoded == input_data

    output_data_alias = ry.brotli(input_data, quality=quality)  # type: ignore[arg-type]
    assert output_data == output_data_alias


@pytest.mark.parametrize("quality", [-1, 12, 20])
def test_quality_out_of_range(quality: int) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    with pytest.raises(ValueError, match="Compression level must be an integer 0-11"):
        ry.brotli_encode(input_data, quality=quality)  # type: ignore[arg-type]
