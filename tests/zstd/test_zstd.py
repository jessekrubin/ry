from __future__ import annotations

import subprocess as sp

import pytest

import ry

ZSTD_COMPRESSION_FUNCTIONS = [
    ry.zstd_compress,
    ry.zstd_encode,
    ry.zstd,
    ry.zstd.compress,
    ry.zstd_compress,
]


def test_compression_level_range() -> None:
    input_data = b"XXXXXXXXXXYYYYYYYYYY"
    for level in range(1, 23):  # max level is 22
        output_data = ry.zstd_encode(input_data, level)
        assert output_data is not None
        decoded = ry.zstd_decode(output_data)
        assert decoded == input_data
    for level in (-1, 0, 23, 24):
        with pytest.raises(ValueError):
            ry.zstd_encode(input_data, level)


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
