from __future__ import annotations

import gzip
import io
import typing as t

import pytest

import ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_compress(input_data)
    assert output_data is not None
    decoded = ry.gzip_decompress(output_data)
    assert ry.is_gzip(output_data)
    assert decoded == input_data


def test_gzip_compression_level() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_compress(input_data, level=1)
    assert output_data is not None
    decoded = ry.gzip_decompress(output_data)
    assert decoded == input_data

    output_data_c9 = ry.gzip_compress(input_data, level=9)
    assert output_data_c9 is not None
    decoded = ry.gzip_decompress(output_data_c9)
    assert decoded == input_data
    assert output_data_c9 != output_data


class TestGzipArgsKwargs:
    def test_level_positional_or_keyword(self) -> None:
        data = b"XXXXXXXXXXYYYYYYYYYY"
        assert ry.gzip_compress(data, 1) == ry.gzip_compress(data, level=1)
        assert ry.gzip_compress(data, "best") == ry.gzip_compress(data, level=9)

    def test_extra_positional_args(self) -> None:
        with pytest.raises(TypeError):
            ry.gzip_compress(b"data", 1, 2)  # type: ignore[call-arg]  # ty:ignore[too-many-positional-arguments]

    def test_data_is_positional_only(self) -> None:
        with pytest.raises(TypeError):
            ry.gzip_compress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.gzip_decompress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.is_gzip(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]

    @pytest.mark.parametrize("level", [-1, 10, "invalid", 5.5, None])
    def test_gzip_level_value_error(
        self,
        level: str | float | None,
    ) -> None:
        with pytest.raises(
            ValueError,
            match="Invalid compression level; valid levels are int 0-9 or string 'fast' or 'best'",
        ):
            _r = ry.gzip_compress(b"data", level=level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


def test_is_gzip() -> None:
    assert ry.is_gzip(ry.gzip_compress(b"data"))
    assert not ry.is_gzip(b"data")
    assert not ry.is_gzip(b"")


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\xff\x8b\x88\x80\x81H8\x00\x00\x8bS\xd8\xaf\x14\x00\x00\x00"
    decoded = ry.gzip_decompress(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


def test_ry_vs_stdlib_compress() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"

    ry_compressed = ry.gzip_compress(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib gzip
    with io.BytesIO(ry_compressed) as f, gzip.GzipFile(fileobj=f, mode="rb") as gzf:
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

    # ry.gzip_decompress the stdlib compressed data
    ry_decompressed = ry.gzip_decompress(stdlib_compressed)
    assert ry_decompressed == input_data


def test_cross_compatibility() -> None:
    input_data = b"Cross compatibility test string."

    ry_compressed = ry.gzip_compress(input_data)
    assert ry_compressed is not None

    # Decompress with stdlib gzip
    with io.BytesIO(ry_compressed) as f, gzip.GzipFile(fileobj=f, mode="rb") as gzf:
        stdlib_decompressed = gzf.read()
    assert stdlib_decompressed == input_data

    # stdlib gzip
    with io.BytesIO() as f:
        with gzip.GzipFile(fileobj=f, mode="wb") as gzf:
            gzf.write(input_data)
        stdlib_compressed = f.getvalue()
    assert stdlib_compressed is not None

    # ry
    ry_decompressed = ry.gzip_decompress(stdlib_compressed)
    assert ry_decompressed == input_data


@pytest.mark.parametrize("level", [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"])
def test_level_gzip(
    level: t.Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, "best", "fast"],
) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.gzip_compress(input_data, level=level)
    assert output_data is not None
    decoded = ry.gzip_decompress(output_data)
    assert isinstance(decoded, ry.Bytes)
    assert decoded == input_data
