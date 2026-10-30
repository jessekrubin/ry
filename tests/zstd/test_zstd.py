from __future__ import annotations

import typing as t

import pytest

import ry

_ZSTD_COMPRESSION_LEVELS: tuple[
    t.Literal[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
    ],
    ...,
] = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22)


def test_compression_level_range() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    for level in _ZSTD_COMPRESSION_LEVELS:
        output_data = ry.zstd_compress(input_data, level)
        assert output_data is not None
        decoded = ry.zstd_decompress(output_data)
        assert decoded == input_data
    for bad_level in (-1, 0, 23, 24):
        with pytest.raises(
            ValueError,
            match="zstd-compression-level must be an integer between 1 and 22",
        ):
            ry.zstd_compress(input_data, bad_level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


class TestZstdVersion:
    def test_version(self) -> None:
        assert isinstance(ry.ZSTD_VERSION, str)
        assert ry.ZSTD_VERSION == ".".join(map(str, ry.ZSTD_VERSION_INFO))

    def test_version_info(self) -> None:
        assert isinstance(ry.ZSTD_VERSION_INFO, tuple)
        assert len(ry.ZSTD_VERSION_INFO) == 3
        assert all(isinstance(n, int) for n in ry.ZSTD_VERSION_INFO)
        assert ry.ZSTD_VERSION_INFO >= (1, 5, 0)


class TestZstdArgsKwargs:
    def test_level_positional_or_keyword(self) -> None:
        data = b"XXXXXXXXXXYYYYYYYYYY"
        assert ry.zstd_compress(data, 9) == ry.zstd_compress(data, level=9)

    def test_extra_positional_args(self) -> None:
        with pytest.raises(TypeError):
            ry.zstd_compress(b"data", 1, 2)  # type: ignore[call-arg]  # ty:ignore[too-many-positional-arguments]

    def test_data_is_positional_only(self) -> None:
        with pytest.raises(TypeError):
            ry.zstd_compress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.zstd_decompress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.is_zstd(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]


@pytest.mark.parametrize("level", [-5, 0, 23, 100, "snorkel", b"dingo"])
def test_compression_level_invalid(
    level: int | str | bytes | None,
) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    if isinstance(level, int):
        with pytest.raises(
            ValueError,
            match="zstd-compression-level must be an integer between 1 and 22",
        ):
            ry.zstd_compress(input_data, level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]
    else:
        with pytest.raises(TypeError):
            ry.zstd_compress(input_data, level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


def test_zstd_decompress_error() -> None:
    d = b"this is not zstd compressed data"
    with pytest.raises(ValueError, match="Unknown frame descriptor"):
        ry.zstd_decompress(d)


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.zstd_compress(input_data)
    assert isinstance(output_data, ry.Bytes)
    decoded = ry.zstd_decompress(output_data)
    assert isinstance(decoded, ry.Bytes)
    assert decoded == input_data


def test_zstd_compression_level() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.zstd_compress(input_data, 1)
    assert ry.is_zstd(output_data)
    decoded = ry.zstd_decompress(output_data)
    assert decoded == input_data

    output_data_c9 = ry.zstd_compress(input_data, 9)
    decoded = ry.zstd_decompress(output_data_c9)
    assert decoded == input_data
    assert output_data_c9 != output_data


def test_is_zstd() -> None:
    assert ry.is_zstd(ry.zstd_compress(b"data"))
    assert not ry.is_zstd(b"data")
    assert not ry.is_zstd(b"")


def test_decompress() -> None:
    _10x_10y_compressed = b"(\xb5/\xfd\x00X]\x00\x00\x18XXY\x02\x00\xa0\x08\x02`\x01"
    decoded = ry.zstd_decompress(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"
