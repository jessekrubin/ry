from __future__ import annotations

import subprocess as sp
import typing as t

import pytest

import ry

ZSTD_COMPRESSION_FUNCTIONS = [
    ry.zstd_compress,
    ry.zstd_encode,
    ry.zstd,
    ry.zstd.compress,
    ry.zstd_compress,
]

_ZSTD_COMPRESSION_LEVELS: tuple[
    t.Literal[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
    ],
    ...,
] = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22)


def test_compression_level_range() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    for level in _ZSTD_COMPRESSION_LEVELS:
        output_data = ry.zstd_encode(input_data, level)
        assert output_data is not None
        decoded = ry.zstd_decode(output_data)
        assert decoded == input_data
    for bad_level in (-1, 0, 23, 24):
        with pytest.raises(ValueError):
            ry.zstd_encode(input_data, bad_level)  # type: ignore[arg-type]


@pytest.mark.parametrize("level", [-5, 0, 23, 100, "snorkel", b"dingo"])
def test_compression_level_invalid(
    level: int | str | bytes | None,
) -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    if isinstance(level, int):
        with pytest.raises(ValueError):
            ry.zstd_encode(input_data, level)  # type: ignore[arg-type]
    else:
        with pytest.raises(TypeError):
            ry.zstd_encode(input_data, level)  # type: ignore[arg-type]


def test_zstd_decode_error() -> None:
    d = b"this is not zstd compressed data"
    with pytest.raises(ValueError):
        ry.zstd_decode(d)


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
    assert ry.is_zstd(output_data)
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


@pytest.mark.parametrize(
    "py_expr",
    [
        "from ry import zstd",
        "import ry.zstd",
        "from ry.ryo3 import zstd",
    ],
)
def test_module_import(
    py_expr: str,
) -> None:
    import sys

    python_exe = sys.executable
    result = sp.run(
        [python_exe, "-c", py_expr],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, result.stderr
