from __future__ import annotations

import gzip
import io
from typing import Literal

import pytest

import ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_encode(input_data)
    assert output_data is not None
    decoded = ry.gzip_decode(output_data)
    assert decoded == input_data


def test_gzip_compression_level() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_encode(input_data, 1)
    assert output_data is not None
    decoded = ry.gzip_decode(output_data)
    assert decoded == input_data

    output_data_c9 = ry.gzip_encode(input_data, 9)
    assert output_data_c9 is not None
    decoded = ry.gzip_decode(output_data_c9)
    assert decoded == input_data
    assert output_data_c9 != output_data


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\xff\x8b\x88\x80\x81H8\x00\x00\x8bS\xd8\xaf\x14\x00\x00\x00"
    decoded = ry.gzip_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


def test_ry_vs_stdlib_compress() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"

    # Compress with ry.gzip_encode
    ry_compressed = ry.gzip_encode(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib gzip
    with io.BytesIO(ry_compressed) as f:
        with gzip.GzipFile(fileobj=f, mode="rb") as gzf:
            stdlib_decompressed = gzf.read()
    assert stdlib_decompressed == input_data


def test_ry_vs_stdlib_decompress() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"

    # Compress with stdlib gzip
    with io.BytesIO() as f:
        with gzip.GzipFile(fileobj=f, mode="wb") as gzf:
            gzf.write(input_data)
        stdlib_compressed = f.getvalue()
    assert stdlib_compressed is not None

    # ry.gzip_decode the stdlib compressed data
    ry_decompressed = ry.gzip_decode(stdlib_compressed)
    assert ry_decompressed == input_data


def test_cross_compatibility() -> None:
    input_data = b"Cross compatibility test string."

    # Compress with ry.gzip_encode
    ry_compressed = ry.gzip_encode(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib gzip
    with io.BytesIO(ry_compressed) as f:
        with gzip.GzipFile(fileobj=f, mode="rb") as gzf:
            stdlib_decompressed = gzf.read()
    assert stdlib_decompressed == input_data

    # stdlib gzip
    with io.BytesIO() as f:
        with gzip.GzipFile(fileobj=f, mode="wb") as gzf:
            gzf.write(input_data)
        stdlib_compressed = f.getvalue()
    assert stdlib_compressed is not None

    # ry
    ry_decompressed = ry.gzip_decode(stdlib_compressed)
    assert ry_decompressed == input_data


@pytest.mark.parametrize("quality", [0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
def test_quality_gzip(quality: Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_encode(input_data, quality=quality)
    assert output_data is not None
    decoded = ry.gzip_decode(output_data)
    assert isinstance(decoded, ry.Bytes)
    assert decoded == input_data


@pytest.mark.parametrize("quality", ["best", "fast"])
def test_quality_gzip_string(quality: Literal["best", "fast"]) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_encode(input_data, quality=quality)
    assert output_data is not None
    decoded = ry.gzip_decode(output_data)
    assert decoded == input_data


def test_gzip_quality_value_error() -> None:
    with pytest.raises(ValueError) as e:
        ry.gzip(b"test", quality=10)  # type: ignore[arg-type]
    s = str(e.value)
    assert (
        "Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'"
        in s
    )
