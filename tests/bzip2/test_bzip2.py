from __future__ import annotations

import bz2
import typing as t

import pytest

import ry
from ry._types import Buffer


def test_10x10y_bzip2_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.bzip2_encode(input_data)
    output_alias = ry.bzip2(input_data)
    assert output_data == output_alias
    assert output_data is not None
    decoded = ry.bzip2_decode(output_data)
    assert decoded == input_data


def test_bzip2_compression_level() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.bzip2_encode(input_data, 1)
    assert output_data is not None
    decoded = ry.bzip2_decode(output_data)
    assert decoded == input_data

    output_data_c9 = ry.bzip2_encode(input_data, 9)
    assert output_data_c9 is not None
    decoded = ry.bzip2_decode(output_data_c9)
    assert decoded == input_data
    assert output_data_c9 != output_data


_BzipEncodeFn: t.TypeAlias = t.Callable[
    [
        Buffer,
        t.Literal[1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"],
    ],
    ry.Bytes,
]


@pytest.mark.parametrize("quality", [*range(1, 10), "best", "fast"])
@pytest.mark.parametrize("bzfn", [ry.bzip2_encode, ry.bzip2])
def test_bzip2_compression_quality(
    quality: t.Literal[1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"],
    bzfn: _BzipEncodeFn,
) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = bzfn(input_data, quality)
    assert output_data is not None
    assert isinstance(output_data, ry.Bytes)
    decoded = ry.bzip2_decode(output_data)
    assert decoded == input_data


@pytest.mark.parametrize("quality", [-1, 10, "invalid", 5.5, None])
def test_bzip2_compression_level_invalid(quality: str | float | None) -> None:
    _match = (
        "Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'"
    )
    with pytest.raises(ValueError, match=_match):
        ry.bzip2_encode(b"data", quality=quality)  # type: ignore[arg-type]
    with pytest.raises(ValueError, match=_match):
        ry.bzip2(b"data", quality=quality)  # type: ignore[arg-type]


def test_bzip2_decompress() -> None:
    _10x_10y_compressed = bz2.compress(b"XXXXXXXXXXYYYYYYYYYY")
    decoded = ry.bzip2_decode(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


def test_ry_vs_stdlib_bzip2_compress() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"

    # Compress with ry.bzip2_encode
    ry_compressed = ry.bzip2_encode(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib bz2
    stdlib_decompressed = bz2.decompress(ry_compressed)
    assert stdlib_decompressed == input_data


def test_ry_vs_stdlib_bzip2_decompress() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"

    # Compress with stdlib bz2
    stdlib_compressed = bz2.compress(input_data)
    assert stdlib_compressed is not None

    # Decompress with ry.bzip2_decode
    ry_decompressed = ry.bzip2_decode(stdlib_compressed)
    assert ry_decompressed == input_data


def test_bzip2_cross_compatibility() -> None:
    input_data = b"Cross compatibility test string."

    # Compress with ry.bzip2_encode
    ry_compressed = ry.bzip2_encode(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib bz2
    stdlib_decompressed = bz2.decompress(ry_compressed)
    assert stdlib_decompressed == input_data

    # Compress with stdlib bz2
    stdlib_compressed = bz2.compress(input_data)
    assert stdlib_compressed is not None

    # Decompress with ry.bzip2_decode
    ry_decompressed = ry.bzip2_decode(stdlib_compressed)
    assert ry_decompressed == input_data
