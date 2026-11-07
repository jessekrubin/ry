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


def test_10x10y_round_trip_magic_number() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data_magic_true = ry.brotli_compress(input_data, magic_number=True)
    output_data_magic_false = ry.brotli_compress(input_data, magic_number=False)
    assert output_data_magic_false is not None
    assert output_data_magic_true is not None
    assert output_data_magic_false != output_data_magic_true
    decoded = ry.brotli_decompress(output_data_magic_true)
    assert decoded == input_data


def test_decompress() -> None:
    _10x_10y_compressed = b"\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81G\x02\x8a"
    decoded = ry.brotli_decompress(_10x_10y_compressed)
    assert decoded == b"XXXXXXXXXXYYYYYYYYYY"


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
            ry.brotli_compress(b"data", 1, 2)  # type: ignore[call-arg,arg-type]  # ty:ignore[too-many-positional-arguments]

    def test_magic_number_is_keyword_only(self) -> None:
        data = b"XXXXXXXXXXYYYYYYYYYY"
        assert ry.brotli_compress(data, 11, magic_number=True) != ry.brotli_compress(
            data, 11
        )

    def test_data_is_positional_only(self) -> None:
        with pytest.raises(TypeError):
            ry.brotli_compress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]
        with pytest.raises(TypeError):
            ry.brotli_decompress(data=b"data")  # type: ignore[call-arg]  # ty:ignore[positional-only-parameter-as-kwarg]

    @pytest.mark.parametrize("level", [-1, 12, 20, "best", 5.5, None])
    def test_level_out_of_range(self, level: int) -> None:
        with pytest.raises(
            ValueError, match="Compression level must be an integer 0-11"
        ):
            ry.brotli_compress(b"XXXXXXXXXXYYYYYYYYYY", level=level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]


@pytest.mark.parametrize("level", list(range(12)))
def test_level(level: int) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    output_data = ry.brotli_compress(input_data, level=level)  # type: ignore[arg-type]  # ty:ignore[invalid-argument-type]
    assert output_data is not None
    decoded = ry.brotli_decompress(output_data)
    assert decoded == input_data
