from __future__ import annotations

import pytest

import ry


def test_10x10y_round_trip() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_compress(input_data)
    assert output_data is not None
    assert isinstance(output_data, ry.Bytes)
    decoded = ry.brotli_decompress(output_data)
    assert isinstance(decoded, ry.Bytes)
    assert decoded == input_data


def test_default_level() -> None:
    data = b"XXXXXXXXXXYYYYYYYYYY" * 100
    assert ry.brotli_compress(data) == ry.brotli_compress(data, level=5)


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02\x8a"
    decoded = ry.brotli_decompress(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


@pytest.mark.parametrize(
    "encoded",
    [
        # Produced by the previous brotli encoder at levels 6-9 and 10-11,
        # respectively, both without and with its nonstandard magic header.
        b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02I",
        b"k\x11\x00\xe1\x97\x80\x01\x141\x01\x00@\n+\xab\x1ex$\x90\x04",
        b"\x1b\x13\x00\xf8\xa5\xb0\xb2\x82\x84\x00\x00\x1e",
        b"k\x11\x00\xe1\x97\x80\x01\x141\x01\x80_\n++H\x08\x00\xe0\x01",
    ],
    ids=["level6-9", "level6-9-magic", "level10-11", "level10-11-magic"],
)
def test_decompress_legacy_streams(encoded: bytes) -> None:
    assert ry.brotli_decompress(encoded) == b"XXXXXXXXXXYYYYYYYYYY"


def test_invalid_decompress() -> None:
    invalid_data = b"\x00\x01\x02\x03\x04\x05"
    with pytest.raises(ValueError, match="Invalid Data"):
        ry.brotli_decompress(invalid_data)


class TestBrotliArgsKwargs:
    def test_level_positional_or_keyword(self) -> None:
        data = b"XXXXXXXXXXYYYYYYYYYY"
        assert ry.brotli_compress(data, 1) == ry.brotli_compress(data, level=1)

    def test_extra_positional_args(self) -> None:
        with pytest.raises(TypeError):
            ry.brotli_compress(b"data", 1, 2)  # type: ignore[call-arg]  # ty:ignore[too-many-positional-arguments]

    @pytest.mark.parametrize("magic_number", [False, True])
    def test_magic_number_removed(self, *, magic_number: bool) -> None:
        with pytest.raises(TypeError, match="magic_number"):
            ry.brotli_compress(b"data", magic_number=magic_number)  # type: ignore[call-arg]  # ty:ignore[unknown-argument]

    def test_data_is_positional_only(self) -> None:
        with pytest.raises(TypeError):
            ry.brotli_compress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.brotli_decompress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]

    @pytest.mark.parametrize("level", [-1, *range(6, 12), 12, 20, "best", 5.5, None])
    def test_level_out_of_range(self, level: int) -> None:
        with pytest.raises(
            ValueError, match="Compression level must be an integer 0-5"
        ):
            ry.brotli_compress(b"XXXXXXXXXXYYYYYYYYYY", level=level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


@pytest.mark.parametrize("level", list(range(6)))
@pytest.mark.parametrize(
    "input_data",
    [b"", b"XXXXXXXXXXYYYYYYYYYY", bytes(range(256)), b"hello world\n" * 100_000],
    ids=["empty", "text", "binary", "large"],
)
def test_level(level: int, input_data: bytes) -> None:
    output_data = ry.brotli_compress(input_data, level=level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]
    assert output_data is not None
    decoded = ry.brotli_decompress(output_data)
    assert decoded == input_data


def test_buffer_inputs() -> None:
    data = b"XXXXXXXXXXYYYYYYYYYY"
    for buffer in (data, ry.Bytes(data), bytearray(data), memoryview(data)):
        encoded = ry.brotli_compress(buffer)
        assert isinstance(encoded, ry.Bytes)
        for encoded_buffer in (
            bytes(encoded),
            encoded,
            bytearray(encoded),
            memoryview(encoded),
        ):
            assert ry.brotli_decompress(encoded_buffer) == data
